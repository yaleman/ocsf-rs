use std::error::Error;


#[derive(Debug)]
pub struct OcsfCodegenError{
    pub errortext: String,
}

impl Error for OcsfCodegenError {}

impl OcsfCodegenError {
    pub fn new(errortext: String) -> Self {
        OcsfCodegenError { errortext }
    }
}

impl From<Box<dyn Error>> for OcsfCodegenError {
    fn from(value: Box<dyn Error>) -> Self {
        Self {
            errortext: format!("{:?}", value)
        }
    }
}

impl From<serde_json::Error> for OcsfCodegenError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            errortext: format!("{:?}", value)
        }
    }
}