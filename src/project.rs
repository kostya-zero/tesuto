use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(default)]
pub struct Step {
    pub name: Option<String>,
    pub run: Option<String>,
    pub quite: Option<bool>,
}

impl Step {
    pub fn is_name_empty(&self) -> bool {
        self.name.clone().unwrap_or_default().is_empty()
    }

    pub fn is_run_empty(&self) -> bool {
        self.run.clone().unwrap_or_default().is_empty()
    }

    pub fn get_quite(&self) -> bool {
        self.quite.unwrap_or_default()
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    pub fn get_run(&self) -> String {
        self.run.clone().unwrap_or_default()
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Shell {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct WithOptions {
    pub cwd: Option<String>,
    pub shell: Option<Shell>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Project {
    name: Option<String>,
    require: Option<Vec<String>>,
    with: Option<WithOptions>,
    jobs: IndexMap<String, Vec<Step>>,
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Failed to deserialize project with error: {0}")]
    DeserializeFailed(String),
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: Some(String::from("TesutoProject")),
            require: None,
            with: None,
            jobs: IndexMap::new(),
        }
    }
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: Some(String::from(name)),
            ..Default::default()
        }
    }

    pub fn example(name: &str) -> Self {
        let mut jobs: IndexMap<String, Vec<Step>> = IndexMap::new();
        jobs.insert(
            "hello".into(),
            vec![Step {
                name: Some("Print 'Hello world!'".to_string()),
                run: Some(String::from("echo \"Hello World!\"")),
                ..Default::default()
            }],
        );
        Self {
            name: Some(String::from(name)),
            jobs,
            ..Default::default()
        }
    }

    pub fn from(json_string: &str) -> Result<Self, ProjectError> {
        match serde_yaml::from_str::<Project>(json_string) {
            Ok(project) => Ok(project),
            Err(e) => Err(ProjectError::DeserializeFailed(e.to_string())),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    pub fn get_jobs(&self) -> IndexMap<String, Vec<Step>> {
        self.jobs.clone()
    }

    pub fn get_require(&self) -> Vec<String> {
        self.require.clone().unwrap_or_default()
    }

    pub fn get_with_options(&self) -> WithOptions {
        self.with.clone().unwrap_or_default()
    }

    pub fn is_required_empty(&self) -> bool {
        self.require.clone().unwrap_or_default().is_empty()
    }

    pub fn is_jobs_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    pub fn is_job_exists(&self, job_name: &str) -> bool {
        self.jobs.contains_key(job_name)
    }

    pub fn is_with_empty(&self) -> bool {
        self.with.is_none()
    }
}
