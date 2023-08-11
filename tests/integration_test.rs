use std::collections::HashMap;

use notion_daily_rustport::{configure, task::Task, generator};

#[test]
fn load_config() {
    let config = configure::Configuration::load("tests/test_files/config_test.json").unwrap();
    assert_eq!(config.tasks_status.doing_tasks.clone(), "doing test".to_string());
    assert_eq!(config.tasks_status.finished_tasks.clone(), "done test".to_string());
    assert_eq!(config.tasks_status.pending_tasks, "pending test".to_string());
    assert_eq!(config.texts.beginning_of_message, "text 3".to_string());
    assert_eq!(config.texts.finished_tasks_status, "text 1".to_string());
    assert_eq!(config.texts.doing_tasks_status, "text 2".to_string());
    assert_eq!(config.texts.pending_tasks_beginning, "text 4".to_string());
    assert_eq!(config.texts.farewell, "text 5".to_string());
}

#[test]
fn generate_report() {
    let config = configure::Configuration::load("tests/test_files/config_test.json").unwrap();
    let mut tasks: HashMap<String, Vec<Task>> = HashMap::new();
    tasks.insert("done test".to_string(), vec![Task {title: "Finished".to_string(), section: "done test".to_string(), sub_tasks: vec![("subtask".to_string(), true)]}]);
    tasks.insert("doing test".to_string(), vec![Task {title: "Doing".to_string(), section: "doing test".to_string(), sub_tasks: vec![]}]);
    let report_generator = generator::ReportGenerator::from(config);
    let report = report_generator.generate_from_tasks(tasks);
    assert_eq!(report, "text 31) Finished → text 1\n    - subtask →  text 1\n\n2) Doing → text 2\n\ntext 5".to_string());
}
