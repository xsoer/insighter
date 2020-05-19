#[warn(dead_code)]
use std::env;
use std::fs;
use std::path::Path;
use std::io;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
struct FileEntry {
    name: String,
    typs: String,
    parent: String,
    depth: usize,
    content: String
}

fn main() {
    // 必须赋予类型
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("参数必须至少是2个！");
        return;
    }

    let path = Path::new(&args[1]);

    let res = walk_dir(path);
    let files = res.unwrap();

    for file in files {
        if file.typs == "file" {
            println!("{}", file.content)
        }
    }

}

fn walk_dir(path: &Path) -> Result<Vec<FileEntry>, io::Error> {
    let mut files: Vec<FileEntry> = vec![];

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let path = entry.unwrap();
        let parent = path.path().parent().unwrap();
        let typs = if path.path().is_dir() { "dir" } else { "file" };
        let content = if path.path().is_dir() {
            String::from("dir")
        } else {
            let filename = path.path().to_string_lossy().into_owned();
            fs::read_to_string(filename).expect("Something went wrong reading the file")
        };

        let file = FileEntry {
            name: path.file_name().to_string_lossy().into_owned(),
            typs: String::from(typs),
            parent: parent.file_name().unwrap().to_string_lossy().into_owned(),
            depth: path.depth(),
            content: content
        };
        files.push(file);
    }
    // println!("{:#?}", files);
    Ok(files)
}

// 设计隐藏的文件或目录
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".") || s.starts_with("target") || s.ends_with(".jpg") || s.ends_with(".png"))
        .unwrap_or(false)
}
