use pest::Parser;

#[derive(Parser)]
#[grammar = "../grammar/number_tokens.pest"]
struct NumberTokensParser;