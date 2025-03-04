use tauri::State;

use crate::{calendar, config::Config};

#[tauri::command]
pub async fn fetch_calendar_events(config: State<'_, Config>) -> Result<(), String> {
    return calendar::fetch_calendar_events(config.inner()).await;
}
