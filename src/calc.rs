mod tokenize;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    
    LBrac,
    RBrac,

    Var(String),

    Assign,

    Keyword(String),
}

pub struct Calc {
    tokens: Vec<Token>,
    current: usize,
}

impl Calc {
    pub fn new() -> Calc {
        Calc { tokens: vec![], current: 0 }
    }

    // API to run a specific command and capture its output
    pub fn run(&mut self, query: &str) -> String {
        // This function is supposed to tokenize the given query
        self.tokens = tokenize::tokenize(query);

        // FIX: Debug
        format!{"{:?}", self.tokens}
        // String::new()
    }
}
