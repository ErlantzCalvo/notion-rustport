extern crate futures;
mod configure;

use notion::ids::{DatabaseId, AsIdentifier};
use notion::NotionApi;
use envfile::EnvFile;
use notion::models::Block;
use notion::models::{Database, Page, ListResponse};
use notion::models::properties::PropertyValue;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::{path::Path, str::FromStr};
use futures::future::join_all;

#[derive(Debug)]
enum MainErrors {
    EnvFileError(std::io::Error),
    ConfigFileError(std::io::Error),
    ApiKeyNotFoundInEnv,
    ApiKeyError,
    DbIdNotFoundInEnv,
    NotionApiError(notion::Error)
}

#[derive(Debug)]
enum BlockErrors {
    GetBlockIdError,
    GetPageBlockError
}

#[derive(Debug)]
struct Task {
    section: String,
    title: String,
    sub_tasks: Vec<(String, bool)>
}

#[tokio::main]
async fn main() -> Result<(), MainErrors>{
    let config = configure::Configuration::load("./config.json").map_err(MainErrors::ConfigFileError)?;
    println!("----- CONFIG --->{:?}", config);
    let envfile = load_envfile()?;
    let api_key = load_api_key(&envfile)?;

    if let Ok(notion_api) = NotionApi::new(api_key.to_string()) {
        let db_id = get_db_id(envfile)?;
        let db = get_db(&notion_api, db_id).await?;
        let tasks = get_tasks_from_db(&notion_api, db).await?;
        generate_daily_report(tasks);
    } else {
        return Err(MainErrors::ApiKeyError);
    }

    Ok(())
}


fn load_envfile() -> Result<EnvFile, MainErrors>{
    EnvFile::new(&Path::new("./.env")).map_err(|err| MainErrors::EnvFileError(err))
}

fn load_api_key(envfile: &EnvFile) -> Result<String, MainErrors> {
    match envfile.get("NOTION_API_KEY") {
        Some(api_key) => Ok(api_key.to_string()),
        _ => Err(MainErrors::ApiKeyNotFoundInEnv)
    }
}

fn get_db_id(envfile: EnvFile) -> Result<String, MainErrors> {
    match envfile.get("NOTION_PAGE_ID") {
        Some(id) => Ok(id.to_string()),
        _ => Err(MainErrors::DbIdNotFoundInEnv)
    }

}

async fn get_db(notion_api: &NotionApi, id: String) -> Result<Database, MainErrors> {
    match DatabaseId::from_str(&id) {
        Ok(db_id) => notion_api.get_database(db_id.as_id()).await.map_err(|err| MainErrors::NotionApiError(err)),
        Err(_) => Err(MainErrors::DbIdNotFoundInEnv)
    }
}

async fn get_tasks_from_db(notion_api: &NotionApi, db: Database) -> Result<HashMap<String, Vec<Task>>, MainErrors>{
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
    let block_children = get_page_block_children(notion_api, task).await.unwrap();
    get_block_todo_fields(block_children)
}

async fn get_page_block_children(notion_api: &NotionApi, task: &Page) -> Result<ListResponse<Block>, BlockErrors> {
    let page_id = &task.id.to_string();
    match notion::ids::BlockId::from_str(page_id) {
        Ok(id) => notion_api.get_block_children(id.as_id()).await.or(Err(BlockErrors::GetPageBlockError)),
        Err(_) => Err(BlockErrors::GetBlockIdError)
    }
}

fn get_block_todo_fields(blocks: ListResponse<Block>) -> Vec<(String, bool)> {
    blocks.results()
        .into_iter()
        .filter(|b| matches!(b, Block::ToDo {..}))
        .map(|td| {
            match td {
                Block::ToDo {to_do, ..} => (to_do.rich_text[0].plain_text().to_string(), to_do.checked),
                _ => panic!("Strange ToDo not expected")
            }
        })
        .collect::<Vec<(String, bool)>>()
}

fn generate_daily_report(tasks: HashMap<String, Vec<Task>>) {
    let sections = vec!["Done 🙌", "Doing"];
    let mut report = String::from("");
    let mut start_idx = 1;
    for section in sections {
        let section_tasks = tasks.get(section);
        if let Some(t) = section_tasks {
            generate_section_report(t, &mut report, start_idx);
            start_idx += t.len();
        }
    }
    println!("------> OUTPUT: {}", report);
}

fn generate_section_report(tasks: &Vec<Task>, out: &mut String, start_idx: usize) {
    tasks.iter().enumerate().for_each(|(idx, task)| {
        let mut task_title = format!("{}) {} ----------> Status\n", idx + start_idx, task.title);
        let mut sub_stask_text = String::from("");
        for st in &task.sub_tasks {
            let text = format!("\t- {} ---------> {}\n", st.0, st.1);
            sub_stask_text.push_str(&text);   
        }
        task_title.push_str(&sub_stask_text);
        out.push_str(&(task_title + "\n"));
    })
}
