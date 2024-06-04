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
        self.name.is_none() || self.name.as_ref().unwrap().is_empty()
    }
    pub fn is_run_empty(&self) -> bool {
        self.run.is_none() || self.run.as_ref().unwrap().is_empty()
    }

    pub fn get_quite(&self) -> bool {
        self.quite.unwrap()
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
    require: Option<Vec<String>>,
    jobs: BTreeMap<String, Vec<Step>>,
}

pub enum ProjectError {
    BadStructure,
}

impl Default for Project {
    fn default() -> Self {
        let mut jobs: BTreeMap<String, Vec<Step>> = BTreeMap::new();
        jobs.insert(
            "hello".into(),
            vec![Step {
                name: Some("Print 'Hello world!'".to_string()),
                run: Some(String::from("echo \"Hello World!\"")),
                quite: None,
            }],
        );
        Self {
            name: String::from("TesutoProject"),
            require: None,
            jobs,
        }
    }
}

impl Project {
    pub fn from(json_string: &str) -> Result<Self, ProjectError> {
        match serde_yaml::from_str::<Project>(json_string) {
            Ok(project) => Ok(project),
            Err(_) => Err(ProjectError::BadStructure),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_jobs(&self) -> BTreeMap<String, Vec<Step>> {
        self.jobs.clone()
    }

    pub fn get_require(&self) -> Vec<String> {
        self.require.clone().unwrap_or_default()
    }

    pub fn is_jobs_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    pub fn is_job_exists(&self, job_name: &str) -> bool {
        self.jobs.contains_key(job_name)
    }
}
