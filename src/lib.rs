use proc_macro::Group;
use proc_macro::Punct;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
#[derive(Debug)]
enum ParsedToken {
    Operator(Punct),
    Getter(TokenStream),
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
    for i in &parsed {
        println!("{:?}", i);
    }

    let mut check = TokenStream::new();
    for i in parsed {
        check.extend(match i {
            ParsedToken::Operator(punct) => TokenStream::from(TokenTree::Punct(punct)),
            ParsedToken::Getter(token_stream) => token_stream,
        });
    }
    println!("AAAAAAAAAA");
    println!("{}", check);
    println!("LOOOK ABOVE");
    input
}
