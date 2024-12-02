pub trait Day {
    fn parts() -> Vec<Box<dyn Fn() -> String>>;
}
