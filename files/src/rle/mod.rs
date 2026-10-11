const MAX_LENGTH: u8 = 15;
const BYTE_LEN: u8 = 8;
const MAX_BYTE_POSITION: u8 = 7;
const GROUP_SHIFT: u8 = 4;

pub fn coder(buf: Vec<u8>) -> Vec<u8> {
    let mut code: Vec<u8> = Vec::new();
    let mut index = 0usize;
    let mut bit_count = 0u8;
    let mut byte_out = 0u8;
    let mut current_bit = 0u8;
    //println!("Начали обработку массива");    
    loop {
        if index == buf.len() {
            println!("последний обрабатываемый бит {}", current_bit);
            byte_out = add_group(current_bit, bit_count, byte_out);
            code.push(byte_out);
            break;
        }
        let mut byte = buf[index];
        println!("\n------------------------------------------------------------------\n");
        println!("Обрабатываем {} байт: {:08b}", index + 1, byte);
        
        for _ in 0 .. BYTE_LEN {
            let bit: u8 = byte >> MAX_BYTE_POSITION;
            byte <<= 1;
            println!("\tОбрабатываемый бит: {}", bit);
            if bit == current_bit {
                bit_count += 1;
                println!("\tОбрабатываемый бит равен current_bit, увеличиваем счётчик {}", bit_count);
            } else {
                println!("\tОбрабатываемый бит не равен current_bit");
                byte_out = add_group(current_bit, bit_count, byte_out);
                println!("\t\tПоместили счётчик {} в группу {:08b}", bit_count, byte_out);
                if current_bit == 1 {
                    code.push(byte_out);
                    println!("\t\tПоместили byte_out в code и обнулили {:08b}", byte_out);
                    byte_out = 0;
                }
                current_bit = !current_bit & 1;
                bit_count = 1;
                println!("\t\tИнвертировали current_bit и сбросили bit_count в 1");
            }
            if bit_count == MAX_LENGTH {
                byte_out = add_group(current_bit, bit_count, byte);
                if current_bit == 1 {
                    code.push(byte_out);
                    byte_out = 0;
                }
                bit_count = 0;
                current_bit = !current_bit & 1;
            }
        }
        index += 1;
    }
    code 
}

pub fn coder1(buf: Vec<u8>) -> Vec<u8> {
    let mut code: Vec<u8> = Vec::new();

    let groups: Vec<(u8, u64)> = grouping(buf);

    let mut current_group = 0u8;
    let mut byte = 0u8;

    while let Some(group) = groups.iter().next() {

        let dg = div_group(group);

        if group.0 == current_group {

        }
    }

    code
    
}

fn div_group(group: &(u8, u64)) -> Vec<u8> {
    let mut dg: Vec<u8> = Vec::new();
    let count = group.1;

    let steps = count / MAX_LENGTH as u64;
    for _ in 0 .. steps {
        dg.push(MAX_LENGTH);
    } 
    let ost = count % MAX_LENGTH as u64;
    dg.push(ost as u8);
    dg
}

fn grouping(buf: Vec<u8>) -> Vec<(u8, u64)> {
    let mut groups: Vec<(u8, u64)> = Vec::new();

    let mut current_bit: u8 = if (buf[0] & 128) != 0 {1} else {0};
    
    let mut count = 0u64;
    for e in buf {
        let bits = get_bits(e);
        for bit in bits {
            if bit == current_bit {
                count += 1;
            } else {
                groups.push((current_bit, count));
                current_bit = !current_bit & 1;
                count = 1;
            }
        }
    }
    groups
}

fn get_bits(byte: u8) -> Vec<u8> {

    (0u8 .. 8u8)
        .map(|i| if (byte >> (7 - i) & 1) != 0 {1} else {0})
        .collect()
}


fn add_group(bit: u8, bit_count: u8, byte: u8) -> u8 {
    if bit == 0 {
       bit_count << GROUP_SHIFT
    }
    else {
       byte + bit_count
    }
}

pub fn decoder(buf: Vec<u8>) -> Vec<u8> {
    let mut decode: Vec<u8> = Vec::new();
    let mut position = 0u8;
    let mut byte = 0u8;
    let mut can_insert = 0u8;
    let aref = &buf;
    for e in aref.into_iter() {
        let mut zero_count = e >> 4;
        let mut one_count = e & 15;
        println!("\n--------------------------------------------------------------------------");
        println!("Текущий обрабатыемый байт {:08b}", e);
        println!("zero_count {}, one_count {}", zero_count, one_count);
        
        //let number: u16 = (1 << one_count) - 1;
        can_insert = 8 - position; // количество незаполненных бит в текущем байте
        while can_insert < zero_count {
            decode.push(byte);
            println!("Положили Байт при обработке нолей {:08b}", byte);
            byte = 0;
            position = 0;
            zero_count -= can_insert;
            can_insert = 8 - position;
        }
        position += zero_count;
            
        println!("Позиция перед вставкой ед.  {}", position);    
        can_insert = 8 - position; // количество незаполненных бит в текущем байте

        while can_insert < one_count {
            let overflow: u16 = 1 << can_insert;
            let number: u8 = (overflow - 1) as u8; //=3 
            byte |= number;
            decode.push(byte);
            println!("Положили Байт при обработке единиц {:08b}", byte);
            byte = 0;
            position = 0;
            one_count -= can_insert;
            can_insert = 8 - position;
        }
        let overflow: u16 = 1 << one_count;
        let mut number: u8 = (overflow - 1) as u8; //=3 
        number <<= get_shift(position, one_count);
                
        byte |= number;
        //println!("Byte {:08b}, number после сдвига {:08b}", byte, number);
        
        position += one_count;
        //println!("count {}, {:08b}", one_count, number);

    }
    decode.push(byte);
    // for e in decode.iter()   {
    //      println!("{:08b}", e);        
    // }

    decode
}

fn get_shift(position: u8, count: u8) -> u8 {
    8 - position - count
}

fn insert_bits(byte: u8, position: u8, count: u8) -> (u8, u8) {
    let mut byte_out = byte;
    let mut next_position = position;

    let mut index = 0u8;
    while next_position < 8 && index < count {
        byte_out = byte_out | (1 << (MAX_BYTE_POSITION - next_position));
        index += 1;
        next_position += 1;
    }
    (byte_out, index)
}
