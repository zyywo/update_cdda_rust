use crate::updater::Config;
/// 包含下载和解压两个工具
///
use curl::Error;
use std::io::Write;
use zip::ZipArchive;

pub fn compare_version(cfg: &mut Config) -> Result<(), ()> {
    // 获取最新发布的版本号
    {
        if cfg.latestbuild.build_number == "" {
            cfg.latestbuild.pull();
        }
    }

    if cfg.current_game.build_number == cfg.latestbuild.build_number {
        let log = format!(
            "当前安装的已经是最新版本{}，不需要更新。",
            cfg.current_game.build_number
        );
        println!("{log}");
        Ok(())
    } else {
        let log = format!(
            "当前版本{}\n最新版本{}",
            cfg.current_game.build_number, cfg.latestbuild.build_number
        );
        println!("{log}");
        Err(())
    }
}

// 下载器
pub fn downloader(url: &String, path: &String, proxy: &String) -> Result<(), curl::Error> {
    if let Ok(fp) = std::fs::File::open(path) {
        if let Ok(_) = ZipArchive::new(fp) {
            println!("{}已经存在，不需要下载。", &path);
            return Ok(());
        }
    }
    let mut file = std::fs::File::create(path).expect(format!("创建{path}失败").as_str());
    let mut easy = curl::easy::Easy::new();
    if proxy == "" {
        easy.url(url).expect("设置url出错");
        println!("没有设置加速器")
    } else if proxy.ends_with('/') {
        easy.url(format!("{}{}", proxy, url).as_str())
            .expect("设置url出错");
        println!("使用加速器：{proxy}{url}");
    } else {
        easy.url(format!("{}/{}", proxy, url).as_str())
            .expect("设置url出错");
        println!("使用加速器：{proxy}/{url}");
    }
    easy.progress(true).expect("下载出错");
    {
        let mut transfer = easy.transfer();
        transfer
            .progress_function(|a, b, _c, _d| {
                if a != 0.0 {
                    print!("\r下载进度：{}/{}", b, a);
                }
                true
            })
            .expect("下载出错");
        transfer
            .write_function(|data| {
                file.write(data).expect("读取buffer失败");
                Ok(data.len())
            })
            .expect("保存文件出错");
        transfer.perform().expect("下载出错");
    }
    if let Ok(http_code) = easy.response_code() {
        if http_code == 404 {
            let mut e = Error::new(56);
            e.set_extra("下载链接不存在".to_string());
            return Err(e);
        }
    }
    if easy.download_size().unwrap() == 0.0 {
        let mut e = curl::Error::new(56);
        e.set_extra("下载量为0".to_string());
        return Err(e);
    }
    println!();
    Ok(())
}

// 解压zip和tar.gz
pub fn unpack(fname: &String, _path: &String) {
    let _path = std::path::Path::new(_path);
    if !_path.exists() {
        std::fs::create_dir_all(_path).unwrap();
    }

    let filetype = std::path::Path::new(fname).extension().unwrap();
    let file = std::fs::File::open(fname).unwrap();

    if filetype == "zip" {
        let mut archive = zip::ZipArchive::new(file).expect(format!("解压{}失败", fname).as_str());
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
