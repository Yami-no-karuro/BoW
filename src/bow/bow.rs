use std::collections::HashMap;

use crate::tokenizer::{
    Token, 
    TokenType, 
    Tokenizer
};

#[derive(Debug)]
pub struct BoW {
    bag: HashMap<String, usize>
}

impl BoW {
    pub fn build(input: String) -> Self {
        let mut bag: HashMap<String, usize> = HashMap::new();
        let mut tokenizer: Tokenizer = Tokenizer::new(input);

        let tokens: Vec<Token> = tokenizer.tokenize();
        for token in tokens {
            if token.token_type == TokenType::Identifier {
                let value: String = token.value;

                *bag.entry(value)
                    .or_insert(0) += 1;
            }
        }

        return Self { bag };
    }

    pub fn to_serialized(&self) -> String {
        let mut pairs: Vec<String> = Vec::new();
        for (key, value) in &self.bag {
            let pair_string: String = format!("{}:{}", key, value);
            pairs.push(pair_string);
        }

        let result: String = pairs.join(",");
        return result;
    }

    pub fn from_serialized(serialized: &str) -> Self {
        let mut bag: HashMap<String, usize> = HashMap::new();
        for pair in serialized.split(",") {
            let mut splits = pair.split(":");
            let word: String = splits.next()
                .unwrap()
                .to_string();

            let count: usize = splits.next()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            bag.insert(word, count);
        }
        
        return Self { bag };
    }

    pub fn get_vocabulary_size(&self) -> usize {
        return self.bag.len();
    }

    pub fn get_total_occurences(&self) -> usize {
        return self.bag.values()
            .sum();
    }
}
