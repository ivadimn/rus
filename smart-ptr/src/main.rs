#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::cell::{Cell, RefCell};
use std::error::Error;
use std::rc::Rc;


struct Counter {
    value: Cell<u32>,
    observers: RefCell<Vec<Box<dyn Fn(u32)>>>,
}

impl Counter {
    fn new() -> Self {
        Self {
            value: Cell::new(0),
            observers: RefCell::new(Vec::new()),
        }
    }

    fn subscribe<F>(&self, observer: F)
    where
        F: Fn(u32) + 'static,
    {
        self.observers.borrow_mut().push(Box::new(observer));
    }

    fn increment(&self) {
        let next = self.value.get() + 1;
        self.value.set(next);

        for observer in self.observers.borrow().iter() {
            observer(next);
        }
    }
} 

#[derive(Debug)]
struct Node {
    value: i32,
    neighbors: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            neighbors: RefCell::new(Vec::new()),
        })
    }

    fn add_neighbor(&self, neighbor: &Rc<Node>) {
        self.neighbors.borrow_mut().push(Rc::clone(neighbor));
    }
} 

struct SquareCache {
    input: u32,
    cached: RefCell<Option<u32>>,
}

impl SquareCache {
    fn new(input: u32) -> Self {
        Self {
            input,
            cached: RefCell::new(None),
        }
    }

    fn value(&self) -> u32 {
        if let Some(value) = self.cached.borrow().as_ref().copied() {
            return value;
        }

        let computed = self.input * self.input;
        *self.cached.borrow_mut() = Some(computed);
        computed
    }
}

struct Page {
    title: String,
    views: Cell<u32>,
}

impl Page {
    fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            views: Cell::new(0),
        }
    }

    fn open(&self) {
        self.views.set(self.views.get() + 1);
    }

    fn views(&self) -> u32 {
        self.views.get()
    }
}

fn subs1(value: u32) {
    println!("Subs1 {}", value);
}

fn subs2(value: u32) {
    println!("Subs2 {}", value);
}

fn subs3(value: u32) {
    println!("Subs3 {}", value);
}

fn subs4(value: u32) {
    println!("Subs4 {}", value);
}

#[derive(Debug)]
struct ComputationCache {
    slots: Vec<RefCell<Option<u32>>>,
}

impl ComputationCache {
    fn new(size: usize) -> Self {
        Self {
          slots: vec![RefCell::new(None); size]  
        } 
    }

    fn remember(&self, index: usize, value: u32) -> Result<(), CacheError> {

        let val = self.slots
            .get(index)
            .ok_or(CacheError::OutOfBounds(index))?;

        *val.borrow_mut() = Some(value);
                
        Ok(())
    }

    fn remember_pair(&self, left: usize, right: usize, 
                     left_value: u32, right_value: u32) -> Result<(), CacheError> {

        if left == right {
            return Err(CacheError::SameSlot)
        }

        self.remember(left, left_value)?;
        self.remember(right, right_value)?;

        Ok(())
    } 

    fn get(&self, index: usize) -> Result<Option<u32>, CacheError> {

        if let Some(cache) = self.slots.get(index) {
            Ok(*cache.borrow())
        } 
        else {
            Err(CacheError::OutOfBounds(index))
        }
    }

    fn snapshot(&self) -> Vec<Option<u32>> {
        self.slots.iter().map(|cache| *cache.borrow()).collect()
    }

}

#[derive(Debug, PartialEq)]
enum CacheError {
    OutOfBounds(usize),
    SameSlot,
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::OutOfBounds(index) => 
                write!(f, "Индекс выходит {} за границы", index),
            CacheError::SameSlot => write!(f, "Одинаковые слоты блокируются"),
        }
    }
}

impl Error for CacheError {}

fn main() {
    let cache = ComputationCache::new(4);
    if let Err(err) =  cache.remember(0, 10){
        println!("Ошибка: {}", err);
    }

    if let Err(err) =  cache.remember_pair(1, 2, 20, 30){
        println!("Ошибка: {}", err);
    }

    println!("cache = {:?}", cache.snapshot());
    
    if let Ok(c) = cache.get(2) {
        println!("slot 2 = {:?}", c);
    }
    
    if let Err(err) =  cache.remember_pair(2, 2, 20, 30){
        println!("Ошибка: {}", err);
    
    }
    
    
}
