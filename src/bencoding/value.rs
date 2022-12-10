use std::borrow::Cow;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    String,
    Integer,
    List,
    Dictionary,
}

/**
Strings are length-prefixed base ten followed by a colon and the string.
For example 4:spam corresponds to 'spam'.

Integers are represented by an 'i' followed by the number in base 10 followed by an 'e'.
For example i3e corresponds to 3 and i-3e corresponds to -3.
Integers have no size limitation.
i-0e is invalid.
All encodings with a leading zero, such as i03e, are invalid, other than i0e, which of course corresponds to 0.

Lists are encoded as an 'l' followed by their elements (also bencoded) followed by an 'e'.
For example l4:spam4:eggse corresponds to ['spam', 'eggs'].

Dictionaries are encoded as a 'd' followed by a list of alternating keys and their corresponding values followed by an 'e'.
For example, d3:cow3:moo4:spam4:eggse corresponds to {'cow': 'moo', 'spam': 'eggs'} and d4:spaml1:a1:bee corresponds to {'spam': ['a', 'b']}.
Keys must be strings and appear in sorted order (sorted as raw strings, not alphanumerics).

Note that in the context of bencoding strings including dictionary keys are arbitrary byte sequences (uint8_t[]).
BEP authors are encouraged to use ASCII-compatible strings for dictionary keys and UTF-8 for human-readable data. Implementations must not rely on this.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<Value>),
    Dictionary(Vec<(Value, Value)>),
}

impl Value {
    fn get<'v>(&'v self, index: &Value) -> Option<&'v Value> {
        match index {
            Value::String(_) => match self {
                Value::Dictionary(kv) => {
                    let res = kv
                        .iter()
                        .find_map(|(k, v)| if k == index { Some(v) } else { None });
                    res
                }
                _ => None,
            },
            Value::Integer(ind) => match self {
                Value::List(vs) => vs.get(*ind as usize),
                _ => None,
            },
            Value::List(_) => None,
            Value::Dictionary(_) => None,
        }
    }
    pub fn to_lossy_str(&self) -> Option<Cow<str>> {
        match self {
            Value::String(bytes) => Some(String::from_utf8_lossy(bytes)),
            _ => None,
        }
    }
    pub fn keys(&self) -> Keys<'_> {
        self.into()
    }
    pub fn values(&self) -> Values<'_> {
        self.into()
    }
    pub fn entries(&self) -> KeyValues<'_> {
        self.into()
    }
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::String(_) => ValueType::String,
            Value::Integer(_) => ValueType::Integer,
            Value::List(_) => ValueType::List,
            Value::Dictionary(_) => ValueType::Dictionary,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Value::String(s) => s.len(),
            Value::Integer(_) => 1,
            Value::List(vs) => vs.len(),
            Value::Dictionary(kv) => kv.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
pub trait IntoValue {
    fn into_value(&self) -> Value;
}

impl IntoValue for i64 {
    fn into_value(&self) -> Value {
        Value::Integer(*self)
    }
}

impl IntoValue for str {
    fn into_value(&self) -> Value {
        let bytes = self.bytes().collect();
        Value::String(bytes)
    }
}

impl<'k, T: IntoValue, S: AsRef<str>> IntoValue for [(S, T)] {
    fn into_value(&self) -> Value {
        let mut kv: Vec<(Value, Value)> = Vec::new();

        for (k, v) in self {
            let key_str: &str = k.as_ref();
            let value: Value = v.into_value();
            kv.push((key_str.into_value(), value));
        }

        Value::Dictionary(kv)
    }
}

impl<T: IntoValue> IntoValue for [T] {
    fn into_value(&self) -> Value {
        let mut vals = Vec::new();
        for x in self {
            vals.push(x.into_value());
        }
        Value::List(vals)
    }
}

pub struct Keys<'v> {
    value: &'v Value,
    current_index: usize,
}

impl<'v> Iterator for Keys<'v> {
    type Item = &'v Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            Value::String(_) => None,
            Value::Integer(_) => None,
            Value::List(_) => None,
            Value::Dictionary(kv) => {
                if self.current_index < kv.len() {
                    let res = kv.get(self.current_index).map(|(k, _)| k);
                    self.current_index += 1;
                    res
                } else {
                    None
                }
            }
        }
    }
}

impl<'v> From<&'v Value> for Keys<'v> {
    fn from(value: &'v Value) -> Keys<'v> {
        Keys {
            value,
            current_index: 0,
        }
    }
}

impl std::ops::Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::String(_) => unreachable!(),
            Value::Integer(_) => unreachable!(),
            Value::List(values) => &values[index],
            Value::Dictionary(_) => unreachable!(),
        }
    }
}
impl std::ops::Index<&str> for Value {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Value::Dictionary(_) => {
                let value = index.into_value();
                return self.get(&value).unwrap();
            }
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<&Value> for Value {
    type Output = Value;

    fn index<'v>(&'v self, index: &Value) -> &'v Value {
        return self.get(index).unwrap();
    }
}

pub struct Values<'v> {
    value: &'v Value,
    current_index: usize,
}

impl<'v> Iterator for Values<'v> {
    type Item = &'v Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            str @ Value::String(_) => {
                if self.current_index == 0 {
                    self.current_index += 1;
                    Some(str)
                } else {
                    None
                }
            }
            int @ Value::Integer(_) => {
                if self.current_index == 0 {
                    self.current_index += 1;
                    Some(int)
                } else {
                    None
                }
            }
            Value::List(vals) => {
                if self.current_index < vals.len() {
                    let res = &vals[self.current_index];
                    self.current_index += 1;
                    Some(res)
                } else {
                    None
                }
            }
            Value::Dictionary(kv) => {
                if self.current_index < kv.len() {
                    let res = &kv[self.current_index].1;
                    self.current_index += 1;
                    Some(res)
                } else {
                    None
                }
            }
        }
    }
}

impl<'v> From<&'v Value> for Values<'v> {
    fn from(val: &'v Value) -> Self {
        Values {
            value: val,
            current_index: 0,
        }
    }
}

pub struct KeyValues<'v> {
    value: &'v Value,
    current_index: usize,
}

impl<'v> Iterator for KeyValues<'v> {
    type Item = (&'v Value, &'v Value);

    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            Value::String(_) => None,
            Value::Integer(_) => None,
            Value::List(_) => None,
            Value::Dictionary(kv) => {
                if self.current_index < kv.len() {
                    let (k, v) = &kv[self.current_index];
                    self.current_index += 1;
                    Some((k, v))
                } else {
                    None
                }
            }
        }
    }
}

impl<'v> From<&'v Value> for KeyValues<'v> {
    fn from(val: &'v Value) -> Self {
        KeyValues {
            value: val,
            current_index: 0,
        }
    }
}
