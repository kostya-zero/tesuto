use std::{fs, process::exit};
use tesuto_project::Project;

use crate::term::Term;

pub struct Manager;
impl Manager {
    pub fn load_project() -> Project {
        match fs::read_to_string("tesuto.yml") {
            Ok(project_string) => match serde_yaml::from_str::<Project>(&project_string) {
                Ok(project) => project,
                Err(_) => {
                    Term::error("Failed to format file content as structure.");
                    exit(1)
                }
            },
            Err(_) => {
                Term::error("Failed to load project. Looks like it's bad structured.");
                exit(1)
            }
        }
    }

    pub fn write_project(project: Project) {
        match serde_yaml::to_string(&project) {
            Ok(project_string) => match fs::write("tesuto.yml", project_string) {
                Ok(_) => {}
                Err(_) => {
                    Term::error("Cant write project file.");
                    exit(1);
                }
            },
            Err(_) => {
                Term::error("Cant format project structure as string.");
                exit(1);
            }
        }
    }
}
