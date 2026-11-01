#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

struct Rectangle {
    width: f64,
    height: f64,
}

fn area(rect: Rectangle) -> f64 {
    rect.width * rect.height
}

fn perimeter(rect: Rectangle) -> f64 {
    (rect.width + rect.height) * 2.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn swap_tuple(pair: (i32, i32)) -> (i32, i32) {
    (pair.1, pair.0)
}

enum Command {
    Show(&'static str),
    Add(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
    Stop,
}

fn calculate(command: &Command) -> Option<i32> {
    match command {
        Command::Add(a, b) => Some(a + b),
        Command::Multiply(a, b) => Some(a * b),
        Command::Divide(a, b) if *b != 0 => Some(a / b),
        _ => None,
    }
}

fn announce(project: String) -> String {
    println!("Проект: {project}");
    project
}

fn add_suffix(text: String) -> (String, String) {
    let tagged = format!("{text}-v1");
    (tagged, text)
}

struct Course {
    title: String,
    lessons_completed: u16,
}

impl Course {
    fn title(&self) -> &str {
        &self.title
    }

    fn complete_lesson(&mut self) {
        self.lessons_completed += 1;
    }
}

fn longest_word<'a>(words: &[&'a str]) -> Option<&'a str> {
    words.iter().copied().max_by_key(|w| w.len())
}

struct Lesson {
    title: String,
    notes: String,
    views: u32,
}

impl Lesson {
    fn new(title: String, notes: String) -> Self {
        Self {
            title,
            notes,
            views: 0,
        }
    }
    fn title(&self) -> &str {
        &self.title
    }
    fn notes_len(&self) -> usize {
        self.notes.len()
    }
    fn add_note(&mut self, extra: &str) {
        self.notes.push_str(extra);
    }
    fn view(&mut self) {
        self.views += 1;
    }
    fn publish(self) -> String {
        format!("Урол '{}' опубликован.", self.title)
    }
}

fn print_title(title: &str) {
    println!("{}", title);
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
    fn first(&self) -> &T {
        &self.first
    }
    fn second(&self) -> &T {
        &self.second
    }
}

impl Pair<f64> {
    fn average(&self) -> f64 {
        (self.first + self.second) / 2.0
    }
}

trait Summary {
    const KIND: &'static str;

    fn summary(&self) -> String;

    fn headline(&self) -> String {
        format!("[{}] summary", Self::KIND)
    }
}

struct Article {
    title: String,
    author: String,
}

struct Note {
    text: String,
}

impl Summary for Article {
    const KIND: &'static str = "Article";

    fn summary(&self) -> String {
        format!("[{}] {} ({})", Self::KIND, self.title, self.author)
    }
}

impl Summary for Note {
    const KIND: &'static str = "Note";

    fn summary(&self) -> String {
        format!("[{}] {}", Self::KIND, self.text)
    }
}

fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summary());
}

#[derive(Default)]
struct FeedOptions {
    compact: bool,
    limit: u8,
}

fn main() {
    let price = 12.3456;
    let name = "Rust";

    println!("{name}");
    println!("{:.2}", price);
    println!("|{:>8}|", name);
    println!("|{:<8}|", name);
    println!("|{:^8}|", name);
    println!("|{:0>6}|", 31);
    println!("{0} / {1} / {0}", "Display", "Debug");
}
