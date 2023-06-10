/// 包含下载和解压两个工具
use std::io::Write;



// #[allow(dead_code)]
// pub fn compare_version(cfg: Arc<Mutex<Config>>) {
//     // 获取最新发布的版本号
//     #[cfg(debug_assertions)]
//     {
//         *cfg.lock().unwrap().latestbuild.build_number = "2023-05-16-2259".to_string();
//     }
//     #[cfg(not(debug_assertions))]
//     cfg.latestbuild.pull();

//     if *cfg.lock().unwrap().current_game.build_number == *cfg.lock().unwrap().latestbuild.build_number {
//         // log = format!(
//         //     "当前版本{}已经是最新的，不需要更新。",
//         //     cfg.current_game.build_number
//         // );
//         vec![cfg.current_game.build_number, cfg.latestbuild.build_number]
//     } else {
//         // log = format!("当前版本{}\n最新版本{}", cfg.current_game.build_number, cfg.latestbuild.build_number);
//         vec![cfg.current_game.build_number, cfg.latestbuild.build_number]
//     }
// }



// 下载器
#[allow(unused)]
pub fn downloader(url: &String, path: &String, proxy: &String) -> Result<(), curl::Error>{
    if std::path::Path::new(&path).exists() {
        println!("{}已经存在，不需要下载。", &path);
        return Ok(());
    }
    let mut file = std::fs::File::create(path).expect(format!("创建{path}失败").as_str());
    let mut easy = curl::easy::Easy::new();
    if proxy == "" {
        easy.url(url);
    } else if proxy.ends_with('/') {
        easy.url(format!("{}{}", proxy, url).as_str());
    } else {
        easy.url(format!("{}/{}", proxy, url).as_str());
    }
    easy.progress(true);
    let mut ret: Result<(), curl::Error>;
    {    let mut transfer = easy.transfer();
        transfer.progress_function(|a, b, c, d| {
            if a != 0.0 {
                print!("\r下载进度：{}/{}", b, a);
            }
            true
        });
        transfer.write_function(|data| {
            file.write(data);
            Ok(data.len())
        });
        transfer.perform()?;
    }
    if easy.download_size().unwrap() == 0.0 {
        let mut e = curl::Error::new(56);
        e.set_extra("下载量为0".to_string());
        return Err(e);
    }
    Ok(())
}

// 解压zip和tar.gz
#[allow(unused)]
pub fn unpack(fname: &String, _path: &String) {
    let _path = std::path::Path::new(_path);
    if !_path.exists() {
        std::fs::create_dir_all(_path).unwrap();
    }

    let filetype = std::path::Path::new(fname).extension().unwrap();
    let file = std::fs::File::open(fname).unwrap();

    if filetype == "zip" {
        let mut archive = zip::ZipArchive::new(file).expect(format!("解压{}失败", _path.to_str().unwrap()).as_str());
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
                Some(path) => _path.join(path.to_owned()),
                None => continue,
            };

            if (*file.name()).ends_with('/') {
                // println!("File {} extracted to \"{}\"", i, outpath.display());
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                // println!(
                //     "File {} extracted to \"{}\" ({} bytes)",
                //     i,
                //     outpath.display(),
                //     file.size()
                // );
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = std::fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    } else if filetype == "gz" {
        let tar = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(_path).unwrap();
    }
}
