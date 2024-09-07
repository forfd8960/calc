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
                        idx = self.pop_util_left_parenthesis(idx)?;
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

    fn pop_util_left_parenthesis(&mut self, idx: usize) -> anyhow::Result<usize> {
        loop {
            if self.op_stack.len() < 1 {
                bail!(CaculatorError::UnBalancedParenthesis);
            }

            let op = self.op_stack.pop_back();
            if op.is_none() {
                bail!(CaculatorError::InvalidExpression(self.exp.clone()));
            }
            let operator = op.unwrap();
            if operator == '(' {
                break;
            }

            self.calc_with_op(operator)?;
        }

        Ok(idx + 1)
    }

    // push or do calculation
    // 1. if there are no operators in the operator stack, push the new operator into the operator stack,
    // 2. else if current operator has higher precendence than the new operator: op,
    // then do calculation with current op in the stack, and pop the current op.
    // 3. then push the new operator into the operator stack
    fn push_or_calc(&mut self, op: char, idx: usize) -> anyhow::Result<usize> {
        if self.op_stack.len() < 1 {
            self.op_stack.push_back(op);
            return Ok(idx + 1);
        }

        let current_op = self.op_stack.back().unwrap();
        let current_prec = PRECEDENCE.get(current_op).unwrap();
        let new_prec = PRECEDENCE.get(&op).unwrap();
        if current_prec >= new_prec {
            self.calc_with_op(*current_op)?;
            self.op_stack.pop_back();
        }

        self.op_stack.push_back(op);
        Ok(idx + 1)
    }

    fn pop_all_operators(&mut self) -> anyhow::Result<()> {
        if self.op_stack.len() < 1 {
            return Ok(());
        }

        loop {
            if self.op_stack.len() < 1 {
                break;
            }

            let op = self.op_stack.pop_back().unwrap();
            self.calc_with_op(op)?;
        }
        Ok(())
    }

    fn calc_with_op(&mut self, operator: char) -> anyhow::Result<()> {
        if self.result.len() < 2 {
            bail!(CaculatorError::MissingOperand(operator, 2));
        }

        let num2 = self.result.pop_back().unwrap();
        let num1 = self.result.pop_back().unwrap();

        println!("pop {} and {} from result stack", num1, num2);

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

        println!("push result: {} into result stack", result);

        self.result.push_back(result);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
    fn test_calc_with_op() -> anyhow::Result<()> {
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
            caculator.result.push_back(case.1);
            caculator.result.push_back(case.2);

            caculator.calc_with_op(case.0)?;
            let r = caculator.result.pop_back();
            println!("{:?}, {:?}", case, r);
            assert_eq!(r, case.3);
        }

        Ok(())
    }

    #[test]
    fn test_push_or_calc() -> anyhow::Result<()> {
        let exp = "1 + 8 - 6";
        let mut caculator = Caculator::new(exp.to_string());

        caculator.op_stack = VecDeque::from(vec![]);
        caculator.result = VecDeque::from(vec![1 as f64]);
        let t_cases = vec![
            (
                '+',
                8 as f64,                                 // operand to push
                VecDeque::from(vec!['+']),                // expect op stack
                VecDeque::from(vec![1 as f64, 8 as f64]), // expect result
            ),
            (
                '-',
                6 as f64,
                VecDeque::from(vec!['-']),           // expect op stack
                VecDeque::from(vec![9.0, 6 as f64]), // result
            ),
        ];

        for tc in t_cases {
            caculator.push_or_calc(tc.0, 0)?;
            caculator.result.push_back(tc.1);

            assert_eq!(caculator.op_stack, tc.2);
            assert_eq!(caculator.result, tc.3);
        }

        Ok(())
    }

    #[test]
    fn test_calcuate() -> anyhow::Result<()> {
        let exp = "1 + 8 - 6";
        let mut caculator = Caculator::new(exp.to_string());
        let r = caculator.calculate()?;
        assert_eq!(r, 3.0);

        let t_cases = vec![
            ("1 + 8 - 6", 3.0),
            ("1 + 8 - 6 * 2", -3.0),
            ("1 + 8 - 6 * 2 / 2", 3.0),
            ("2^10", 1024.0),
            ("2*10", 20.0),
            ("2/10", 0.2),
            ("2/10 + 10", 10.2),
            ("2/10 + 10 - 2", 8.2),
            ("(99 + 1) * (1 - 0.9) / 2^3", 1.2499999999999998),
        ];

        for tc in t_cases {
            let mut caculator = Caculator::new(tc.0.to_string());
            let r = caculator.calculate()?;
            assert_eq!(r, tc.1, "exp: {} is not equal to {}", tc.0, tc.1);
        }

        Ok(())
    }

    #[test]
    fn test_calcuate1() -> anyhow::Result<()> {
        let exp = "(99 + 1) * (1 - 0.9) / 2^3";
        let mut caculator = Caculator::new(exp.to_string());
        let r = caculator.calculate()?;
        assert_eq!(r, 1.2499999999999998);
        Ok(())
    }
}
