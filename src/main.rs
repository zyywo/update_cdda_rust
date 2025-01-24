#[macro_use]
extern crate ini;
mod updater;
use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(disable_help_flag(true))]
#[command(disable_version_flag(true))]
struct Cli {
    /// CDDA的安装路径
    #[arg(short, long)]
    path: Option<String>,

    /// 指定版本
    #[arg(short, long, value_name("BUILD"))]
    build: Option<String>,

    /// github下载加速器
    #[arg(long)]
    proxy: Option<String>,

    /// 打印帮助
    #[arg(short, long, action = ArgAction::Help)]
    help: Option<bool>,

    /// 显示版本
    #[arg(short, long, action = ArgAction::Version)]
    version: Option<bool>,
}

fn main() {
    let (cdda, proxy, keep_dirs, keep_files) = match ini!("config.ini").get("default") {
        Some(x) => {
            let cdda = match x.get("cdda") {
                Some(cp) => cp.clone(),
                None => None,
            };
            let proxy = match x.get("proxy") {
                Some(py) => py.clone(),
                None => None,
            };
            let keep_dirs = match x.get("keep_dirs") {
                Some(kpd) => kpd
                    .as_ref()
                    .unwrap()
                    .split(',')
                    .map(|x| x.to_string())
                    .collect(),
                None => vec![String::from("")],
            };
            let keep_files = match x.get("keep_files") {
                Some(kpf) => kpf
                    .as_ref()
                    .unwrap()
                    .split(',')
                    .map(|x| x.to_string())
                    .collect(),
                None => vec![String::from("")],
            };
            (cdda, proxy, keep_dirs, keep_files)
        }
        None => (None, None, vec![String::from("")], vec![String::from("")]),
    };

    let cli = Cli::parse();

    // 命令行参数中的路径优先级大于配置文件中的路径优先级
    let mut cfg = match cli.path {
        Some(path) => updater::config::Config::new(path.as_str()),
        None => updater::config::Config::new(cdda.unwrap().as_str()),
    };

    // 命令行参数中的加速器优先级大于配置文件中的加速器优先级
    match cli.proxy {
        Some(proxy) => cfg.proxy = proxy,
        None => cfg.proxy = proxy.unwrap(),
    };

    if let Some(n) = cli.build {
        cfg.latestbuild.build_number = n
    };

    cfg.keep_dirs = keep_dirs;
    cfg.keep_files = keep_files;

    updater::updater(cfg);
}
