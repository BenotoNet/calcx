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

    Assign,

    Var(String),

    Keyword(String),
    Unknown(String),
}

enum Expr {
    Number(f64),
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    }
}

pub struct Calc {
    tokens: Vec<Token>,
    current: usize,
}

impl Calc {
    pub fn new() -> Calc {
        Calc { tokens: vec![], current: 0 }
    }

    fn advance(&mut self) -> Option<Token> {
        self.current += 1;
        self.tokens.get(self.current).cloned()
    }

    fn peek(&mut self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn peek_n(&mut self, n: usize) -> Option<Token> {
        self.tokens.get(n).cloned()
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Some(Token::Number(num)) => {return Expr::Number(num)},

            _ => {panic!{"Unexpected Token"}},
        }
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
