fn main() {
    let parts = ["Is", "Chicago", "Not", "Chicago?"];
    assert_eq!(parts.join(" "), "Is Chicago Not Chicago?");
    assert_eq!(parts.join(","), "Is,Chicago,Not,Chicago?");
    assert_eq!(parts.join(""), "IsChicagoNotChicago?");

    let a = "Is Chicago";
    let b = "Not Chicago?";
    assert_eq!(a.to_owned() + " " + b, "Is Chicago Not Chicago?");
}
