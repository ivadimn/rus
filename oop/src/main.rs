use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::{Arc, Mutex};


struct AsyncCache<K, V> 
where 
    K: Eq + Hash + Clone,
    V: Clone,
{
    data: Arc<Mutex<HashMap<K, V>>>,
    max_size: usize,
} 

impl<K, V> AsyncCache<K, V>
where 
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(max_size: usize) -> Self {
        Self { 
            data: Arc::new( Mutex::new(HashMap::new())), 
            max_size 
        }        
    }

    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        let data = self.data.clone();
        if data.lock().unwrap().len() >= self.max_size {
            if let Some(first_key) = data.lock().unwrap().keys().next().cloned() {
                data.lock().unwrap().remove(&first_key);
            }
        }
        data.lock().unwrap().insert(key, value)
    }
}

fn main() {
    

}
