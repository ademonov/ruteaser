mod sentence;

fn main() {
    let text = "Mr. Smith bought cheapsite.com for 1.5 million dollars, i.e. he paid a lot for it. Did he mind? Adam Jones Jr. thinks he didn't. In any case, this isn't true... Well, with a probability of .9 it isn't. C. elegans should do it. Ю";
    for s in sentence::split_sentences(text, vec![".", "!", "?"], vec!["Mr.", "i.e.", "Jr."]) {
        println!("{}", s);
    }
}
