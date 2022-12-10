use super::value::Value;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnsignedIntegerExpected,
    ColonExpected,
    UnexpectedEndOfString,
    NegativeZeroOccurred,
    IntegerSuffixExpected,
    InvalidPrefix,
    KeyExpectedToBeAString,
    ExpectedDictionaryKey,
}

pub type IParseResult<T> = std::result::Result<T, ParseError>;

fn try_parse_value_from_peekable<Bytes>(
    iter: &mut std::iter::Peekable<Bytes>,
) -> IParseResult<Value>
where
    Bytes: Iterator<Item = u8>,
{
    let prefix = iter.peek();
    match prefix {
        Some(b) => match b {
            b'i' => parse_integer(iter),
            b'l' => parse_list(iter),
            b'd' => parse_dictionary(iter),
            b'0'..=b'9' => parse_string(iter),
            _ => Err(ParseError::InvalidPrefix),
        },
        None => unreachable!(),
    }
}

fn parse_unsigned_integer<Bytes: Iterator<Item = u8>>(
    it: &mut std::iter::Peekable<Bytes>,
) -> IParseResult<u64> {
    let mut num: u64 = 0;
    let mut first = false;
    while let Some(b) = it.peek() {
        if b.is_ascii_digit() {
            first = true;
            num = num * 10 + (b - b'0') as u64;
            it.next();
        } else {
            break;
        }
    }
    if first {
        Ok(num)
    } else {
        Err(ParseError::UnsignedIntegerExpected)
    }
}
fn parse_string<Bytes: Iterator<Item = u8>>(
    it: &mut std::iter::Peekable<Bytes>,
) -> IParseResult<Value> {
    let len = parse_unsigned_integer(it)?;
    match it.next() {
        Some(b':') => {
            let mut bytes = Vec::with_capacity(len as usize);
            for _ in 0..len {
                match it.next() {
                    Some(byte) => bytes.push(byte),
                    None => return Err(ParseError::UnexpectedEndOfString),
                };
            }
            Ok(Value::String(bytes))
        }
        Some(_) => Err(ParseError::ColonExpected),
        None => Err(ParseError::ColonExpected),
    }
}

fn parse_dictionary<Bytes: Iterator<Item = u8>>(
    it: &mut std::iter::Peekable<Bytes>,
) -> IParseResult<Value> {
    it.next();
    let mut dict = Vec::new();
    loop {
        match it.peek() {
            Some(b'e') => {
                it.next();
                break Ok(Value::Dictionary(dict));
            }
            Some(_) => {
                let key = try_parse_value_from_peekable(it)?;
                let value = try_parse_value_from_peekable(it)?;
                match key {
                    Value::String(_) => {}
                    _ => return Err(ParseError::KeyExpectedToBeAString),
                }
                dict.push((key, value));
            }
            None => return Err(ParseError::ExpectedDictionaryKey),
        }
    }
}
fn parse_list<Bytes: Iterator<Item = u8>>(
    it: &mut std::iter::Peekable<Bytes>,
) -> IParseResult<Value> {
    it.next();
    let mut list = Vec::new();
    while let Some(e) = it.peek() {
        if *e == b'e' {
            it.next();
            break;
        } else {
            list.push(try_parse_value_from_peekable(it)?);
        }
    }

    Ok(Value::List(list))
}

fn parse_integer<Bytes: Iterator<Item = u8>>(
    it: &mut std::iter::Peekable<Bytes>,
) -> IParseResult<Value> {
    it.next();
    let sign: i8 = match it.peek() {
        Some(b'-') => {
            it.next();
            -1
        }
        _ => 1,
    };
    let unsigned = parse_unsigned_integer(it)?;

    if unsigned == 0 && sign == -1 {
        return Err(ParseError::NegativeZeroOccurred);
    }

    match it.peek() {
        Some(b'e') => {
            it.next();
            Ok(Value::Integer(sign as i64 * unsigned as i64))
        }
        Some(_) => Err(ParseError::IntegerSuffixExpected),
        None => Err(ParseError::IntegerSuffixExpected),
    }
}

pub fn try_parse_value<T: Iterator<Item = u8>>(source: T) -> IParseResult<Value> {
    let mut iter = source.into_iter().peekable();
    try_parse_value_from_peekable(&mut iter)
}

#[cfg(test)]
mod tests {
    use crate::bencoding::utils::str_to_value;

    use super::*;

    #[test]
    fn test_parsing_of_string() {
        assert_eq!(
            str_to_value("4:spam"),
            Ok(Value::String(
                "spam".bytes().into_iter().collect::<Vec<u8>>()
            ))
        )
    }
    #[test]
    fn test_parsing_of_integer() {
        assert_eq!(str_to_value("i0e"), Ok(Value::Integer(0)));
        assert_eq!(str_to_value("i-0e"), Err(ParseError::NegativeZeroOccurred));
        assert_eq!(str_to_value("i-42e"), Ok(Value::Integer(-42)));
        assert_eq!(str_to_value("i42e"), Ok(Value::Integer(42)));
        assert_eq!(str_to_value("i42"), Err(ParseError::IntegerSuffixExpected));
        assert_eq!(str_to_value("i42:"), Err(ParseError::IntegerSuffixExpected));
    }

    #[test]
    fn test_parsing_of_lists() {
        assert_eq!(
            str_to_value("l4:spam4:eggse"),
            Ok(Value::List(
                ["spam", "eggs"]
                    .into_iter()
                    .map(|str| Value::String(str.bytes().into_iter().collect::<Vec<u8>>()))
                    .collect::<Vec<_>>()
            ))
        )
    }

    #[test]
    fn test_parsing_of_dictionaries() {
        assert_eq!(
            str_to_value("d3:cow3:moo4:spam4:eggse"),
            Ok(Value::Dictionary(
                [("cow", "moo"), ("spam", "eggs"),]
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            Value::String(k.bytes().into_iter().collect::<Vec<u8>>()),
                            Value::String(v.bytes().into_iter().collect::<Vec<u8>>()),
                        )
                    })
                    .collect::<Vec<_>>()
            ))
        )
    }
}
