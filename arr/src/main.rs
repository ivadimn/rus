#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::collections::{BinaryHeap, HashMap};

struct WordFreq {
    freq: usize,
    word: String,
}
impl PartialEq for WordFreq {
    fn eq (&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}
impl Eq for WordFreq {
}
impl PartialOrd for WordFreq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}
impl Ord for WordFreq {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}
// Как использовать Reverse(WordFreq):
// top_words.push(std::cmp::Reverse(WordFreq{word: "hello".into(), freq: 2}));
// Обратиться к WordFreq из Reverse можно по `.0`

struct Analyzer {
    freq: HashMap<String, usize>,
}
impl Analyzer {
    fn new(text: &str) -> Self {
        let mut freq: HashMap<String, usize> = HashMap::new();
        let clear_text = text.replace( &['!', ',', '.', ':', '(', ')'], "");
        let words: Vec<&str> = clear_text.split_whitespace().collect();
        
        for word in words {
            *freq.entry(String::from(word)).or_insert(0) += 1;
        }
        Self { freq }
    }
    fn freq(&self) -> &HashMap<String, usize> {
        &self.freq
    }
    fn words(&self) -> Vec<String> {
        self.freq.keys().cloned().collect()
    }
    
    fn top_words(&self, count: usize) -> Vec<String> {
        let mut heap = BinaryHeap::<std::cmp::Reverse<WordFreq>>::new();
        let mut top: Vec<String> = Vec::new();
        for (word, freq) in self.freq.iter()  {
            heap.push(std::cmp::Reverse(WordFreq {freq: *freq, word: word.into()}));
        }
        let mut index = 0usize;
        while let Some(revers) = heap.pop() {
            top.push(revers.0.word);
            index += 1;
            if index == count {
                break;
            }
        }
        top
    }
}


fn main() {
    let text = "pub auto trait Unpin.
Types that do not require any pinning guarantees.
For information on what “pinning” is, see the pin module documentation.
Implementing the Unpin trait for T expresses the fact that T is pinning-agnostic: it shall not expose nor rely on any pinning guarantees. This, in turn, means that a Pin-wrapped pointer to such a type can feature a fully unrestricted API. In other words, if T: Unpin, a value of type T will not be bound by the invariants which pinning otherwise offers, even when “pinned” by a Pin<Ptr> pointing at it. When a value of type T is pointed at by a Pin<Ptr>, Pin will not restrict access to the pointee value like it normally would, thus allowing the user to do anything that they normally could with a non-Pin-wrapped Ptr to that value.
The idea of this trait is to alleviate the reduced ergonomics of APIs that require the use of Pin for soundness for some types, but which also want to be used by other types that don’t care about pinning. The prime example of such an API is Future::poll. There are many Future types that don’t care about pinning. These futures can implement Unpin and therefore get around the pinning related restrictions in the API, while still allowing the subset of Futures which do require pinning to be implemented soundly.
For more discussion on the consequences of Unpin within the wider scope of the pinning system, see the section about Unpin in the pin module.
Unpin has no consequence at all for non-pinned data. In particular, mem::replace happily moves !Unpin data, which would be immovable when pinned (mem::replace works for any &mut T, not just when T: Unpin).
However, you cannot use mem::replace on !Unpin data which is pinned by being wrapped inside a Pin<Ptr> pointing at it. This is because you cannot (safely) use a Pin<Ptr> to get a &mut T to its pointee value, which you would need to call mem::replace, and that is what makes this system work.";
        
    let an = Analyzer::new(text);
    println!("Words freq: {:?}", an.freq());
    println!("____________________________________________");
    println!("Words: {:?}", an.words());
    println!("____________________________________________");
    println!("Top words: {:?}", an.top_words(10));
  

}


