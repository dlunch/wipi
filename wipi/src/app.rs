pub trait App {
    fn on_paint(&mut self);
    fn on_pause(&mut self);
    fn on_resume(&mut self);
    fn on_event(&mut self);
}
