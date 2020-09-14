fn main() {
    let mylist = [1, 4, -5, 10, -7, 2, 3, -1];
    let positive_list: Vec<_> = mylist.iter().filter(|n| **n > 0).collect();
    println!("{:?}", positive_list);
    let negative_list: Vec<_> = mylist.iter().filter(|n| **n < 0).collect();
    println!("{:?}", negative_list);

    let addresses = [
        "5412 N CLARK",
        "5148 N CLARK",
        "5800 E 58TH",
        "2122 N CLARK",
        "5645 N RAVENSWOOD",
        "1060 W ADDISON",
        "4801 N BROADWAY",
        "1039 W GRANVILLE",
    ];
    let counts = [0, 3, 10, 4, 1, 7, 6, 1];
    let a: Vec<_> = addresses
        .iter()
        .zip(counts.iter())
        .filter(|x| *x.1 > 5)
        .map(|x| x.0)
        .collect();
    println!("{:?}", a);
}
