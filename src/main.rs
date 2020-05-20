mod insight;

use insight::rust::run;
use std::env;
use std::path::Path;

fn main() {
    // 必须赋予类型
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("参数必须至少是2个！");
        return;
    }

    let path = Path::new(&args[1]);

    if path.is_relative() {
        panic!("路径必须是绝对的，不能是相对路径")
    }

    run(path);
}
