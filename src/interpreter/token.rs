use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    ttype: String,
    value: Option<String>,
}

impl Token {
    pub fn new(ttype: &str, value: Option<String>) -> Token {
        Token {
            ttype: ttype.to_string(),
            value: value,
        }
    }

    pub fn update_value(&mut self, value: Option<String>) {
        if !value.is_none() && !self.value.is_none() {
            self.value.as_mut().unwrap().push_str(value.unwrap().as_str());
        }
    }

    pub fn get_type(&self) -> String {
        return self.ttype.clone()
    }

    pub fn get_value(&self) -> String {
        return self.value.as_ref().unwrap().clone();
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token ({}, {:?})", self.ttype, self.value)
    }
}