use crate::basic_calc;

pub struct Parser {

}

impl Parser {
    pub fn new() -> Parser {
        Parser {  }
    }

    fn internal_parse(&self, query: &str) -> String {
        let mut output = String::new();
        let factors = query.split("*").collect::<Vec<&str>>();
        if factors.len() == 2 {
            // Try to parse both into numbers
            // TODO
        };
        for factor in &factors {
            // Test if only a single number is left: 
            match factor.parse::<i32>() {
                Ok(num) => {output += &format!{"{num}"};},
                Err(_) => {output += &self.internal_parse(factor);}
            }
        }
        return output;
    }

    pub fn parse(&self, query: &str) {
        println!{"{}", self.internal_parse(query)};
    }
}
