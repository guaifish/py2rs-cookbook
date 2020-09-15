use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)/(\d+)/(\d+)").unwrap();
    assert!(re.is_match(r"11/27/2012"));
    let caps = re.captures(r"11/27/2012").unwrap();
    assert_eq!(caps.get(1).unwrap().as_str(), "11");
    assert_eq!(caps.get(2).unwrap().as_str(), "27");
    assert_eq!(caps.get(3).unwrap().as_str(), "2012");
    assert_eq!(caps.get(0).unwrap().as_str(), "11/27/2012");
}
