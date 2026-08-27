// copywrite Marat Purnyn 2026
fn progress_count(miles: Vec<i32>) -> i32 {
    print!("saturday miles: {:?}\n", miles);
    let mut count = 0;
    for i in 1..miles.len() {
        if miles[i] > miles[i - 1] {
            count += 1;
        }
    }
    println!("progress days: {}\n", count);
    count
}

fn main() {
    assert_eq!(progress_count(vec![3, 4, 1, 2]), 2);
    assert_eq!(progress_count(vec![10, 11, 12, 9, 10]), 3);
    assert_eq!(progress_count(vec![6, 5, 4, 3, 2, 9]), 1);
    assert_eq!(progress_count(vec![9, 9]), 0);
}
