use crate::project::Project;
use std::fs;

pub enum ManagerError {
    ReadError,
    BadStructure,
    FileSystemError,
    FormatError,
}

pub struct Manager;
impl Manager {
    pub fn load_project() -> Result<Project, ManagerError> {
        match fs::read_to_string("tesuto.yml") {
            Ok(project_string) => match serde_yaml::from_str::<Project>(&project_string) {
                Ok(project) => Ok(project),
                Err(_) => Err(ManagerError::BadStructure),
            },
            Err(_) => Err(ManagerError::ReadError),
        }
    }
}
