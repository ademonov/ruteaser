mod sentence;

fn main() {
    let text = "Mr. Smith bought cheapsite.com for 1.5 million dollars, i.e. he paid a lot for it. Did he mind? Adam Jones Jr. thinks he didn't. In any case, this isn't true... Well, with a probability of .9 it isn't. C. elegans should do it. Ю";
    for s in sentence::split_sentences(
        text,
        vec![],
        vec![r#"\.\s*"#, r#"!\s*"#, r#"\?\s*"#],
        vec!["Mr\\.\\s*", "i\\.e\\.\\s*", "Jr\\.\\s*"],
        vec![r#"\d\.\d"#, r#"\s\.\d"#]) {
        println!("{}", s);
    }
}
