use std::io;
use std::fs;

fn crawl_dir_tree(path: &str, spaces: usize) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let filename = entry.file_name();

        let is_entry_dir = entry.file_type()?.is_dir();

        if let Some(file_name) = filename.to_str() {
            let mut dir_spaces = spaces;
            while dir_spaces > 0 {
                print!("| ");
                dir_spaces -= 1;
            }

            print!("|- {file_name}\n");

            if is_entry_dir == true {
                if let Some(file_path) = entry.path().to_str() {
                    crawl_dir_tree(file_path, spaces + 1)?;
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Ok(_) = crawl_dir_tree(".", 0) {
        println!("Everything works fine");
    };
}
