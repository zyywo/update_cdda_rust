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

    pub fn get_download_file(&self) -> String {
        match self.platform {
            Platform::Android => {
                format!("cdda-android-bundle-{}.aab", self.latestbuild.build_number)
            }
            Platform::Linux => String::from(""),
            Platform::OSx => String::from(""),
            Platform::Windows => {
                if self.tiles && self.sounds {
                    String::from(format!(
                        "cdda-windows-with-graphics-and-sounds-x64-{}.zip",
                        self.latestbuild.build_number
                    ))
                } else if self.tiles && !self.sounds {
                    String::from(format!(
                        "cdda-windows-with-graphics-x64-{}.zip",
                        self.latestbuild.build_number
                    ))
                } else {
                    String::from("")
                }
            }
            _ => String::from(""),
        }
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
        assert_eq!(cfg.get_download_file(), "");

        // Windows, tiles false, sounds true
        cfg.platform = Platform::Windows;
        cfg.tiles = false;
        cfg.sounds = true;
        assert_eq!(cfg.get_download_file(), "");

        // Windows, tiles true, sounds true
        cfg.platform = Platform::Windows;
        cfg.tiles = true;
        cfg.sounds = true;
        assert_eq!(
            cfg.get_download_file(),
            "cdda-windows-with-graphics-and-sounds-x64-2023-05-16-2259.zip"
        );

        // Windows, tiles true, sounds false
        cfg.platform = Platform::Windows;
        cfg.tiles = true;
        cfg.sounds = false;
        assert_eq!(
            cfg.get_download_file(),
            "cdda-windows-with-graphics-x64-2023-05-16-2259.zip"
        );

        // Linux, tiles false, sounds false
        cfg.platform = Platform::Linux;
        cfg.tiles = false;
        cfg.sounds = false;
        assert_eq!(cfg.get_download_file(), "");

        // Linux, tiles true, sounds false
        cfg.platform = Platform::Linux;
        cfg.tiles = true;
        cfg.sounds = false;
        assert_eq!(cfg.get_download_file(), "");

        // Linux, tiles false, sounds true
        cfg.platform = Platform::Linux;
        cfg.tiles = false;
        cfg.sounds = true;
        assert_eq!(cfg.get_download_file(), "");

        // Linux, tiles true, sounds true
        cfg.platform = Platform::Linux;
        cfg.tiles = true;
        cfg.sounds = true;
        assert_eq!(cfg.get_download_file(), "");
    }
}
