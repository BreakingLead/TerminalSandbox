use crossterm::terminal::enable_raw_mode;
pub fn init_all() {
    enable_raw_mode().expect("Cannot enable raw mode!");
}
