use regex::Regex;

fn main() {
    let text = r"yeah, but no, but yeah, but no, but yeah";
    assert_eq!(
        text.replace("yeah", "yep"),
        r"yep, but no, but yep, but no, but yep"
    );

    let text = r"Today is 11/27/2012. PyCon starts 3/13/2013.";
    let re = Regex::new(r"(?P<month>\d+)/(?P<day>\d+)/(?P<year>\d+)").unwrap();
    let result = re.replace_all(text, r"$year-$month-$day");
    assert_eq!(result, r"Today is 2012-11-27. PyCon starts 2013-3-13.");
}
