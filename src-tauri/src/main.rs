// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs, path::Path};

static GB_CONTENT_ROOT: &'static str = r"C:\Program Files (x86)\Steam\steamapps\common\Ground Branch\GroundBranch\Content\GroundBranch";

static GB_INVENTORY_PATH: &'static str = r"C:\Program Files (x86)\Steam\steamapps\common\Ground Branch\GroundBranch\Content\GroundBranch\Inventory";
static GB_PATCHES_PATH: &'static str = r"C:\Program Files (x86)\Steam\steamapps\common\Ground Branch\GroundBranch\Content\GroundBranch\PATCHES";

fn get_mod_type(mod_name: &str) {
    let mod_path = env::current_dir()
        .unwrap()
        .join(r"C:\Users\conno\Desktop\workbench\gb-mod-manager\src\mods\")
        .join(mod_name);
    todo!()
}

fn is_inventory_dir(dir_name: &str) -> bool {
    let dir_name_segments = dir_name.split("\\").collect::<Vec<&str>>();
    let inventory_category_dirs = get_inventory_category_dirs();

    for category in inventory_category_dirs {
        if dir_name.contains(&category) & dir_name_segments.ends_with(&[&category]) {
            return true;
        }
    }
    false
}

fn mod_pathfinder(mod_name: &str) -> Option<String> {
    let mod_path = env::current_dir()
        .unwrap()
        .join(r"C:\Users\conno\Desktop\workbench\gb-mod-manager\src\mods\")
        .join(mod_name);

    if !mod_path.is_dir() {
        print!("Path is not a directory");
        return None;
    }

    for entry in fs::read_dir(mod_path).unwrap() {
        let path = entry.unwrap().path();
        if is_inventory_dir(&path.to_str().unwrap()) {
            println!("A path was most certainly found: {}", path.display());
            return Some(path.to_str().unwrap().to_string());
        } else if path.is_dir() {
            mod_pathfinder(path.to_str().unwrap());
            
        }
    }
    None
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_if_dir_exists(path: &str) -> (String, bool) {
    (path.to_string(), Path::new(path).is_dir())
}

#[tauri::command]
fn read_mods_dir() -> Vec<String> {
    let mods_dir = env::current_dir()
        .unwrap()
        .join(r"C:\Users\conno\Desktop\workbench\gb-mod-manager\src\mods");

    let mut mods = Vec::new();
    for entry in fs::read_dir(mods_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        mods.push(path.file_name().unwrap().to_str().unwrap().to_string());
    }
    mods
}

#[tauri::command]
fn copy_mod(mod_name: &str) {
    // let mod_path = env::current_dir()
    //     .unwrap()
    //     .join(r"C:\Users\conno\Desktop\workbench\gb-mod-manager\src\mods\")
    //     .join(mod_name);

    let path_to_copy = mod_pathfinder(&mod_name);

    match path_to_copy {
        Some(path) => {
            println!("Path to copy: {}", path);
        }
        None => println!("No path found"),
    }
}

#[tauri::command]
fn get_inventory_category_dirs() -> Vec<String> {
    let dir = env::current_dir().unwrap().join(GB_INVENTORY_PATH);

    let mut inventory_category_dirs = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        inventory_category_dirs.push(path.file_name().unwrap().to_str().unwrap().to_string());
    }
    inventory_category_dirs
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            check_if_dir_exists,
            read_mods_dir,
            copy_mod,
            get_inventory_category_dirs
        ])
        .plugin(tauri_plugin_fs_watch::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
