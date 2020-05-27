use walkdir::DirEntry;

// 设计隐藏的文件或目录
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            s.starts_with(".")
                || s.starts_with("target")
                || s.ends_with(".jpg")
                || s.ends_with(".png")
        })
        .unwrap_or(false)
}

pub fn padding_space(i: usize) -> String {
    let mut s = String::new();
    for _n in 0..i * 4 {
        s += " ";
    }
    s
}
