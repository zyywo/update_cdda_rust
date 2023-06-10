pub mod config;
pub mod utils;
pub mod current_game;
mod platform;
mod lastest_build;

use config::Config;
use std::{time::Duration, sync::{Mutex, Arc}, thread,};


// pub fn wait_3s() -> Vec<String> {
//     thread::sleep(Duration::from_secs(3));

//     vec!["abc".to_string(), "123".to_string()]
// }
pub fn wait_1m(cfg: Arc<Mutex<Config>>, log: Arc<Mutex<String>>, updating: Arc<Mutex<bool>>) {
    let _temp_dir = std::path::Path::new(&cfg.lock().unwrap().current_game.path).join("temp_cdda_update");
    // println!("{:?}", &cfg.lock().unwrap().current_game.path.join("temp_cdda_update"));
    thread::sleep(Duration::from_secs(4));
    log.lock().unwrap().push_str("abc\n");
    thread::sleep(Duration::from_secs(3));
    log.lock().unwrap().push_str("11111\n");
    thread::sleep(Duration::from_secs(2));
    log.lock().unwrap().push_str("IIIIIII\n");
    *updating.lock().unwrap() = false;
}

#[allow(dead_code)]
pub fn updater(cfg: Arc<Mutex<Config>>, log: Arc<Mutex<String>>, updating: Arc<Mutex<bool>>) {

    let temp_dir = std::path::Path::new(&cfg.lock().unwrap().current_game.path).join("temp_cdda_update");
    log.lock().unwrap().push_str("创建临时文件夹...");
    println!("aaaaaaaaaaaaaaaaaa");
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir).unwrap();
    }

    let download_url = format!(
        "https://github.com/CleverRaven/Cataclysm-DDA/releases/download/cdda-experimental-{}/{}",
        cfg.lock().unwrap().latestbuild.build_number,
        cfg.lock().unwrap().get_download_file()
    );
    log.lock().unwrap().push_str(format!("下载链接：{}", download_url).as_str());

    let f = utils::downloader(&download_url, &temp_dir.join(cfg.lock().unwrap().get_download_file()).to_str().unwrap().to_string(), &String::from("https://gitproxy.zhangyongyao.com/"));
    if let Err(s) = f {
        println!("{}", &s);
        log.lock().unwrap().push_str(s.to_string().as_str());
        return
    }

    if cfg.lock().unwrap().backup_configdir {
        log.lock().unwrap().push_str("备份配置文件...");
        fs_extra::dir::copy(
            std::path::Path::new(&cfg.lock().unwrap().current_game.path).join("config"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();
    }
    if cfg.lock().unwrap().backup_savedir {
        println!("备份存档...");
        fs_extra::dir::copy(
            std::path::Path::new(&cfg.lock().unwrap().current_game.path).join("save"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();
    }
    if cfg.lock().unwrap().backup_templates {
        println!("备份人物模板...");
        fs_extra::dir::copy(
            std::path::Path::new(&cfg.lock().unwrap().current_game.path).join("templates"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();
    }

    log.lock().unwrap().push_str("删除旧版本");
    let mut need_delete = Vec::new();
    for entry in std::path::Path::new(&cfg.lock().unwrap().current_game.path).read_dir().expect("read_dir error") {
        if let Ok(entry) = entry {
            if entry.path() == temp_dir {
                continue;
            }
            need_delete.push(entry.path());
        }
    }
    fs_extra::remove_items(&need_delete).unwrap();

    log.lock().unwrap().push_str("安装最新版本和恢复备份文件...");
    for entry in temp_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if None == entry.path().extension() {
                fs_extra::dir::copy(&entry.path(), &cfg.lock().unwrap().current_game.path, &fs_extra::dir::CopyOptions::new()).unwrap();
            }
            else {
                utils::unpack(&entry.path().to_str().unwrap().to_string(), &cfg.lock().unwrap().current_game.path);
            }
        }
    }

    log.lock().unwrap().push_str("删除临时文件夹...");
    fs_extra::dir::remove(&temp_dir).unwrap();
    *updating.lock().unwrap() = false;
}