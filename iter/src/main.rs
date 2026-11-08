use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;
use std::ptr::swap;

struct ErrorFreq {
    freq: usize,
    error: String,
}
impl PartialEq for ErrorFreq {
    fn eq (&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}
impl Eq for ErrorFreq {
}
impl PartialOrd for ErrorFreq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}
impl Ord for ErrorFreq {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

struct LogAnalyzer {
    errors: HashMap<String, usize>,
}
impl LogAnalyzer {
    fn new () -> Self {
        // let mut vs: HashMap<String, usize> = HashMap::new();
        // for line in text.lines() {
        //     let words = line.split_whitespace();
        //     let description = words.clone().skip(3).collect::<Vec<&str>>().join(" ");
        //                
        //     *vs.entry(description).or_insert(0) += 1;
        // }
        Self {errors: HashMap::new()}
    }
    fn consume(&mut self, input: &str) {
        let mut errors = HashMap::new();
        std::mem::swap(&mut errors, &mut self.errors);
        self.errors = 
            input.lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let words = line.split_whitespace();
                    let status = words.clone().skip(1).next().unwrap();
                    let description = words.skip(3).collect::<Vec<_>>().join(" ");
                    (status, description)
                })
                .filter_map(|(s, d)| { 
                    (s == "[ERROR]").then_some(d) 
                })
                .fold(errors, |mut acc, d| {
                    *acc.entry(d).or_insert(0) += 1;
                    acc
                });
    }
    fn errors(&self) -> &HashMap<String, usize> {
        &self.errors
    }
    fn top_errors(&self, count: usize) -> Vec<String> {
        let mut top: Vec<String> = Vec::new();
        let mut heap: BinaryHeap<ErrorFreq> = BinaryHeap::new();
        for (k, v) in self.errors.iter() {
            let eq = ErrorFreq { freq: *v, error: k.clone() };
            heap.push(eq);
        }
        heap.iter().take(count).for_each(|eq| top.push(eq.error.clone()));
        top
    }
}


fn main() {
    let logs = "\
   2025-05-02T09:00 [STATUS] ReqId= App launched
   2025-05-02T09:00 [STATUS] ReqId= Looking for config
   2025-05-02T09:00 [WARNING] ReqId= No config provided, using defaults
   2025-05-02T09:01 [STATUS] ReqId= Service started
   2025-05-02T09:10 [STATUS] ReqId=1 Got request
   2025-05-02T09:10 [ERROR] ReqId=1 Unauthorized
   2025-05-02T09:13 [STATUS] ReqId=2 Got request
   2025-05-02T09:14 [ERROR] ReqId=2 Bad request format, dumped
   2025-05-02T09:20 [STATUS] ReqId=3 Got request
   2025-05-02T09:20 [STATUS] ReqId=3 Ok
   2025-05-02T09:25 [STATUS] ReqId=4 Got request
   2025-05-02T09:25 [STATUS] ReqId=4 Ok
   2025-05-02T09:27 [STATUS] ReqId=5 Got request
   2025-05-02T09:27 [STATUS] ReqId=5 Ok
   2025-05-02T10:00 [STATUS] ReqId=6 Got request
   2025-05-02T10:00 [STATUS] ReqId=6 Ok
   2025-05-02T10:09 [STATUS] ReqId=7 Got request
   2025-05-02T10:09 [ERROR] ReqId=7 Missing request required field, dumped
   2025-05-02T10:00 [STATUS] ReqId=8 Got request
   2025-05-02T10:00 [STATUS] ReqId=8 Ok
   2025-05-02T10:13 [STATUS] ReqId=9 Got request
   2025-05-02T10:14 [ERROR] ReqId=9 Bad request format, dumped
   2025-05-02T10:27 [STATUS] ReqId=10 Got request
   2025-05-02T10:27 [STATUS] ReqId=10 Ok
   2025-05-02T10:30 [STATUS] ReqId=11 Got request
   2025-05-02T10:31 [STATUS] ReqId=11 Ok
   2025-05-02T10:33 [STATUS] ReqId=12 Got request
   2025-05-02T10:33 [STATUS] ReqId=12 Ok
   2025-05-02T10:40 [STATUS] ReqId=13 Got request
   2025-05-02T10:42 [ERROR] ReqId=13 Internal service error
   2025-05-02T10:45 [STATUS] ReqId=14 Got request
   2025-05-02T10:45 [ERROR] ReqId=14 Missing request required field, dumped
   2025-05-02T10:47 [STATUS] ReqId=15 Got request
   2025-05-02T10:47 [STATUS] ReqId=15 Ok
   2025-05-02T10:49 [STATUS] ReqId=16 Got request
   2025-05-02T10:49 [STATUS] ReqId=16 Ok
   2025-05-02T10:52 [STATUS] ReqId=17 Got request
   2025-05-02T10:53 [STATUS] ReqId=17 Ok
   2025-05-02T10:55 [STATUS] ReqId=18 Got request
   2025-05-02T10:55 [ERROR] ReqId=18 Missing request required field, dumped
   2025-05-02T10:59 [STATUS] ReqId=19 Got request
   2025-05-02T10:59 [STATUS] ReqId=19 Ok
   2025-05-02T11:00 [STATUS] ReqId=20 Got request
   2025-05-02T11:01 [ERROR] ReqId=20 Missing request required field, dumped\
   ";
    
    let mut a = LogAnalyzer::new();
    a.consume(logs);
    println!("errors: {:?}", a.errors());
    println!("top 2 errors: {:?}", a.top_errors(2));
}
