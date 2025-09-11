
pub type Padding = i16;
pub struct Border {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub top_padding: Padding,
    pub bottom_padding: Padding, 
    pub left_padding: Padding,
    pub right_padding: Padding
}

impl Default for Border {
    fn default() -> Self {
       Self {
        top: String::from('#'),
        bottom: String::from('#'),
        left: String::from('#'),
        right: String::from('#'),
        top_padding: 1,
        bottom_padding: 1,
        left_padding: 1,
        right_padding: 1,
       } 
    }
}