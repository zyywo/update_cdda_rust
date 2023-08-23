pub mod config;
pub mod current_game;
mod lastest_build;
mod platform;
pub mod utils;

use config::Config;
use fs_extra::error::ErrorKind;

pub fn updater(mut cfg: Config) {
    if let Ok(_) = utils::compare_version(&mut cfg) {
        return;
    }

    let temp_dir = std::path::Path::new(&cfg.current_game.path).join("temp_cdda_update");
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
        &String::from("https://gitproxy.zhangyongyao.com/"),
    );
    if let Err(s) = f {
        println!("{}", &s);
        std::fs::remove_dir_all(&temp_dir).unwrap();
        return;
    }

    if cfg.backup_configdir {
        println!("备份配置文件...");
        if let Err(e) = fs_extra::dir::copy(
            std::path::Path::new(&cfg.current_game.path).join("config"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        ) {
            if let ErrorKind::NotFound = e.kind {
                println!("配置文件不存在，跳过");
            }
        }
    }
    if cfg.backup_savedir {
        println!("备份存档...");
        if let Err(e) = fs_extra::dir::copy(
            std::path::Path::new(&cfg.current_game.path).join("save"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        ) {
            if let ErrorKind::NotFound = e.kind {
                println!("存档不存在，跳过");
            }
        }
    }
    if cfg.backup_templates {
        println!("备份人物模板...");
        if let Err(e) = fs_extra::dir::copy(
            std::path::Path::new(&cfg.current_game.path).join("templates"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        ) {
            if let ErrorKind::NotFound = e.kind {
                println!("人物模板不存在，跳过");
            }
        }
    }

    println!("删除旧版本");
    let mut need_delete = Vec::new();
    for entry in std::path::Path::new(&cfg.current_game.path)
        .read_dir()
        .expect("read_dir error")
    {
        if let Ok(entry) = entry {
            if entry.path() == temp_dir {
                continue;
            }
            need_delete.push(entry.path());
        }
    }
    fs_extra::remove_items(&need_delete).unwrap();

    println!("安装最新版本和恢复备份文件...");
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
