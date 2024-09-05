use std::collections::{HashMap, VecDeque};

use anyhow::Result;

use std::sync::LazyLock;

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
}
