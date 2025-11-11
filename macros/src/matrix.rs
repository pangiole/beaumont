use proc_macro2::{TokenStream, TokenTree};
use syn::{Result, Error};
use crate::decimals::maybe_decimal_number;

pub fn expand(tokens: TokenStream) -> Result<TokenStream> {
    // dbg!(&tokens);

    let mut buf = "Matrix::from((vec![".to_string();
    let (rows, cols) = scan(tokens, &mut buf)?;

    buf.push_str("].into_boxed_slice(), ");
    buf.push_str(rows.to_string().as_str());
    buf.push_str(", ");
    buf.push_str(cols.to_string().as_str());
    buf.push_str("))");
    Ok(buf.parse::<TokenStream>().unwrap())
}



// TODO inline the following function as it's only used once
fn scan(tokens: TokenStream, buf: &mut String) -> Result<(usize, usize)> {
    let mut result = Ok((0, 0));
    let mut previous = ' ';
    let mut first_row = true;
    let mut commas = 0;
    for token in tokens.into_iter() {
        match &token {
            TokenTree::Literal(literal) => {
                if previous == ' ' || previous == ',' {
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
                    previous = 'L';
                }
                else {
                    let advice =
                        if previous == '|' { Some(" (maybe prepending a '|' would fix it)") }
                        else { None };

                    result = Err(unexpected(token, advice));
                    break;
                }
            }
            TokenTree::Punct(t) if t.as_char() == ',' => {
                if previous == 'L' {
                    commas += 1;
                    let recorded_cols = recorded_cols(&result);
                    if first_row {
                        result = increment_cols(result);
                    }
                    else {
                        // after the first row
                        if commas >= recorded_cols {
                            result = Err(unexpected(token, Some(" (columns count mismatch)")));
                            break;
                        }
                    }
                    buf.push_str(", ");
                    previous = ',';
                }
                else {
                    result = Err(unexpected(token, None));
                    break;
                }
            }
            TokenTree::Punct(t) if t.as_char() == '|' => {
                if previous == 'L' {
                    buf.push_str(", ");
                    if first_row {
                        result = increment_cols(result);
                        result = increment_rows(result);
                    }
                    previous = '|';
                }
                else if previous == '|' {
                    // double pipe scenario
                    result = increment_rows(result);
                    first_row = false;
                    commas = 0;
                    previous = ' ';
                }
                else {
                    result = Err(unexpected(token, None));
                    break;
                }
            }
            _ => {
                result = Err(unexpected(token, None));
                break;
            }
        }
    }
    result
}



fn increment_cols(res: Result<(usize, usize)>) -> Result<(usize, usize)> {
    res.map(|(rows, cols)| (rows, cols + 1))
}


fn increment_rows(res: Result<(usize, usize)>) -> Result<(usize, usize)> {
    res.map(|(rows, cols)| (rows + 1, cols))
}


fn recorded_cols(res: &Result<(usize, usize)>) -> usize {
    res.as_ref().map_or(0, |(_, cols)| *cols)
}

fn unexpected(token: TokenTree, advice: Option<&str>) -> Error {
    Error::new(
        token.span(),
        format!("unexpected character '{}'{}",  token, advice.unwrap_or(""))
    )
}