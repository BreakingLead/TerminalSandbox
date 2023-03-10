#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crossterm::{
    cursor, execute,
    style::{Color, PrintStyledContent},
    terminal,
};
use std::io::{stdout, Write};

pub fn too_small_hint() {}
pub struct MessageBox {
    // The position of the top-left corner
    x: u16,
    y: u16,
    // The width and height of the box
    width: u16,
    height: u16,
    // The border color and fill color
    border_color: Color,
    fill_color: Color,
    // The message and its color
    message: String,
    message_color: Color,
}
