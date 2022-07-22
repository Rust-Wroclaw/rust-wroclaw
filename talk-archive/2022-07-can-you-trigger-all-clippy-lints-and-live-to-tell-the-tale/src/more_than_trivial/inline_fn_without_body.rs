#[allow(unused_attributes)]

trait Process {
    // This is not fine
    #[inline]
    fn name(&self) -> String;

    // This is fine
    #[inline]
    fn echo(&self) -> String {
        format!("{}", self.name())
    }
}
