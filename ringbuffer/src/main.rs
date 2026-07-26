
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
            rb.data[rb.write_idx] = *d;
            count += 1;
            rb.write_idx += 1;
        }
    }
    if rb.read_idx == len {
        rb.read_idx = 0;
    }    
    count
}

fn read<'a>(rb: &mut RingBuffer, count: usize, data: &'a mut Vec<u8> ) -> &'a [u8] {

    let len = rb.data.len();
    let mut cread: usize = 0;

    if rb.read_idx >= rb.write_idx {
        return &[];
    }

    for i in 0 .. count {
        if rb.read_idx < rb.write_idx {
            data.insert(i, rb.data[rb.read_idx]);
            rb.data[rb.read_idx] = 0;
            rb.read_idx += 1;
            cread += 1;
        }
    }
    if rb.read_idx == len {
        rb.read_idx = 0;
    }

    let s =  &data[..cread];
    s
}

fn print_buffer(rb: &RingBuffer) {
    println!("{:?}", rb.data);
}

fn main() {
    println!("Hello, world!");

    let mut ring = create(3);

    let mut writed = write(&mut ring, &[1, 2]);
    println!("Writed: {} bytes, Result: {:?}", writed, ring.data);

    writed = write(&mut ring, &[3, 4]);
    println!("Writed: {} bytes, Result: {:?}", writed, ring.data);

    
}
