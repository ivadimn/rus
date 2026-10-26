use rand::RngExt;

fn random_number() -> i32 {
    let mut rng = rand::rng();
    rng.random_range(1 ..= 10)
}

pub fn generate_task() -> (i32, i32) {
    (random_number(), random_number())
}
