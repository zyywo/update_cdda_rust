
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
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
