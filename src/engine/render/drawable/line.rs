use super::Text;

pub struct Line {
    sections: Vec<Text>,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for sec in self.sections.iter() {
            if let Err(e) = write!(f, "{sec}") {
                return std::fmt::Result::Err(e);
            }
        }
        std::fmt::Result::Ok(())
    }
}
