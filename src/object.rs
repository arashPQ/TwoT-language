use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::Display,
    hash::{Hash, Hasher},
};

use crate::{ast::{BlockStatement, Identifier, Node}, builtins::Builtin};

pub type BuiltinFunction = fn(Vec<Object>) -> Object;


#[derive(Debug, Clone)]
//      Add types in our programming language
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Error(String),
    Function(Function),
    StringObject(String),
    Builtin(BuiltinFunction),
    Array(Vec<Object>),
    DictObject(DictStruct),
    Null,
}

impl Object {
    pub fn object_type(&self) -> String{
        match self {
            Self::Integer(_) => String::from("INTEGER"),
            Self::Boolean(_) => String::from("BOOLEAN"),
            Self::ReturnValue(_) => String::from("RETURN_VALUE"),
            Self::Error(_) => String::from("ERROR"),
            Self::Function(_) => String::from("FUNCTION"),
            Self::StringObject(_) => String::from("STRING"),
            Self::Builtin(_) => String::from("BUILTIN"),
            Self::Array(_) => String::from("ARRAY"),
            Self::DictObject(_) => String::from("DICTIONARY"),
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
            Self::Function(function) => {
                let mut out = String::from("");
                let mut parameters = vec![];

                for parameter in &function.parameters {
                    parameters.push(parameter.print_string());
                }

                out.push_str("function");
                out.push_str("(");
                out.push_str(parameters.join(", ").as_str());
                out.push_str(") { \n");
                out.push_str(function.body.print_string().as_str());
                out.push_str("}\n");

                write!(f, "{}", out)
            }
            Self::StringObject(string) => write!(f, "{}", string),
            Self::Builtin(_) => write!(f, "builtin function"), 
            Self::Array(elements) => {
                let mut out = String::from("");
                let mut els = vec![];

                for element in elements {
                    els.push(format!("{}", element));
                }

                out.push_str("[");
                out.push_str(els.join(", ").as_str());
                out.push_str("]");

                write!(f, "{}", out)
            }
            Self::DictObject(dictionary) => {
                let mut out = String::from("");
                let mut pairs = vec![];

                for(_ , pair) in &dictionary.pairs {
                    pairs.push(format!("{}: {}", pair.key, pair.value))
                }

                out.push_str("{");
                out.push_str(pairs.join(", ").as_str());
                out.push_str("}");

                write!(f, "{}", out)
            }
            Self::Null => write!(f, ""),
        
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_environment() -> Environment {
        let mut environment_map = HashMap::new();
        Self::init_builtin (&mut environment_map);
        Environment {
            store: environment_map,
            outer: None
        }
    }

    pub fn new_enclosed_evironment(outer: Box<Environment>) -> Environment {
        let mut environment_map = HashMap::new();
        Self::init_builtin (&mut environment_map);
        Environment {
            store: environment_map,
            outer: Some(outer),
        }
    }

    fn init_builtin(hashmap: &mut HashMap<String, Object>) {
        let builtins_functions = Builtin;
        let builtins = builtins_functions.all_builtins();
        for (name, object) in builtins {
            hashmap.insert(name, object);
        }
    }


    pub fn get(&self, name: String) -> Option<Object> {
        match self.store.get(name.as_str()) {
            Some(object) => Some(object.clone()),
            None => match &self.outer {
                Some(environment) => environment.get(name),
                None => match &self.outer {
                    Some(environment) => environment.get(name),
                    None => None
                },
            },
        }
    }
    pub fn set(&mut self, name: String, value: Object) -> Option<Object> {
        self.store.insert(name.clone(), value);
        return self.get(name);
    }
}


#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub environment: Environment,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct DictKey{
    pub object_type: String,
    pub value: i64
}


impl Hash for DictKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.object_type.hash(state);
    }
}


pub trait Dictado {
    fn dict_key(&self) -> Result<DictKey, String>;
}

impl Dictado for Object {
    fn dict_key(&self) -> Result<DictKey, String> {
        match &self {
            Object::Boolean(bool) => {
                let value = if *bool {1} else {0};
                Ok(DictKey {
                    object_type: self.object_type(),
                    value
                })
            }
            Object::Integer(int) => Ok(DictKey {
                object_type: self.object_type(),
                value: *int,
            }),
            Object::StringObject(string) => {
                let mut hasher = DefaultHasher::new();
                string.hash(&mut hasher);
                Ok(DictKey {
                    object_type: self.object_type(),
                    value: hasher.finish() as i64,
                })
            }
            other => Err(format!("unusable as Dictionary key: {}", other.object_type())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DictPair {
    pub key: Object,
    pub value: Object,
}

#[derive(Debug, Clone)]
pub struct DictStruct {
    pub pairs: HashMap<DictKey, DictPair>,
}

#[cfg(test)]
mod test{
    use super::{Dictado, Object};


    #[test]
    fn test_string_dict_key() {
        let hello1 = Object::StringObject("Hello World".to_string());
        let hello2 = Object::StringObject("Hello World".to_string());
        let some_other = Object::StringObject("Some Other".to_string());

        assert_eq!(
            hello1.dict_key(),
            hello2.dict_key(),
            "Entered String with same content, have diffrent Dictionary key"
        );

        assert_ne!(
            hello1.dict_key(),
            some_other.dict_key(),
            "Entered String with different content, have same Dictionary key"
        );
    }
}