pub trait UIElement<R> {
    fn update(&mut self) -> Option<R>;
    fn output(&self) -> String;
}
