use regex::Regex;

fn main() {
    let re = Regex::new(r#""(.*?)""#).unwrap();
    let text1 = r#"Computer says "no.""#;
    let result = re.captures(text1).unwrap();
    assert_eq!(result.get(1).unwrap().as_str(), "no.");

    let text2 = r#"Computer says "no." Phone says "yes.""#;
    let result: Vec<_> = re
        .captures_iter(text2)
        .map(|x| x.get(1).unwrap().as_str())
        .collect();
    assert_eq!(result, vec!["no.", "yes."]);
}
