pub trait Reportable {
    fn report(&self) -> String;
}

impl<T: Reportable> Reportable for &T {
    fn report(&self) -> String {
        (*self).report()
    }
}
