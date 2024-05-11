use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone)]
pub struct Step {
    name: Option<String>,
    run: Option<String>,
    quite: Option<bool>,
}

impl Step {
    pub fn is_name_empty(&self) -> bool {
        if self.name.is_none() || self.name.as_ref().unwrap().is_empty() {
            return true;
        }
        false
    }
    pub fn is_run_empty(&self) -> bool {
        if self.run.is_none() || self.run.as_ref().unwrap().is_empty() {
            return true;
        }
        false
    }
    pub fn get_name(&self) -> String {
        self.name.as_ref().unwrap().clone()
    }

    pub fn get_run(&self) -> String {
        self.run.as_ref().unwrap().clone()
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Project {
    name: String,
    jobs: BTreeMap<String, Vec<Step>>,
}

pub enum ProjectError {
    BadStructure
}

impl Default for Project {
    fn default() -> Self {
        let mut new_btree: BTreeMap<String, Vec<Step>> = BTreeMap::new();
        new_btree.insert(
            "hello".to_string(),
            vec![Step {
                name: Some("Print 'Hello world!'".to_string()),
                run: Some(String::from("echo \"Hello World!\"")),
                quite: None,
            }],
        );
        Self {
            name: String::from("TesutoProject"),
            jobs: new_btree,
        }
    }
}

impl Project {
    pub fn from(json_string: &str) -> Result<Self, ProjectError> {
        match serde_yaml::from_str::<Project>(json_string) {
            Ok(project) => Ok(project),
            Err(_) => Err(ProjectError::BadStructure)
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_jobs(&self) -> BTreeMap<String, Vec<Step>> {
        self.jobs.clone()
    }

    pub fn is_jobs_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    pub fn is_job_exists(&self, job_name: &str) -> bool {
        self.jobs.contains_key(job_name)
    }
}
