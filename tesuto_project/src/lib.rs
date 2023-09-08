use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Stages = HashMap<String, Stage>;
pub type Vars = HashMap<String, String>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Stage {
    before_script: Vec<String>,
    script: Vec<String>,
    variables: Vars,
    quite: bool,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            before_script: vec![],
            script: vec!["echo \"Hello World!\"".to_string()],
            variables: HashMap::new(),
            quite: false,
        }
    }
}

impl Stage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_before_scripts(&self) -> Vec<String> {
        self.before_script.clone()
    }

    pub fn get_scripts(&self) -> Vec<String> {
        self.script.clone()
    }

    pub fn get_variables(&self) -> Vars {
        self.variables.clone()
    }

    pub fn get_quite(&self) -> bool {
        self.quite
    }
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    #[serde(default)]
    scenarios: HashMap<String, Vec<String>>,
    stages: Stages,
}

impl Default for Project {
    fn default() -> Self {
        let mut default_stage: Stages = HashMap::new();
        default_stage.insert(
            String::from("hello"),
            Stage {
                ..Default::default()
            },
        );
        Self {
            name: String::from("TesutoProject"),
            stages: default_stage,
            scenarios: HashMap::new(),
        }
    }
}

impl Project {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_project_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_stages(&self) -> Stages {
        self.stages.clone()
    }

    pub fn get_stage(&self, name: &str) -> Stage {
        self.stages.get(name).unwrap().clone()
    }

    pub fn stage_exists(&self, stage_name: &str) -> bool {
        self.stages.contains_key(stage_name)
    }

    pub fn add_stage(&mut self, name: &str, stage: Stage) {
        self.stages.insert(name.to_string(), stage);
    }

    pub fn scenario_exists(&self, name: &str) -> bool {
        self.scenarios.contains_key(name)
    }

    pub fn get_scenario(&self, name: &str) -> Vec<String> {
        self.scenarios.get(name).unwrap().clone()
    }

    pub fn add_scenario(&mut self, name: &str) {
        self.scenarios
            .insert(String::from(name), vec!["hello".to_string()]);
    }
}
