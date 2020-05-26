use super::utils::{is_hidden, padding_space};
use std::ffi::OsStr;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct FileEntry {
    name: String,
    typs: String,
    parent: String,
    depth: usize,
    path: String,
    suffix: String,
}

// 进行路径遍历，打印文件结构及代码框架
pub fn run(path: &Path) {
    let res = walk_dir(path);
    let files = res.unwrap();

    let mut outline = String::from("===========目录结构============\n\n");

    let mut content = String::from("\n===========文件结构===========\n");

    for file in files {
        let name = padding_space(file.depth) + file.name.as_str() + "\n";
        outline.push_str(name.as_str());

        if file.suffix == "rs" {
            let name = String::from("\n")
                + padding_space(file.depth - 1).as_str()
                + file.parent.as_str()
                + "\n"
                + padding_space(file.depth).as_str()
                + file.name.as_str()
                + "\n";

            content.push_str(name.as_str());
            let res = read_outline(file.path, file.depth + 1);
            content.push_str(res.unwrap().as_str());
        }
    }

    let w = outline + content.as_str();
    writer(w).expect("写入outline失败");
    println!("解析目录完毕")
}

// 遍历目录文件
fn walk_dir(path: &Path) -> Result<Vec<FileEntry>, io::Error> {
    let mut files: Vec<FileEntry> = vec![];

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let path = entry.unwrap();
        let parent = path.path().parent().unwrap();
        let typs = if path.path().is_dir() { "dir" } else { "file" };

        let file = FileEntry {
            name: path.file_name().to_string_lossy().into_owned(),
            typs: String::from(typs),
            parent: parent.file_name().unwrap().to_string_lossy().into_owned(),
            depth: path.depth(),
            path: path.path().to_string_lossy().into_owned(),
            suffix: path
                .path()
                .extension()
                .unwrap_or_else(|| OsStr::new(""))
                .to_string_lossy()
                .into_owned(),
        };
        files.push(file);
    }
    // println!("{:#?}", files);
    Ok(files)
}

// 读取文件
// fn reader(path: String) -> io::Result<()> {
//     let f = File::open(path)?;
//     let f = BufReader::new(f);

//     for line in f.lines() {
//         if let Ok(line) = line {
//             println!("{:?}", line);
//         }
//     }
//     Ok(())
// }

fn read_outline(path: String, depath: usize) -> Result<String, io::Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    let mut s = String::new();

    for line in f.lines() {
        if let Ok(line) = line {
            if line.starts_with("use") || line.starts_with("fn") || line.starts_with("struct") {
                let l = padding_space(depath) + line.as_str() + "\n";
                s.push_str(l.as_str())
            }
        }
    }
    Ok(s)
}

// 写入文件内容
fn writer(content: String) -> io::Result<()> {
    let dir_name = "output";
    let file_name = "outline.md";

    let output_file = String::from(dir_name) + "/" + file_name;

    if !Path::new(&dir_name).exists() {
        fs::create_dir(dir_name)?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .append(true)
        .truncate(true)
        .open(output_file);

    match file {
        Ok(mut stream) => {
            stream.write_all(content.as_str().as_bytes())?;
        }
        Err(err) => {
            println!("open file error.{:?}", err);
        }
    }
    Ok(())
}
