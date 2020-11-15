mod conf;
mod insight;

use conf::cmd::cli_options;
use insight::rust;
use std::env;
use std::path::Path;

fn main() {
    let options = cli_options();
    let dir = options.get("dir").unwrap().as_str();
    let output = options.get("output").unwrap().as_str();

    let mut cur_path = env::current_dir().unwrap();
    let path = if dir.starts_with("/") {
        Path::new(dir)
    } else {
        cur_path.push(dir);
        cur_path.as_path()
    };

    if !path.exists() {
        println!("^<o>^ 目标文件夹不存在:{:#?}", path);
        return
    }

    rust::run(path, output);
}
