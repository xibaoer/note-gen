// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use chrono::{DateTime, Utc};

// 生成文件名
fn generate_file_name() -> String {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.timestamp();
    format!("note_gen_{}.png", timestamp)
}

// 截图
#[tauri::command]
fn screenshot() -> String {
    let screenshot_path = std::env::temp_dir().join(generate_file_name());

    let _ = Command::new("screencapture")
        .arg("-i")
        .arg(screenshot_path.to_str().unwrap())
        .output();

    screenshot_path.to_str().unwrap().to_string()
}

// 关键词提取
use jieba_rs::Jieba;
use jieba_rs::{TfIdf, KeywordExtract};
#[tauri::command]
fn cut_words(str: String) -> Vec<String> {
    let jieba = Jieba::new();
    let keyword_extractor = TfIdf::default();
    let top_key = keyword_extractor.extract_keywords(
        &jieba,
        &str,
        5,
        vec![],
    );
    top_key.iter().map(|x| x.keyword.clone()).collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![screenshot, cut_words])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
