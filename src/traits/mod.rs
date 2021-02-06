pub trait Markup {
    fn process(&mut self, text: String) -> String;
}
