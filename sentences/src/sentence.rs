use regex::Regex;
use std::collections::BTreeSet;

//todo: create a struct for splitting rules
pub fn split_sentences<'a>(text: &'a str, words_delimiters: Vec<&str>, regex_delimiters: Vec<&str>, words_exceptions: Vec<&str>, regex_exceptions: Vec<&str>) -> Vec<&'a str> {
    let mut breakpoints = BTreeSet::new(); // contains all the positions we are going to split out text by.

    dbg!(text);
    handle_indices(text, words_delimiters, true, |x| { breakpoints.insert(x); });
    dbg!(&breakpoints);
    handle_indices(text, regex_delimiters, false, |x| { breakpoints.insert(x); });
    dbg!(&breakpoints);
    handle_indices(text, words_exceptions, true, |x| { breakpoints.remove(&x); });
    dbg!(&breakpoints);
    handle_indices(text, regex_exceptions, false, |x| { breakpoints.remove(&x); });
    dbg!(&breakpoints);

    for i in 1..text.chars().count() {
        let current = i as usize;
        let previous = i - 1 as usize;
        if breakpoints.contains(&current) && breakpoints.contains(&previous) {
            breakpoints.remove(&previous);
        }
    }

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

fn handle_indices(text: &str, patterns: Vec<&str>, insensitive: bool, mut handle_index: impl FnMut(usize))
{
    for p in patterns {
        let p = if insensitive {
            String::from(p)
        } else {
            "(?i)".to_owned() + p
        };

        let re = Regex::new(p.as_str()).unwrap(); //todo: get rid of unwraps
        for m in re.find_iter(text) {
            for i in m.start()..m.end() {
                handle_index(i);
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
