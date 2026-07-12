// Вам нужно реализовать программу обработки команд для дисплея.
// На вход пользователь подает:
// * 2 числа: размер дисплея
// * 1 число: цвет дисплея по-умолчанию (1 - красный, 2 - зеленый, 3 - синий)
// * Последовательность команд: набор чисел.
//
// Дисплей поддерживает следующие команды:
// * 1 x y - переместить курсор в позицию x y
// * 2 colour - перекрасить пиксель в цвет colour
//
// Пример входных данных:
// 4 4
// 1
// 1 2 2 2 3
// В результате пиксель по позиции (2,2) будет перекрашен в синий цвет

// Обновлять состояние дисплея нужно через метод matrix.set_colour(pos_x, pos_y, colour)

// Важно! Обязательна проверка на ошибки. Если пользователь просит переместиться на пиксель за пределами дисплея
// или ввел неправильный цвет, то вам нужно кинуть панику!

use std::{io::{self, Write}, process::Command};
mod matrix;
use matrix::Matrix;

struct Display {
    screen: (u32, u32),
    cursor: (u32, u32),
    matrix: Matrix,
}

impl Display {

    fn move_cursor(&mut self, x: u32, y: u32) {
        if x  < self.screen.0 && y < self.screen.1 {
            self.cursor.0 = x;
            self.cursor.1 = y;
        } 
        else {
            println!("Вышли за границы дисплея!");
        }
    }
    fn set_colour(&mut self, color: u8) {
        self.matrix.set_colour(self.cursor.0 as u64, self.cursor.1 as u64, color);
    }
}

fn create_display(max_width: u32, max_height: u32, default_colour: u8) -> Display {
    // ваш код сюда
    Display {
        screen: (max_width, max_height),
        cursor: (0, 0),
        matrix: Matrix::new(max_width, max_height, default_colour),
    }
}

fn process_commands(display: &mut Display, input: Vec<u64>) {
    // ваш код сюда
    let len = input.len();
    let mut index: usize = 0;
    while index < len {

        let command = input[index];
        
        match command {
            1 =>  if  index + 2 < len {
                        let x = input[index + 1] as u32;
                        let y = input[index + 2] as u32;
                        display.move_cursor(x, y);    
                        index += 3;
                  },
            2 =>  if  index +1 < len {
                        display.set_colour(input[index + 1] as u8);    
                        index += 2;
                  },
            _ => println!("Не правильная команда!")      
        }
            
    }
}


fn get_number() -> i32 {
    let mut s = String::new();
    //print!("Введите число: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();
    let number = s.trim().parse().unwrap();
    number
}

fn get_numbers() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    println!("Введите размеры дисплея (ширина высота):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (width, height) = parse_dimensions(&input);

    println!("Введите стандартный цвет дисплея (1 - красный, 2 - зеленый, 3 - синий):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let default_colour = match input.trim() {
        "1" => 1, // Красный
        "2" => 2, // Зеленый
        "3" => 3, // Синий
        _ => panic!("Неверный ввод цвета. Ожидалось 1, 2 или 3."),
    };

    // Создаём дисплей и заполняем его стандартным цветом
    let mut display = create_display(width, height, default_colour);

    // Ввод действий
    println!("Введите строку с действиями:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let commands = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // Отображение дисплея
    process_commands(&mut display, commands);

    display.matrix.display();
}

fn parse_dimensions(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Неверный ввод размера"))
        .collect();
    if parts.len() != 2 {
        panic!("Ожидалось два числа для размеров дисплея.");
    }
    (parts[0], parts[1])
}
