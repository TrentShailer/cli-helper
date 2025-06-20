use core::{error::Error, fmt};

use simply_colored::{BOLD, RED, RESET};

/// Result that coerces any error into a report for display.
pub type ReportResult<T, E = Report> = Result<T, E>;

/// Report for printing a nice error report.
pub struct Report(Box<dyn Error>);

impl<E> From<E> for Report
where
    E: Error + 'static,
{
    fn from(value: E) -> Self {
        Self(Box::new(value))
    }
}

impl fmt::Debug for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_error = Some(self.0.as_ref());

        writeln!(f, "`{}` exited unsuccessfully", env!("CARGO_PKG_NAME"))?;

        let mut index = 1;
        while let Some(error) = current_error {
            writeln!(f, "  {BOLD}{RED}{index}{RESET}{BOLD}:{RESET} {error}")?;
            current_error = error.source();
            index += 1;
        }

        Ok(())
    }
}
