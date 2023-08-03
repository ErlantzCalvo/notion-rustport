use std::collections::HashMap;

use crate::{configure::Configuration, task::Task};

pub struct Generator {
    config: Configuration
}

impl From<Configuration> for Generator {
    fn from(config: Configuration) -> Self {
        Self { config }
    }

}

impl Generator {
    pub fn generate_from_tasks(&self, tasks: HashMap<String, Vec<Task>>) -> String {
        let mut report = String::from("");
        let mut task_start_index: usize = 1;

        if let Some(finished_tasks) = tasks.get(&self.config.tasks_status.finished_tasks) {
            self.add_current_tasks_report(finished_tasks, &self.config.texts.finished_tasks_status, &mut report, task_start_index);
            task_start_index += finished_tasks.len();
        }

        if let Some(doing_tasks) = tasks.get(&self.config.tasks_status.doing_tasks) {
            self.add_current_tasks_report(doing_tasks, &self.config.texts.doing_tasks_status, &mut report, task_start_index);
        }

        println!("------> OUTPUT: {}", report);

        report
    }


    fn add_current_tasks_report(&self, tasks: &Vec<Task>, status_text: &str, out: &mut String, start_idx: usize) {
        tasks.iter().enumerate().for_each(|(idx, task)| {
            let mut task_title = format!("{}) {} → {}\n", idx + start_idx, task.title, status_text);
            let mut sub_stask_text = String::from("");
            for st in &task.sub_tasks {
                let status = if st.1 {&self.config.texts.finished_tasks_status} else {&self.config.texts.doing_tasks_status};
                let text = format!("\t- {} →  {}\n", st.0, status);
                sub_stask_text.push_str(&text);   
            }

            task_title.push_str(&sub_stask_text);
            out.push_str(&(task_title + "\n"));
        })
    }
}

