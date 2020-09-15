use htmlescape::{decode_html, encode_minimal};

fn main() {
    let s1 = r#"Elements are written as "<tag>text</tag>"."#;
    let s2 = "Elements are written as &quot;&lt;tag&gt;text&lt;/tag&gt;&quot;.";
    assert_eq!(encode_minimal(s1), s2);
    assert_eq!(s1, decode_html(s2).unwrap());
}
