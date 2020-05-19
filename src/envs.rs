
fn read_dir(path: &Path) -> io::Result<()> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.
    println!("{:#?}", entries);
    Ok(())
}

fn print_file(entry: &DirEntry) {
    println!(
        "files-->{:#?}, file_name--->{:#?}, file_type--->{:#?}",
        entry.path(),
        entry.file_name(),
        entry.file_type()
    );
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // println!("{:#?}", path);
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn iter_env() {
    for arg in env::args() {
        println!("{}", arg);
    }
}

fn iter_env_vars() {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}

fn dirs() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display());
}
