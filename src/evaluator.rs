use crate::
{ast::{ExpressionNode, Program, StatementNode}, 
object::Object};

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;


pub struct Evaluator{

}


impl Evaluator{
    pub fn new() -> Evaluator {
        Evaluator {}
    }
    pub fn eval_program(&self, program: Program) ->Object{
        let mut result = Object::Null;
        
        for stmt in program.statements {
            result = self.eval_statement(stmt);
        }
        result
    }
    fn eval_statement(&self, stmt: StatementNode) -> Object {
        match stmt {
            StatementNode::Expression(exp_stmt) => self.eval_expression(exp_stmt.expression),
            _ => Object::Null,
        }
    }

    fn eval_expression(&self, expression: Option<ExpressionNode>) -> Object {
        if let Some(exp) = expression {
            return match exp {
                ExpressionNode::Integer(int) => Object::Integer(int.value),
                ExpressionNode::BooleanNode(bool) => {
                    Self::native_bool_to_boolean_objevt(bool.value)
                }
                ExpressionNode::Prefix(prefix_exp) => {
                    let right = self.eval_expression(Some(*prefix_exp.right));
                    return Self::eval_prefix_expression(prefix_exp.operator, right);
                }
                _ => Object::Null
            };
        }
        Object::Null
    }

    fn eval_prefix_expression(operator: String, right: Object) -> Object {
        match operator.as_str() {
            "!" => Self::eval_bang_operator_expression(right),
            "-" => Self::eval_minu_prefix_operator_expression(right),
            _ => NULL,
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
            _ => NULL
        }
    }

    fn native_bool_to_boolean_objevt(bool: bool) -> Object {
        if bool {
            TRUE
        } else {
            FALSE
        }
    }
}
#[cfg(test)]

mod test {
    use crate::{lexer::Lexer, object::Object, parser::Parser};

    use super::Evaluator;


    #[test]
    fn test_evaluation_integer_expression(){
        let tests = vec![("5", 5), ("10", 10), ("-5", -5), ("-10", -10)];

        for test in tests {
            let evaluated = test_eval(test.0);
            test_integer_object(evaluated, test.1);
        }
    }

    #[test]
    fn test_boolean_expression(){
        let tests = vec![("true", true), ("false", false)];

        for test in tests {
            let evaluated = test_eval(test.0);
            test_boolean_object(evaluated, test.1);
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
        let evaluator = Evaluator::new();
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
}