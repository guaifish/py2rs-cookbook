use textwrap::{NoHyphenation, Wrapper};

fn main() {
    let s = "Look into my eyes, look into my eyes, the eyes, the eyes, \
            the eyes, not around the eyes, don't look around the eyes, \
            look into my eyes, you're under.";

    println!("{}", textwrap::fill(s, 70));
    println!("\n");

    let wrapper = Wrapper::with_splitter(40, NoHyphenation)
        .initial_indent("    ")
        .subsequent_indent("");
    println!("{}", wrapper.fill(s));
}
