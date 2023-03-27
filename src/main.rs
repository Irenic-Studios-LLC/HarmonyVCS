use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        return;
    }

    match args[1].as_str() {
        "init" => init_repository(),
        "add" => {
            if args.len() < 3 {
                println!("Usage: {} add <file>", args[0]);
            } else {
                add_file(&args[2]);
            }
        }
        _ => println!("Unknown command"),
    }
}

fn init_repository() {
    let repo_path = Path::new(".hvcs");
    if !repo_path.exists() {
        fs::create_dir(repo_path).expect("Failed to create .harmonyvcs directory");
        println!("Initialized empty HarmonyVCS repository");
    } else {
        println!("Repository already exists");
    }
}

fn add_file(file_path: &str) {
    let path = Path::new(file_path);
    if !path.exists() {
        println!("File not found: {}", file_path);
        return;
    }
    let repo_path = Path::new(".hvcs");

    if !repo_path.exists() {
        println!("Not a HarmonyVCS repository (or any of the parent directories)");
        return;
    }
    let repo_path = Path::new(".hvcs");
    if !repo_path.exists() {
        println!("Not a HarmonyVCS repository (or any of the parent directories)");
        return;
    }
    
    // Copy the file to the HarmonyVCS repository.
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let dest_path = repo_path.join(file_name);

    match fs::copy(path, dest_path) {
    Ok(_) => println!("Added file: {}", file_path),
    Err(e) => println!("Failed to add file: {}", e),
    }
}
    
