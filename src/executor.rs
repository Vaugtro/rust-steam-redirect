/// Program execution utilities.
use crate::error::Result;
use crate::error::WrapperError;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

/// Execute a program with the given arguments.
///
/// On Unix systems, this uses `exec` to replace the current process, avoiding fork/exec issues
/// with Proton and Wine. On Windows, it spawns the process and waits for it to complete.
///
/// # Arguments
/// * `program` - The path to the program to execute
/// * `args` - The arguments to pass to the program
///
/// # Returns
/// On Unix: Never returns on success (process is replaced); returns error on failure
/// On Windows: Returns the exit code of the executed program, or an error if execution fails
pub fn execute_program(program: &str, args: &[String]) -> Result<i32> {
    let mut cmd = Command::new(program);
    cmd.args(args);

    #[cfg(unix)]
    {
        // On Unix, use exec to replace the current process
        // This avoids fork/exec issues with Proton
        let err = cmd.exec();
        return Err(WrapperError::ExecutionError(format!(
            "Failed to execute '{}': {}",
            program, err
        )));
    }

    #[cfg(not(unix))]
    {
        // On non-Unix systems (Windows), spawn and wait
        let status = cmd.status().map_err(|e| {
            WrapperError::ExecutionError(format!("Failed to execute '{}': {}", program, e))
        })?;

        Ok(status.code().unwrap_or(1))
    }
}
