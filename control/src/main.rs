fn main() {
    let day = 10;
    let weekday = match day {
        1 | 2 | 3 | 4 | 5 => "рабочий",
        6 | 7 => "выходной",
        _ => unreachable!("дней недели всего 7"),
    };
    println!("Сегодня {weekday} - день");
}
