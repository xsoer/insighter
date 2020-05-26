mod insight;

use clap::{load_yaml, App};
use insight::rust::run;
// use std::env;
use std::path::Path;

fn main() {
    let dir = options();

    let path = Path::new(dir.as_str());
    if path.is_relative() {
        panic!("路径必须是绝对的，不能是相对路径")
    }

    run(path);
}

fn options() -> String {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    String::from(matches.value_of("DIR").unwrap())
}

// fn arg() {
// let args: Vec<String> = env::args().collect();
// if args.len() < 2 {
// println!("参数必须至少是2个！");
// return;
// }
// }
