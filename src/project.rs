use std::collections::{HashMap, BTreeMap};

use serde::{Deserialize, Serialize};

pub type Stages = BTreeMap<String, Stage>;
pub type Vars = Vec<Variable>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Stage {
    name: Option<String>,
    script: Vec<String>,
    variables: Option<Vars>,
    quite: Option<bool>,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            name: Some(String::new()),
            script: vec!["echo \"Hello World!\"".to_string()],
            variables: Some(Vec::new()),
            quite: Some(false),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Variable {
    name: String,
    value: String,
}

impl Variable {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl Stage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }

    pub fn get_scripts(&self) -> Vec<String> {
        self.script.clone()
    }

    pub fn get_variables(&self) -> Option<Vars> {
        self.variables.clone()
    }

    pub fn get_quite(&self) -> Option<bool> {
        self.quite
    }
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    stages: Stages,
}

impl Default for Project {
    fn default() -> Self {
        Self { name: "TesutoProject".to_string(), stages: BTreeMap::from([("hello-job".to_string(), Stage::default())]) }
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
        self.stages[name].clone()
    }

    pub fn stage_exists(&self, name: &str) -> bool {
        self.stages.iter().any(|i| i.0.eq(name))
    }
}
