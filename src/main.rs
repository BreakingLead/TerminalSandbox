#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;
use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::event::{poll, KeyCode, KeyModifiers};
use crossterm::event::{read, Event};
use sandboxtui::game_map::init_map;
use sandboxtui::game_map::World;
use sandboxtui::utils::init_all;

use crossterm::terminal::size;

fn main() -> Result<(), Box<dyn Error>> {
    init_all();
    let mut world = World::default();
    init_map(&mut world);

    let (cols, rows) = size()?;
    let mut stdout = stdout();

    'a: loop {
        if poll(Duration::from_millis(2000))? {
            if let Event::Key(e) = read()? {
                if e.code == KeyCode::Char('c') && e.modifiers == KeyModifiers::CONTROL {
                    break 'a;
                }
            }
        } else {
            world.draw()?;
        }
        stdout.flush()?;
    }
    Ok(())
}
