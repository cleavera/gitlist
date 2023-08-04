use std::{fs, path::PathBuf, env};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let cwd = args.get(1).map(|a| a.to_string()).unwrap_or(env::current_dir()?.to_str().expect("No directory provided").to_string());
    let repos = get_repos(&cwd)?;

    for r in repos {
        println!("{}", r)
    }

    Ok(())
}

fn get_repos(root: &str) -> Result<Vec<String>, std::io::Error> {
    let dirs = get_dirs(root)?;

    if dirs.len() == 0 {
        return Ok(vec![]);
    }

    if has_git(&dirs) {
        return Ok(vec![root.to_string()]);
    }

    return Ok(dirs.into_iter().flat_map(|dir| {
        return get_repos(&dir).into_iter();
    }).flatten().collect::<Vec<String>>());
}

fn get_dirs(root: &str) -> Result<Vec<String>, std::io::Error> {
    return Ok(fs::read_dir(root)?
              .into_iter()
              .filter_map(|entry| {
                let path: PathBuf = entry.ok()?.path();
                let metadata = fs::metadata(&path).ok()?;

                if !metadata.is_dir() {
                    return None;
                }

                return Some(path.to_str()?.to_string());
              }).collect::<Vec<String>>());
}

fn has_git(dirs: &Vec<String>) -> bool {
    return dirs.iter().any(|entry| {
        return entry.contains(".git");
    });
}
