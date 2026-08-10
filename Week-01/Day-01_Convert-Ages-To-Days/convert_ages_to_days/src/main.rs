fn calc_age(age: i32) -> i32 {
    age * 365
}

fn main() {
    assert_eq!(calc_age(65), 23725);
    assert_eq!(calc_age(20), 7300);
    assert_eq!(calc_age(0), 0);
}1