use std::{ fs, env };
use std::path::Path;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let contents = fs::read_dir(current_dir).unwrap();

    for item in contents {
        let item = item.unwrap().path();

        let item_filename = Path::new(item.file_name().unwrap()).display();

        match item.is_dir() {
            true => println!("{}", item_filename),
            false => println!("{}", item_filename)
        }
    }
}