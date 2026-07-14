

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
    let mut ran_arr: [usize; 4];
    let len = s.len() / 4;
    let ost: usize = s.len() % 4;

    if ost == 0 {
        ran_arr = [len; 4];
    }
    else {
        ran_arr = [len, len, len, len + ost]
    }

    let mut left: usize = 0;
    for (l, i) in ran_arr.iter().enumerate() {
        arr_slice[*i] = &s[left .. left + l];
        left += l;
    }
    for sl in arr_slice {
        println!("{:?}", sl);
    }
    
}

fn main() {

    let x = get_mem(&mut (27, 28), true);
    let mut arr  = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut arr1  = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let s1 = &mut arr[ .. ];    
    let s2 = &mut arr1[ .. ];    

    //let e =  get_elem(s1, 5);
    //let e1 =  get_elem1(s2, 5);

    //let (sl1, sl2) = get_slices(s1, 5);

    //println!("{:?} {:?}", sl1, sl2);
    get_4slices(s1);

}
