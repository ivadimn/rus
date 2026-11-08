use std::collections::{BTreeSet, HashMap};

fn make_key(word: &str) -> [usize; 128] {
    let mut key = [0usize; 128];
    for c in word.chars() {
        key[c as usize] += 1;
    }
    return key;
}

fn main() {
    let input = vec!["aba", "cab", "baa", "aab", "acb", "abc"];
    
        let mut groups: HashMap<[usize;128],BTreeSet<&str>> = Default::default();
    
        for word in input.iter() { // word имеет тип &str
    
            let key = make_key(word);
            match groups.entry(key) {
                std::collections::hash_map::Entry::Vacant(v) => {
                    let mut group = BTreeSet::new();
                    group.insert(*word);
                    v.insert(group);
                },
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().insert(*word);
                },
            }
        }
        
        let mut count = 0;
        for (_, words) in groups.iter() {
            println!("{}:", count);
            for word in words.iter() {
                println!("    {}", word);
            }
            count += 1;
        }
}
