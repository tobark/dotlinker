use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

pub fn link(filename: PathBuf) {
    let home = home_dir().unwrap();
    let dotfiles = home.join("dotfiles");
    let src = dotfiles.join(&filename);
    let dst = home.join(&filename);
    fs::hard_link(src, dst).unwrap();
}
