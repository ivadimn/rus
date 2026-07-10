use std::{collections::HashMap, thread::scope};
// pub struct Vec<T> {
//     ptr: *mut T,
//     cap: usize,
//     len: usize,
// }

enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
}

fn vec_stat(v : &Vec<i32>) {
    let ve : Vec<i32> = v.iter().map(|x| *x).collect();
    let sum: i32 = ve.iter().sum();
    println!("Avg = {}", sum / (ve.len() as i32));

}


fn main() {
//    let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//     }
//     println!("{:?}", v);

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Float(10.12),
    //     SpreadsheetCell::Text(String::from("blue")),
    // ];

    // v.extend([1, 2, 3]);

    // for x in &v {
    //     println!("{x}");
    // }

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("s3 is {s3}");

    // for c in "ЗдоровоMike".chars() {
    //     println!("{c}");
    // }

    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("{score}");

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    vec_stat(&v);

    let all = &v[..];
    println!("{:?}", all);
    let first = &v[..1];
    println!("{:?}", first);
    let all_but_first = &v[1..];
    println!("{:?}", all_but_first);
    let middle = &v[2..5];
    println!("{:?}", middle);



}
