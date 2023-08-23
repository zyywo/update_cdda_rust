mod updater;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// CDDA的安装路径
    #[arg(short, long)]
    path: String,

    /// 指定版本
    #[arg(long)]
    build_number: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut cfg = updater::config::Config::new(cli.path.as_str());

    match cli.build_number {
        Some(number) => cfg.latestbuild.build_number = number,
        None => (),
    }

    updater::updater(cfg);
}
