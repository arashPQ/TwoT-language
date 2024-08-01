use std::ops::Deref;

use crate::
{ast::{BlockStatement, ExpressionNode, Identifier, IfExpression, Program, StatementNode}, 
object::{Environment, Function, Object}};

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;


pub struct Evaluator{
    environment: Environment,
}


impl Evaluator{
    pub fn new() -> Evaluator {
        Evaluator {
            environment: Environment::new_environment(),
        }
    }
    pub fn eval_program(&mut self, program: Program) ->Object{
        let mut result = Object::Null;
        
        for stmt in program.statements {
            result = self.eval_statement(stmt);

            if let Object::ReturnValue(ret) = result {
                return *ret;
            }
            if let Object::Error(_) = result {
                return result;
            }
        }
        result
    }
    fn eval_statement(&mut self, stmt: StatementNode) -> Object {
        match stmt {
            StatementNode::Expression(exp_stmt) => self.eval_expression(exp_stmt.expression),
            StatementNode::Return(return_stmt) => {
                let value = self.eval_expression(return_stmt.return_value);

                if Self::is_error(&value) {
                    return value;
                }
                return Object::ReturnValue(Box::new(value));
            }
            StatementNode::Say(say_stmt) => {
                let value = self.eval_expression(say_stmt.value);
                if Self::is_error(&value) {
                    return value;
                }
                self.environment.set(say_stmt.name.value, value).unwrap()
            }
            _ => Object::Null,
        }
    }

    fn eval_expression(&mut self, expression: Option<ExpressionNode>) -> Object {
        if let Some(exp) = expression {
            return match exp {
                ExpressionNode::Integer(int) => Object::Integer(int.value),
                ExpressionNode::BooleanNode(bool) => {
                    Self::native_bool_to_boolean_object(bool.value)
                }
                ExpressionNode::Prefix(prefix_exp) => {
                    let right = self.eval_expression(Some(*prefix_exp.right));
                    if Self::is_error(&right) {
                        return right;
                    }
                    return Self::eval_prefix_expression(prefix_exp.operator, right);
                }
                ExpressionNode::Infix(infix_exp) => {
                    let left = self.eval_expression(Some(*infix_exp.left));
                    if Self::is_error(&left) {
                        return left;
                    }
                    
                    let right = self.eval_expression(Some(*infix_exp.right));
                    if Self::is_error(&right) {
                        return right;
                    }

                    return Self::eval_infix_expression(infix_exp.operator, &left, &right);
                }
                ExpressionNode::IfExpressionNode(if_exp) => self.eval_if_expression(if_exp),
                ExpressionNode::IdentifierNode(identifier) => self.eval_identifier(identifier),
                ExpressionNode::Function(function_literal) => Object::Function(Function{
                    parameters: function_literal.parameters,
                    body: function_literal.body,
                    environment: self.environment.clone(),
                }),

                ExpressionNode::Call(call_exp) => {
                    let function = self.eval_expression(Some(call_exp.function.deref().clone()));

                    if Self::is_error(&function) {
                        return function;
                    }
                    let arguments = self.eval_expressions(call_exp.arguments);
                
                    if arguments.len() == 1 && Self::is_error(&arguments[0]) {
                        return arguments[0].clone();
                    }

                    self.apply_function(function, arguments)
                }
                ExpressionNode::StringExp(string_literal) =>
                    Object::StringObject(string_literal.value),
                _ => Object::Null
            };
        }
        Object::Null
    }

    fn apply_function (&mut self, function: Object, arguments: Vec<Object>) -> Object {
        match function {
            Object::Function(function) => {
                let old_environment = self.environment.clone();
                let extended_environment = self.extended_function_environment(function.clone(), arguments);
                self.environment = extended_environment;
                let evaluated = self.eval_block_statement(function.body);
                self.environment = old_environment;
                
                return Self::unwarp_return_value(evaluated);
            
            }
            other => Object::Error(format!("not a function: {}", other.object_type()))
        }
    }

    fn extended_function_environment(&self, function: Function, arguments: Vec<Object>) -> Environment {
        let mut environment = Environment::new_enclosed_evironment(Box::new(function.environment));

        for (idx, parameter) in function.parameters.into_iter().enumerate() {
            environment.set(parameter.value, arguments[idx].clone());
        }

        environment
    }

    fn unwarp_return_value(object: Object) -> Object {
        match object {
            Object::ReturnValue(ret) => *ret ,
            _ => object
        }
    }

    fn eval_expressions(&mut self, expression: Vec<ExpressionNode>) -> Vec<Object> {
        let mut result = vec![];

        for exp in expression {
            let evaluated = self.eval_expression(Some(exp));
            if Self::is_error(&evaluated) {
                return vec![evaluated];
            }
            result.push(evaluated);
        }
        result
    }

    fn eval_prefix_expression(operator: String, right: Object) -> Object {
        match operator.as_str() {
            "!" => Self::eval_bang_operator_expression(right),
            "-" => Self::eval_minu_prefix_operator_expression(right),
            _ => Object::Error(format!(
                "unknown operator: {} {}",
                operator,
                right.object_type()
            )),
        }
    }

    fn eval_infix_expression(operator: String, left: &Object, right: &Object) -> Object {
        if left.object_type() != right.object_type() {
            return Object::Error(format!(
                "type mismatch: {} {} {}",
                left.object_type(),
                operator,
                right.object_type(),
            ));
        }
        match (left, right, operator) {
            (Object::Integer(left), Object::Integer(right), op) => {
                Self::eval_integer_infix_expression(op, *left, *right)
            }
            (Object::StringObject(left_string), Object::StringObject(right_string), operator) => {
                return match operator.as_str() {
                    "+" => 
                        Object::StringObject(format!("{}{}", left_string, right_string)),
                    _ => Object::Error(format!(
                        "unknown operator: {} {} {}",
                        left.object_type(),
                        operator,
                        right.object_type()
                    )),
                    
                };
            }   
                    
            
            (Object::Boolean(l), Object::Boolean(r), operator) => {         // l: left, r: right
                return match operator.as_str() {
                    "==" =>Self::native_bool_to_boolean_object(l == r),
                    "!=" =>Self::native_bool_to_boolean_object(l != r),
                    _ => Object::Error(format!(
                        "unknown operator: {} {} {}",
                        left.object_type(),
                        operator,
                        right.object_type()
                    )),
                };
            }
            (left, right, operator) =>
                Object::Error(format!(
                    "unknown operator: {} {} {}",
                    left.object_type(),
                    operator,
                    right.object_type()
                ))
        }
    }

    fn eval_integer_infix_expression(operator: String, left: i64, right: i64) -> Object {
        match operator.as_str() {
            "+" => Object::Integer(left + right),
            "-" => Object::Integer(left - right),
            "*" => Object::Integer(left * right),
            "/" => Object::Integer(left / right),
            "<" => Self::native_bool_to_boolean_object(left < right),
            ">" => Self::native_bool_to_boolean_object(left > right),
            "==" => Self::native_bool_to_boolean_object(left == right),
            "!=" => Self::native_bool_to_boolean_object(left != right),
            _ => NULL
        }
    }

    fn eval_bang_operator_expression(right: Object) -> Object {
        match right {
            Object::Boolean(true) => FALSE,
            Object::Boolean(false) => TRUE,
            Object::Null => TRUE,
            _ => FALSE

        }
            
    }

    fn eval_minu_prefix_operator_expression(right: Object) -> Object {
        match right {
            Object::Integer(int) => Object::Integer(-int),
            _ => Object::Error(format!("unknown operator: -{}", right.object_type())),
        }
    }

    fn native_bool_to_boolean_object(bool: bool) -> Object {
        if bool {
            TRUE
        } else {
            FALSE
        }
    }

    fn eval_if_expression(&mut self, exp: IfExpression) -> Object {
        let condition = self.eval_expression(Some(*exp.condition));

        return if Self::is_truthy(condition){
            self.eval_block_statement(exp.consequence)
        } else if let Some(alternative) = exp.alternative {
            self.eval_block_statement(alternative)
        } else {
            NULL
        } ;
    }

    fn is_truthy(object: Object) -> bool {
        match object {
            Object::Null => false,
            Object::Boolean(true) => true,
            Object::Boolean(false) => false,
            _ => true
        }
    }

    fn eval_block_statement(&mut self, block: BlockStatement) -> Object {
        let mut result = NULL;
        for stmt in block.statements {
            result = self.eval_statement(stmt);

            if result.object_type() == "RETURN_VALUE" || result.object_type() == "ERROR" {
                return result;
            }
        }
        result
    }

    fn eval_identifier(&self, identifier: Identifier) -> Object{
        let value = self.environment.get(identifier.value.clone());
        match value {
            Some(value) => value,
            None => Object::Error(format!("Identifier not found: {}", identifier.value))
        }
    }

    fn is_error(object: &Object) -> bool {
        object.object_type() == "ERROR"
    }
}
#[cfg(test)]

mod test {
    use crate::
        {lexer::Lexer,
        object::Object,
        parser::Parser,
        ast::Node};

    use super::Evaluator;


    #[test]
    fn test_evaluation_integer_expression(){
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("3 * 3 * 3 * 3 * 3", 243),
            ("-132 + 264 + -132", 0),
            ("8 * 4 + 8", 40),
            ("36 / 3 * 3 + 14", 50),
            ("3 * (15 + 5)", 60),
            ("4 * 4 * 4 + 16", 80),
            ("8 + 4 * 2", 16),
            ("8 * (4 + 2)", 48),
            ("(8 + 4 * 2 + 20 / 5) * 2 + -20", 20)];

        for test in tests {
            let evaluated = test_eval(test.0);
            test_integer_object(evaluated, test.1);
        }
    }

    #[test]
    fn test_say_statements() {
        let tests = vec![
            ("say a = 12; a;", 12),
            ("say a = 12 * 5; a;", 60),
            ("say a = 12; say b = a; b;", 12),
            ("say a = 12; say b = a; say c = a + b + 12; c;", 36),
        ];

        for test in tests {
            test_integer_object(test_eval(test.0), test.1);
        }
    }

    #[test]
    fn test_boolean_expression(){
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 > 2", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        for test in tests {
            let evaluated = test_eval(test.0);
            test_boolean_object(evaluated, test.1);
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = vec![
            ("if (true) {10}", 10),
            ("if (false) {10}", -0),
            ("if (1) {10}", 10),
            ("if (1 < 2) {10}", 10),
            ("if (1 > 2) {10}", -0),
            ("if (1 > 2) {10} else {20}", 20),
            ("if (1 < 2) {10} else {20}", 10),
        ];

        for test in tests {
            let evaluated = test_eval(test.0);
            if test.1 == -0 {
                test_null_object(evaluated);
            } else {
                test_integer_object(evaluated, test.1);
            }
        }
    }
    fn test_null_object(object: Object) {
        match object {
            Object::Null => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_return_statement() {
        let tests = vec![
            ("return 2003", 2003),
            ("return 1383; 1382", 1383),
            ("return 3 * 5; 25;", 15),
            ("-5; return 2 * 3; -12;", 6),
            ("if (2 > 1) {
                if (2 > 1) {
                    return 0;
                }
                return 1;
            }",
            0),
        ];

        for test in tests {
            let evaluated = test_eval(test.0);
            test_integer_object(evaluated, test.1);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) {
                if (10 > 1) {
                return true + false;
                }
                return 1;
                }
                ",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "Identifier not found: foobar"),
            (r#""Hello" - "World""#, "unknown operator: STRING - STRING"),
        ];

        for test in tests {
            let evaluated = test_eval(test.0);
            match evaluated {
                Object::Error(err) => assert_eq!(err, test.1),
                other => panic!("no error object returned. got={:?}", other),
            }
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!1383", false),
            ("!!true", true),
            ("!!false", false),
            ("!!1383", true)
        ];

        for test in tests {
            let evaluated = test_eval(test.0);
            test_boolean_object(evaluated, test.1);

        }
    }

    fn test_eval(input: &str) -> Object{
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new();
        evaluator.eval_program(program.unwrap())
    }

    fn test_boolean_object(object: Object, expected: bool) {
        match object {
                Object::Boolean(bool) => assert_eq!(
                    bool, expected,
                    "obkect wrong value. got={}, want={}", bool, expected),
                    other => panic!("object is not boolean, got={}", other)
            }
        }

    fn test_integer_object(object: Object, expected: i64){
        match object {
            Object::Integer(int) => assert_eq!(
                int, expected,
                "Object has wrong value, got={}, want={}",
                int, expected
            ),
            other => panic!("Object is not Integer, got={:?}", other),
        }
    }

    #[test]
    fn test_closures() {
        let input = r#"
        say newAdder = function(x) {
            function(y) {x + y};
        };

        say addTwo = newAdder(25);
        addTwo(25);
        "#;

        test_integer_object(test_eval(input), 50);
    }

    #[test]
    fn test_function_object() {
        let input = "function(x) {x + 23}";
        let evaluated = test_eval(input);

        match evaluated {
            Object::Function(function) => {
                assert_eq!(function.parameters.len(),
                1,
                "function has wrong parameters length, got={}",
                function.parameters.len()
                );
                assert_eq!(
                    function.parameters[0].print_string(),
                    "x",
                    "parameter is not 'x', got={}",
                    function.parameters[0].print_string()
                );
                assert_eq!(
                    function.body.print_string(),
                    "(x + 23)",
                    "body is not '(x + 23)', got={}",
                    function.body.print_string()
                );
            }
            other => panic!("Entered Object is not function, got={}", other)
        }
    }

    #[test]
    fn test_function_application() {
        let tests = vec![
            ("say identity = function(x) { x; }; identity(5);", 5),
            ("say identity = function(x) { return x; }; identity(5);", 5),
            ("say double = function(x) { x * 2; }; double(5);", 10),
            ("say add = function(x, y) { x + y; }; add(5, 5);", 10),
            ("say add = function(x, y) { x+ y; }; add(5 + 5, add(5, 5));", 20),
            ("function(x) { x; }(5)", 5),
        ];

        for test in tests {
            test_integer_object(test_eval(test.0), test.1);
        }
    }


    #[test]
    fn test_string_literal() {
        let input = r#""hello world!!""#;
        let evaluated = test_eval(input);

        match evaluated {
            Object::StringObject(string) => {
                assert_eq!(
                    string,
                    "hello world!!",
                    "string has wrong value, got={}",
                    string
                );
            }

            other => panic!("Entered object is not string: got={:?}", other)
        }
    }

    #[test]
    fn test_string_concatenation() {
        let input = r#"
        
            "Hello" + " " + "World!!"
        
        "#;
        let evaluated = test_eval(input);
        match evaluated {
            Object::StringObject(string) => assert_eq!(
                string,
                "Hello World!!",
                "string has wrong value, got={}",
                string
            ),
            other => panic!(
                "Entered object is not string, got={:?}",
                other
            )
        }
    }
}