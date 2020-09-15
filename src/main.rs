use regex::Regex;

fn main() {
    let line = "asdf fjdk; afed, fjek,asdf, foo";

    let re = Regex::new(r"[;,\s]\s*").unwrap();
    let fields: Vec<_> = re.split(line).collect();
    assert_eq!(fields, ["asdf", "fjdk", "afed", "fjek", "asdf", "foo"]);
}
