use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;


struct AsyncCache<K, V> 
where 
    K: Eq + Hash + Clone,
    V: Clone,
{
    data: Arc<HashMap<K, V>>,
    max_size: usize,
} 

impl<K, V> AsyncCache<K, V>
where 
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(max_size: usize) -> Self {
        Self { 
            data: Arc::new(HashMap::new()), 
            max_size 
        }        
    }

    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        let data = self.data.deref();
        if data.len() >= self.max_size {
            if let Some(first_key) = data.keys().next().cloned() {
                data.remove(&first_key);
            }
        }
        data.insert(key, value)
    }
}

fn main() {
    

}
