use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Action {
    name: Option<String>,
    program: Option<String>,
    args: Option<Vec<String>>,
}

impl Action {
    pub fn is_name_empty(&self) -> bool {
        if self.name.is_none() || self.name.as_ref().unwrap().is_empty() {
            return true;
        }
        false
    }
    pub fn is_program_empty(&self) -> bool {
        if self.program.is_none() || self.program.as_ref().unwrap().is_empty() {
            return true;
        }
        false
    }
    pub fn is_args_empty(&self) -> bool {
        if self.args.is_none() || self.args.as_ref().unwrap().is_empty() {
            return true;
        }
        false
    }
    pub fn get_name(&self) -> String {
        self.name.as_ref().unwrap().clone()
    }

    pub fn get_program(&self) -> String {
        self.program.as_ref().unwrap().clone()
    }
    pub fn get_args(&self) -> Vec<String> {
        self.args.as_ref().unwrap().clone()
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Project {
    name: Option<String>,
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
            jobs: Some(new_btree),
        }
    }
}

impl Project {
    pub fn get_name(&self) -> String {
        let name = self.name.clone();
        name.unwrap_or_default()
    }

    pub fn get_jobs(&self) -> BTreeMap<String, Vec<Action>> {
        let jobs = self.jobs.clone();
        jobs.unwrap_or_default()
    }

    pub fn is_jobs_empty(&self) -> bool {
        self.jobs.is_none() || self.jobs.as_ref().unwrap().is_empty()
    }

    pub fn is_job_exists(&self, job_name: &str) -> bool {
        let jobs = self.jobs.clone();
        jobs.unwrap().contains_key(job_name)
    }
}
