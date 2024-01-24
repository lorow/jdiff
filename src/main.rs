use anyhow::Result;
use anyhow::Ok;
pub mod event;
pub mod model;
pub mod store;
pub mod ui;
pub mod update;

use ui::manager::UiManager;

fn main() -> Result<()> {
    let mut ui_manager = UiManager::new();
    ui_manager.run()?;
    Ok(())
}
