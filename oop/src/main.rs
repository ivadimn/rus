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

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let new_curr = self.next;

        self.curr = new_curr;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() ->Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x* 2)
        .collect();
    println!("Doubled values: {:?}", doubled);
    
    let even: Vec<&i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .collect();
    println!("Even values: {:?}", even);

    // fold() - выполняет произвольную агрегацию
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Произведение: {}", product);

    let iter = numbers.iter()
        .map(|x| {
            println!("Умножаем {} на 2", x);
            x * 2
        })
        .filter(|x| {
            println!("Проверяем {} на чётность", x);
            x % 2 == 0
        });
    println!("Итератор создан, но вычисления ещё не начались");
    let result: Vec<_> = iter.collect();
    println!("Результат: {:?}", result);

    let fib: Vec<u32> = fibonacci()
        .take(10)
        .collect();
    println!("Числа Фибоначчи: {:?}", fib);

    let text = "The quick brown fox jumps over the lazy dog";
    let words_count = text.split_whitespace()
        .map(|word| word.to_lowercase())
        .fold(std::collections::HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        });
        println!("Количество слов {:?}",  words_count);

    let measurements = vec![1.5, 2.3, 4.7, 8.2, 3.1, 5.6];
    let stats = measurements.iter()
        .fold((0.0, 0.0), |acc, &x| {
            (acc.0 + x, acc.1 + 1.0)
        });
    let average = stats.0 / stats.1;
    println!("Среднее значение: {:.2}", average);

    // Пример 3: Работа с вложенными структурами
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    
    let flat: Vec<i32> = matrix.iter()
        .flat_map(|row| row.iter().cloned())
        .collect();
    println!("Плоский массив: {:?}", flat);

    
}
