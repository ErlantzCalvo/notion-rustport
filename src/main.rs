mod configure;
mod task;
mod generator;

use clap::Parser;
use task::get_tasks_from_db;
use notion::ids::{DatabaseId, AsIdentifier};
use notion::NotionApi;
use envfile::EnvFile;
use notion::models::Database;
use std::{path::Path, str::FromStr};
use arboard::{Clipboard, SetExtLinux};

#[derive(Debug)]
enum MainErrors {
    EnvFileError(std::io::Error),
    ConfigFileError(std::io::Error),
    ApiKeyNotFoundInEnv,
    ApiKeyError,
    DbIdNotFoundInEnv,
    NotionApiError(notion::Error),
    CopyToClipboardError(arboard::Error)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(author="Erlantz Calvo", version, about="A daily report creator based on a Notion Task List page")]
struct Args {

    #[arg(short, long, default_value_t=false)]
    copy_to_clipboard: bool,

    #[arg(long, default_value_t=String::from("./config.json"))]
    config_path: String
}

#[tokio::main]
async fn main() -> Result<(), MainErrors>{
    let args = Args::parse();
    let config = configure::Configuration::load(&args.config_path).map_err(MainErrors::ConfigFileError)?;
    let envfile = load_envfile()?;
    let api_key = load_api_key(&envfile)?;

    if let Ok(notion_api) = NotionApi::new(api_key.to_string()) {
        let db_id = get_db_id(envfile)?;
        let db = get_db(&notion_api, db_id).await?;
        let tasks = get_tasks_from_db(&notion_api, db).await.map_err(MainErrors::NotionApiError)?;
        
        let report_generator = generator::ReportGenerator::from(config);
        let report = report_generator.generate_from_tasks(tasks);

        println!("{}", &report);
        if args.copy_to_clipboard {
            copy_to_clipboard(report)?;
        }

    } else {
        return Err(MainErrors::ApiKeyError);
    }

    Ok(())
}

fn load_envfile() -> Result<EnvFile, MainErrors>{
    EnvFile::new(&Path::new("./.env")).map_err(MainErrors::EnvFileError)
}

fn load_api_key(envfile: &EnvFile) -> Result<String, MainErrors> {
    envfile.get("NOTION_API_KEY").map(|key| key.to_string()).ok_or(MainErrors::ApiKeyNotFoundInEnv)
}

fn get_db_id(envfile: EnvFile) -> Result<String, MainErrors> {
    envfile.get("NOTION_PAGE_ID").map(|id| id.to_string()).ok_or(MainErrors::DbIdNotFoundInEnv)
}

async fn get_db(notion_api: &NotionApi, id: String) -> Result<Database, MainErrors> {
    match DatabaseId::from_str(&id) {
        Ok(db_id) => notion_api.get_database(db_id.as_id()).await.map_err(MainErrors::NotionApiError),
        Err(_) => Err(MainErrors::DbIdNotFoundInEnv)
    }
}

fn copy_to_clipboard(text: String) -> Result<(), MainErrors> {
    Clipboard::new().map_err(MainErrors::CopyToClipboardError)?
        .set().wait().text(text).map_err(MainErrors::CopyToClipboardError)?;
    Ok(())
}
