
use crate::{evaluator::NULL, object::Object};

pub struct Builtin ;

impl Builtin {
    pub fn all_builtins(&self) -> Vec<(String, Object)> {
        vec![
            (String::from("len"), Object::Builtin(builtin_len)),
            (String::from("first"), Object::Builtin(builtin_first)),
            (String::from("last"), Object::Builtin(builtin_last)),
            (String::from("last"), Object::Builtin(builtin_rest)),
            (String::from("push"), Object::Builtin(builtin_push)),
            (String::from("tellme"), Object::Builtin(builtin_tellme)),
            
            ]
    }
}

fn builtin_len(arguments: Vec<Object>) -> Object {
    if arguments.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            arguments.len()
        ));
    }
    return match &arguments[0] {
        Object::StringObject(string_literal) => Object::Integer(string_literal.len() as i64),
        Object::Array(array) => Object::Integer(array.len() as i64),
        other => Object::Error(format!(
            "argument to 'len' not supported, got={}",
            other.object_type()
        ))
    }
}

fn builtin_first(arguments: Vec<Object>) -> Object {
    if arguments.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            arguments.len()
        ));
    }

    if arguments[0].object_type() != "ARRAY" {
        return Object::Error(format!("entered argument to 'first' must be array, got={}", arguments[0].object_type()));
    }

    if let Object::Array(array) = &arguments[0] {
        if array.len() > 0 {
            return array[0].clone();
        }
    }
    NULL
}

fn builtin_last(arguments: Vec<Object>) -> Object {
    if arguments.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            arguments.len()
        ));
    }

    if arguments[0].object_type() != "ARRAY" {
        return Object::Error(format!("entered argument to 'last' must be array, got={}", arguments[0].object_type()));
    }

    if let Object::Array(array) = &arguments[0] {
        if array.len() > 0 {
            return array[array.len() - 1].clone();
        }
    }
    NULL
}

fn builtin_rest(arguments: Vec<Object>) -> Object {
    if arguments.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            arguments.len()
        ));
    }

    if arguments[0].object_type() != "ARRAY" {
        return Object::Error(format!(
            "argument to `first` must be ARRAY, got={}",
            arguments[0].object_type()
        ));
    }

    if let Object::Array(array) = &arguments[0] {
        if array.len() > 0 {
            let new_elements = array[1..].to_vec();
            return Object::Array(new_elements);
        }
    }
    NULL
}

fn builtin_push(arguments: Vec<Object>) -> Object {
    if arguments.len() != 2 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=2",
            arguments.len()
        ));
    }

    if arguments[0].object_type() != "ARRAY" {
        return Object::Error(format!(
            "argument to `first` must be ARRAY, got={}",
            arguments[0].object_type()
        ));
    }

    if let Object::Array(array) = &arguments[0] {
        if array.len() > 0 {
            let mut new_elements = array.clone();
            new_elements.push(arguments[1].clone());
            return Object::Array(new_elements);
        }
    }
    NULL
}

fn builtin_tellme(arguments: Vec<Object>) -> Object {
    for argument in arguments {
        println!("{}", argument);
    }
    NULL
}
  