use proc_macro2::{Group, TokenStream, TokenTree};
use syn::{parse_str, Lit};

pub fn transform(item: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    for token in item {
        match &token {
            TokenTree::Literal(literal) => {
                let string = literal.to_string();
                let parsed = parse_str::<Lit>(string.as_str()).unwrap();

                let suffix = parsed.suffix();
                if suffix.is_empty() || suffix != "d" {
                    // Totally skip this literal as there's no interesting suffix to bother with
                    output.extend([token]);
                    continue;
                }

                let token_stream = match &parsed {
                    Lit::Int(int_literal) => {
                        token_stream_from(int_literal.base10_digits())
                    }

                    Lit::Float(float_literal) => {
                        token_stream_from(float_literal.base10_digits())
                    }

                    Lit::Str(string_literal) => {
                        token_stream_from(string_literal.value().as_str())
                    }

                    _ => {
                        TokenStream::from(token)
                    }
                };

                output.extend(token_stream);
            }

            TokenTree::Group(group) => {
                output.extend([TokenTree::Group(
                    Group::new(
                        group.delimiter(),
                        // recursively transform the group's stream
                        transform(group.stream())
                    )
                )]);
            }

            _ => {
                output.extend([token])
            }
        }
    }
    output
}


fn token_stream_from(digits: &str) -> TokenStream {
    format!("\"{}\".parse::<Decimal>().unwrap()", digits).parse::<TokenStream>().unwrap()
}


