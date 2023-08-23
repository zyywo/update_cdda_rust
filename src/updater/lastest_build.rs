#[allow(unused)]
#[derive(Debug, Clone)]
pub struct LatestBuild {
    pub build_number: String,
}
impl LatestBuild {
    pub fn new() -> LatestBuild {
        LatestBuild {
            build_number: "".to_string(),
        }
    }
    pub fn pull(&mut self) {
        let mut dst = Vec::new();
        println!("获取最新版本号...");
        {
            let mut easy = curl::easy::Easy::new();
            easy.url("https://cataclysmdda.org/experimental/").unwrap();
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    dst.extend_from_slice(data);
                    Ok(data.len())
                })
                .expect("连接失败");
            transfer.perform().expect("连接失败");
        }

        let s = String::from_utf8(dst).unwrap();
        let re =
            regex::Regex::new(r"<h2 id=.*?>.*(?P<build>\d{4}-\d{2}-\d{2}-\d{4})</h2>").unwrap();
        let caps = re.captures(s.as_str()).unwrap();
        self.build_number = caps["build"].to_string();
    }
}
