//! Debounced abort evaluation.
//!
//! This is a cut-down model of the real ground software abort operator: same
//! problem, same structure, none of the expression engine.
//!
//! # The problem
//!
//! An abort watches a condition over sensor channels - say "tank pressure
//! above the burst disc rating" - and when it trips, it drives valves to a
//! safe state. Sensors are noisy, so a single bad sample must not vent the
//! stand. Each abort therefore carries a `min_cycles`: the condition has to
//! hold for that many *consecutive* control cycles before anything fires.
//!
//! Consecutive is the entire point. One false cycle in the middle means the
//! condition did not hold continuously, and the count starts over.
//!
//! # Rust notes
//!
//! - `HashMap<K, V>` iteration order is unspecified. Tests that depend on it
//!   are flaky tests. The tests below use a single abort config for that
//!   reason.
//! - `run` collects the ids to fire into a `Vec` before touching the write
//!   queue. That is not a stylistic choice: you cannot hold an immutable
//!   borrow of `context.abort_configs` and a mutable borrow of
//!   `context.write_queue` through the same `&mut context` at once. Splitting
//!   the read phase from the write phase is the normal way out.
//! - `?` propagates an error to the caller. `evaluate` can fail, so `run`
//!   returns `Result` too.

use std::collections::HashMap;

/// Identifies a physical channel on the stand.
pub type ChannelId = u16;

/// Identifies one configured abort.
pub type AbortId = u16;

/// Something went wrong evaluating an abort.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbortError {
    /// A condition referenced a channel that has no reading this cycle. We do
    /// not guess, and we do not treat a missing reading as "condition false":
    /// a dead sensor feeding a safety interlock is itself a problem.
    MissingReading(ChannelId),
}

/// How two channels are compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    GreaterThan,
    LessThan,
}

/// "channel `lhs` <comparison> channel `rhs`".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Condition {
    comparison: Comparison,
    lhs: ChannelId,
    rhs: ChannelId,
}

impl Condition {
    pub fn new(comparison: Comparison, lhs: ChannelId, rhs: ChannelId) -> Self {
        Self {
            comparison,
            lhs,
            rhs,
        }
    }

    /// Evaluates the condition against this cycle's readings.
    ///
    /// # Errors
    ///
    /// [`AbortError::MissingReading`] if either channel is absent.
    pub fn evaluate(&self, readings: &HashMap<ChannelId, f64>) -> Result<bool, AbortError> {
        let lhs = *readings
            .get(&self.lhs)
            .ok_or(AbortError::MissingReading(self.lhs))?;
        let rhs = *readings
            .get(&self.rhs)
            .ok_or(AbortError::MissingReading(self.rhs))?;

        Ok(match self.comparison {
            Comparison::GreaterThan => lhs > rhs,
            Comparison::LessThan => lhs < rhs,
        })
    }
}

/// A setpoint for an analog output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalogCommand {
    pub id: ChannelId,
    pub value: u16,
}

/// Discrete output state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalState {
    Lo,
    Hi,
}

/// A command for a digital output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitalCommand {
    pub id: ChannelId,
    pub state: DigitalState,
}

/// One abort: what to watch, what to do about it, and how long to wait.
#[derive(Debug, Clone, PartialEq)]
pub struct AbortConfig {
    condition: Condition,
    analog_outputs: Vec<AnalogCommand>,
    digital_outputs: Vec<DigitalCommand>,
    min_cycles: u16,
}

impl AbortConfig {
    /// A `min_cycles` of 0 is treated as 1. Firing on zero consecutive true
    /// cycles would mean firing unconditionally.
    pub fn new(
        condition: Condition,
        analog_outputs: Vec<AnalogCommand>,
        digital_outputs: Vec<DigitalCommand>,
        min_cycles: u16,
    ) -> Self {
        Self {
            condition,
            analog_outputs,
            digital_outputs,
            min_cycles: min_cycles.max(1),
        }
    }
}

/// Everything the operator writes out this cycle.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CompositeCommand {
    pub analog_outputs: Vec<AnalogCommand>,
    pub digital_outputs: Vec<DigitalCommand>,
}

/// Accumulates commands until the control loop flushes them to hardware.
#[derive(Debug, Clone, Default)]
pub struct DeviceWriteQueue {
    analog_outputs: Vec<AnalogCommand>,
    digital_outputs: Vec<DigitalCommand>,
}

impl DeviceWriteQueue {
    /// Drains the queue into a single command. An empty queue yields an empty
    /// command rather than `None`, so callers have one code path.
    pub fn flush_to_composite_command(&mut self) -> CompositeCommand {
        CompositeCommand {
            analog_outputs: std::mem::take(&mut self.analog_outputs),
            digital_outputs: std::mem::take(&mut self.digital_outputs),
        }
    }
}

/// The slice of control-loop state an abort needs. The real thing carries a
/// hardware interface and much more.
#[derive(Debug, Clone, Default)]
pub struct LoopContext {
    analog_readings: HashMap<ChannelId, f64>,
    abort_configs: HashMap<AbortId, AbortConfig>,
    write_queue: DeviceWriteQueue,
}

impl LoopContext {
    pub fn analog_readings_mut(&mut self) -> &mut HashMap<ChannelId, f64> {
        &mut self.analog_readings
    }

    pub fn abort_configs_mut(&mut self) -> &mut HashMap<AbortId, AbortConfig> {
        &mut self.abort_configs
    }

    pub fn device_write_queue(&mut self) -> &mut DeviceWriteQueue {
        &mut self.write_queue
    }
}

/// Evaluates every configured abort once per control cycle.
///
/// Tracks, per abort, how many consecutive cycles its condition has held.
#[derive(Debug, Clone, Default)]
pub struct AbortOperator {
    consecutive_true_cycles: HashMap<AbortId, u16>,
}

impl AbortOperator {
    /// Runs one control cycle.
    ///
    /// While an abort is tripped it re-issues its outputs every cycle. Valves
    /// can be commanded elsewhere in the same loop, so an abort that fires
    /// once and goes quiet could be overridden a cycle later.
    ///
    /// # Errors
    ///
    /// Propagates [`AbortError`] from condition evaluation.
    pub fn run(&mut self, context: &mut LoopContext) -> Result<(), AbortError> {
        let mut aborts_to_fire: Vec<AbortId> = Vec::new();

        for (abort_id, config) in &context.abort_configs {
            let condition_met = config.condition.evaluate(&context.analog_readings)?;
            let cycles = self.consecutive_true_cycles.entry(*abort_id).or_insert(0);

            if condition_met {
                *cycles += 1;
            }

            if *cycles >= config.min_cycles {
                aborts_to_fire.push(*abort_id);
            }
        }

        for abort_id in aborts_to_fire {
            let Some(config) = context.abort_configs.get(&abort_id) else {
                continue;
            };
            context
                .write_queue
                .analog_outputs
                .extend_from_slice(&config.analog_outputs);
            context
                .write_queue
                .digital_outputs
                .extend_from_slice(&config.digital_outputs);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    /// Channel that reads high, and the channel it gets compared against.
    const HIGH_CHANNEL: ChannelId = 1;
    const LOW_CHANNEL: ChannelId = 0;
    const ABORT_ID: AbortId = 1;

    /// An rstest *fixture*: any test that takes a `context: LoopContext`
    /// parameter gets a freshly built one. This is how the real abort tests in
    /// trel3 get their `LoopActionContext`, and it beats copy-pasting setup
    /// into fifteen tests.
    ///
    /// Readings are placed so that `HIGH_CHANNEL > LOW_CHANNEL`. Tests flip
    /// the *condition* between GreaterThan and LessThan to make it true or
    /// false, rather than rewriting the readings.
    #[fixture]
    fn context() -> LoopContext {
        let mut context = LoopContext::default();
        context.analog_readings_mut().insert(HIGH_CHANNEL, 2.0);
        context.analog_readings_mut().insert(LOW_CHANNEL, 1.0);
        context
    }

    /// Helper, not a test. Private, lives in the tests module, and does no
    /// asserting - it only builds data.
    fn config_with(comparison: Comparison, min_cycles: u16) -> AbortConfig {
        AbortConfig::new(
            Condition::new(comparison, HIGH_CHANNEL, LOW_CHANNEL),
            vec![AnalogCommand { id: 5, value: 123 }],
            vec![DigitalCommand {
                id: 7,
                state: DigitalState::Hi,
            }],
            min_cycles,
        )
    }

    #[rstest]
    #[case(2)]
    #[case(5)]
    #[case(12)]
    fn test_run_issues_outputs_after_min_cycles_of_continuous_truth(
        #[case] min_cycles: u16,
        mut context: LoopContext,
    ) {
        context
            .abort_configs_mut()
            .insert(ABORT_ID, config_with(Comparison::GreaterThan, min_cycles));
        let mut operator = AbortOperator::default();

        for _ in 0..min_cycles {
            operator.run(&mut context).expect("run should succeed");
        }

        let command = context.device_write_queue().flush_to_composite_command();
        assert_eq!(
            command.analog_outputs,
            vec![AnalogCommand { id: 5, value: 123 }]
        );
        assert_eq!(
            command.digital_outputs,
            vec![DigitalCommand {
                id: 7,
                state: DigitalState::Hi
            }]
        );
    }

    #[rstest]
    #[case(2)]
    #[case(5)]
    #[case(12)]
    fn test_run_issues_no_outputs_before_min_cycles(
        #[case] min_cycles: u16,
        mut context: LoopContext,
    ) {
        context
            .abort_configs_mut()
            .insert(ABORT_ID, config_with(Comparison::GreaterThan, min_cycles));
        let mut operator = AbortOperator::default();

        for _ in 0..min_cycles - 1 {
            operator.run(&mut context).expect("run should succeed");
        }

        let command = context.device_write_queue().flush_to_composite_command();
        assert!(command.analog_outputs.is_empty());
        assert!(command.digital_outputs.is_empty());
    }

    #[rstest]
    fn test_run_returns_error_when_a_referenced_channel_has_no_reading(mut context: LoopContext) {
        context.analog_readings_mut().remove(&HIGH_CHANNEL);
        context
            .abort_configs_mut()
            .insert(ABORT_ID, config_with(Comparison::GreaterThan, 2));
        let mut operator = AbortOperator::default();

        let result = operator.run(&mut context);

        assert_eq!(result, Err(AbortError::MissingReading(HIGH_CHANNEL)));
    }

    // ------------------------------------------------------------------
    // TODO(you): the tests above pass. They also miss a real bug.
    //
    // Everything above tests a condition that is continuously true or
    // continuously false. Nothing tests the case the whole feature exists
    // for: the condition goes true, then false, then true again. Consecutive
    // means consecutive.
    //
    // Write:
    //
    //   test_run_issues_no_outputs_after_discontinuous_min_cycles
    //
    //   - #[case] over min_cycles 2, 5, and 12, plus the `context` fixture
    //   - run one cycle with a GreaterThan condition (true)
    //   - swap the stored AbortConfig to LessThan and run one cycle (false)
    //   - swap it back to GreaterThan and run min_cycles - 1 more cycles
    //   - assert the write queue is empty
    //
    // To swap the config mid-test, reach into the map:
    //
    //     if let Some(config) = context.abort_configs_mut().get_mut(&ABORT_ID) {
    //         *config = config_with(Comparison::LessThan, min_cycles);
    //     }
    //
    // That test will FAIL against the code in this file. The bug is real and
    // it is the kind that ships: the happy paths are covered, the tests are
    // green, and the debounce does not actually debounce.
    //
    // Do it in this order, as two commits in one PR:
    //
    //   1. Add the failing test. Run it. Read the failure and make sure you
    //      understand what the code did wrong before you touch it.
    //   2. Fix `AbortOperator::run`. Re-run the full suite - all four tests,
    //      not just yours.
    //
    // The two-commit sequence is not ceremony. It is the proof, visible in
    // your PR diff, that the test actually catches the bug. A test written
    // after the fix passes on the first run and demonstrates nothing.
    //
    // The real ground software has a test with very nearly this name, against
    // a full expression engine. You will recognize it when you get there.
    // ------------------------------------------------------------------
}
