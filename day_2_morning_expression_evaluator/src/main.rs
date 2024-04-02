
fn main() {
    println!("Hello, world!");
}
/// An operation to perform on two subexpressions.
#[allow(dead_code)]
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[allow(dead_code)]
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A literal value
    Value(i64),
}

#[allow(dead_code)]
fn eval(e: Expression) -> Result<i64, String> {
    return match e {
        Expression::Op { op, left, right } => {
            let left_val = match eval(*left) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            let right_val = match eval(*right) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };

            let rs = match op {
                Operation::Add => left_val + right_val,
                Operation::Div => {
                    if right_val == 0 {
                        return Err(String::from("division by zero"));
                    } else {
                        left_val / right_val
                    }
                },
                Operation::Mul => left_val * right_val,
                Operation::Sub => left_val - right_val,
            };

            return Ok(rs);
        }
        Expression::Value(v) => Ok(v),
    };
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}
