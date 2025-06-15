use simply_colored::{BOLD, CYAN, GREEN, RED, RESET};

const LINE_START: &str = "\x1b[1G";
const ERASE_LINE: &str = "\x1b[0K";
const LINE_UP: &str = "\x1b[1F";

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
}

impl Action {
    /// Create and report a new in progress action.
    pub fn new<S1: ToString, S2: ToString, S3: ToString>(
        actioning_verb: S1,
        actioned_verb: S2,
        detail: S3,
    ) -> Self {
        let progress = Self {
            state: State::InProgress,
            actioning_verb: actioning_verb.to_string(),
            actioned_verb: actioned_verb.to_string(),
            detail: detail.to_string(),
        };
        progress.print();
        progress
    }

    /// Update the state of the action.
    pub fn set_state(&mut self, state: State) {
        self.state = state;
        self.print();
    }

    fn print(&self) {
        let actioning = &self.actioning_verb;
        let actioned = &self.actioned_verb;
        let detail = &self.detail;

        let width = actioning.len().max(actioned.len()) + "Failed ".len();
        let failed_width = width - "Failed ".len();

        match self.state {
            State::InProgress => {
                println!("{LINE_START}{ERASE_LINE}{CYAN}{BOLD}{actioning:>width$}{RESET} {detail}")
            }
            State::Success => {
                println!(
                    "{LINE_UP}{LINE_START}{ERASE_LINE}{GREEN}{BOLD}{actioned:>width$}{RESET} {detail}"
                )
            }
            State::Error => {
                println!(
                    "{LINE_UP}{LINE_START}{ERASE_LINE}{RED}{BOLD}Failed {actioning:>failed_width$}{RESET} {detail}"
                )
            }
        }
    }
}
