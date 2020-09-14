struct Subscriber<'a> {
    addr: &'a str,
    joined: &'a str,
}

impl<'a> Subscriber<'a> {
    fn new(addr: &'a str, joined: &'a str) -> Subscriber<'a> {
        Subscriber { addr, joined }
    }
}

fn main() {
    let sub = Subscriber::new("jonesy@example.com", "2012-10-19");
    println!("{:?}", sub.addr);
    println!("{:?}", sub.joined);
}
