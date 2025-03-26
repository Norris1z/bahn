use crate::constants::IMAGE_CODE;

#[allow(dead_code)]
pub enum RepresentationType {
    Ascii(Option<char>),
    Image(Option<char>),
}

impl RepresentationType {
    pub fn from(code: char, option: Option<char>) -> Self {
        match code {
            IMAGE_CODE => RepresentationType::Image(option),
            _ => RepresentationType::Ascii(option),
        }
    }
}
