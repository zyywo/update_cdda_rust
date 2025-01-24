#[macro_use]
extern crate ini;
mod updater;
use clap::{ArgAction, Parser};
use home::home_dir;
use std::fs::create_dir_all;
use std::fs::File;

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
    let config_path = home_dir().unwrap().join(".config/update_cdda/config.ini");
    if !config_path.exists() {
        match create_dir_all(&config_path.parent().unwrap()) {
            Ok(_) => {
                if let Err(e) = File::create(&config_path) {
                    println!("创建配置文件失败: {}", e);
                } else {
                    println!("创建空配置文件: {}", &config_path.to_str().unwrap());
                }
            }
            Err(e) => {
                println!("创建配置文件失败: {}", e)
            }
        }
    }

    let (cdda, proxy, keep_dirs, keep_files) =
        match ini!(config_path.to_str().unwrap()).get("default") {
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
        None => {
            if cdda == None {
                println!("必须在配置文件或选项中设置游戏路径");
                return ();
            }
            updater::config::Config::new(cdda.unwrap().as_str())
        }
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
