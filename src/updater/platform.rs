

#[derive(Debug)]
#[allow(dead_code)]
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
        return Platform::Windows;
        #[cfg(target_os = "linux")]
        return  Platform::Linux;
        #[cfg(target_os = "macos")]
        return Platform::OSx;
    }
}
