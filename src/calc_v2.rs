#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Plus,
    Multiply,
    Minus,
    Divide,
    Power,
    LeftParen,
    RightParen,
}

pub struct CalcV2;

impl CalcV2 {
    fn precedence(token: &Token) -> i32 {
        match token {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            Token::Power => 3,
            _ => 0,
        }
    }

    fn is_right_associative(token: &Token) -> bool {
        matches!(token, Token::Power)
    }

    fn parse_expression(tokens: &[Token], min_precedence: i32) -> (i32, usize) {
        let (mut lhs, mut pos) = CalcV2::parse_primary(&tokens);

        while pos < tokens.len() {
            let op = &tokens[pos];

            if CalcV2::precedence(op) < min_precedence {
                break;
            }

            pos += 1;

            let next_min_precedence = if CalcV2::is_right_associative(op) {
                CalcV2::precedence(op)
            } else {
                CalcV2::precedence(op) + 1
            };

            let (rhs, rhs_pos) = CalcV2::parse_expression(&tokens[pos..], next_min_precedence);

            pos += rhs_pos;

            lhs = CalcV2::apply_op(op, lhs, rhs);
        }

        (lhs, pos)
    }

    fn apply_op(op: &Token, lhs: i32, rhs: i32) -> i32 {
        match op {
            Token::Plus => lhs + rhs,
            Token::Multiply => lhs * rhs,
            Token::Divide => lhs / rhs,
            Token::Minus => lhs - rhs,
            Token::Power => lhs.pow(rhs as u32),
            _ => panic!("Invalid operator"),
        }
    }

    fn parse_primary(tokens: &[Token]) -> (i32, usize) {
        match &tokens[0] {
            Token::Number(n) => (*n, 1),
            Token::LeftParen => {
                let (expr, pos) = CalcV2::parse_expression(&tokens[1..], 0);
                if pos + 1 >= tokens.len() || tokens[pos + 1] != Token::RightParen {
                    panic!("Mismatched parentheses");
                }
                (expr, pos + 2)
            }
            _ => panic!("Expected a number or left parenthesis"),
        }
    }

    pub fn calculate(tokens: &[Token]) -> i32 {
        let (result, _) = CalcV2::parse_expression(tokens, 0);
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

        assert_eq!(CalcV2::calculate(&tokens), 11);
    }

    #[test]
    fn test_calculate_simple() {
        let tokens = vec![Token::Number(1), Token::Plus, Token::Number(2)];

        assert_eq!(CalcV2::calculate(&tokens), 3);
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

        assert_eq!(CalcV2::calculate(&tokens), 10);
    }
}
