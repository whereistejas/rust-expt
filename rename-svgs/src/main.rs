use std::fs::read_dir;

fn main() {
    let entries = read_dir("./icons/").unwrap();

    for entry in entries {
        let file = entry.unwrap();
        let path = file.path();

        if path.extension().unwrap() == "svg" {
            let old_name = path.file_name().unwrap().to_str().unwrap();
            let new_name = get_new_file_name(old_name);

            let new_path = path.to_str().unwrap().replace(old_name, new_name);
            if !new_name.is_empty() {
                println!("Renaming {old_name} to {new_name}");
                std::fs::rename(path, new_path).expect("Failed to rename file.");
            }
        }
    }
}

fn get_new_file_name(old_name: &str) -> &'static str {
    match old_name {
        _ => "",
    }
}
