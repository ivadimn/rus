#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
    price: f64,
}

struct Inventory {
    items: Vec<InventoryItem>,
    item_index: HashMap<u32, usize>,  // Быстрый поиск по id
    name_index: HashMap<String, Vec<usize>>, // Поиск по имени
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            items: Vec::new(),
            item_index: HashMap::new(),
            name_index: HashMap::new(),
        }
    }
    fn add_item(&mut self, item: InventoryItem) {
        let idx = self.items.len();
        self.items.push(item.clone());
        self.item_index.insert(item.id, idx);
        self.name_index.entry(item.name.clone())
            .or_insert_with(Vec::new)
            .push(idx);
    }
    fn find_by_id(&self, id: u32) -> Option<&InventoryItem> {
        self.item_index.get(&id)
            .and_then(|&idx| self.items.get(idx))
    }
    fn find_by_name(&self, name: &str) -> Vec<&InventoryItem> {
        self.name_index.get(name)
            .map(|indices| {
                indices.iter()
                    .filter_map(|&idx| self.items.get(idx))
                    .collect()
            })
            .unwrap_or_else(Vec::new)
    }
    fn total_value(&self) -> f64 {
        self.items.iter()
            .map(|item| item.price * item.quantity as f64)
            .sum()
    }
}

struct VotingSystem {
    voting: HashMap<String, u32>
}

impl VotingSystem {
    fn new() -> Self {
        Self {
          voting: HashMap::new()  
        }
    }

    fn vote_for(&mut self, candidate: String) {
        *self.voting.entry(candidate)
            .or_insert(0) += 1;
            
    }

    fn get_results(&self) -> &HashMap<String, u32> {
        &self.voting
    }

    fn get_winners(&self) -> Vec<&String> {

        let mut v: Vec<(&String, &u32)> = self.voting.iter()
            .collect::<Vec<_>>();
        v.sort_by(|a, b| b.1.cmp(a.1));
        v.iter().take(3).map(|(s, n)| *s).collect()
        
    }
}



fn main() {
    let mut vs = VotingSystem::new();
        vs.vote_for("C".into());
        vs.vote_for("Java".into());
        vs.vote_for("C++".into());
        vs.vote_for("Java".into());
        vs.vote_for("Rust".into());
        vs.vote_for("Rust".into());
        vs.vote_for("Go".into());
        vs.vote_for("Julia".into());
        vs.vote_for("Go".into());
        vs.vote_for("Rust".into());
        println!("results: {:?}", vs.get_results());
        println!("winners: {:?}", vs.get_winners());
}
