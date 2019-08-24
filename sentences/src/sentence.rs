use regex::Regex;
use std::collections::BTreeSet;

pub fn split_sentences<'a>(text: &'a str, delimiters: Vec<&str>, exceptions: Vec<&str>) -> Vec<&'a str> {
    let mut breakpoints = BTreeSet::new();
    for p in delimiters {
        let p_len = p.chars().count();
        for m in text.match_indices(p) {
            for i in m.0..m.0 + p_len {
                breakpoints.insert(i);
            }
        }
    }

    for p in exceptions {
        let p_len = p.chars().count();
        for m in text.match_indices(p) {
            println!("-{}", p);
            for i in m.0..m.0 + p_len {
                match breakpoints.remove(&i) {
                    false => println!("--{}", i),
                    true => println!("-+{}", i),
                }
            }
            println!("_");
        }
    }

    let r_exceptions = vec![r#"\d\.\d"#, r#"\s\.\d"#];

    for p in r_exceptions {
        let re = Regex::new(p).unwrap(); //todo: get rid of unwraps
        println!("regex: {}", p);
        for m in re.find_iter(text) {
            println!("{}: {}..{}", m.as_str(), m.start(), m.end());
        }
    }




    for i in breakpoints.iter() {
        print!("{}, ", i)
    }
    println!("---");

    for i in 1..text.chars().count() {
        let current = i as usize;
        let previous = i - 1 as usize;
        if breakpoints.contains(&current) && breakpoints.contains(&previous) {
            //todo: unimplemented
        }
    }

    for (i, c) in text.chars().enumerate() {
        let marker = if breakpoints.contains(&i) {
            "*"
        } else {
            " "
        };

        println!("{} {}", c, marker);
    }

    let result = vec![];
    result
}

#[cfg(test)]
mod tests {
    use crate::sentence::split_sentences;

    #[test]
    fn it_works() {
        let text = "Mr. Smith bought cheapsite.com for 1.5 million dollars, i.e. he paid a lot for it. Did he mind? Adam Jones Jr. thinks he didn't. In any case, this isn't true... Well, with a probability of .9 it isn't. C. elegans should do it. Ю";
        for s in split_sentences(text) {
            println!("{}", s);
        }
        //assert_eq!(s.chars().count(), 10);
    }
}
