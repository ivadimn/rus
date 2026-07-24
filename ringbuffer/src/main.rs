
// Слайсы. (мы спрашиваем эту задачку на собеседования на уровено Junior Engineer)
// Ring Buffer (кольцевой буффер) - структура данных, которая позволяет очень удобно реализовывать очередь на массиве фиксированного размера.
// https://ru.wikipedia.org/wiki/%D0%9A%D0%BE%D0%BB%D1%8C%D1%86%D0%B5%D0%B2%D0%BE%D0%B9_%D0%B1%D1%83%D1%84%D0%B5%D1%80
// Ключевая идея в том, что заполняя буффер до конца мы переходим в начало
// Пример API, вызовов и как меняется состояние буффера:
// [ _ _ _ ] create(3)
// [ a b _ ] write "ab" -> return 2
// [ a b c ] write "cd" -> return 1
// [ _ b c ] read(1) -> return "a"
// [ e b c ] write "e" -> return 1
// [ e _ _ ] read(2) -> return "bc"
// Ваша задача написать такой буффер и добавить тесты

struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
}

fn create(size: usize) -> RingBuffer {
    RingBuffer { read_idx: 0, write_idx: 0, data: vec![0; size]}

}

fn write(rb: &mut RingBuffer, data: &[u8]) -> usize {
    let mut count: usize = 0;
    let len = rb.data.len();

    for d in data {
        if rb.write_idx < len {
            //rb.data.insert(rb.write_idx, *d);
            rb.data[rb.write_idx] = *d;
            count += 1;
            rb.write_idx += 1;
        }
        else {
            rb.write_idx = 0;
        }
    }
    count
}

fn read<'a>(rb: &mut RingBuffer, count: usize, data: &'a mut Vec<u8> ) -> &'a [u8] {

    let len = rb.data.len();
    let mut cread: usize = 0;

    for i in 0 .. count {
        if rb.read_idx < len {
            data.insert(i, rb.data[rb.read_idx]);
            rb.read_idx += 1;
            cread += 1;
        }
        else {
            rb.read_idx = 0;
        }
    }
    let s =  &data[..cread];
    s
    
}

fn main() {
    println!("Hello, world!");
    let mut v: Vec<u8> = vec![0; 10];
    let r: std::ops::Range<usize> = 0 .. 5;  

    for i in r {
        v[i] = 1;
    }

    println!("len: {}, {:?}", v.len(), v);
}
