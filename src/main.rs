#[warn(dead_code)]
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

type AvlTreeNode = Option<Box<FileTree>>;

#[derive(Debug, Clone)]
struct FileTree {
    name: String,
    children: Vec<AvlTreeNode>,
    files: Vec<String>,
}

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
    println!("{:#?}", path);
    // walk_dir_tree(path)
    walk_dir(path)

    // println!("root-->{:#?}", root)
}

fn walk_dir_tree(path: &Path) {
    let mut file_tree = FileTree {
        name: path.to_str().unwrap().to_string(),
        children: [].to_vec(),
        files: vec![],
    };
    let res = dir_tree(path, &mut file_tree);
    println!("{:#?}", res);
    println!("file tree {:#?}", file_tree)
}

fn dir_tree(p: &Path, file_tree: &mut FileTree) -> io::Result<()> {
    if p.is_dir() {
        file_tree.name = p.to_str().unwrap().to_string();
        for entry in fs::read_dir(p)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut child = FileTree {
                    name: path.to_str().unwrap().to_string(),
                    children: [].to_vec(),
                    files: [].to_vec(),
                };
                // file_tree.children.push(Some(Box::new(child)));
                dir_tree(&path, &mut child);
            } else {
                let p = path.to_str().unwrap().to_string();
                file_tree.files.push(p);
            }
        }
    }
    Ok(())
}

fn walk_dir(path: &Path) {
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

    println!("{:#?}", files);
}

// 设计隐藏的文件或目录
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".") || s.starts_with("target") || s.ends_with(".jpg") || s.ends_with(".png"))
        .unwrap_or(false)
}
