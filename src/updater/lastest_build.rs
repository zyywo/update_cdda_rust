
#[allow(unused)]
#[derive(Debug)]
#[derive(Clone)]
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
