use regex::Regex;

fn main() {
    let text = r"UPPER PYTHON, lower python, Mixed Python";
    let re = Regex::new(r"(?i)python").unwrap();
    let result = re.replace_all(text, "snake");
    assert_eq!(result, r"UPPER snake, lower snake, Mixed snake");
}
