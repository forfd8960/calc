use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Result};

use std::sync::LazyLock;

use crate::errors::CaculatorError;

// operator precendence
static PRECEDENCE: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    HashMap::from([
        ('+', 2),
        ('-', 1),
        ('*', 4),
        ('/', 3),
        ('^', 5),
        ('(', 0),
        (')', 6),
    ])
});

#[derive(Debug, PartialEq)]
enum Token {
    Op(char),
    Num(f64),
}

#[derive(Debug, Clone)]
pub struct Caculator {
    pub exp: String,
    pub chars: Vec<char>,
    pub result: VecDeque<f64>,
    pub op_stack: VecDeque<char>,
    pub current: usize,
}

impl Caculator {
    pub fn new(exp: String) -> Self {
        Self {
            exp: exp.clone(),
            chars: exp.chars().collect(),
            result: VecDeque::new(),
            op_stack: VecDeque::new(),
            current: 0,
        }
    }

    pub fn calculate(&self) -> Result<f64> {
        todo!()
    }

    fn parse_chars(&self) -> anyhow::Result<Vec<Token>> {
        if self.chars.len() == 0 {
            bail!(CaculatorError::InvalidExpression(self.exp.clone()));
        }

        let mut tokens: Vec<Token> = Vec::new();
        let mut idx = 0;

        loop {
            if idx >= self.chars.len() {
                break;
            }

            let ch = self.chars[idx];
            match ch {
                '0'..='9' => {
                    let (num, new_idx) = self.parse_num(ch, idx);
                    tokens.push(Token::Num(num.parse()?));
                    idx = new_idx;
                }
                ' ' | '\n' | '\t' | '\r' => idx += 1,
                '+' | '-' | '*' | '/' | '^' | '(' | ')' => {
                    tokens.push(Token::Op(ch));
                    idx += 1;
                }
                _ => {
                    bail!(CaculatorError::UnsupportedOperator(ch));
                }
            }
        }

        Ok(tokens)
    }

    fn parse_num(&self, ch: char, idx: usize) -> (String, usize) {
        let mut num = String::new();
        num.push(ch);

        let mut new_idx = idx;
        new_idx += 1;

        while new_idx < self.chars.len() {
            let ch = self.chars[new_idx];
            if ch == '.' {
                num.push(ch);
                new_idx += 1;
            } else if ch.is_ascii_digit() {
                num.push(ch);
                new_idx += 1;
            } else {
                break;
            }
        }

        (num, new_idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tokens() -> anyhow::Result<()> {
        let exp = "1+2*3";
        let caculator = Caculator::new(exp.to_string());
        let tokens = caculator.parse_chars()?;

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Num(1 as f64));
        assert_eq!(tokens[1], Token::Op('+'));
        assert_eq!(tokens[2], Token::Num(2 as f64));
        assert_eq!(tokens[3], Token::Op('*'));
        assert_eq!(tokens[4], Token::Num(3 as f64));

        Ok(())
    }

    #[test]
    fn test_parse_tokens1() -> anyhow::Result<()> {
        let exp = "1.099+2.5*3.89";
        let caculator = Caculator::new(exp.to_string());
        let tokens = caculator.parse_chars()?;

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Num(1.099 as f64));
        assert_eq!(tokens[1], Token::Op('+'));
        assert_eq!(tokens[2], Token::Num(2.5 as f64));
        assert_eq!(tokens[3], Token::Op('*'));
        assert_eq!(tokens[4], Token::Num(3.89 as f64));

        Ok(())
    }

    #[test]
    fn test_parse_tokens2() -> anyhow::Result<()> {
        let exp = "1.099 + 2.5 * (3.89 - 1)";
        let caculator = Caculator::new(exp.to_string());
        let tokens = caculator.parse_chars()?;

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0], Token::Num(1.099 as f64));
        assert_eq!(tokens[1], Token::Op('+'));
        assert_eq!(tokens[2], Token::Num(2.5 as f64));
        assert_eq!(tokens[3], Token::Op('*'));
        assert_eq!(tokens[4], Token::Op('('));
        assert_eq!(tokens[5], Token::Num(3.89 as f64));
        assert_eq!(tokens[6], Token::Op('-'));
        assert_eq!(tokens[7], Token::Num(1 as f64));
        assert_eq!(tokens[8], Token::Op(')'));

        Ok(())
    }
}
