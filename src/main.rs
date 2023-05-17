mod utils;

fn updater() {
    let mut cfg = utils::Config::new();

    // TODO 配置选项
    cfg.current_game = utils::CurrentGame::new(&r"D:\Program Files\CDDA - 副本".to_string());


    // 获取最新发布的版本号
    #[cfg(debug_assertions)]
    {
        cfg.latestbuild.build_number = "2023-05-16-2259".to_string();
    }
    #[cfg(not(debug_assertions))]
    cfg.latestbuild.pull();



    if cfg.current_game.build_number == cfg.latestbuild.build_number {
        println!(
            "当前版本{}已经是最新的，不需要更新。",
            cfg.current_game.build_number
        );
        return;
    } else {
        println!("当前版本{}", cfg.current_game.build_number);
        println!("最新版本{}", cfg.latestbuild.build_number);
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
    println!("下载链接：{}", download_url);

    let f = utils::downloader(&download_url, &temp_dir.join(cfg.get_download_file()).to_str().unwrap().to_string(), &String::from("https://gitproxy.zhangyongyao.com/"));
    if let Err(s) = f {
        println!("{}", &s);
        return
    }

    if cfg.backup_configdir {
        println!("备份配置文件...");
        fs_extra::dir::copy(
            std::path::Path::new(&cfg.current_game.path).join("config"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();
    }
    if cfg.backup_savedir {
        println!("备份存档...");
        fs_extra::dir::copy(
            std::path::Path::new(&cfg.current_game.path).join("save"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();
    }
    if cfg.backup_templates {
        println!("备份人物模板...");
        fs_extra::dir::copy(
            std::path::Path::new(&cfg.current_game.path).join("templates"),
            &temp_dir,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();
    }

    println!("删除旧版本");
    let mut need_delete = Vec::new();
    for entry in std::path::Path::new(&cfg.current_game.path).read_dir().expect("read_dir error") {
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
                fs_extra::dir::copy(&entry.path(), &cfg.current_game.path, &fs_extra::dir::CopyOptions::new()).unwrap();
            }
            else {
                utils::unpack(&entry.path().to_str().unwrap().to_string(), &cfg.current_game.path);
            }
        }
    }

    println!("删除临时文件夹...");
    fs_extra::dir::remove(&temp_dir).unwrap();
}

fn main() {
    updater();
}
