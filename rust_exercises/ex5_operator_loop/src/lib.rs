//! The control loop: every operator, every cycle, in a fixed order.
//!
//! The ground software does not have one big function that knows about
//! sequences and aborts and everything else. It has a list of *operators*,
//! each responsible for one job, and a loop that runs every one of them once
//! per control cycle against shared state:
//!
//! ```text
//! cycle N:  sequence operator  ->  abort operator  ->  (flush outputs to hardware)
//!                writes "open valve 4"     writes "close valve 4"
//!                                          ^ runs last, so it wins
//! ```
//!
//! Two rules make that safe:
//!
//! 1. **Safety operators run last.** When a sequence and an abort both set the
//!    same valve in the same cycle, the abort's write is the one that sticks.
//! 2. **One operator failing does not stop the others.** If the sequence hits
//!    an error, the abort still has to run that cycle.
//!
//! # Rust notes for this file
//!
//! - `Operator` is a *trait*: a list of methods a type promises to have. The
//!   loop does not know or care which concrete operators it holds, only that
//!   each one implements `Operator`.
//! - `Box<dyn Operator>` means "some type that implements `Operator`, stored on
//!   the heap". It is how one `Vec` can hold different operator types.
//! - `priority` has a *default implementation* in the trait. Operators that do
//!   not override it get `Priority::Normal`.
//! - `#[derive(PartialOrd, Ord)]` on an enum orders the variants in the order
//!   they are declared, so `Priority::Normal < Priority::Safety`.

use std::collections::HashMap;

/// Identifies a physical channel on the stand.
pub type ChannelId = u16;

/// Discrete output state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalState {
    Lo,
    Hi,
}

/// Something went wrong inside one operator this cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorError {
    /// The operator needed a reading for this channel and there was none.
    MissingReading(ChannelId),
}

/// When an operator runs within a cycle. Lower runs first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Normal,
    Safety,
}

/// Everything operators share during one cycle.
#[derive(Debug, Clone, Default)]
pub struct LoopContext {
    /// Which cycle this is, counting from 0.
    pub cycle: u64,
    /// This cycle's sensor readings.
    pub readings: HashMap<ChannelId, f64>,
    /// Output states to send to hardware. If two operators set the same
    /// channel, the later write replaces the earlier one.
    pub outputs: HashMap<ChannelId, DigitalState>,
}

/// One job the control loop runs every cycle.
pub trait Operator {
    /// A short human-readable name, for logs.
    fn name(&self) -> &str;

    /// When this operator runs within a cycle. Most operators are `Normal`.
    fn priority(&self) -> Priority {
        Priority::Normal
    }

    /// Does this operator's work for one cycle.
    ///
    /// # Errors
    ///
    /// Any [`OperatorError`] the operator hits. It is reported, not fatal.
    fn run(&mut self, context: &mut LoopContext) -> Result<(), OperatorError>;
}

/// A finished example of an operator: sets outputs on scheduled cycles. A tiny
/// stand-in for the real sequence operator.
pub struct ScheduledOutputs {
    /// `(cycle, channel, state)`: on `cycle`, set `channel` to `state`.
    schedule: Vec<(u64, ChannelId, DigitalState)>,
}

impl ScheduledOutputs {
    pub fn new(schedule: Vec<(u64, ChannelId, DigitalState)>) -> Self {
        Self { schedule }
    }
}

impl Operator for ScheduledOutputs {
    fn name(&self) -> &str {
        "scheduled outputs"
    }

    fn run(&mut self, context: &mut LoopContext) -> Result<(), OperatorError> {
        for &(cycle, id, state) in &self.schedule {
            if cycle == context.cycle {
                context.outputs.insert(id, state);
            }
        }
        Ok(())
    }
}

/// Trips when one channel reads above a limit, then drives outputs to a safe
/// state. A cut-down version of the abort operator from exercise 2.
pub struct ThresholdAbort {
    watch: ChannelId,
    limit: f64,
    safe_outputs: Vec<(ChannelId, DigitalState)>,
    tripped: bool,
}

impl ThresholdAbort {
    pub fn new(watch: ChannelId, limit: f64, safe_outputs: Vec<(ChannelId, DigitalState)>) -> Self {
        Self {
            watch,
            limit,
            safe_outputs,
            tripped: false,
        }
    }

    pub fn is_tripped(&self) -> bool {
        self.tripped
    }
}

impl Operator for ThresholdAbort {
    fn name(&self) -> &str {
        "threshold abort"
    }

    fn priority(&self) -> Priority {
        Priority::Safety
    }

    /// - No reading for `watch` this cycle: return
    ///   `OperatorError::MissingReading(watch)` and change nothing.
    /// - Reading strictly above `limit`: trip.
    /// - Once tripped, stay tripped, even if the reading drops back down. An
    ///   abort that un-trips the moment pressure dips would hand the valves
    ///   straight back to the sequence that caused the problem.
    /// - While tripped, write every `safe_outputs` entry into
    ///   `context.outputs`, every cycle.
    ///
    /// TODO(you): implement.
    fn run(&mut self, context: &mut LoopContext) -> Result<(), OperatorError> {
        // This line only exists so the unimplemented stub compiles without
        // "unused" errors. Delete it when you implement the function.
        let _ = (context, self.watch, self.limit, &self.safe_outputs);
        todo!("ex5: implement ThresholdAbort::run")
    }
}

/// Runs every operator, once per cycle.
#[derive(Default)]
pub struct ControlLoop {
    operators: Vec<Box<dyn Operator>>,
}

impl ControlLoop {
    /// Adds an operator.
    ///
    /// After every `add`, operators must be in run order: every `Normal`
    /// operator before every `Safety` one, no matter what order they were
    /// added in. Operators with the same priority keep the order they were
    /// added, so a sequence added before another sequence still runs first.
    ///
    /// TODO(you): implement.
    pub fn add(&mut self, operator: Box<dyn Operator>) {
        // This line only exists so the unimplemented stub compiles without
        // "unused" errors. Delete it when you implement the function.
        let _ = operator;
        todo!("ex5: implement ControlLoop::add")
    }

    /// The operators' names, in the order they run.
    pub fn operator_names(&self) -> Vec<&str> {
        self.operators
            .iter()
            .map(|operator| operator.name())
            .collect()
    }

    /// Runs one cycle: every operator in order against `context`, then
    /// advances `context.cycle` by one.
    ///
    /// An operator that returns an error does NOT stop the cycle. Keep going,
    /// and return every error from this cycle, in the order they happened. An
    /// empty `Vec` means a clean cycle.
    ///
    /// TODO(you): implement.
    pub fn run_cycle(&mut self, context: &mut LoopContext) -> Vec<OperatorError> {
        // This line only exists so the unimplemented stub compiles without
        // "unused" errors. Delete it when you implement the function.
        let _ = context;
        todo!("ex5: implement ControlLoop::run_cycle")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const PRESSURE: ChannelId = 1;
    const MAIN_VALVE: ChannelId = 4;

    /// Helper, not a test. Trips above 500 and closes the main valve.
    fn pressure_abort() -> ThresholdAbort {
        ThresholdAbort::new(PRESSURE, 500.0, vec![(MAIN_VALVE, DigitalState::Lo)])
    }

    /// Helper, not a test. A cycle-0 context with one pressure reading.
    fn context_reading(pressure: f64) -> LoopContext {
        let mut context = LoopContext::default();
        context.readings.insert(PRESSURE, pressure);
        context
    }

    #[rstest]
    fn test_add_runs_safety_operators_last_whatever_order_they_were_added() {
        let mut control_loop = ControlLoop::default();

        control_loop.add(Box::new(pressure_abort()));
        control_loop.add(Box::new(ScheduledOutputs::new(Vec::new())));

        assert_eq!(
            control_loop.operator_names(),
            vec!["scheduled outputs", "threshold abort"]
        );
    }

    #[rstest]
    fn test_run_cycle_applies_scheduled_outputs_on_their_cycle() {
        let mut control_loop = ControlLoop::default();
        control_loop.add(Box::new(ScheduledOutputs::new(vec![(
            1,
            MAIN_VALVE,
            DigitalState::Hi,
        )])));
        let mut context = context_reading(0.0);
        control_loop.run_cycle(&mut context);

        let errors = control_loop.run_cycle(&mut context);

        assert!(errors.is_empty());
        assert_eq!(context.outputs.get(&MAIN_VALVE), Some(&DigitalState::Hi));
        assert_eq!(context.cycle, 2);
    }

    // ------------------------------------------------------------------
    // TODO(you): Write these, in the same style as the tests above:
    //
    //   1. `ThresholdAbort::run` returns `MissingReading` when the watched
    //      channel has no reading.
    //   2. `ThresholdAbort::run` trips only when the reading is above the
    //      limit. Use `#[case]` for just below, exactly at, and just above 500.
    //   3. `ThresholdAbort::run` keeps writing its safe outputs after the
    //      reading drops back below the limit.
    //   4. `run_cycle` lets the abort override the sequence when both set the
    //      same valve in the same cycle. `add` the abort FIRST, so the test
    //      proves the order comes from priority, not from the order of `add`.
    //   5. `run_cycle` still runs the abort when an earlier operator fails,
    //      and returns that operator's error. For this you need an operator
    //      that always fails: write a small struct here in the tests module
    //      and `impl Operator` for it. That is completely normal.
    //
    // Sabotage check: make `run_cycle` `break` out of its loop at the first
    // error. Test 5 should fail. If it does not, look at it again.
    // ------------------------------------------------------------------
}
