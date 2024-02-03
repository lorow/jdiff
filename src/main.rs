use anyhow::Ok;
use anyhow::Result;
pub mod event;
pub mod models;
pub mod store;
pub mod ui;

use ui::manager::UiManager;

fn main() -> Result<()> {
    let mut ui_manager = UiManager::new();
    ui_manager.run()?;
    Ok(())
}
