#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

use std::io::{stdout, Write};

pub fn init_map(world: &mut World) {
    for column in 0..=10 {
        for row in 0..=40 {
            world.add_cell(Cell {
                x: row,
                y: column,
                z: 0,
                draw_content: '.',
            });
        }
    }
}

struct Cell {
    x: i32,
    y: i32,
    z: i32,
    draw_content: char,
}

pub struct World {
    width: u32,
    height: u32,
    map: Vec<Cell>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            map: Default::default(),
        }
    }
}

struct Chuck {
    x: i32,
    y: i32,
}

impl World {
    fn add_cell(&mut self, cell: Cell) {
        self.map.push(cell);
    }

    pub fn draw(&self) -> Result<()> {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for i in &self.map {
            stdout
                .queue(cursor::MoveTo(i.x as u16, i.y as u16))?
                .queue(style::PrintStyledContent(i.draw_content.white()))?;
        }
        Ok(())
    }
}
