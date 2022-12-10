use super::value::Value;

pub fn write_as_json(value: &Value, w: &mut impl std::io::Write) -> std::io::Result<()> {
    match value {
        Value::String(v) => {
            w.write("\"".as_bytes())?;
            // TODO: add escaping
            w.write(v)?;
            w.write("\"".as_bytes())?;
            Ok(())
        }
        Value::Integer(i) => {
            write!(w, "{}", i)
        }
        Value::List(ls) => {
            w.write("[".as_bytes())?;
            let mut first = true;
            for x in ls {
                if first {
                    first = false;
                } else {
                    w.write(",".as_bytes()).map(|_| ())?
                }
                write_as_json(x, w)?;
            }
            w.write("]".as_bytes()).map(|_| ())
        }
        Value::Dictionary(kv) => {
            w.write("{".as_bytes()).map(|_| ())?;
            let mut first = true;
            for (k, v) in kv {
                if first {
                    first = false;
                } else {
                    w.write(",".as_bytes()).map(|_| ())?
                }
                write_as_json(k, w)?;
                w.write(":".as_bytes()).map(|_| ())?;
                write_as_json(v, w)?;
            }
            w.write("}".as_bytes()).map(|_| ())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::utils::str_to_json;
    #[test]
    fn test_string() {
        assert_eq!(str_to_json("4:spam"), "\"spam\"");
    }
    #[test]
    fn test_integer() {
        assert_eq!(str_to_json("i4e"), "4");
        assert_eq!(str_to_json("i-4e"), "-4");
        assert_eq!(str_to_json("i0e"), "0");
    }
    #[test]
    fn test_list() {
        assert_eq!(str_to_json("l4:spam3:egge"), "[\"spam\",\"egg\"]");
    }
    #[test]
    fn test_dict() {
        assert_eq!(
            str_to_json("d4:spam3:egg5:spam24:egg2e"),
            "{\"spam\":\"egg\",\"spam2\":\"egg2\"}"
        );
    }
}
