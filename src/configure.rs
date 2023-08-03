use std::{path::Path, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct TasksStatus {
        pub pending_tasks: String,
        pub doing_tasks: String,
        pub finished_tasks: String
    }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
 pub struct Texts {
        pub finished_tasks_status: String,
        pub doing_tasks_status: String,
        pub beginning_of_message: String,
        pub pending_tasks_beginning: String,
        pub farewell: String

    }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Configuration {
    pub texts: Texts,
    pub tasks_status: TasksStatus
}

impl Configuration {
    pub fn load(file_path: &str) -> Result<Self, std::io::Error> {

        let file = File::open(Path::new(file_path))?;
        let reader = BufReader::new(file);
        let config: Configuration = serde_json::from_reader(reader)?;

        Ok(config)
    }
}
