use std::fs;
use dirs::home_dir;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filename: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let filename = args.filename;
    let home = home_dir().unwrap();
    let dotfiles = home.join("dotfiles");
    let src = dotfiles.join(&filename);
    let dst = home.join(&filename);
    fs::hard_link(src, dst).unwrap();
}
