use crate::updater::{current_game::CurrentGame, lastest_build::LatestBuild, platform::Platform};

#[derive(Debug)]
pub struct Config {
    pub platform: Platform,
    pub proxy: String,
    pub tiles: bool,
    pub sounds: bool,
    pub keep_dirs: Vec<String>,
    pub keep_files: Vec<String>,
    pub current_game: CurrentGame,
    pub latestbuild: LatestBuild,
}

impl Config {
    pub fn new(cdda_path: &str) -> Config {
        Config {
            platform: Platform::default(),
            proxy: "".to_string(),
            tiles: true,
            sounds: false,
            keep_dirs: vec![String::from("")],
            keep_files: vec![String::from("")],
            current_game: CurrentGame::new(cdda_path),
            latestbuild: LatestBuild::new(),
        }
    }

    pub fn generate_cdda_file_name(&self) -> Result<String, &str> {
        match self.platform {
            Platform::Android => Ok(format!(
                "cdda-android-bundle-{}.aab",
                self.latestbuild.build_number
            )),
            Platform::Linux => {
                if self.tiles && self.sounds {
                    Ok(String::from(format!(
                        "cdda-linux-with-graphics-and-sounds-x64-{}.tar.gz",
                        self.latestbuild.build_number
                    )))
                } else if self.tiles && !self.sounds {
                    Ok(String::from(format!(
                        "cdda-linux-with-graphics-x64-{}.tar.gz",
                        self.latestbuild.build_number
                    )))
                } else {
                    Err("没有找到合适的Linux版本，请检查配置是否正确")
                }
            }
            Platform::OSx => Err("MacOS 暂不支持"),
            Platform::Windows => {
                if self.tiles && self.sounds {
                    Ok(String::from(format!(
                        "cdda-windows-with-graphics-and-sounds-x64-{}.zip",
                        self.latestbuild.build_number
                    )))
                } else if self.tiles && !self.sounds {
                    Ok(String::from(format!(
                        "cdda-windows-with-graphics-x64-{}.zip",
                        self.latestbuild.build_number
                    )))
                } else {
                    Err("没有找到合适的windows版本，请检查配置是否正确")
                }
            }
            _ => Err("选项错误：没有对应的版本"),
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "最新版本：{}\n当前版本：{}\n贴图：{}\n音乐：{}\n游戏目录：{}\n更新时保留文件夹：{}\n更新时保留文件：{}\ngithub加速器：{}",
            self.latestbuild.build_number,
            self.current_game.build_number,
            self.tiles,
            self.sounds,
            self.current_game.path,
            self.keep_dirs.join(", "),
            self.keep_files.join(", "),
            self.proxy
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_file() {
        let mut cfg = Config::new("");
        cfg.latestbuild.build_number = "2023-05-16-2259".to_string();

        // Windows, tiles false, sounds false
        cfg.platform = Platform::Windows;
        cfg.tiles = false;
        cfg.sounds = false;
        assert_eq!(cfg.generate_cdda_file_name(), Err("没有找到合适的windows版本，请检查配置是否正确"));

        // Windows, tiles false, sounds true
        cfg.platform = Platform::Windows;
        cfg.tiles = false;
        cfg.sounds = true;
        assert_eq!(cfg.generate_cdda_file_name(), Err("没有找到合适的windows版本，请检查配置是否正确"));

        // Windows, tiles true, sounds true
        cfg.platform = Platform::Windows;
        cfg.tiles = true;
        cfg.sounds = true;
        assert_eq!(
            cfg.generate_cdda_file_name(),
            Ok("cdda-windows-with-graphics-and-sounds-x64-2023-05-16-2259.zip".to_string())
        );

        // Windows, tiles true, sounds false
        cfg.platform = Platform::Windows;
        cfg.tiles = true;
        cfg.sounds = false;
        assert_eq!(
            cfg.generate_cdda_file_name(),
            Ok("cdda-windows-with-graphics-x64-2023-05-16-2259.zip".to_string())
        );

        // Linux, tiles false, sounds false
        cfg.platform = Platform::Linux;
        cfg.tiles = false;
        cfg.sounds = false;
        assert_eq!(cfg.generate_cdda_file_name(), Err("没有找到合适的Linux版本，请检查配置是否正确"));

        // Linux, tiles true, sounds false
        cfg.platform = Platform::Linux;
        cfg.tiles = true;
        cfg.sounds = false;
        assert_eq!(cfg.generate_cdda_file_name(), Ok("cdda-linux-with-graphics-x64-2023-05-16-2259.tar.gz".to_string()));

        // Linux, tiles false, sounds true
        cfg.platform = Platform::Linux;
        cfg.tiles = false;
        cfg.sounds = true;
        assert_eq!(cfg.generate_cdda_file_name(), Err("没有找到合适的Linux版本，请检查配置是否正确"));

        // Linux, tiles true, sounds true
        cfg.platform = Platform::Linux;
        cfg.tiles = true;
        cfg.sounds = true;
        assert_eq!(cfg.generate_cdda_file_name(), Ok("cdda-linux-with-graphics-and-sounds-x64-2023-05-16-2259.tar.gz".to_string()));
    }

    #[test]
    fn test_config_display() {
        let cfg = Config::new(r"C:\Users\zyy\Downloads\cdda");
        println!("{}", cfg);
    }
}
