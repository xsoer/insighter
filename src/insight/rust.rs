use super::utils::{is_hidden, padding_space};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use walkdir::WalkDir;

/// 定义文件架构
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
pub fn run(path: &Path, output: &str) {
    let res = walk_dir(path);
    let files = res.unwrap();

    let mut outline = String::from("## 目录结构\n\n");
    let mut content = String::from("\n## 文件结构\n");
    let mut total_lines = 0;
    let mut file_types = HashMap::new();

    for file in files {
        let name = padding_space(file.depth + 1) + file.name.as_str() + "\n";
        outline.push_str(name.as_str());

        // 不是文件类型的就过滤掉
        if file.typs != "file" {
            continue;
        }
        let suffix = file.suffix.clone();

        if file_types.contains_key(&suffix) {
            let num = file_types.get_mut(&suffix).unwrap();
            *num += 1;
        } else {
            file_types.insert(suffix, 1);
        }

        if file.suffix != "rs" {
            continue;
        }

        let name = String::from("\n")
            + padding_space(file.depth - 1).as_str()
            + file.parent.as_str()
            + "\n"
            + padding_space(file.depth).as_str()
            + file.name.as_str()
            + "  ";
        content.push_str(name.as_str());
        let res = read_outline(file.path, file.depth + 1).unwrap();
        let total_line = res.get("total_line").unwrap();
        total_lines += total_line.parse::<u32>().unwrap();
        content.push_str(&(total_line.to_string() + "行\n"));
        content.push_str(res.get("content").unwrap().as_str());
    }

    let mut file_genre = String::from("\n## 文件类型: \n");
    for (key, val) in file_types.iter() {
        file_genre.push_str(&(key.to_string() + ": "));
        file_genre.push_str(val.to_string().as_str());
        file_genre.push_str("\n");
    }

    let w = outline
        + "\n"
        + &file_genre
        + "\n共有"
        + &total_lines.to_string()
        + "行有效代码！\n"
        + content.as_str();

    writer(w, output).expect("写入outline失败");
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
    Ok(files)
}

fn read_outline(path: String, depath: usize) -> Result<HashMap<String, String>, io::Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);

    let mut s = String::new();
    let mut total_line = 0;

    for line in f.lines() {
        if let Ok(line) = line {
            // 先去除前后空格
            let line = line.trim();
            // 以开头名称匹配
            if line.starts_with("use")
                || line.starts_with("fn")
                || line.starts_with("struct")
                || line.starts_with("impl")
                || line.starts_with("enum")
                || line.starts_with("type")
                || line.starts_with("pub fn")
                || line.starts_with("pub struct")
                || line.starts_with("macro_rules!")
            {
                let l = padding_space(depath) + line + "\n";
                s.push_str(l.as_str())
            }
            // 有效行+1
            if !(line.is_empty() || line.starts_with("//") || line.starts_with("/*")) {
                total_line += 1;
            }
        }
    }
    let mut res = HashMap::new();
    res.insert("content".to_string(), s);
    res.insert("total_line".to_string(), total_line.to_string());
    Ok(res)
}

// 写入文件内容
fn writer(content: String, file_name: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .append(true)
        .truncate(true)
        .open(file_name);

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
