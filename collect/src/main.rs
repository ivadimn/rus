use std::collections::BTreeSet;
// Самобалансирующиеся деревья со своими структурами - не проблема
// Но свой тип должен реализовывать Eq. А Eq зависит от PartialEq.
// Eq - про равенство. Для порядка в дереве аналогично нужны PartialOrd и Ord
#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct Subscription {
    user: &'static str,
    service: &'static str,
}

fn main() {
    let mut students_class_a: BTreeSet<&'static str> = ["Bob", "Alice"].into();
    let next_student = "Eva";
    // .insert() вернёт, был ли элемент Новым
    if students_class_a.insert(next_student) {
        println!("added New user: {}", next_student);
    } else {
        println!("added Existing user: {}", next_student);
    }
    let mut students_class_b = BTreeSet::new();
    students_class_b.insert("Bob");
    for in_a_but_not_in_b in students_class_a.difference(&students_class_b) {
        println!("student '{}' is not in class b", in_a_but_not_in_b);
    }
    println!(
        "the first one, who'll go to the board is: {}",
        students_class_a.first().unwrap()
    );
    println!(
        "the last one, who'll go to the board is: {}",
        students_class_a.last().unwrap()
    );
    // Элементы отсортированы по возрастанию, в каком бы порядке ни добавлялись
    assert_eq!(students_class_a.first().unwrap(), &"Alice");
    students_class_a.remove("Bob");
    assert!(!students_class_a.contains("Bob"));
    students_class_a.pop_last();
    assert!(!students_class_a.contains("Eva"));
    students_class_a.pop_first();
    assert!(students_class_a.is_empty());

    let mut subscriptions = BTreeSet::new();
    subscriptions.insert(Subscription {
        user: "he",
        service: "practicum",
    });
    subscriptions.insert(Subscription {
        user: "she",
        service: "practicum",
    });
    subscriptions.insert(Subscription {
        user: "he",
        service: "other",
    });
    assert_eq!(subscriptions.len(), 3);
}
