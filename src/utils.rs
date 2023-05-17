use std::io::Write;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Platform {
    Windows,
    Linux,
    OSx,
    Android,
    Unknown,
}

impl Platform {
    /// 根据当前系统设置值
    pub fn default() -> Platform {
        #[cfg(target_os = "windows")]
        let _s = Platform::Windows;
        #[cfg(target_os = "linux")]
        let _s = Platform::Linux;
        #[cfg(target_os = "macos")]
        let _s = Platform::OSx;
        _s
    }
}

pub struct CurrentGame {
    pub path: String,
    pub build_number: String,
}
impl CurrentGame {
    #[allow(unused)]
    pub fn new(gamedir: &String) -> CurrentGame {
        use std::path::Path;
        let mut bn = "".to_string();
        let version_file = Path::new(gamedir).join("VERSION.txt");
        if version_file.exists() {
            let file_content = String::from_utf8(std::fs::read(version_file).unwrap()).unwrap();
            let re = regex::Regex::new(r"(?ms)build number: (?P<version>.*?)\ncommit sha").unwrap();
            bn = re.captures(&file_content).unwrap()["version"].to_string();
        }
        CurrentGame {
            path: gamedir.to_string(),
            build_number: bn.to_string(),
        }
    }
}

#[allow(unused)]
pub struct LatestBuild {
    pub build_number: String,
}
impl LatestBuild {
    #[allow(unused)]
    pub fn new() -> LatestBuild {
        LatestBuild {
            build_number: "".to_string(),
        }
    }
    #[allow(unused)]
    pub fn pull(&mut self) {
        let mut dst = Vec::new();

        {
            let mut easy = curl::easy::Easy::new();
            easy.url("https://cataclysmdda.org/experimental/").unwrap();
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    dst.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        let s = String::from_utf8(dst).unwrap();
        let re =
            regex::Regex::new(r"<h2 id=.*?>.*(?P<build>\d{4}-\d{2}-\d{2}-\d{4})</h2>").unwrap();
        let caps = re.captures(s.as_str()).unwrap();
        self.build_number = caps["build"].to_string();
    }
}

// 实现 Downloader
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

#[allow(dead_code)]
pub struct Config {
    pub platform: Platform,
    pub tiles: bool,
    pub sounds: bool,
    pub backup_configdir: bool,
    pub backup_savedir: bool,
    pub backup_templates: bool,
    pub current_game: CurrentGame,
    pub latestbuild: LatestBuild,
}

impl Config {
    pub fn new() -> Config {
        Config { 
            platform: Platform::default(), 
            tiles: true,
            sounds: false,
            backup_configdir: true,
            backup_savedir: true,
            backup_templates: true,
            current_game: CurrentGame{ path: String::from(""), build_number: String::from("") },
            latestbuild: LatestBuild::new()
        }
    }

    pub fn get_download_file(&self) -> String {
        let mut retval;

        match self.platform {
            Platform::Android => {
                retval = format!("cdda-android-bundle-{}.aab", self.latestbuild.build_number)
            },
            Platform::Linux => {
                retval = String::from("cdda-linux");
                if self.tiles {
                    retval = format!("{retval}-tiles");
                    if self.sounds {
                        retval = format!("{retval}-sounds-x64-{}.tar.gz", self.latestbuild.build_number);
                    }
                    else {
                        retval = format!("{retval}-x64-{}.tar.gz", self.latestbuild.build_number);
                    }
                }
                else {
                    retval = format!("{retval}-curses-x64-{}.tar.gz", self.latestbuild.build_number);
                }
            },
            Platform::OSx => {
                retval = String::from("cdda-osx");
                if self.tiles {
                    retval = format!("{retval}-tiles");
                }
                else {
                    retval = format!("{retval}-curses");
                }
                retval = format!("{retval}-universal-{}.dmg", self.latestbuild.build_number);
            },
            Platform::Windows => {
                retval = String::from("cdda-windows-tiles");
                if self.sounds {
                    retval = format!("{retval}-sounds-x64-msvc-{}.zip", self.latestbuild.build_number);
                }
                else {
                    retval = format!("{retval}-x64-msvc-{}.zip", self.latestbuild.build_number);
                }
            },
            _ => {
                retval = String::from("");
            }
        }
        retval
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_file() {
        let mut cfg = Config::new();
        cfg.latestbuild.build_number = "2023-05-16-2259".to_string();

        // Windows, tiles false, sounds false
        cfg.platform = Platform::Windows;
        cfg.tiles = false;
        cfg.sounds = false;
        assert_eq!(cfg.get_download_file(), "cdda-windows-tiles-x64-msvc-2023-05-16-2259.zip");

        // Windows, tiles false, sounds true
        cfg.platform = Platform::Windows;
        cfg.tiles = false;
        cfg.sounds = true;
        assert_eq!(cfg.get_download_file(), "cdda-windows-tiles-sounds-x64-msvc-2023-05-16-2259.zip");

        // Windows, tiles true, sounds true
        cfg.platform = Platform::Windows;
        cfg.tiles = true;
        cfg.sounds = true;
        assert_eq!(cfg.get_download_file(), "cdda-windows-tiles-sounds-x64-msvc-2023-05-16-2259.zip");

        // Windows, tiles true, sounds false
        cfg.platform = Platform::Windows;
        cfg.tiles = true;
        cfg.sounds = false;
        assert_eq!(cfg.get_download_file(), "cdda-windows-tiles-x64-msvc-2023-05-16-2259.zip");

        // Linux, tiles false, sounds false
        cfg.platform = Platform::Linux;
        cfg.tiles = false;
        cfg.sounds = false;
        assert_eq!(cfg.get_download_file(), "cdda-linux-curses-x64-2023-05-16-2259.tar.gz");

        // Linux, tiles true, sounds false
        cfg.platform = Platform::Linux;
        cfg.tiles = true;
        cfg.sounds = false;
        assert_eq!(cfg.get_download_file(), "cdda-linux-tiles-x64-2023-05-16-2259.tar.gz");

        // Linux, tiles false, sounds true
        cfg.platform = Platform::Linux;
        cfg.tiles = false;
        cfg.sounds = true;
        assert_eq!(cfg.get_download_file(), "cdda-linux-curses-x64-2023-05-16-2259.tar.gz");

        // Linux, tiles true, sounds true
        cfg.platform = Platform::Linux;
        cfg.tiles = true;
        cfg.sounds = true;
        assert_eq!(cfg.get_download_file(), "cdda-linux-tiles-sounds-x64-2023-05-16-2259.tar.gz");
    }
}