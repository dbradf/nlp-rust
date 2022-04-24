use std::collections::HashMap;

struct Node {
    next: HashMap<char, Node>,
    index: Option<usize>,
}

impl Node {
    pub fn insert(&mut self, content: &str, current_index: usize) {
        if content.is_empty() {
            self.index = Some(current_index);
            return;
        }
        let mut char_iter = content.chars();
        let head = char_iter.next().unwrap();
        let rest: String = char_iter.collect();
        let node = self.next.entry(head).or_insert_with(|| {
            Node {
                next: HashMap::new(),
                index: None,
            }
        });
        node.insert(&rest, current_index);
    }

    pub fn get_index(&self, content: &str) -> Option<usize> {
        if content.is_empty() {
            return self.index;
        }

        let mut char_iter = content.chars();
        let head = char_iter.next().unwrap();
        let rest: String = char_iter.collect();
        if let Some(next) = self.next.get(&head) {
            next.get_index(&rest)
        } else {
            None
        }
    }
}

pub struct Vocab {
    head: Node,
    current_index: usize,
}

impl Vocab {
    pub fn new(word_list: &[&str]) -> Self {
        let word_count = word_list.len();
        let mut head = Node {
            next: HashMap::new(),
            index: None,
        };
        for (i, word) in word_list.iter().enumerate() {
            head.insert(word, i);
        }

        Vocab {
            head,
            current_index: word_count,
        }
    }

    pub fn add_words(&mut self, word_list: &[&str]) {
        let word_count = word_list.len();

        for (i, word) in word_list.iter().enumerate() {
            self.head.insert(word, i + self.current_index);
        }

        self.current_index += word_count;
    }

    pub fn get_index(&self, word: &str) -> Option<usize> {
        self.head.get_index(word)
    }
}

fn main() {
    let dictionary_file = std::env::args()
        .nth(1)
        .expect("missing dictionary for 1st arg");
    let search_term = std::env::args()
        .nth(2)
        .expect("missing search term for 2nd arg");

    let dictionary = std::fs::read_to_string(dictionary_file).unwrap();
    let word_list: Vec<&str> = dictionary.lines().collect();

    let vocab = Vocab::new(&word_list);

    println!("Searching for word: {}", &search_term);

    if let Some(index) = vocab.get_index(&search_term) {
        println!("Found word at index {}!", index);
    } else {
        println!("Word not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_a_vocab() {
        let dictionary = vec!["words", "in", "the", "dictionary", "what", "where", "when"];

        let vocab = Vocab::new(&dictionary);

        assert!(vocab.get_index("dictionary").is_some());
        assert!(vocab.get_index("where").is_some());
        assert!(vocab.get_index("whereabouts").is_none());
        assert!(vocab.get_index("d").is_none());
    }

    #[test]
    fn test_updating_a_vocab() {
        let dictionary = vec!["words", "in", "the", "dictionary"];

        let mut vocab = Vocab::new(&dictionary);

        vocab.add_words(&vec!["what", "where", "when"]);

        assert!(vocab.get_index("dictionary").is_some());
        assert!(vocab.get_index("where").is_some());
        assert!(vocab.get_index("whereabouts").is_none());
        assert!(vocab.get_index("d").is_none());
    }
}
