extern crate globmatch;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn item_search(
    rootpath: &PathBuf,
    extension: &str,
    searchword: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let pattern = format!("**/*.{}", extension);

    let rootpath = Path::new(rootpath);

    let builder = globmatch::Builder::new(&pattern)
        .case_sensitive(false)
        .build(rootpath)?;

    let result =
        globmatch::IterAll::filter_entry(builder.into_iter(), |p| !globmatch::is_hidden_entry(p))
            .flatten()
            .filter(|x| is_matchword(searchword)(x))
            .collect::<Vec<_>>();

    Ok(result)
}

fn is_matchword<'a>(searchword: &'a str) -> impl Fn(&'a PathBuf) -> bool {
    // x:PathBufの中にsearchwordが含まれるかを判定する関数を返す。部分適用関数
    move |x: &PathBuf| {
        let path_string = x.display().to_string().to_lowercase();
        if searchword.is_empty() {
            return true;
        }

        for wd in searchword.split_whitespace() {
            if !path_string.contains(&wd.to_lowercase()) {
                return false;
            }
        }

        true
    }
}

pub fn opendir(fpath: &Path) -> Result<(), std::io::Error> {
    // 対象ファイルのフォルダをファインダーで開く
    #[cfg(target_os = "macos")]
    let parentpath = fpath.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "parent path isnot found")
    })?;

    Command::new("open").arg(parentpath).spawn()?;

    #[cfg(target_os = "windows")]
    Command::new("explorer")
        .arg(fpath.parent().unwrap())
        .spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(fpath.parent().unwrap())
        .spawn()?;

    Ok(())
}
