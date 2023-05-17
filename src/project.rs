use std::{path::Path, fs};
use serde::{Serialize, Deserialize};
use serde_yaml;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub program: String,
    pub args: Vec<String>,
    pub expectFail: bool,
    pub showOnlyErrors: bool
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub requiredTools: Vec<String>,
    pub stages: Vec<Stage>
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: "TesutoProject".to_string(), 
            requiredTools: Vec::new(), 
            stages: vec![Stage {
                name: "Test echo.".to_string(),
                program: "echo".to_string(),
                args: vec!["\"hello world\"".to_string()],
                expectFail: false,
                showOnlyErrors: false
            }]
        }
    }
}

pub struct Manager;
impl Manager {
    pub fn check() -> bool {
        Path::new("tesuto.yml").exists() 
    }

    pub fn generate() {
        let project: Project = Project { ..Default::default() };
        let content: String = serde_yaml::to_string(&project).unwrap();
        fs::write("tesuto.yml", content).unwrap();
    }

    pub fn read() -> Project {
        let content: String = fs::read_to_string("tesuto.yml").unwrap();
        let config: Project = serde_yaml::from_str(&content).unwrap();
        config
    }

    pub fn write(project: Project) {
        let content: String = serde_yaml::to_string(&project).unwrap();
        fs::write("tesuto.yml", content).unwrap();
    }
}
