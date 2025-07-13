use std::io::{Write, stdout};

use ts_rust_helper::style::*;

/// Extension trait to update an action state.
pub trait ActionResult {
    /// Use self to update the action.
    fn bind_result(self, action: Action) -> Self;

    /// Use self to error the action.
    fn bind_error(self, action: &mut Action) -> Self;
}

impl<T, E> ActionResult for Result<T, E> {
    fn bind_result(self, mut action: Action) -> Self {
        match &self {
            Ok(_) => action.set_state(State::Success),
            Err(_) => action.set_state(State::Error),
        }

        self
    }

    fn bind_error(self, action: &mut Action) -> Self {
        if self.is_err() {
            action.set_state(State::Error)
        }
        self
    }
}
impl<T> ActionResult for Option<T> {
    fn bind_result(self, mut action: Action) -> Self {
        match &self {
            Some(_) => action.set_state(State::Success),
            None => action.set_state(State::Error),
        }

        self
    }

    fn bind_error(self, action: &mut Action) -> Self {
        if self.is_none() {
            action.set_state(State::Error)
        }
        self
    }
}

/// Action State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// The action is in progress.
    InProgress,
    /// The action was a success.
    Success,
    /// The action was an error.
    Error,
}

/// Action progress reporter.
pub struct Action {
    /// The current state of the action
    pub state: State,
    /// Verb for the in progress action.
    pub actioning_verb: String,
    /// Verb for the completed action.
    pub actioned_verb: String,
    /// Details for the action.
    pub detail: String,
    /// The number of characters to indent the action by.
    pub indent: usize,
    /// Should the action overwrite the previous line when printing.
    pub should_overwrite: bool,
}

impl Action {
    /// Create and report a new in progress action.
    pub fn new<S1: ToString, S2: ToString, S3: ToString>(
        actioning_verb: S1,
        actioned_verb: S2,
        detail: S3,
        indent: usize,
    ) -> Self {
        let mut progress = Self {
            state: State::InProgress,
            actioning_verb: actioning_verb.to_string(),
            actioned_verb: actioned_verb.to_string(),
            detail: detail.to_string(),
            indent,
            should_overwrite: false,
        };
        progress.print();
        progress
    }

    /// Update the state of the action.
    pub fn set_state(&mut self, state: State) {
        self.state = state;
        self.print();
    }

    /// Print the message for this action.
    pub fn print(&mut self) {
        let mut stdout = stdout().lock();

        let message = self.message_string(self.state);

        if self.should_overwrite {
            stdout.write_all(ERASE_LINE_UP.as_bytes()).unwrap();
        }

        stdout.write_all(format!("{message}\n").as_bytes()).unwrap();

        stdout.flush().unwrap();

        self.should_overwrite = true;
    }

    /// Set the action to not overwrite the previous line.
    pub fn dont_overwrite(&mut self) {
        self.should_overwrite = false;
    }

    fn message_string(&self, state: State) -> String {
        let indent = " ".repeat(self.indent);
        let actioning = &self.actioning_verb;
        let actioned = &self.actioned_verb;
        let detail = &self.detail;

        match state {
            State::InProgress => format!("{indent}{CYAN}{BOLD}{actioning}{RESET} {detail}"),
            State::Success => format!("{indent}{GREEN}{BOLD}{actioned}{RESET} {detail}"),
            State::Error => {
                format!("{indent}{RED}{BOLD}{actioning}{RESET} {detail} {RED}{BOLD}failed{RESET}")
            }
        }
    }
}
