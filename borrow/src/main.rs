

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

    let x = get_mem(&mut (27, 28), true);
    let mut arr  = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut arr1  = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let s1 = &mut arr[ .. ];    
    let s2 = &mut arr1[ .. ];    

    //let e =  get_elem(s1, 5);
    //let e1 =  get_elem1(s2, 5);

    //let (sl1, sl2) = get_slices(s1, 5);

    //println!("{:?} {:?}", sl1, sl2);
    get_4slices(s1);

    let x = 10;
    print_number(x);
    println!("x всё ещё доступен: {x}");

    let s = String::from("ownership");
    let s = take_and_return(s);
    println!("Строка вернулась обратно: {s}");

    let x = 42;
    let y = x;

    let project = String::from("calculator");
    let project = announce(project);
    println!("Длина project: {}", project.len());

    let release = String::from("rust");
    let tagged = add_suffix(release.clone());
    println!("Исходная метка: {release}");
    println!("Новая метка: {tagged}");

    let label = String::from("move");
    let label_copy = label.clone();
    println!("label = {label}, label_copy = {label_copy}");

    println!("x = {x}, y = {y}");


}

fn print_number(value: i32) {
    println!("value = {value}");
}

fn take_and_return(text: String) -> String {
    println!("Получили: {text}");
    text
}


fn announce(project: String) -> String {
    println!("Проект: {project}");
    project
}

fn add_suffix(text: String) -> String {
    format!("{text}-v1")
}

