

fn get_mem(t : &mut (i32, i32), flag: bool) -> &mut i32 {
    let r : &mut i32;
    if flag  {
        r = &mut t.0;
    }
    else { 
        r = &mut t.1;
    }
    r
}

fn get_elem(s: &mut [i32], index: usize) -> &mut i32 {
    println!("первый элемент среза: {}", s[index]);
    println!("в срезе {} элементов", s.len());
    &mut s[index]
}

fn get_elem1(s: &mut [i32], index: usize) -> &mut i32 {
    &mut s[s.len() - index]
}

fn get_slices(s: &[i32], index: usize) -> (&[i32], &[i32]) {
    (&s[ .. index], &s[index .. ])
}

fn get_4slices(s: &mut [i32]) {

    let mut arr_slice: [&[i32]; 4] = [&[]; 4];
    let len = s.len() / 4;
    let ost: usize = s.len() % 4;

    let ran_arr: [usize; 4] = if ost == 0 {
        [len; 4]
    } else if ost == 1 {
        [len, len, len, len + ost]
    } else if ost == 2 {
        [len, len, len + 1, len + 1]
    } else {
        [len, len + 1, len + 1, len + 1]
    };
    
    let mut left: usize = 0;
    for (i, l) in ran_arr.iter().enumerate() {
        arr_slice[i] = &s[left .. left + l];
        left += l;
        println!("[{}] = {}", i, l);
    }
    for sl in arr_slice {
        println!("{:?}", sl);
    }
    
}

fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result =  get_vec_slice(&a);
    println!("{:?}", result);
}

fn get_vec_slice(param_1: &[i32]) -> &[i32] {
    &param_1[0 .. 3]
}

fn get_str_ref<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

fn get_int_ref<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32 {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}



