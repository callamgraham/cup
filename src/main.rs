use std::env;
use std::process::{Command, exit};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [arg1] [arg2] [...]", args[0]);
        exit(1);
    }

    // Extract the command and its arguments
    let command = &args[1];
    let command_arg = &args[2];

    // Execute the command
    match command_arg {
	"update-home".to_string() => {
	        let output = Command::new(command)
		.args(command_arg)
		.output()
		.expect("Failed to execute command");
	}
    }
    


    // Print the command output
    println!("Command output:\n{}", String::from_utf8_lossy(&output.stdout));

    // Check for errors
    if !output.status.success() {
        eprintln!("Command failed with exit code: {}", output.status);
        exit(output.status.code().unwrap_or(1));
    }
}
