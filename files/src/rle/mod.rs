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
            //println!("последний обрабатываемый бит {}", current_bit);
            byte_out = add_group(current_bit, bit_count, byte_out);
            code.push(byte_out);
            break;
        }
        let mut byte = buf[index];
        //println!("\n------------------------------------------------------------------\n");
        //println!("Обрабатываем {} байт: {:08b}", index + 1, byte);
        
        for _ in 0 .. BYTE_LEN {
            let bit: u8 = byte >> MAX_BYTE_POSITION;
            byte <<= 1;
            //println!("\tОбрабатываемый бит: {}", bit);
            if bit == current_bit {
                bit_count += 1;
                //println!("\tОбрабатываемый бит равен current_bit, увеличиваем счётчик {}", bit_count);
            } else {
                //println!("\tОбрабатываемый бит не равен current_bit");
                byte_out = add_group(current_bit, bit_count, byte_out);
                //println!("\t\tПоместили счётчик {} в группу {:08b}", bit_count, byte_out);
                if current_bit == 1 {
                    code.push(byte_out);
                    //println!("\t\tПоместили byte_out в code и обнулили {:08b}", byte_out);
                    byte_out = 0;
                }
                current_bit = !current_bit & 1;
                bit_count = 1;
                //println!("\t\tИнвертировали current_bit и сбросили bit_count в 1");
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

    let mut byte_out = 0u8;
    let mut count: u8;

/*  00100010
    00010010
    01000001
    00010010
    10001111
    00000001
    10000000
*/
    println!("\nВ декодере\n");
    for byte in buf {
        
        let zero_count =  byte >> 4;
        println!("Zero count {}", zero_count);
        if zero_count > 0 {
            position += zero_count;
            if position > MAX_BYTE_POSITION {
                decode.push(byte_out);
                println!("{:08b}", byte_out);
                byte_out = 0;
                position = zero_count - BYTE_LEN;
            }
        }
        let mut one_count = byte & 15;
        println!("One count {}", one_count);
        (byte_out, count) = insert_bits(byte_out, position, one_count);
        one_count -= count;
        println!("{:08b}", byte_out);
        position += count;

        while one_count > 0 {
            
            
            
            print!("One count в цикле {}", one_count);
            if one_count == 0 {
                decode.push(byte_out);
                println!("{:08b}", byte_out);
                byte_out = 0;
                position = 0;    
            }   
            else {
                
            }  
        }
    }
    decode
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
