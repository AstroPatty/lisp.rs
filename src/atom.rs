#[derive(Debug, Clone)]
pub(crate) enum AtomValue {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Id(String),
}

pub(crate) fn parse_atom(input: &str) -> (AtomValue, usize) {
    let atom: String = if let Some(index) = input.find(&[' ', ')']) {
        input.chars().take(index).collect()
    } else {
        panic!();
    };
    let len = atom.len();
    if let Ok(val) = atom.parse::<i64>() {
        return (AtomValue::Int(val), len);
    } else if let Ok(val) = atom.parse::<f64>() {
        return (AtomValue::Float(val), len);
    } else {
        return (AtomValue::Id(atom), len);
    }
}
