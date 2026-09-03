fn main() {
    let tic_tac_toe: [[char; 3]; 3] = [
        [' ', 'X', '0'],
        ['0', 'X', ' '],
        ['X', ' ', '0'],
    ];
    
    println!("Первоначальное значение массива");
    print_tic_tac_toe(&tic_tac_toe);


}

fn print_tic_tac_toe(board: &[[char; 3]; 3]) {
    for row in board.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
} 
