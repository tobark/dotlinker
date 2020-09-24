use std::fs;
use std::env;
use dirs::home_dir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let home = home_dir().unwrap();
    let dotfiles = home.join("dotfiles");
    let src = dotfiles.join(filename);
    let dst = home.join(filename);
    fs::hard_link(src, dst).unwrap();
}
