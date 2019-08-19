use counter::Counter;
use regex::Regex;
use std::collections::HashSet;

struct Teaser {
    stop_words: HashSet<String>,
    ideal: f64,
}

impl Teaser {
    fn new() -> Teaser {
        Teaser {
            stop_words: HashSet::new(),
            ideal: 20.0,
        }
    }

    //from collections import Counter
    //from re import split as regex_split, sub as regex_sub, UNICODE as REGEX_UNICODE
    /*
        stopWords = set(["-", " ", ",", ".", "a", "e", "i", "o", "u", "t", "about", "above",
        "above", "across", "after", "afterwards", "again", "against", "all", "almost", "alone",
        "along", "already", "also", "although", "always", "am", "among", "amongst", "amoungst",
        "amount", "an", "and", "another", "any", "anyhow", "anyone", "anything", "anyway",
        "anywhere", "are", "around", "as", "at", "back", "be", "became", "because", "become",
        "becomes", "becoming", "been", "before", "beforehand", "behind", "being", "below",
        "beside", "besides", "between", "beyond", "both", "bottom", "but", "by", "call", "can",
        "cannot", "can't", "co", "con", "could", "couldn't", "de", "describe", "detail", "did",
        "do", "done", "down", "due", "during", "each", "eg", "eight", "either", "eleven", "else",
        "elsewhere", "empty", "enough", "etc", "even", "ever", "every", "everyone", "everything",
        "everywhere", "except", "few", "fifteen", "fifty", "fill", "find", "fire", "first",
        "five", "for", "former", "formerly", "forty", "found", "four", "from", "front", "full",
        "further", "get", "give", "go", "got", "had", "has", "hasnt", "have", "he", "hence",
        "her", "here", "hereafter", "hereby", "herein", "hereupon", "hers", "herself", "him",
        "himself", "his", "how", "however", "hundred", "i", "ie", "if", "in", "inc", "indeed",
        "into", "is", "it", "its", "it's", "itself", "just", "keep", "last", "latter", "latterly",
        "least", "less", "like", "ltd", "made", "make", "many", "may", "me", "meanwhile", "might",
        "mill", "mine", "more", "moreover", "most", "mostly", "move", "much", "must", "my",
        "myself", "name", "namely", "neither", "never", "nevertheless", "new", "next", "nine",
        "no", "nobody", "none", "noone", "nor", "not", "nothing", "now", "nowhere", "of", "off",
        "often", "on", "once", "one", "only", "onto", "or", "other", "others", "otherwise", "our",
        "ours", "ourselves", "out", "over", "own", "part", "people", "per", "perhaps", "please",
        "put", "rather", "re", "said", "same", "see", "seem", "seemed", "seeming", "seems",
        "several", "she", "should", "show", "side", "since", "sincere", "six", "sixty", "so",
        "some", "somehow", "someone", "something", "sometime", "sometimes", "somewhere", "still",
        "such", "take", "ten", "than", "that", "the", "their", "them", "themselves", "then",
        "thence", "there", "thereafter", "thereby", "therefore", "therein", "thereupon", "these",
        "they", "thick", "thin", "third", "this", "those", "though", "three", "through",
        "throughout", "thru", "thus", "to", "together", "too", "top", "toward", "towards",
        "twelve", "twenty", "two", "un", "under", "until", "up", "upon", "us", "use", "very",
        "via", "want", "was", "we", "well", "were", "what", "whatever", "when", "whence",
        "whenever", "where", "whereafter", "whereas", "whereby", "wherein", "whereupon",
        "wherever", "whether", "which", "while", "whither", "who", "whoever", "whole", "whom",
        "whose", "why", "will", "with", "within", "without", "would", "yet", "you", "your",
        "yours", "yourself", "yourselves", "the", "reuters", "news", "monday", "tuesday",
        "wednesday", "thursday", "friday", "saturday", "sunday", "mon", "tue", "wed", "thu",
        "fri", "sat", "sun", "rappler", "rapplercom", "inquirer", "yahoo", "home", "sports", "1",
        "10", "2012", "sa", "says", "tweet", "pm", "home", "homepage", "sports", "section",
        "newsinfo", "stories", "story", "photo", "2013", "na", "ng", "ang", "year", "years",
        "percent", "ko", "ako", "yung", "yun", "2", "3", "4", "5", "6", "7", "8", "9", "0",
        "time", "january", "february", "march", "april", "may", "june", "july", "august",
        "september", "october", "november", "december", "government", "police"])
        ideal = 20.0
    */

    fn Summarize(&self, title: &str, text: &str) -> Vec<String> {
        let mut summaries = vec![];
        let sentences = split_sentences(text);
        let keys = self.keywords(text);
        let titleWords = self.split_words(title);
        if sentences.len() <= 5 {
            return sentences.into_iter().map(|x| x.to_string()).collect();
        }

        //score sentences, and use the top 5 sentences
        let ranks = self.score(sentences, titleWords, keys).most_common(5);
        for rank in ranks {
            summaries.append(rank[0]);
        }

        return summaries;
    }

    /// score sentences based on different features
    fn score(&self, sentences: Vec<&str>, titleWords: Vec<&str>, keywords: Vec<&str>) -> Vec<(&str, f64)> {
        /*
        def score(sentences, titleWords, keywords):

        senSize = len(sentences)
        ranks = Counter()
        for i, s in enumerate(sentences):
            sentence = split_words(s)
            titleFeature = title_score(titleWords, sentence)
            sentenceLength = length_score(sentence)
            sentencePosition = sentence_position(i+1, senSize)
            sbsFeature = sbs(sentence, keywords)
            dbsFeature = dbs(sentence, keywords)
            frequency = (sbsFeature + dbsFeature) / 2.0 * 10.0

            #weighted average of scores from four categories
            totalScore = (titleFeature*1.5 + frequency*2.0 + sentenceLength*1.0 + sentencePosition*1.0) / 4.0
            ranks[s] = totalScore

        return ranks
        */

        let sen_size = sentences.len();
        let ranks = Counter::new();



        unimplemented!();
    }

    /*
    def sbs(words, keywords):
    score = 0.0
    if len(words) == 0:
    return 0
    for word in words:
    if word in keywords:
    score += keywords[word]
    return (1.0 / fabs(len(words)) * score)/10.0
    */

    /*
    def dbs(words, keywords):
    if (len(words) == 0):
    return 0

    summ = 0
    first = []
    second = []

    for i, word in enumerate(words):
    if word in keywords:
    score = keywords[word]
    if first == []:
    first = [i, score]
    else:
    second = first
    first = [i, score]
    dif = first[0] - second[0]
    summ += (first[1]*second[1]) / (dif ** 2)

    # number of intersections
    k = len(set(keywords.keys()).intersection(set(words))) + 1
    return (1/(k*(k+1.0))*summ)
    */

    /// splits a string into array of words
    fn split_words(&self, text: &str) -> Vec<&str> {
        /*
        def split_words(text):
        #
        try:
        text = regex_sub(r'[^\w ]', '', text, flags=REGEX_UNICODE)  # strip special chars
        return [x.strip('.').lower() for x in text.split()]
        except TypeError:
        print "Error while splitting characters"
        return None
        */
        unimplemented!()
    }

    /// get the top 10 keywords and their frequency scores
    /// ignores blacklisted words in stopWords,
    /// counts the number of occurrences of each word
    fn keywords(&self, text:&str) -> Vec<&str> {
        /*
        def keywords(text):

        text = split_words(text)
        numWords = len(text)  # of words before removing blacklist words
        freq = Counter(x for x in text if x not in stopWords)

        minSize = min(10, len(freq))  # get first 10
        keywords = {x: y for x, y in freq.most_common(minSize)}  # recreate a dict

        for k in keywords:
        articleScore = keywords[k]*1.0 / numWords
        keywords[k] = articleScore * 1.5 + 1

        return keywords
        */
        unimplemented!();
    }

    fn length_score(&self, sentence: &str) -> f64
    {
        /*
        def length_score(sentence):
        return 1 - fabs(ideal - len(sentence)) / ideal
        */
        let len = s.chars().count();
        return 1.0 - f64::abs(self.ideal - len) / self.ideal;
    }

    fn title_score(&self, title: &str, sentence: &str) {
        /*
        def title_score(title, sentence):
        title = [x for x in title if x not in stopWords]
        count = 0.0
        for word in sentence:
        if (word not in stopWords and word in title):
        count += 1.0

        if len(title) == 0:
        return 0.0

        return count/len(title)
        */
        unimplemented!();
    }
}

/// The regular expression matches all sentence ending punctuation and splits the string at those points.
/// At this point in the code, the list looks like this ["Hello, world", "!" ... ]. The punctuation and all quotation marks
/// are separated from the actual text.
fn split_sentences(text: &str) -> Vec<&str> {
    let re = Regex::new(r##"(?<![A-ZА-ЯЁ])([.!?]"?)(?=\s+\"?[A-ZА-ЯЁ])"##).unwrap();
    let split = re.split(text);

    // The first s_iter line turns each group of two items in the list into a tuple,
    // excluding the last item in the list (the last item in the list does not need to have this performed on it). Then,
    // the second s_iter line combines each tuple in the list into a single item and removes any whitespace at the beginning
    // of the line. Now, the s_iter list is formatted correctly but it is missing the last item of the sentences list. The
    // second to last line adds this item to the s_iter list and the last line returns the full list.
    /*
    sentences = regex_split(u'(?<![A-ZА-ЯЁ])([.!?]"?)(?=\s+\"?[A-ZА-ЯЁ])', text, flags=REGEX_UNICODE)
        s_iter = zip(*[iter(sentences[:-1])] * 2)
        s_iter = [''.join(map(unicode,y)).lstrip() for y in s_iter]
        s_iter.append(sentences[-1])
        return s_iter
    */
    unimplemented!();
}

// different sentence positions indicate different probability of being an important sentence
fn sentence_position(i: f64, size: usize) -> f64 {
    let normalized = i / (size as f64);
    match normalized {
        x if x.in_range(0.0, 0.1) => 0.17,
        x if x.in_range(0.1, 0.2) => 0.23,
        x if x.in_range(0.2, 0.3) => 0.14,
        x if x.in_range(0.3, 0.4) => 0.08,
        x if x.in_range(0.4, 0.5) => 0.05,
        x if x.in_range(0.5, 0.6) => 0.04,
        x if x.in_range(0.6, 0.7) => 0.06,
        x if x.in_range(0.7, 0.8) => 0.04,
        x if x.in_range(0.8, 0.9) => 0.04,
        x if x.in_range(0.9, 1.0) => 0.15,
        _ => 0.0
    }
}

trait InRange {
    fn in_range(&self, begin: Self, end: Self) -> bool;
}

impl InRange for f64 {
    fn in_range(&self, begin: f64, end: f64) -> bool {
        *self > begin && *self <= end
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
