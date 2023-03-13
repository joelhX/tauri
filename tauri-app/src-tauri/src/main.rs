// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use screenshots::Screen;
use std::{fs,time::Instant,process::Command};
use tauri::SystemTray;
use std::os::windows::process::CommandExt;
//use captis::{init_capturer, Capturer};
#[tauri::command]
fn get_area(x:i32, y:i32, w:u32, h:u32) -> String {
    let start = Instant::now();
    let screens = Screen::all().unwrap();
    let image = screens[0].capture_area(x, y, w, h).unwrap();
    let buffer = image.buffer();
    fs::write(format!("cap.png" ), buffer).unwrap();
    let elapsed = start.elapsed().as_millis();
    println!("{},{},{},{}",x, y, w, h);

    let out = Command::new("tesseract")
    .args(["cap.png","stdout","-l","kor+eng","--psm","4"])
    .creation_flags(0x08000000)
    .env(" user_defined_dpi", "128")
    .output()
    .expect("failed to execute process");
    let result=out.stdout;
    //println!("{}",String::from_utf8_lossy(&result));
    format!("elapsed {}\n ____text____\n{}", elapsed,String::from_utf8_lossy(&result))
}



fn main() {
    let tray = SystemTray::new();
    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![get_area])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
        
}
