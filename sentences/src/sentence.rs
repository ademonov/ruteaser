use regex::Regex;
use std::collections::BTreeSet;

//todo: create a struct for splitting rules
pub fn split_sentences<'a>(text: &'a str, words_delimiters: Vec<&str>, regex_delimiters: Vec<&str>, words_exceptions: Vec<&str>, regex_exceptions: Vec<&str>) -> Vec<&'a str> {
    let mut breakpoints = BTreeSet::new(); // contains all the positions we are going to split out text by.

    dbg!(text);
    process_words(text, words_delimiters, |start, end| breakpoints.add_indices(start, end));
    dbg!(&breakpoints);
    process_regex(text, regex_delimiters, |start, end| breakpoints.add_indices(start, end));
    dbg!(&breakpoints);
    process_words(text, words_exceptions, |start, end| breakpoints.remove_indices(start, end));
    dbg!(&breakpoints);
    process_regex(text, regex_exceptions, |start, end| breakpoints.remove_indices(start, end));
    dbg!(&breakpoints);

    breakpoints.keep_last_in_consequence();
//    for i in 1..text.chars().count() {
//        let current = i as usize;
//        let previous = i - 1 as usize;
//        if breakpoints.contains(&current) && breakpoints.contains(&previous) {
//            breakpoints.remove(&previous);
//        }
//    }

    let mut result = vec![];
    let mut cut = 0usize;
    for i in breakpoints {
        let next_cut = i + 1; // cut sentences by character right after the current delimiter
        result.push(&text[cut..next_cut]);
        cut = next_cut;
    }
    result.push(&text[cut..]); // add the latest sentence

    result
}

fn process_words(text: &str, patterns: Vec<&str>, mut handle_indices: impl FnMut(usize, usize)) {
    for p in patterns {
        let p = p.to_lowercase();
        for m in text.to_lowercase().match_indices(p.as_str()) { //todo: consider to convert text into lower case just once
            handle_indices(m.0, m.0 + p.len());
        }
    }
}

fn process_regex(text: &str, patterns: Vec<&str>, mut handle_indices: impl FnMut(usize, usize)) {
    for p in patterns {
        let re = Regex::new(p).unwrap(); //todo: get rid of unwraps
        for m in re.find_iter(text) {
            handle_indices(m.start(), m.end());
        }
    }
}

trait Breakpoints {
    fn add_indices(&mut self, start: usize, end: usize);
    fn remove_indices(&mut self, start: usize, end: usize);
    fn keep_last_in_consequence(&mut self);
}


impl Breakpoints for BTreeSet<usize> {
    fn add_indices(&mut self, start: usize, end: usize) {
        for i in start..end {
            self.insert(i);
        }
    }

    fn remove_indices(&mut self, start: usize, end: usize) {
        for i in start..end {
            self.remove(&i);
        }
    }

    fn keep_last_in_consequence(&mut self) {
        if self.len() < 2 { return; } // nothing to remove

        let copy = self.iter().cloned().collect::<Vec<_>>();
        for i in 1..self.len() {
            let previous = copy[i - 1];
            let current = copy[i];
            if current - previous == 1 {
                &self.remove(&previous);
            }
        }
    }
}


#[cfg(test)]
mod tests {
//    use crate::sentence::split_sentences;

    #[test]
    fn it_works() {
        let text = r"Maße, MASSE, mase, ΜΤΣ, abc, Abc, ABC, μτς";
        let re = regex::Regex::new(r"(?i)μτς").unwrap();

        let m: Vec<_> = re.find_iter(text).collect();
        assert_eq!(m.len(), 2);


//        let text = "Mr. Smith bought cheapsite.com for 1.5 million dollars, i.e. he paid a lot for it. Did he mind? Adam Jones Jr. thinks he didn't. In any case, this isn't true... Well, with a probability of .9 it isn't. C. elegans should do it. Ю";
//        for s in split_sentences(text) {
//            println!("{}", s);
//        }
        //assert_eq!(s.chars().count(), 10);
    }
}
