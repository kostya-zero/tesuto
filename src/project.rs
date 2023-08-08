use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, path::Path};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stage {
    pub name: String,
    pub program: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub expectFail: bool,
    #[serde(default)]
    pub showOnlyErrors: bool,
}

impl Default for Stage {
    fn default() -> Self {
        Self {  
            name: String::from("stage"),
            program: String::from(""),
            args: vec![],
            expectFail: false,
            showOnlyErrors: false,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub options: Options,
    pub stages: Vec<Stage>,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            options: Options {
                name: String::from("TesutoProject"),
            },
            stages: vec![Stage { ..Default::default() }],
        }
    }
}

pub struct Manager;
impl Manager {
    pub fn check() -> bool {
        Path::new("tesuto.yml").exists()
    }

    pub fn generate() {
        let project: Project = Project {
            ..Default::default()
        };
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
