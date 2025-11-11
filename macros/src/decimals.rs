use proc_macro2::{Group, Literal, TokenStream, TokenTree};
use syn::Lit;

pub fn transform(item: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    for token in item {
        match &token {
            TokenTree::Literal(literal) => {
                match maybe_decimal_number(literal) {
                    Some(digits) => {
                        output.extend(token_stream_from(&digits))
                    }
                    None => {
                        output.extend(TokenStream::from(token))
                    }
                }
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

pub fn maybe_decimal_number(literal: &Literal) -> Option<String> {
    let string = literal.to_string();
    let parsed = syn::parse_str::<Lit>(string.as_str()).unwrap();
    let suffix = parsed.suffix();
    if suffix.is_empty() || suffix != "d" {
        return None
    }

    match &parsed {
        Lit::Int(int_literal) => {
            Some(int_literal.base10_digits().to_string())
        }
        Lit::Float(float_literal) => {
            Some(float_literal.base10_digits().to_string())
        }
        Lit::Str(string_literal) => {
            Some(string_literal.value())
        }
        _ => {
            None
        }
    }
}

pub fn token_stream_from(digits: &str) -> TokenStream {
    format!("\"{}\".parse::<Decimal>().unwrap()", digits).parse::<TokenStream>().unwrap()
}


