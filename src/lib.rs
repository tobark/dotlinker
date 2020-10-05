use std::fs;
use toml::value::Value;

pub mod app;
mod item;
use item::Item;

pub fn load() -> Result<Vec<Item>, std::io::Error> {
    // let home = home_dir().unwrap();
    // let dotfiles = home.join("dotfiles");
    let entries = fs::read_to_string(".dotlinker")?;
    let entries = entries.parse::<Value>().unwrap();
    let mut items: Vec<Item> = vec![];

    if let Value::Table(table) = entries {
        for (key, value) in table.iter() {
            let mut source: String = String::new();
            if let Value::String(src) = &value["source"] {
                source = src.to_string();
            }
            let mut target = String::new();
            if let Value::String(tar) = &value["target"] {
                target = tar.to_string();
            }
            items.push(Item::new(key, source, target))
        }
    }
    Ok(items)
}
