
#[derive(Debug)]
#[derive(Clone)]
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
