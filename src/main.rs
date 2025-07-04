#![allow(dead_code)]

mod bow;
mod tokenizer;

use bow::BoW;

fn main() {
    let source: String = String::from("Hello, my name is Yami-no-karuro and i'm a software developer.");
    let bag: BoW = BoW::build(source);
    dbg!(bag);
}
