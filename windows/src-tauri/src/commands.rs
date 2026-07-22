//! Tauri command surface invoked from the web frontend.

use crate::dto::{self, RulesDTO, ScanResult};
use crate::ignore_store::IgnoreStore;
use crate::{actions, scan};
use std::sync::Mutex;
use tauri::State;

pub struct AppState(pub Mutex<IgnoreStore>);

#[tauri::command]
pub async fn scan_ports(state: State<'_, AppState>) -> Result<ScanResult, String> {
    // Scan shells out to PowerShell; keep it off the UI thread.
    let ports = tauri::async_runtime::spawn_blocking(scan::scan)
        .await
        .map_err(|e| e.to_string())?;

    let store = state.0.lock().map_err(|e| e.to_string())?;
    let dtos = ports
        .iter()
        .map(|p| dto::port_dto(p, store.is_ignored(p)))
        .collect();
    let rules = RulesDTO {
        processes: store.rules.processes.iter().cloned().collect(),
        ports: store.rules.ports.iter().copied().collect(),
    };
    Ok(ScanResult { ports: dtos, rules })
}

#[tauri::command]
pub fn ignore_process(state: State<AppState>, name: String) {
    if let Ok(mut s) = state.0.lock() {
        s.ignore_process(&name);
    }
}

#[tauri::command]
pub fn ignore_port(state: State<AppState>, port: u16) {
    if let Ok(mut s) = state.0.lock() {
        s.ignore_port(port);
    }
}

#[tauri::command]
pub fn unignore_process(state: State<AppState>, name: String) {
    if let Ok(mut s) = state.0.lock() {
        s.unignore_process(&name);
    }
}

#[tauri::command]
pub fn unignore_port(state: State<AppState>, port: u16) {
    if let Ok(mut s) = state.0.lock() {
        s.unignore_port(port);
    }
}

#[tauri::command]
pub fn unignore_matching(state: State<AppState>, port: u16, command: String, display_name: String) {
    if let Ok(mut s) = state.0.lock() {
        s.unignore_matching(port, &command, &display_name);
    }
}

#[tauri::command]
pub fn kill(pid: i32) -> bool {
    actions::kill(pid)
}

#[tauri::command]
pub fn open_url(url: String) {
    actions::open_url(&url);
}
