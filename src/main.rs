#[warn(dead_code)]
use std::env;
use std::path::Path;
use walkdir::WalkDir;
use std::io;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct FileTree {
    name: String,
    children: Vec<Option<Box<FileTree>>>,
    files: Vec<String>
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
    walk_dir_tree(path)

    // println!("root-->{:#?}", root)

}


fn walk_dir_tree(path: &Path) {
    let mut file_tree = FileTree {
        name: path.to_str().unwrap().to_string(),
        children: vec![],
        files: vec![]
    };
    dir_tree(path, &mut file_tree);
    println!("{:#?}", file_tree)
}

fn dir_tree(p: &Path, file_tree: &mut FileTree) -> io::Result<()> {
    if p.is_dir() {
        file_tree.name = p.to_str().unwrap().to_string();
        for entry in fs::read_dir(p)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut child = FileTree{
                    name: path.to_str().unwrap().to_string(),
                    children:vec![],
                    files: [].to_vec()
                };
                file_tree.children.push(Some(Box::new(child)));
                dir_tree(&path, &mut child)?;
            } else {
                // cb(&entry);
                let p = path.to_str().unwrap().to_string();
                file_tree.files.push(p);
            }
        }
    }
    Ok(())
}


fn walk_dir(path: &Path) {
    let mut root: Vec<String> = vec![];

    let entries = WalkDir::new(path);

    for entry in WalkDir::new(path) {
        let path = entry.unwrap();
        if path.path().is_dir() {
            root.push(path.path().to_string_lossy().into_owned());
        }
        println!("{}---> {:#?}-->{}", path.path().display(), path.path().parent(), path.path().is_dir());
    }
}