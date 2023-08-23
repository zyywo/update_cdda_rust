
use crate::updater::{platform::Platform, current_game::CurrentGame, lastest_build::LatestBuild};


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
    pub fn new(current_game_path: &str) -> Config {

        Config { 
            platform: Platform::default(), 
            tiles: true,
            sounds: false,
            backup_configdir: true,
            backup_savedir: true,
            backup_templates: true,
            current_game: CurrentGame::new(&current_game_path.to_string()),
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
        let mut cfg = Config::new("");
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
