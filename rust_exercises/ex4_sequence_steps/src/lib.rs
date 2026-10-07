//! A sequence: commands issued in order, on a schedule.
//!
//! A hotfire is mostly a script. Open the purge valve, wait, open the main
//! valves, wait, ignite, wait, shut everything. The ground software runs that
//! script as a sequence of steps, advanced one control cycle at a time:
//!
//! ```text
//! step 0: [OpenValve(3)]                  hold for 2 cycles
//! step 1: [OpenValve(5), OpenValve(6)]    hold for 1 cycle
//!
//! cycle 1: issues [OpenValve(3)]                 (step 0, cycle 1 of 2)
//! cycle 2: issues nothing                        (step 0, cycle 2 of 2)
//! cycle 3: issues [OpenValve(5), OpenValve(6)]   (step 1, cycle 1 of 1)
//!          -> Complete
//! cycle 4: issues nothing, ever again
//! ```
//!
//! A step's commands go out once, on the first cycle of that step. The control
//! loop around this holds outputs where they were last set, so there is no
//! need to repeat them.
//!
//! # Rust notes for this file
//!
//! - `SequenceState::Running` carries data (which step, and for how long).
//!   That data only exists while running, so it lives inside the variant
//!   rather than as loose fields that are meaningless the rest of the time.
//! - `SequenceState` is `Copy`, so `let state = self.state;` makes a copy you
//!   can look at freely while you work out the next state.
//! - `tick` is where the borrow checker can bite. See its doc comment.

/// Identifies a physical output on the stand.
pub type ChannelId = u16;

/// One thing the sequence tells the hardware to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    OpenValve(ChannelId),
    CloseValve(ChannelId),
    SetAnalog { id: ChannelId, value: u16 },
}

/// A group of commands, and how many cycles to hold before the next step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    commands: Vec<Command>,
    duration_cycles: u32,
}

impl Step {
    /// A `duration_cycles` of 0 is treated as 1. Every step lasts at least the
    /// cycle its commands go out on.
    pub fn new(commands: Vec<Command>, duration_cycles: u32) -> Self {
        Self {
            commands,
            duration_cycles: duration_cycles.max(1),
        }
    }
}

/// Where a sequence is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceState {
    /// Built, not started.
    Idle,
    /// On `steps[step]`, and has been for `cycles_in_step` cycles.
    Running { step: usize, cycles_in_step: u32 },
    /// Ran every step.
    Complete,
    /// Stopped early. Never resumes.
    Aborted,
}

/// Why a sequence refused to start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceError {
    /// `start` was called when the sequence was not `Idle`. Holds the state it
    /// was actually in.
    NotIdle(SequenceState),
    /// There is nothing to run.
    NoSteps,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    steps: Vec<Step>,
    state: SequenceState,
}

impl Sequence {
    pub fn new(steps: Vec<Step>) -> Self {
        Self {
            steps,
            state: SequenceState::Idle,
        }
    }

    pub fn state(&self) -> SequenceState {
        self.state
    }

    /// Stops the sequence for good. Aborting a sequence that has not started
    /// yet stops it from ever starting. A completed sequence stays complete.
    pub fn abort(&mut self) {
        if matches!(
            self.state,
            SequenceState::Idle | SequenceState::Running { .. }
        ) {
            self.state = SequenceState::Aborted;
        }
    }

    /// Moves an `Idle` sequence to the first cycle of its first step.
    ///
    /// # Errors
    ///
    /// - [`SequenceError::NotIdle`] if it is in any other state. Starting a
    ///   sequence twice is a bug in the caller, and restarting an aborted one
    ///   would undo the abort.
    /// - [`SequenceError::NoSteps`] if there are no steps.
    pub fn start(&mut self) -> Result<(), SequenceError> {
        if self.state != SequenceState::Idle {
            return Err(SequenceError::NotIdle(self.state));
        }
        if self.steps.is_empty() {
            return Err(SequenceError::NoSteps);
        }
        self.state = SequenceState::Running {
            step: 0,
            cycles_in_step: 0,
        };
        Ok(())
    }

    /// Runs one control cycle and returns the commands to send this cycle.
    ///
    /// - Not `Running`: returns no commands and changes nothing.
    /// - On the first cycle of a step (`cycles_in_step == 0`): returns that
    ///   step's commands.
    /// - Then counts this cycle. Once the step has lasted `duration_cycles`,
    ///   moves to the first cycle of the next step, or to `Complete` if this
    ///   was the last one.
    ///
    /// Two ways to write this both work: `match &mut self.state` and edit the
    /// fields in place, or copy the state out (`let state = self.state;`), work
    /// out the next one, and assign `self.state` once at the end. We suggest
    /// the second. With the first, calling a `&mut self` method inside the
    /// match arm and then using `step` afterwards is a compile error, and the
    /// message (E0499) is confusing the first time you meet it.
    ///
    /// TODO(you): implement.
    pub fn tick(&mut self) -> Vec<Command> {
        // This line only exists so the unimplemented stub compiles without
        // "unused" errors. Delete it when you implement the function.
        let _ = self
            .steps
            .first()
            .map(|step| (&step.commands, step.duration_cycles));
        todo!("ex4: implement Sequence::tick")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// Helper, not a test. A two-step sequence whose first step lasts
    /// `first_duration` cycles and whose second step lasts one.
    fn two_steps(first_duration: u32) -> Sequence {
        Sequence::new(vec![
            Step::new(vec![Command::OpenValve(3)], first_duration),
            Step::new(vec![Command::OpenValve(5), Command::OpenValve(6)], 1),
        ])
    }

    #[rstest]
    fn test_abort_stops_a_sequence_that_has_not_started() {
        let mut sequence = two_steps(2);

        sequence.abort();

        assert_eq!(sequence.state(), SequenceState::Aborted);
    }

    #[rstest]
    fn test_tick_issues_the_first_steps_commands_on_the_first_cycle() {
        let mut sequence = two_steps(2);
        sequence.start().expect("a new sequence should start");

        let commands = sequence.tick();

        assert_eq!(commands, vec![Command::OpenValve(3)]);
    }

    #[rstest]
    fn test_start_returns_no_steps_when_the_sequence_is_empty() {
        let mut sequence = Sequence::new(Vec::new());

        let result = sequence.start();

        assert_eq!(result, Err(SequenceError::NoSteps));
    }

    #[rstest]
    fn test_start_returns_not_idle_when_already_running() {
        let mut sequence = two_steps(2);
        sequence.start().expect("a new sequence should start");

        let result = sequence.start();

        assert_eq!(
            result,
            Err(SequenceError::NotIdle(SequenceState::Running {
                step: 0,
                cycles_in_step: 0
            }))
        );
    }

    // ------------------------------------------------------------------
    // TODO(you): `tick` has one test. Write the rest:
    //
    //   1. `tick` returns no commands on the second cycle of a step that lasts
    //      three cycles.
    //   2. `tick` issues the next step's commands after `duration_cycles`
    //      cycles. Use `#[case]` for durations 1, 2, and 5, and a
    //      `for _ in 0..n` loop to run the cycles before the one you check.
    //   3. `tick` moves the sequence to `Complete` after the last step.
    //   4. `tick` returns no commands after the sequence is aborted mid-run.
    //
    // Sabotage check: make `tick` return the step's commands on every cycle,
    // not just the first. Which of your tests catch it? At least one should.
    // ------------------------------------------------------------------
}
