use std::{path::Path, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TasksStatus {
        pending_tasks: String,
        doing_tasks: String,
        finished_tasks: String
    }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
 pub struct Texts {
        finished_tasks_status: String,
        doing_tasks_status: String,
        beginning_of_message: String,
        pending_tasks_beginning: String,
        farewell: String

    }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Configuration {
    texts: Texts,
    tasks_status: TasksStatus
}

impl Configuration {
    pub fn load(file_path: &str) -> Result<Self, std::io::Error> {

        let file = File::open(Path::new(file_path))?;
        let reader = BufReader::new(file);
        let config: Configuration = serde_json::from_reader(reader)?;

        Ok(config)
    }
}
