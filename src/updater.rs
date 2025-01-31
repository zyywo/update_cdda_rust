pub mod config;
pub mod current_game;
mod lastest_build;
mod platform;
pub mod utils;
use config::Config;
use std::{collections::HashSet, path::PathBuf};

pub fn updater(cfg: Config) {
    let temp_dir = std::path::Path::new(&cfg.current_game.path).join("cdda_update_temp");
    println!("创建临时文件夹...");
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir).unwrap();
    }

    let download_url = format!(
        "https://github.com/CleverRaven/Cataclysm-DDA/releases/download/cdda-experimental-{}/{}",
        cfg.latestbuild.build_number,
        cfg.get_download_file()
    );
    println!("{}", format!("下载链接：{}", download_url));

    let f = utils::downloader(
        &download_url,
        &temp_dir
            .join(cfg.get_download_file())
            .to_str()
            .unwrap()
            .to_string(),
        &cfg.proxy,
    );
    if let Err(s) = f {
        println!("{}", &s);
        std::fs::remove_dir_all(&temp_dir).unwrap();
        return;
    }

    println!("删除旧版本...");
    let mut need_delete = Vec::new();
    let keep_list: HashSet<PathBuf> = [
        cfg.keep_dirs,
        cfg.keep_files,
        vec!["cdda_update_temp".to_string()],
    ]
    .concat()
    .iter()
    .map(|x| {
        [cfg.current_game.path.clone(), x.to_string()]
            .iter()
            .collect()
    })
    .collect();

    for entry in std::path::Path::new(&cfg.current_game.path)
        .read_dir()
        .expect("read_dir error")
    {
        if let Ok(entry) = entry {
            if keep_list.contains(&entry.path()) {
                continue;
            }
            need_delete.push(entry.path());
        }
    }
    fs_extra::remove_items(&need_delete).unwrap();

    println!("安装新版本...");
    for entry in temp_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if None == entry.path().extension() {
                fs_extra::dir::copy(
                    &entry.path(),
                    &cfg.current_game.path,
                    &fs_extra::dir::CopyOptions::new(),
                )
                .unwrap();
            } else {
                utils::unpack(
                    &entry.path().to_str().unwrap().to_string(),
                    &cfg.current_game.path,
                );
            }
        }
    }

    println!("删除临时文件夹...");
    fs_extra::dir::remove(&temp_dir).unwrap();
}
