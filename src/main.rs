use regex::Regex;

fn main() {
    let re = Regex::new(r"(?s)/\*(.*?)\*/").unwrap();
    let text2 = r#"/* this is a
    multiline comment */
    "#;
    let result = re.captures(text2).unwrap();
    assert_eq!(
        result.get(1).unwrap().as_str(),
        " this is a\n    multiline comment "
    );
}
