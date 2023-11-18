// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn echo_cmd(text: &str) -> String {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "echo", text])
      .output()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg(format!("echo {}", text))
      .output()
      .expect("failed to execute process")
  };

  format!("echo: {}", String::from_utf8_lossy(&output.stdout))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![echo_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
