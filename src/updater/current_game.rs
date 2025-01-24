#[derive(Debug, Clone)]
pub struct CurrentGame {
    pub path: String,
    pub build_number: String,
}
impl CurrentGame {
    pub fn new(gamedir: &str) -> CurrentGame {
        use std::path::Path;
        let version_file = Path::new(gamedir).join("VERSION.txt");
        let bn = if version_file.exists() {
            let file_content = String::from_utf8(std::fs::read(version_file).unwrap()).unwrap();
            let re = regex::Regex::new(r"(?ms)build number: (?P<version>.*?)\ncommit sha").unwrap();
            re.captures(&file_content).unwrap()["version"].to_string()
        } else {
            String::from("")
        };
        CurrentGame {
            path: gamedir.to_string(),
            build_number: bn.to_string(),
        }
    }
}
