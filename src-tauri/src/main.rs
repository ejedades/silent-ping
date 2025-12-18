// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use audio::SilentAudioPlayer;
use std::sync::mpsc::{self, Sender};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State,
};

/// Commands sent to the audio thread
enum AudioCommand {
    Start,
    Stop,
    IsPlaying(Sender<bool>),
}

/// Global application state (Send + Sync safe)
struct AppState {
    audio_tx: Sender<AudioCommand>,
}

#[tauri::command]
fn start_audio(state: State<AppState>) -> Result<bool, String> {
    state
        .audio_tx
        .send(AudioCommand::Start)
        .map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
fn stop_audio(state: State<AppState>) -> Result<bool, String> {
    state
        .audio_tx
        .send(AudioCommand::Stop)
        .map_err(|e| e.to_string())?;
    Ok(false)
}

#[tauri::command]
fn is_playing(state: State<AppState>) -> bool {
    let (tx, rx) = mpsc::channel();
    let _ = state.audio_tx.send(AudioCommand::IsPlaying(tx));
    rx.recv().unwrap_or(false)
}

fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            create_tray(app.handle())?;

            // Audio thread
            let (tx, rx) = mpsc::channel::<AudioCommand>();

            std::thread::spawn(move || {
                let mut player = SilentAudioPlayer::new();

                while let Ok(cmd) = rx.recv() {
                    match cmd {
                        AudioCommand::Start => {
                            let _ = player.start();
                        }
                        AudioCommand::Stop => {
                            player.stop();
                        }
                        AudioCommand::IsPlaying(reply_tx) => {
                            let _ = reply_tx.send(player.is_playing());
                        }
                    }
                }
            });

            app.manage(AppState { audio_tx: tx });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_audio,
            stop_audio,
            is_playing
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}