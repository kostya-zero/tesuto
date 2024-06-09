use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(default)]
pub struct Step {
    name: String,
    run: String,
    quite: bool,
}

impl Step {
    pub fn is_name_empty(&self) -> bool {
        self.name.is_empty()
    }
    pub fn is_run_empty(&self) -> bool {
        self.run.is_empty()
    }

    pub fn get_quite(&self) -> bool {
        self.quite
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_run(&self) -> String {
        self.run.clone()
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Project {
    name: String,
    require: Vec<String>,
    jobs: BTreeMap<String, Vec<Step>>,
}

pub enum ProjectError {
    BadStructure,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::from("TesutoProject"),
            require: Vec::new(),
            jobs: BTreeMap::new(),
        }
    }
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn example(name: &str) -> Self {
        let mut jobs: BTreeMap<String, Vec<Step>> = BTreeMap::new();
        jobs.insert(
            "hello".into(),
            vec![Step {
                name: "Print 'Hello world!'".to_string(),
                run: String::from("echo \"Hello World!\""),
                ..Default::default()
            }],
        );
        Self {
            name: String::from(name),
            jobs,
            ..Default::default()
        }
    }

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
        self.require.clone()
    }

    pub fn is_required_empty(&self) -> bool {
        self.require.is_empty()
    }

    pub fn is_jobs_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    pub fn is_job_exists(&self, job_name: &str) -> bool {
        self.jobs.contains_key(job_name)
    }
}
