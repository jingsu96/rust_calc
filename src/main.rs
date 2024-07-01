/**
 * Precedence Climbing 算法的核心思想如下：
 *
 * 1. 從左到右掃描表達式。
 * 2. 對於每個運算符，比較其優先級與當前的最小優先級。
 * 3. 如果當前運算符的優先級更高，則遞迴處理右側的表達式。
 * 4. 應用運算符到左右操作數。
 * 5. 重複這個過程，直到遇到優先級較低的運算符或表達式結束。
 */
mod calc_v1;

use calc_v1::{CalcV1, Token};

fn main() {
    let tokens = vec![
        Token::Number(3),
        Token::Plus,
        Token::Number(4),
        Token::Multiply,
        Token::Number(2),
    ];

    let result = CalcV1::calculate(&tokens);
    println!("Result: {}", result); // Result: 11
}
