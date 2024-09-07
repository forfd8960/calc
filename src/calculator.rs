use anyhow::{bail, Result};
use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

use crate::errors::CaculatorError;

// operator precendence
lazy_static! {
    static ref PRECEDENCE: HashMap<char, u8> = {
        HashMap::from([
            (')', 6),
            ('^', 5),
            ('*', 4),
            ('/', 3),
            ('+', 2),
            ('-', 1),
            ('(', 0),
        ])
    };
}

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

    pub fn calculate(&mut self) -> Result<f64> {
        let tokens = self.parse_chars()?;
        self.do_calculation(tokens)
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

    fn do_calculation(&mut self, tokens: Vec<Token>) -> Result<f64> {
        let mut idx = 0;
        loop {
            if idx >= tokens.len() {
                break;
            }

            let token = tokens.get(idx).unwrap();
            match *token {
                Token::Num(num) => {
                    self.result.push_back(num);
                    idx += 1;
                }
                Token::Op(op) => match op {
                    '(' => {
                        self.op_stack.push_back(op);
                        idx += 1;
                    }
                    ')' => {
                        idx = self.pop_util_left_parenthesis(&tokens, idx)?;
                    }
                    _ => {
                        idx = self.push_or_calc(op, idx)?;
                    }
                },
            }
        }

        self.pop_all_operators()?;
        let r = self.result.pop_back();
        if r.is_none() {
            bail!(CaculatorError::InvalidExpression(self.exp.clone()));
        }

        Ok(r.unwrap())
    }

    fn pop_util_left_parenthesis(
        &mut self,
        tokens: &Vec<Token>,
        idx: usize,
    ) -> anyhow::Result<usize> {
        Ok(idx)
    }

    fn push_or_calc(&mut self, op: char, idx: usize) -> anyhow::Result<usize> {
        Ok(idx)
    }

    fn pop_all_operators(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn pop_and_calc(&mut self) -> anyhow::Result<()> {
        let op = self.op_stack.pop_back();
        if op.is_none() {
            bail!(CaculatorError::InvalidExpression(self.exp.clone()));
        }

        let operator = op.unwrap();

        if self.result.len() < 2 {
            bail!(CaculatorError::MissingOperand(operator, 2));
        }

        let num2 = self.result.pop_back().unwrap();
        let num1 = self.result.pop_back().unwrap();

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => {
                if num2 == 0.0 {
                    bail!(CaculatorError::DivideByZero);
                }
                num1 / num2
            }
            '^' => num1.powf(num2),
            _ => bail!(CaculatorError::UnsupportedOperator(operator)),
        };

        self.result.push_back(result);
        Ok(())
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

    #[test]
    fn test_pop_and_calc() -> anyhow::Result<()> {
        let exp = "";
        let mut caculator = Caculator::new(exp.to_string());

        let t_cases = vec![
            ('*', 40.96 as f64, 100 as f64, Some(4096 as f64)),
            ('/', 1996 as f64, 100 as f64, Some(19.96 as f64)),
            ('+', 99 as f64, 1 as f64, Some(100 as f64)),
            ('-', 33 as f64, 1 as f64, Some(32 as f64)),
            ('^', 2 as f64, 10 as f64, Some(1024 as f64)),
        ];

        for case in t_cases {
            caculator.op_stack.push_back(case.0);
            caculator.result.push_back(case.1);
            caculator.result.push_back(case.2);

            caculator.pop_and_calc()?;
            let r = caculator.result.pop_back();
            println!("{:?}, {:?}", case, r);
            assert_eq!(r, case.3);
        }

        Ok(())
    }
}
