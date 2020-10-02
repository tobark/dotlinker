use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdout, stdin};
use structopt::StructOpt;

use dotlinker::app::App;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filename: std::path::PathBuf,
}

fn main() {
    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let mut app = App::new(stdout);

    app.start();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('j') => {
                app.down();
                app.render();
            },
            Key::Char('k') => {
                app.up();
                app.render();
            },
            Key::Char('f') => {
                app.link();
                app.render();
            },
            Key::Char('d') => {
                app.unlink();
                app.render();
            },
            Key::Ctrl('c') => break,
            _ => (),
        }
    }

    app.close();
}
