use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Action {
    name: Option<String>,
    program: Option<String>,
    args: Option<Vec<String>>,
}

impl Action {
    pub fn is_name_empty(&self) -> bool {
        if self.name.is_none() || self.name.unwrap().is_empty() {
            return true;
        }
        false
    }

    pub fn is_program_empty(&self) -> bool {
        if self.program.is_none() || self.program.unwrap().is_empty() {
            return true;
        }
        false
    }

    pub fn is_args_empty(&self) -> bool {
        if self.args.is_none() || self.args.unwrap().is_empty() {
            return true;
        }
        false
    }

    pub fn get_name(&self) -> String {
        self.name.unwrap().clone()
    }

    pub fn get_program(&self) -> String {
        self.program.unwrap().clone()
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.unwrap().clone()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Project {
    name: Option<String>,
    require: Option<Vec<String>>,
    jobs: Option<BTreeMap<String, Vec<Action>>>,
}

impl Default for Project {
    fn default() -> Self {
        let mut new_btree: BTreeMap<String, Vec<Action>> = BTreeMap::new();
        new_btree.insert(
            "hello-job".to_string(),
            vec![Action {
                name: Some("Print 'Hello world!'".to_string()),
                program: Some("echo".to_string()),
                args: Some(vec!["Hello world!".to_string()]),
            }],
        );
        Self {
            name: Some(String::from("TesutoProject")),
            require: Some(Vec::new()),
            jobs: Some(new_btree),
        }
    }
}

impl Project {
    pub fn get_name(&self) -> String {
        self.name.unwrap_or(String::from("Unnamed")).clone()
    }

    pub fn get_require(&self) -> Vec<String> {
        self.require.unwrap_or(Vec::new()).clone()
    }

    pub fn get_jobs(&self) -> BTreeMap<String, Vec<Action>> {
        self.jobs.unwrap_or(BTreeMap::new())
    }

    pub fn is_jobs_empty(&self) -> bool {
        self.jobs.is_none() || self.jobs.unwrap().is_empty()
    }
}
