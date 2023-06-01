use crate::project::Stage;
use std::process::{exit, Command, Stdio};

pub struct Runner;
impl Runner {
    pub fn run_stage(stage: Stage) {
        let mut command = Command::new(stage.program);
        command.args(stage.args);

        if stage.showOnlyErrors {
            command
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::inherit());
        } else {
            command
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        let result = command
            .output()
            .expect("Failed to run program. Thread panicked.");

        if stage.expectFail {
            if !result.status.success() {
                println!("Stage passed.");
            } else {
                println!("Stage failed!");
                exit(1);
            }
        } else {
            if result.status.success() {
                println!("Stage passed.");
            } else {
                println!("Stage failed!");
                exit(1);
            }
        }
    }
}
