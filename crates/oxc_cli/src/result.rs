use std::{
    path::PathBuf,
    process::{ExitCode, Termination},
};

#[derive(Debug)]
pub enum CliRunResult {
    None,
    PathNotFound { paths: Vec<PathBuf> },
    MaxWarningsExceeded { number_of_warnings: usize, max_warnings: usize },
    LintResult { number_of_files: usize, number_of_diagnostics: usize },
}

impl Termination for CliRunResult {
    fn report(self) -> ExitCode {
        match self {
            Self::None => ExitCode::from(0),
            Self::PathNotFound { paths } => {
                println!("Path {paths:?} does not exist.");
                ExitCode::from(1)
            }
            Self::MaxWarningsExceeded { number_of_warnings, max_warnings } => {
                println!(
                    "Exceeded --max-warnings value of {max_warnings}. Found {number_of_warnings} warning(s)."
                );
                ExitCode::from(1)
            }
            Self::LintResult { number_of_files, number_of_diagnostics } => {
                println!("Checked {number_of_files} files.");

                if number_of_files > 0 {
                    println!("Found {number_of_diagnostics} diagnostics.");
                    return ExitCode::from(1);
                }

                ExitCode::from(0)
            }
        }
    }
}
