use std::{env, error::Error, io, os::unix::process::ExitStatusExt, process::{Command, ExitStatus}};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let (flags, targets): (Vec<String>, Vec<String>) =
        args.into_iter().partition(|arg| is_flag(arg));

    let cmd_result = if !flags.is_empty() {
        confirm_deletion(&targets).and_then(|confirmed| {
            if confirmed {
                execute_rm(&flags, &targets)
            } else {
                println!("Aborted");
                Ok(ExitStatus::from_raw(0))
            }
        })
    } else {
        execute_rm(&[], &targets)
    };

    match cmd_result {
        Ok(status) if !status.success() => {
            eprintln!("Exited with status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to execute: {}", e);
        }
        _ => {}
    }

    Ok(())
}

fn is_flag(arg: &str) -> bool {
    let cases = ["-r", "-f", "-rf", "-fr"];
    cases.contains(&arg)
}

fn confirm_deletion(targets: &[String]) -> Result<bool, Box<dyn Error>> {
    println!(
        "Did you mean to delete {:?}? Type yes to confirm, or anything else to break",
        targets
    );
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq_ignore_ascii_case("yes"))
}

fn execute_rm(flags: &[String], targets: &[String]) -> Result<ExitStatus, Box<dyn Error>> {
    Command::new("rm")
        .args(flags)
        .args(targets)
        .status()
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}
