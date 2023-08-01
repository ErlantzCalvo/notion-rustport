mod block;
use notion::{models::properties::PropertyValue, NotionApi};
use std::collections::hash_map::Entry;
use futures::future::join_all;
use std::collections::HashMap;
use notion::models::{Database, Page};

#[derive(Debug)]
pub struct Task {
    pub section: String,
    pub title: String,
    pub sub_tasks: Vec<(String, bool)>
}

pub async fn get_tasks_from_db(notion_api: &NotionApi, db: Database) -> Result<HashMap<String, Vec<Task>>, notion::Error>{
    let mut tasks: HashMap<String, Vec<Task> >= HashMap::new();
    let query = notion::models::search::DatabaseQuery::default();
   
    let query_result = notion_api.query_database(db, query).await.unwrap();
    let result_futures: Vec<_> = query_result.results()
        .into_iter()
        .map(|result| async {
                let section = get_task_section_name(result);
                let title = get_task_title(result).unwrap();
                let sub_tasks = get_task_todos(notion_api, result).await;
                Task {section, title, sub_tasks}
        }).collect();
    join_all(result_futures).await
    .into_iter()
    .for_each(|result| fill_tasks(&mut tasks, result));
    Ok(tasks)

}

fn fill_tasks(tasks: &mut HashMap<String, Vec<Task>>, t: Task) {
    match tasks.entry(t.section.clone()) {
        Entry::Vacant(e) => { e.insert(vec![t]);},
        Entry::Occupied(mut e) => {e.get_mut().push(t);}
    };
}

fn get_task_section_name(task: &Page) -> String{
    match task.properties.properties.get("Status") {
        Some(status) => {
            if let PropertyValue::Select { select, .. } = status {
                select.clone().unwrap().name.unwrap_or(String::from(""))
            } else {
                String::from("")
            }
        },
        _ => String::from("")
    }
}

fn get_task_title(task: &Page) -> Option<String>{
    if let PropertyValue::Title { title, .. } = task.properties.properties.get("Name").unwrap() {
        Some(title.get(0).unwrap().plain_text().to_string())
    } else {
        None
    }
}

async fn get_task_todos(notion_api: &NotionApi, task: &Page) -> Vec<(String, bool)> {
    let block_children = block::get_page_block_children(notion_api, task).await.unwrap();
    block::get_block_todo_fields(block_children)
}


