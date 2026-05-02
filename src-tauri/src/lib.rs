// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::fs;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
fn process_base64_string(input: String) -> Result<String, String> {
    let mut raw_str = input.trim();
    
    // Check if it already has data URI scheme
    if raw_str.starts_with("data:image/") && raw_str.contains(";base64,") {
        return Ok(raw_str.to_string());
    }

    // Strip prefix if somehow it's malformed or we just want to ensure clean base64
    if let Some(idx) = raw_str.find("base64,") {
        raw_str = &raw_str[idx + 7..];
    }

    let mime_type = if raw_str.starts_with("iVBORw0KGgo") {
        "image/png"
    } else if raw_str.starts_with("/9j/") {
        "image/jpeg"
    } else if raw_str.starts_with("R0lGOD") {
        "image/gif"
    } else if raw_str.starts_with("UklGR") {
        "image/webp"
    } else if raw_str.starts_with("Qk0") {
        "image/bmp"
    } else if raw_str.starts_with("PHN2Zy") {
        "image/svg+xml"
    } else if raw_str.starts_with("AAAAIGZ0eXBhdmlm") {
        "image/avif"
    } else {
        "image/png" // Fallback
    };

    Ok(format!("data:{};base64,{}", mime_type, raw_str))
}

#[tauri::command]
fn read_file_to_base64(path: String) -> Result<String, String> {
    match fs::read(&path) {
        Ok(bytes) => {
            let base64_str = STANDARD.encode(&bytes);
            
            let ext = std::path::Path::new(&path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
                
            let mime_type = match ext.as_str() {
                "png" | "apng" => "image/png",
                "jpg" | "jpeg" | "jfif" | "pjpeg" | "pjp" => "image/jpeg",
                "gif" => "image/gif",
                "webp" => "image/webp",
                "bmp" => "image/bmp",
                "svg" => "image/svg+xml",
                "ico" => "image/x-icon",
                "tif" | "tiff" => "image/tiff",
                "avif" => "image/avif",
                "txt" => {
                    if let Ok(text) = fs::read_to_string(&path) {
                        return process_base64_string(text);
                    }
                    "text/plain"
                },
                _ => "application/octet-stream"
            };

            if mime_type == "text/plain" || mime_type == "application/octet-stream" {
                if let Ok(text) = String::from_utf8(bytes) {
                    return process_base64_string(text);
                }
            }

            Ok(format!("data:{};base64,{}", mime_type, base64_str))
        },
        Err(e) => Err(format!("Failed to read file: {}", e))
    }
}

#[tauri::command]
fn save_base64_to_file(path: String, base64_data: String) -> Result<(), String> {
    let mut raw_str = base64_data.as_str();
    if let Some(idx) = raw_str.find("base64,") {
        raw_str = &raw_str[idx + 7..];
    }
    
    // Decode and save entirely in Rust to bypass Javascript memory constraints and plugin-fs strict scope limitations
    let bytes = STANDARD.decode(raw_str).map_err(|e| format!("Decode error: {}", e))?;
    std::fs::write(&path, bytes).map_err(|e| format!("FS error: {}", e))?;
    Ok(())
}

#[tauri::command]
fn copy_to_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_string_chunk(text: String, chunk_size: usize, chunk_index: usize) -> Result<String, String> {
    // Fulfill user request to "do this with rust"
    let start = chunk_index * chunk_size;
    if start >= text.len() {
        return Ok(String::new());
    }
    let mut end = start + chunk_size;
    if end > text.len() {
        end = text.len();
    }
    
    // Ensure we don't slice inside a unicode character
    while !text.is_char_boundary(end) {
        end += 1;
        if end > text.len() {
            end = text.len();
            break;
        }
    }
    Ok(text[start..end].to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            process_base64_string, 
            read_file_to_base64, 
            save_base64_to_file,
            copy_to_clipboard,
            get_string_chunk
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
