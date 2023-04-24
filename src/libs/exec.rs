use std::process::{Command, Child, Stdio};
use std::env;
use std::path::Path;
use std::process::exit;

pub fn exec(input: String) {
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(cmd) = commands.next()  {

        let mut parts = cmd.trim().split_whitespace();
        let cmd = parts.next().unwrap();
        let args = parts;

        match cmd {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                },
                "exit" => {
                    exit(0);
                },
                cmd => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let child_proc = Command::new(cmd)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                    match child_proc {
                        Ok(child_proc) => {
                            previous_command = Some(child_proc);
                        },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("Command failed!!\nCaused by: {}", e);
                        }
                    };
                }
        }

        if let Some(ref mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
     }
}
