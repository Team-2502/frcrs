pub trait Command {
    fn run(&self);
    fn stop(&self);
}