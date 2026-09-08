use tauri::{AppHandle, Emitter, Manager, Position, Size, WebviewWindow};

fn main_window(app_handle: &AppHandle) -> Option<WebviewWindow> {
    app_handle.get_webview_window("main")
}

fn present(window: &WebviewWindow) -> tauri::Result<()> {
    window.set_always_on_top(true)?;
    let _ = window.set_skip_taskbar(true);
    window.show()?;
    window.set_focus()?;
    Ok(())
}

pub fn init(app_handle: &tauri::AppHandle) -> tauri::Result<()> {
    if let Some(window) = main_window(app_handle) {
        present(&window)?;
    }
    Ok(())
}

pub fn toggle_panel(app_handle: &AppHandle) {
    let Some(window) = main_window(app_handle) else {
        return;
    };
    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        _ => {
            if let Err(error) = present(&window) {
                log::error!("failed to show panel: {error}");
            }
        }
    }
}

pub fn show_panel_from_tray(app_handle: &AppHandle, view: &str) {
    let Some(window) = main_window(app_handle) else {
        return;
    };
    if let Err(error) = present(&window) {
        log::error!("failed to show panel from tray: {error}");
        return;
    }
    if let Err(error) = app_handle.emit("tray:navigate", view) {
        log::error!("failed to emit tray:navigate {view}: {error}");
    }
}

pub fn close_window_and_show_panel(
    app_handle: &AppHandle,
    label: &str,
    view: &str,
) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window(label) {
        window
            .close()
            .map_err(|error| format!("failed to close {label} window: {error}"))?;
    }
    show_panel_from_tray(app_handle, view);
    Ok(())
}

pub fn toggle_panel_at_tray_rect(
    app_handle: &AppHandle,
    _icon_position: Position,
    _icon_size: Size,
) {
    let Some(window) = main_window(app_handle) else {
        return;
    };
    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        _ => {
            if let Err(error) = present(&window) {
                log::error!("failed to show panel at tray: {error}");
                return;
            }
            if let Err(error) = app_handle.emit("tray:navigate", "home") {
                log::error!("failed to emit tray:navigate home: {error}");
            }
        }
    }
}
