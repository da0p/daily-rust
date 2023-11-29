use std::collections::HashMap;

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let to_name = HashMap::from([
        ('R', "radishes"),
        ('C', "clover"),
        ('G', "grass"),
        ('V', "violets"),
    ]);

    let to_seat = HashMap::from([
        ("Alice", 0),
        ("Bob", 2),
        ("Charlie", 4),
        ("David", 6),
        ("Eve", 8),
        ("Fred", 10),
        ("Ginny", 12),
        ("Harriet", 14),
        ("Ileana", 16),
        ("Joseph", 18),
        ("Kincaid", 20),
        ("Larry", 22),
    ]);

    let mut plants = vec![];
    let seat = to_seat.get(_student).unwrap();
    for row in _diagram.split_whitespace() {
        for name in row.chars().skip(*seat).take(2) {
            let plant = to_name.get(&name).unwrap().to_owned();
            plants.push(plant);
        }
    }

    plants
}
