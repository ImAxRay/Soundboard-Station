#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

static CTRL_PRESSED: AtomicBool = new_atomic();
static ALT_PRESSED: AtomicBool = new_atomic();
static SHIFT_PRESSED: AtomicBool = new_atomic();

const fn new_atomic() -> AtomicBool {
    AtomicBool::new(false)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit Aura", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show Soundboard", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                window.open_devtools();
            }

            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                if let Err(error) = rdev::listen(move |event| {
                    match event.event_type {
                        rdev::EventType::KeyPress(key) => {
                            match key {
                                rdev::Key::ControlLeft | rdev::Key::ControlRight => {
                                    CTRL_PRESSED.store(true, Ordering::Relaxed);
                                    return;
                                }
                                rdev::Key::Alt | rdev::Key::AltGr => {
                                    ALT_PRESSED.store(true, Ordering::Relaxed);
                                    return;
                                }
                                rdev::Key::ShiftLeft | rdev::Key::ShiftRight => {
                                    SHIFT_PRESSED.store(true, Ordering::Relaxed);
                                    return;
                                }
                                _ => {}
                            }

                            let mut parts = Vec::new();
                            if CTRL_PRESSED.load(Ordering::Relaxed) {
                                parts.push("Ctrl");
                            }
                            if ALT_PRESSED.load(Ordering::Relaxed) {
                                parts.push("Alt");
                            }
                            if SHIFT_PRESSED.load(Ordering::Relaxed) {
                                parts.push("Shift");
                            }

                            if let Some(key_str) = rdev_key_to_string(key) {
                                parts.push(&key_str);
                                let combo = parts.join(" + ");
                                let _ = app_handle.emit("trigger-global-hotkey", combo);
                            }
                        }
                        rdev::EventType::KeyRelease(key) => {
                            match key {
                                rdev::Key::ControlLeft | rdev::Key::ControlRight => {
                                    CTRL_PRESSED.store(false, Ordering::Relaxed);
                                }
                                rdev::Key::Alt | rdev::Key::AltGr => {
                                    ALT_PRESSED.store(false, Ordering::Relaxed);
                                }
                                rdev::Key::ShiftLeft | rdev::Key::ShiftRight => {
                                    SHIFT_PRESSED.store(false, Ordering::Relaxed);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }) {
                    eprintln!("Low-level hook error: {:?}", error);
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let _ = window.hide();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn rdev_key_to_string(key: rdev::Key) -> Option<String> {
    match key {
        rdev::Key::KeyA => Some("A".into()),
        rdev::Key::KeyB => Some("B".into()),
        rdev::Key::KeyC => Some("C".into()),
        rdev::Key::KeyD => Some("D".into()),
        rdev::Key::KeyE => Some("E".into()),
        rdev::Key::KeyF => Some("F".into()),
        rdev::Key::KeyG => Some("G".into()),
        rdev::Key::KeyH => Some("H".into()),
        rdev::Key::KeyI => Some("I".into()),
        rdev::Key::KeyJ => Some("J".into()),
        rdev::Key::KeyK => Some("K".into()),
        rdev::Key::KeyL => Some("L".into()),
        rdev::Key::KeyM => Some("M".into()),
        rdev::Key::KeyN => Some("N".into()),
        rdev::Key::KeyO => Some("O".into()),
        rdev::Key::KeyP => Some("P".into()),
        rdev::Key::KeyQ => Some("Q".into()),
        rdev::Key::KeyR => Some("R".into()),
        rdev::Key::KeyS => Some("S".into()),
        rdev::Key::KeyT => Some("T".into()),
        rdev::Key::KeyU => Some("U".into()),
        rdev::Key::KeyV => Some("V".into()),
        rdev::Key::KeyW => Some("W".into()),
        rdev::Key::KeyX => Some("X".into()),
        rdev::Key::KeyY => Some("Y".into()),
        rdev::Key::KeyZ => Some("Z".into()),
        rdev::Key::Num1 => Some("1".into()),
        rdev::Key::Num2 => Some("2".into()),
        rdev::Key::Num3 => Some("3".into()),
        rdev::Key::Num4 => Some("4".into()),
        rdev::Key::Num5 => Some("5".into()),
        rdev::Key::Num6 => Some("6".into()),
        rdev::Key::Num7 => Some("7".into()),
        rdev::Key::Num8 => Some("8".into()),
        rdev::Key::Num9 => Some("9".into()),
        rdev::Key::Num0 => Some("0".into()),
        rdev::Key::Space => Some("Space".into()),
        _ => None,
    }
}