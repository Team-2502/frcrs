pub trait Subsystem {
    fn init(&mut self);
    fn stop(&self);
}