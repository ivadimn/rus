use core::num;

fn main() {
    let tic_tac_toe: [[char; 3]; 3] = [
        [' ', 'X', '0'],
        ['0', 'X', ' '],
        ['X', ' ', '0'],
    ];
    
    println!("Первоначальное значение массива");
    print_tic_tac_toe(&tic_tac_toe);

    let fruits = vec!["apple", "banan", "abricos", "orange"];
    let second = fruits.get(5);
    match second {
        Some(fruit) => println!("Second element: {}", fruit),
        None => println!("Nothing..."),
    }

    let mut numbers = vec![11, 12, 13, 14, 15, 16];
    for number in numbers.iter_mut() {
        *number *= 2;
    }
    println!("numbers: {:?}", numbers);

    let person = ("Alice", 25, true);
    match person {
        (name, age, true) => println!("{}: возраст {} лет, работает", name, age),
        (name, age, false) => println!("{}: возраст {} лет, не работает", name, age),
        _ => println!("Unknown..."),
    }


}

fn print_tic_tac_toe(board: &[[char; 3]; 3]) {
    for row in board.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
} 
