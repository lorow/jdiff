pub trait History {
    fn restore(&mut self);
    fn undo_restore(&mut self);
    fn backup(&mut self);
    fn save(self) -> String;
}
