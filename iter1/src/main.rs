use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct User {
    id: u32,
    interests: Vec<String>,
}
struct Product {
    id: u32,
    tags: Vec<String>,
}

struct RecommendationSystem {
    users: Vec<User>,
    products: Vec<Product>,
}

struct ProductAndTagsCount(u32, usize);

impl PartialEq for ProductAndTagsCount {
    fn eq (&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
impl Eq for ProductAndTagsCount {
}
impl PartialOrd for ProductAndTagsCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}
impl Ord for ProductAndTagsCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl RecommendationSystem {
    fn new () -> Self {
        Self{users: vec![], products: vec![]}
    }
    fn add_user (&mut self, user: User) {
        self.users.push(user);
    }
    fn add_product (&mut self, product: Product) {
        self.products.push(product);
    }
    fn recommend_for_user(&self, user_id: u32, count: usize) -> Vec<u32> {
        // ...
        // 1. Находит интересы пользователя
        // 2. Для каждого продукта вычисляет "релевантность" (количество общих тэгов продукта с интересами пользователя) 
        // 3. Возвращает топ-N продуктов
        // ...
        let int = self.users.iter().clone().find(|u| u.id == user_id);
        let Some(user) = int else {
                println!("User: {} not found!", user_id);
                return vec![];                
        }; 
        self.products.iter().clone()
            .map(|p| { 
                let mut count = 0u32;
                p.tags.iter().clone().fold(count, | | )
                (p.id, count)
            })
            .collect();
        
        println!("{:?}", int);
        
       
        vec![]
    }
}

fn main () {
    let mut system = RecommendationSystem::new();
    system.add_product(Product{id: 0, tags: vec!["малиновая".into(), "газель".into(), "новая".into()]});
    system.add_product(Product{id: 1, tags: vec!["серебристая".into(), "лада".into(), "подержанная".into(), "с подушкой".into()]});
    system.add_product(Product{id: 2, tags: vec!["фиолетовая".into(), "рено".into(), "новая".into(), "с кондеем".into()]});
    system.add_product(Product{id: 3, tags: vec!["зелёная".into(), "bmw".into(), "подержанная".into(), "с холодильником".into()]});
    system.add_product(Product{id: 4, tags: vec!["чёрная".into(), "москвич".into(), "новая".into(), "с тонированием".into()]});
    system.add_user(User{id: 0, interests: vec!["чёрная".into(), "рено".into(), "подержанная".into(), "с кондеем".into()]});
    system.add_user(User{id: 1, interests: vec!["малиновая".into(), "москвич".into(), "новая".into(), "с тонированием".into()]});
    let user0 = 0;
    let product0 = system.recommend_for_user(user0, 2);
    // println!("{} -> {:?}", user0, product0);
    // let user1 = 1;
    // let product1 = system.recommend_for_user(user1, 2);
    // println!("{} -> {:?}", user1, product1);
}