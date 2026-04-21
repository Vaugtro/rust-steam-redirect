//! Steam Proton Game Wrapper - main entry point
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::env;
use std::path::Path;
use std::process::exit;

use steam_redirect::config::load_config;
use steam_redirect::executor::execute_program;
use steam_redirect::{Result, WrapperError};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new("."));
    let (_config_dir, wrapper_config) = load_config(exe_dir)?;

    if should_redirect() {
        let mut program_args = wrapper_config.program_args.clone();
        program_args.extend(env::args().skip(1));
        env::set_var("NO_REDIRECT", "1");

        let exit_code = execute_program(&program_args[0], &program_args[1..])?;
        exit(exit_code);
    }

    let fallback_args = wrapper_config
        .fallback_args
        .as_ref()
        .ok_or(WrapperError::FallbackNotConfigured)?;

    env::set_var("NO_REDIRECT", "1");
    let exit_code = execute_program(&fallback_args[0], &fallback_args[1..])?;
    exit(exit_code);
}

fn should_redirect() -> bool {
    env::var_os("NO_REDIRECT").is_none()
}
