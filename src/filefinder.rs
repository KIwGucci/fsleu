extern crate globmatch;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn item_search(
    rootpath: &str,
    extention: &str,
    searchword: &str,
    deselection: &str,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    // 拡張子
    let pattern = format!("./**/*.{}", extention);

    let rootpath = Path::new(rootpath);
    println!("dsel:{}", deselection);
    // 拡張子で対象フォルダをglobmatch処理
    let builder = match globmatch::Builder::new(&pattern)
        .case_sensitive(false)
        .build(rootpath)
    {
        Ok(m) => m,
        Err(e) => return Err(e.into()),
    };

    //対象ワードにマッチするか判定しboolを返すクロージャ
    let is_matchword = |x: &PathBuf| {
        let mut jud = false;

        let searchwords = searchword.split_whitespace();
        let path_string = x.to_string_lossy().to_lowercase();

        if !deselection.is_empty() && path_string.contains(&deselection.to_lowercase()) {
            jud = false;
        } else if searchword.is_empty() {
            jud = true
        } else {
            for wd in searchwords {
                if path_string.contains(&wd.to_lowercase()) {
                    jud = true;
                } else {
                    jud = false;
                    break;
                }
            }
        }

        jud
    };

    // 検索語でフィルタ処理

    let paths = builder
        .into_iter()
        .filter_map(|x| match x {
            Ok(path) => {
                if is_matchword(&path) {
                    Some(path)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect();

    Ok(paths)
}

pub fn opendir(fpath: &Path) -> Result<(), std::io::Error> {
    // 対象ファイルのフォルダをファインダーで開く
    #[cfg(target_os = "macos")]
    Command::new("open").arg(fpath.parent().unwrap()).spawn()?;

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
