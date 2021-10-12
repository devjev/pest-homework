extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs::read_to_string;
use pest::Parser;

#[derive(Parser)]
#[grammar = "text.pest"]
struct TextParser;


fn main() {
    let text = read_to_string("data/text.txt").unwrap();
    let parsed = TextParser::parse(Rule::text, &text).unwrap_or_else(|e| panic!("{}", e));
    println!("parsed = {:#?}", parsed);
}

