use std::env;

use norad::{Font, designspace::DesignSpaceDocument};

fn main() {
    let wd = env::current_dir().expect("A current dir");
    let mut ds_file = wd.clone();
    ds_file.pop();
    ds_file.push("sources/ZenMaruGothic.designspace");
    if !ds_file.is_file() {
        panic!("{ds_file:?} isn't a file!");
    }
    let ds_doc = DesignSpaceDocument::load(&ds_file).expect("To parse designspace");
    let ds_dir = ds_file.parent().unwrap();
    println!("Loading {} sources...", ds_doc.sources.len());
    for source in ds_doc.sources.iter() {
        let mut ufo_dir = ds_dir.to_path_buf();
        ufo_dir.push(&source.filename);
        println!("  {ufo_dir:?}");
        let _font = Font::load(&ufo_dir).expect("To load sources");
    }
}
