mod configure;
mod task;
mod generator;

use task::get_tasks_from_db;
use notion::ids::{DatabaseId, AsIdentifier};
use notion::NotionApi;
use envfile::EnvFile;
use notion::models::Database;
use std::{path::Path, str::FromStr};


#[derive(Debug)]
enum MainErrors {
    EnvFileError(std::io::Error),
    ConfigFileError(std::io::Error),
    ApiKeyNotFoundInEnv,
    ApiKeyError,
    DbIdNotFoundInEnv,
    NotionApiError(notion::Error)
}


#[tokio::main]
async fn main() -> Result<(), MainErrors>{
    let config = configure::Configuration::load("./config.json").map_err(MainErrors::ConfigFileError)?;
    let envfile = load_envfile()?;
    let api_key = load_api_key(&envfile)?;

    if let Ok(notion_api) = NotionApi::new(api_key.to_string()) {
        let db_id = get_db_id(envfile)?;
        let db = get_db(&notion_api, db_id).await?;
        let tasks = get_tasks_from_db(&notion_api, db).await.map_err(MainErrors::NotionApiError)?;
        
        let report_generator = generator::ReportGenerator::from(config);
        let report = report_generator.generate_from_tasks(tasks);

        println!("{}", report);
    } else {
        return Err(MainErrors::ApiKeyError);
    }

    Ok(())
}

fn load_envfile() -> Result<EnvFile, MainErrors>{
    EnvFile::new(&Path::new("./.env")).map_err(|err| MainErrors::EnvFileError(err))
}

fn load_api_key(envfile: &EnvFile) -> Result<String, MainErrors> {
    envfile.get("NOTION_API_KEY").map(|key| key.to_string()).ok_or(MainErrors::ApiKeyNotFoundInEnv)
}

fn get_db_id(envfile: EnvFile) -> Result<String, MainErrors> {
    envfile.get("NOTION_PAGE_ID").map(|id| id.to_string()).ok_or(MainErrors::DbIdNotFoundInEnv)
}

async fn get_db(notion_api: &NotionApi, id: String) -> Result<Database, MainErrors> {
    match DatabaseId::from_str(&id) {
        Ok(db_id) => notion_api.get_database(db_id.as_id()).await.map_err(|err| MainErrors::NotionApiError(err)),
        Err(_) => Err(MainErrors::DbIdNotFoundInEnv)
    }
}

