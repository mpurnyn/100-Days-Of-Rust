// copywrite Marat Purnyn 2026

fn find_nemo(str: &str) -> String {
    let v: Vec<&str> = str.split(' ').collect();
    for (i, word) in v.iter().enumerate() {
        if *word == "Nemo" {
            let pos = i + 1;
            return format!("I found Nemo at {}!", pos);
        }
    }
    return format!("I can't find Nemo :(");
}

fn main() {
    assert_eq!(find_nemo("I am finding Nemo !"), "I found Nemo at 4!");
    assert_eq!(find_nemo("Nemo is me"), "I found Nemo at 1!");
    assert_eq!(find_nemo("I Nemo am"), "I found Nemo at 2!");
}

