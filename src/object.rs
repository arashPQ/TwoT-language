use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
//      Add types in our programming language
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Error(String),
    Null,
}

impl Object {
    pub fn object_type(&self) -> String{
        match self {
            Self::Integer(_) => String::from("INTEGER"),
            Self::Boolean(_) => String::from("BOOLEAN"),
            Self::ReturnValue(_) => String::from("RETURN_VALUE"),
            Self::Error(_) => String::from("ERROR"),
            Self::Null => String::from("NULL"),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(int) => write!(f, "{}", int),
            Self::Boolean(bool) => write!(f, "{}", bool),
            Self::ReturnValue(return_value) => write!(f, "{}", *return_value),
            Self::Error(error) => write!(f, "ERROR: {}", error),
            Self::Null => write!(f, "null"),
        
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_environment() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None
        }
    }
    pub fn get(&self, name: String) -> Option<Object> {
        match self.store.get(name.as_str()) {
            Some(object) => Some(object.clone()),
            None => match &self.outer {
                Some(environment) => environment.get(name),
                None => None,
            },
        }
    }
    pub fn set(&mut self, name: String, value: Object) -> Option<Object> {
        self.store.insert(name.clone(), value);
        return self.get(name);
    }
}