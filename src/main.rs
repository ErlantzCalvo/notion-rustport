use notion::ids::{DatabaseId, AsIdentifier};
use notion::NotionApi;
use envfile::EnvFile;
use notion::models::Block;
use notion::models::{Database, Page, ListResponse};
use notion::models::properties::PropertyValue;
use std::collections::HashMap;
use std::{path::Path, str::FromStr};

#[derive(Debug)]
enum MainErrors {
    EnvFileError(std::io::Error),
    ApiKeyNotFoundInEnv,
    ApiKeyError,
    DbIdNotFoundInEnv,
    DbGetError,
    NotionApiError(notion::Error)
}

#[derive(Debug)]
enum BlockErrors {
    GetBlockIdError,
    GetPageBlockError
}

#[derive(Debug)]
struct Task {
    title: String,
    sub_tasks: Vec<String>
}

#[tokio::main]
async fn main() -> Result<(), MainErrors>{
    let envfile = load_envfile()?;
    let api_key = load_api_key(&envfile)?;

    if let Ok(notion_api) = NotionApi::new(api_key.to_string()) {
        let db_id = get_db_id(envfile)?;
        let db = get_db(&notion_api, db_id).await?;
        let tasks = get_tasks_from_db(&notion_api, db).await?;
        println!("---- TASKS --> {:?}", tasks);
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

async fn get_tasks_from_db(notion_api: &NotionApi, db: Database) -> Result<HashMap<String, String>, MainErrors>{
    let mut tasks = HashMap::new();
    let query = notion::models::search::DatabaseQuery::default();

    if let Ok(query_result) = notion_api.query_database(db, query).await {
        // for i in 0..5 {
        // // println!("--------->{:?}", query_result.results()[i].properties.title());
        //     query_result
        //         .results()
        //         .iter()
        //         .map(|task| {
        //             let section_name = get_task_section_name(&task);
        //             tasks.insert(task.properties., v)
        //     })  
        // }
        // println!("------> {:?}", query_result.results()[4].properties.properties);
    
        println!("------SECTION---> {}", get_task_section_name(&query_result.results()[1]));
        println!("------TITLE--->{}", get_task_title(&query_result.results()[1]).unwrap());
        println!("-----TO DOs---->{:?}", get_task_todos(notion_api, &query_result.results()[1]).await);

            // let pageId = query_result.results()[0].id.clone();
            // let page = notion_api.get_page(pageId).await;

        Ok(tasks)
    } else {
        Err(MainErrors::DbGetError)
    }

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
