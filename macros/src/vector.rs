use proc_macro2::{TokenStream, TokenTree};
use super::decimals::maybe_decimal_number;
use syn::{Result, Error};

pub fn expand(items: TokenStream) -> Result<TokenStream> {
    // dbg!(&items);
    let mut result = Ok(TokenStream::new());

    let mut buf = "Vector::from(vec![".to_string();
    let mut literal_encountered = false;
    for token in items {
        match &token {
            TokenTree::Literal(literal) => {
                if !literal_encountered {
                    match maybe_decimal_number(literal) {
                        Some(digits) => {
                            buf.push('"');
                            buf.push_str(&digits);
                            buf.push('"');
                            buf.push_str(".parse::<Decimal>().unwrap()");
                        }
                        None =>
                            buf.push_str(literal.to_string().as_str())
                    }
                    literal_encountered = true;
                }
                else {
                    result = Err(Error::new(token.span(), "unexpected literal"));
                    break;
                }
            }

            TokenTree::Punct(t) if t.as_char() == ',' => {
                if literal_encountered {
                    buf.push_str(", ");
                    literal_encountered = false;
                }
                else {
                    result = Err(Error::new(token.span(), "unexpected character ','"));
                    break;
                }
            }

            _ => {
                // we may have encountered a variable identifier or an expression
                buf.push_str(&token.to_string())
            }
        }
    }
    buf.push_str("].into_boxed_slice())");
    result.map(|_| buf.parse::<TokenStream>().unwrap())
}
