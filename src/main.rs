use anyhow::Ok;
use anyhow::Result;

use ui::manager::UiManager;

pub mod database;
pub mod event;
pub mod models;
pub mod ui;

fn main() -> Result<()> {
    let mut ui_manager = UiManager::new();
    ui_manager.run()?;
    Ok(())
}
