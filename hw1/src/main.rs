use std::env;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    
    if args.len() > 1 {
        let path = &args[1];
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                println!("{}", entry.path().display());
            }
        }
    }
    
    
}
