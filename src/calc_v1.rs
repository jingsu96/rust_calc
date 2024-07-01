#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Plus,
    Multiply,
}

pub struct CalcV1;

impl CalcV1 {
    fn precedence(token: &Token) -> i32 {
        match token {
            Token::Plus => 1,
            Token::Multiply => 2,
            _ => 0,
        }
    }

    fn parse_expression(tokens: &[Token], min_precedence: i32) -> (i32, usize) {
        let (mut lhs, mut pos) = CalcV1::parse_primary(&tokens[0]);

        while pos < tokens.len() {
            let op = &tokens[pos];

            if CalcV1::precedence(op) < min_precedence {
                break;
            }

            pos += 1;

            let (rhs, rhs_pos) =
                CalcV1::parse_expression(&tokens[pos..], CalcV1::precedence(op) + 1);

            pos += rhs_pos;

            lhs = CalcV1::apply_op(op, lhs, rhs);
        }

        (lhs, pos)
    }

    fn apply_op(op: &Token, lhs: i32, rhs: i32) -> i32 {
        match op {
            Token::Plus => lhs + rhs,
            Token::Multiply => lhs * rhs,
            _ => panic!("Invalid operator"),
        }
    }

    fn parse_primary(token: &Token) -> (i32, usize) {
        match token {
            Token::Number(n) => (*n, 1),
            _ => panic!("Expected a number"),
        }
    }

    pub fn calculate(tokens: &[Token]) -> i32 {
        let (result, _) = CalcV1::parse_expression(tokens, 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        let tokens = vec![
            Token::Number(3),
            Token::Plus,
            Token::Number(4),
            Token::Multiply,
            Token::Number(2),
        ];

        assert_eq!(CalcV1::calculate(&tokens), 11);
    }

    #[test]
    fn test_calculate_simple() {
        let tokens = vec![Token::Number(1), Token::Plus, Token::Number(2)];

        assert_eq!(CalcV1::calculate(&tokens), 3);
    }

    #[test]
    fn test_calculate_multiply_first() {
        let tokens = vec![
            Token::Number(2),
            Token::Multiply,
            Token::Number(3),
            Token::Plus,
            Token::Number(4),
        ];

        assert_eq!(CalcV1::calculate(&tokens), 10);
    }
}
