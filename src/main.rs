mod insight;

use clap::{load_yaml, App};
use insight::rust::run;
use std::collections::HashMap;
use std::env;
use std::path::Path;

fn main() {
    let options = cli_options();
    let dir = options.get("dir").unwrap().as_str();
    let output = options.get("output").unwrap().as_str();

    let mut cur_path = env::current_dir().unwrap();

    // 判断当前路径
    let path = if dir.starts_with("/") {
        Path::new(dir)
    } else {
        cur_path.push(dir);
        cur_path.as_path()
    };

    if !path.exists() {
        println!("文件夹不存在:{:#?}", path);
        return;
    }

    run(path, output);
}

// 命令执行选项
fn cli_options() -> HashMap<String, String> {
    let yaml = load_yaml!("conf/cli.yml");
    let matches = App::from(yaml).get_matches();

    let mut options = HashMap::new();

    options.insert(
        "dir".to_string(),
        matches.value_of("DIR").unwrap().to_string(),
    );
    options.insert(
        "output".to_string(),
        matches
            .value_of("output")
            .unwrap_or("insighter.md")
            .to_string(),
    );

    options
}