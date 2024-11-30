use proc_macro::Group;
use proc_macro::Punct;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
fn add(x: TokenStream, y: TokenStream) -> TokenStream {
    format!("add({}, {})", x, y).parse().unwrap()
}
fn sub(x: TokenStream, y: TokenStream) -> TokenStream {
    format!("sub({}, {})", x, y).parse().unwrap()
}
fn mul(x: TokenStream, y: TokenStream) -> TokenStream {
    format!("mul({}, {})", x, y).parse().unwrap()
}
fn div(x: TokenStream, y: TokenStream) -> TokenStream {
    format!("div({}, {})", x, y).parse().unwrap()
}
#[derive(Clone, Debug)]
enum ParsedToken {
    Operator(Punct),
    Getter(TokenStream),
}
impl ParsedToken {
    fn get_getter(self) -> TokenStream {
        match self {
            Self::Operator(_) => panic!("called get_getter on a ParsedToken::Operator"),
            Self::Getter(token_stream) => token_stream,
        }
    }
}
#[derive(Clone, Debug)]
enum ParsedTwiceToken {
    Operator(Punct),
    MulDivGroup(Vec<ParsedToken>),
}
impl ParsedTwiceToken {
    fn get_mul_div_group(self) -> Vec<ParsedToken> {
        match self {
            Self::Operator(_) => panic!("called get_mul_div_group on a ParsedTwiceToken::Operator"),
            Self::MulDivGroup(mul_div_group) => mul_div_group,
        }
    }
}
fn parse_mul_div_group(input: Vec<ParsedToken>) -> TokenStream {
    let mut output: Option<TokenStream> = None;
    let mut operator: Option<Punct> = None;
    for token in input {
        match token {
            ParsedToken::Operator(punct) => {
                operator = Some(punct);
            }
            ParsedToken::Getter(token_stream) => match operator {
                Some(ref operator) => match operator.as_char() {
                    '*' => {
                        output = Some(mul(output.unwrap(), token_stream));
                    }
                    '/' => {
                        output = Some(div(output.unwrap(), token_stream));
                    }
                    _ => unimplemented!(),
                },
                None => {
                    output = Some(token_stream);
                }
            },
        }
    }
    output.unwrap()
}
fn mul_div_group_to_token_stream(input: Vec<ParsedToken>) -> TokenStream {
    let mut output = TokenStream::new();
    for token in input {
        match token {
            ParsedToken::Operator(punct) => {
                output.extend(TokenStream::from(TokenTree::Punct(punct)))
            }
            ParsedToken::Getter(token_stream) => output.extend(token_stream),
        }
    }
    output
}
#[proc_macro]
pub fn math(input: TokenStream) -> TokenStream {
    let mut parsed: Vec<ParsedToken> = Vec::new();
    let mut in_progress_getter: Vec<TokenTree> = Vec::new();
    for token in input.clone() {
        match token {
            TokenTree::Punct(ref punct) => match punct.as_char() {
                '+' | '-' | '*' | '/' => {
                    parsed.push(ParsedToken::Getter(TokenStream::from_iter(
                        in_progress_getter,
                    )));
                    parsed.push(ParsedToken::Operator(punct.clone()));
                    in_progress_getter = Vec::new();
                    continue;
                }
                _ => {}
            },
            TokenTree::Group(group) => {
                in_progress_getter.push(TokenTree::Group(Group::new(
                    group.delimiter(),
                    math(group.stream()),
                )));
                continue;
            }
            _ => {}
        }
        in_progress_getter.push(token);
    }
    parsed.push(ParsedToken::Getter(TokenStream::from_iter(
        in_progress_getter,
    )));

    let mut parsed_twice: Vec<ParsedTwiceToken> = Vec::new();
    let mut in_progress_mul_div_group: Vec<ParsedToken> = Vec::new();
    for token in parsed {
        match token {
            ParsedToken::Operator(ref punct) => match punct.as_char() {
                '+' | '-' => {
                    parsed_twice.push(ParsedTwiceToken::MulDivGroup(in_progress_mul_div_group));
                    parsed_twice.push(ParsedTwiceToken::Operator(punct.clone()));
                    in_progress_mul_div_group = Vec::new();
                    continue;
                }
                '*' | '/' => {}
                _ => unimplemented!(),
            },
            ParsedToken::Getter(_) => {}
        }
        in_progress_mul_div_group.push(token);
    }
    parsed_twice.push(ParsedTwiceToken::MulDivGroup(in_progress_mul_div_group));

    //println!("{}", parse_mul_div_group(parsed_twice[0].clone().get_mul_div_group()));
    let mut output: Option<TokenStream> = None;
    let mut operator: Option<Punct> = None;
    for token in parsed_twice {
        match token {
            ParsedTwiceToken::Operator(punct) => {
                operator = Some(punct);
            }
            ParsedTwiceToken::MulDivGroup(mul_div_group) => match operator {
                Some(ref operator) => match operator.as_char() {
                    '+' => {
                        output = Some(add(output.unwrap(), parse_mul_div_group(mul_div_group)));
                    }
                    '-' => {
                        output = Some(sub(output.unwrap(), parse_mul_div_group(mul_div_group)));
                    }
                    _ => unimplemented!(),
                },
                None => {
                    output = Some(parse_mul_div_group(mul_div_group));
                }
            },
        }
    }
    println!("{}", output.clone().unwrap());
    output.unwrap()
}
