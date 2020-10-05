use std::io::Write;
use termion::color;

use crate::item::Item;

pub struct App<W>
where
    W: Write,
{
    view: W,
    items: Vec<Item>,
    index: usize,
}

impl<W> App<W>
where
    W: Write,
{
    pub fn new(view: W) -> Self {
        let items: Vec<Item> = crate::load().unwrap();

        Self {
            view,
            index: 0,
            items,
        }
    }

    pub fn up(&mut self) {
        self.items[self.index].deselect();
        self.index -= 1;
        self.items[self.index].select();
    }

    pub fn down(&mut self) {
        self.items[self.index].deselect();
        self.index += 1;
        self.items[self.index].select();
    }

    pub fn link(&mut self) {
        self.items[self.index].link();
    }

    pub fn unlink(&mut self) {
        self.items[self.index].unlink();
    }

    pub fn start(&mut self) {
        println!("{}{}", termion::clear::All, termion::cursor::Hide);
        self.items[self.index].select();
        self.render();
    }

    pub fn render(&mut self) {
        write!(
            self.view,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::AfterCursor
        )
        .unwrap();
        write!(
            self.view,
            "{}{}{}Press j, k to move around, f to link, d to unlink, ctrl-c to quit{}{}\r\n",
            termion::cursor::Goto(1, 1),
            termion::color::Fg(color::LightWhite),
            termion::style::Italic,
            termion::color::Fg(color::Reset),
            termion::style::Reset,
        )
        .unwrap();
        for item in &self.items {
            item.render();
        }
        self.view.flush().unwrap();
    }

    pub fn close(&mut self) {
        write!(
            self.view,
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::AfterCursor,
            termion::cursor::Show
        )
        .unwrap();
    }
}
