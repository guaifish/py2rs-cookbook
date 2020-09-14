use counter::Counter;

fn main() {
    let words = [
        "look", "into", "my", "eyes", "look", "into", "my", "eyes", "the", "eyes", "the", "eyes",
        "the", "eyes", "not", "around", "the", "eyes", "don't", "look", "around", "the", "eyes",
        "look", "into", "my", "eyes", "you're", "under",
    ];
    let word_counts: Counter<_> = words.iter().collect();
    let top_three = &word_counts.most_common()[..3];
    println!("{:?}", top_three);
}
