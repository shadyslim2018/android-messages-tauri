#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};

/// Bring the main window to the front: show, unminimize, focus.
/// Also updates the tray label and visibility flag.
fn show_main<R: Runtime>(
    win: &WebviewWindow<R>,
    toggle_item: &MenuItem<R>,
    is_visible: &Arc<Mutex<bool>>,
) {
    if let Err(e) = win.show() {
        println!("[show_main] show() error: {e}");
    }
    if let Err(e) = win.unminimize() {
        // Some WMs may error if not minimized; harmless.
        println!("[show_main] unminimize() (maybe harmless): {e}");
    }
    if let Err(e) = win.set_focus() {
        println!("[show_main] set_focus() error: {e}");
    }
    if let Err(e) = toggle_item.set_text("Hide Window") {
        println!("[show_main] toggle_item.set_text error: {e}");
    }
    *is_visible.lock().unwrap() = true;
    println!("[show_main] done (visible=true)");
}

/// Hide the main window and update label/state.
fn hide_main<R: Runtime>(
    win: &WebviewWindow<R>,
    toggle_item: &MenuItem<R>,
    is_visible: &Arc<Mutex<bool>>,
) {
    if let Err(e) = win.hide() {
        println!("[hide_main] hide() error: {e}");
    }
    if let Err(e) = toggle_item.set_text("Show Window") {
        println!("[hide_main] toggle_item.set_text error: {e}");
    }
    *is_visible.lock().unwrap() = false;
    println!("[hide_main] done (visible=false)");
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Track whether the window is visible
            let is_visible = Arc::new(Mutex::new(true));

            // Create the main window
            let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Android Messages")
                .inner_size(800.0, 600.0)
                .build()?;

            // Tray menu items (toggle wrapped in Arc so we can share it across closures)
            let toggle_item = Arc::new(MenuItem::new(app, "toggle", true, None::<&str>)?);
            toggle_item.set_text("Hide Window")?;
            let quit_item = MenuItem::new(app, "quit", true, None::<&str>)?;

            // Build tray menu
            let tray_menu = Menu::with_items(app, &[&*toggle_item, &quit_item])?;

            // Keep the actual MenuIds so we compare by id (not strings)
            let toggle_id = toggle_item.id().clone();
            let quit_id = quit_item.id().clone();

            // Intercept close -> hide to tray and update label/state
            {
                let win_for_handler = window.clone();
                let toggle_for_close = toggle_item.clone();
                let vis_for_close = is_visible.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        hide_main(&win_for_handler, &toggle_for_close, &vis_for_close);
                    }
                });
            }

            // Tray icon + handlers
            {
                let toggle_for_handlers = toggle_item.clone();
                let vis_for_handlers = is_visible.clone();
                let toggle_id_for_handlers = toggle_id.clone();
                let quit_id_for_handlers = quit_id.clone();

                TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&tray_menu)
                    // Right-click tray menu
                    .on_menu_event({
                        let toggle = toggle_for_handlers.clone();
                        let vis = vis_for_handlers.clone();
                        let toggle_id = toggle_id_for_handlers.clone();
                        let quit_id = quit_id_for_handlers.clone();
                        move |app_handle, event| {
                            // Debug: see which id fired
                            println!("[menu] clicked id={:?}", event.id());
                            if event.id() == &toggle_id {
                                if let Some(win) = app_handle.get_webview_window("main") {
                                    let currently_visible = *vis.lock().unwrap();
                                    if currently_visible {
                                        hide_main(&win, &toggle, &vis);
                                    } else {
                                        show_main(&win, &toggle, &vis);
                                    }
                                } else {
                                    println!("[menu] main window not found");
                                }
                            } else if event.id() == &quit_id {
                                std::process::exit(0);
                            }
                        }
                    })
                    // Left-click tray icon toggles show/hide
                    .on_tray_icon_event({
                        let toggle = toggle_for_handlers.clone();
                        let vis = vis_for_handlers.clone();
                        move |tray, tray_event| match tray_event {
                            TrayIconEvent::Click { button, .. } if button == MouseButton::Left => {
                                if let Some(win) = tray.app_handle().get_webview_window("main") {
                                    let currently_visible = *vis.lock().unwrap();
                                    if currently_visible {
                                        hide_main(&win, &toggle, &vis);
                                    } else {
                                        show_main(&win, &toggle, &vis);
                                    }
                                } else {
                                    println!("[tray] main window not found");
                                }
                            }
                            _ => {}
                        }
                    })
                    .build(app)?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
