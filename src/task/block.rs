use notion::models::Block;
use notion::models::{Page, ListResponse};
use notion::NotionApi;
use notion::ids::AsIdentifier;
use std::str::FromStr;

#[derive(Debug)]
pub enum BlockErrors {
    GetBlockIdError,
    GetPageBlockError
}

pub async fn get_page_block_children(notion_api: &NotionApi, task: &Page) -> Result<ListResponse<Block>, BlockErrors> {
    let page_id = &task.id.to_string();
    match notion::ids::BlockId::from_str(page_id) {
        Ok(id) => notion_api.get_block_children(id.as_id()).await.or(Err(BlockErrors::GetPageBlockError)),
        Err(_) => Err(BlockErrors::GetBlockIdError)
    }
}

pub fn get_block_todo_fields(blocks: ListResponse<Block>) -> Vec<(String, bool)> {
    blocks.results()
        .iter()
        .filter(|b| matches!(b, Block::ToDo {..}))
        .map(|td| {
            match td {
                Block::ToDo {to_do, ..} => (to_do.rich_text[0].plain_text().to_string(), to_do.checked),
                _ => panic!("Strange ToDo not expected")
            }
        })
        .collect::<Vec<(String, bool)>>()
}
