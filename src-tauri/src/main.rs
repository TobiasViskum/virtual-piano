// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::{ Duration, Instant };
use crossbeam_channel::{ bounded, Receiver, Sender };
use lazy_static::lazy_static;

use piano_listen::{ listen, play, Alpha, PianoEvent, PianoKeyCode, StateCode };
use tauri::Manager;

pub mod piano_listen;

lazy_static! {
    static ref LISTENER_STATE: Mutex<Option<ListenerState>> = Mutex::new(None);
    
    #[derive(Debug)]
    static ref RECORDINGS: Mutex<HashMap<String, Recording>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct Recording {
    pub recording: Vec<(Duration, Vec<u8>)>,
}

impl Recording {
    pub fn new() -> Self {
        Self { recording: Vec::new() }
    }

    pub fn from(recording: Vec<(Duration, Vec<u8>)>) -> Self {
        Self { recording }
    }

    pub fn push(&mut self, chunk: (Duration, Vec<u8>)) {
        self.recording.push(chunk)
    }
}

struct ListenerState {
    handle: thread::JoinHandle<Option<Recording>>,
    stop_sender: Sender<()>,
}

#[tauri::command]
fn play_recording() -> bool {
    let recording = { RECORDINGS.lock().unwrap().get("First recording").unwrap().clone() };
    let handle = thread::spawn(move || {
        play(recording).unwrap();
    });
    handle.join().unwrap();
    true
}

#[tauri::command]
fn is_listening() -> bool {
    let listener_state = LISTENER_STATE.lock().expect("Error when locking");
    listener_state.is_some()
}

#[tauri::command]
fn end_piano_recording(name: String) -> bool {
    let mut listener_state = LISTENER_STATE.lock().expect("Error when locking");
    if let Some(listener_state) = listener_state.take() {
        let _ = listener_state.stop_sender.send(());
        let recording = listener_state.handle.join().unwrap().unwrap();
        RECORDINGS.lock().unwrap().insert(name, recording);
    }

    true
}

#[tauri::command]
fn spawn_piano_recorder(app: tauri::AppHandle) -> bool {
    let app = Arc::new(Mutex::new(app));
    kill_piano_listener();

    let handler = {
        let app = Arc::clone(&app);
        move |piano_event: Result<PianoEvent, String>| {
            match piano_event {
                Ok(piano_event) => {
                    let event = piano_event.to_client_event();
                    let app = app.lock().expect("Failed to lock AppHandle");
                    app.emit("pianoevent", event).expect("Failed to emit event");
                }
                Err(e) => { println!("Error: {}", e) }
            }
        }
    };

    let (stop_sender, stop_receiver) = bounded(1);

    let handle = thread::spawn(move || {
        let recording = listen(handler, true, stop_receiver).expect("Error when listening");
        Some(Recording::from(recording.unwrap().lock().unwrap().recording.clone()))
    });

    let mut listener_state = LISTENER_STATE.lock().expect("Error when locking");
    *listener_state = Some(ListenerState { handle, stop_sender });

    true
}

#[tauri::command]
fn spawn_piano_listener(app: tauri::AppHandle) -> bool {
    {
        let listener_state = LISTENER_STATE.lock().expect("Error when locking");
        if listener_state.is_some() {
            println!("Already listening!");
            return false;
        }
    }

    let app = Arc::new(Mutex::new(app));

    let handler = {
        let app = Arc::clone(&app);
        move |piano_event: Result<PianoEvent, String>| {
            match piano_event {
                Ok(piano_event) => {
                    let event = piano_event.to_client_event();
                    let app = app.lock().expect("Failed to lock AppHandle");
                    app.emit("pianoevent", event).expect("Failed to emit event");
                }
                Err(e) => { println!("Error: {}", e) }
            }
        }
    };
    let (stop_sender, stop_receiver) = bounded(1);

    let handle = thread::spawn(move || {
        listen(handler, false, stop_receiver).expect("Error when listening");

        None
    });

    let mut listener_state = LISTENER_STATE.lock().expect("Error when locking");
    *listener_state = Some(ListenerState { handle, stop_sender });

    true
}

#[tauri::command]
fn kill_piano_listener() -> bool {
    println!("Began killing connection");

    let mut listener_state = LISTENER_STATE.lock().expect("Error when locking");
    if let Some(listener_state) = listener_state.take() {
        let _ = listener_state.stop_sender.send(()).unwrap();
        drop(listener_state.stop_sender);
        let _ = listener_state.handle.join();
    }

    // Why does this never print
    println!("Killed connection");

    true
}

// Context menus:
// https://github.com/tauri-apps/muda

fn main() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(
            tauri::generate_handler![
                spawn_piano_listener,
                kill_piano_listener,
                spawn_piano_recorder,
                end_piano_recording,
                is_listening,
                play_recording
            ]
        )

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
