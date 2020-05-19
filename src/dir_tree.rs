
use std::io;

type AvlTreeNode = Option<Box<FileTree>>;

#[derive(Debug, Clone)]
struct FileTree {
    name: String,
    children: Vec<AvlTreeNode>,
    files: Vec<String>,
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
