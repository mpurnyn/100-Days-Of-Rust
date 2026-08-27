fn count_pairs(socks: &str) -> i8 {
    let mut count = 0;
    let mut pairs: [bool; 26] = [false; 26];

    // use the 26 fixed length array trick based on the ASCII values of the characters to store the pairs
    // if you've flipped one to true that's one sock
    // if you flip it again that's a pair and you can increment the count and flip it back to false
    // time complexity is O(n) and space complexity is O(1) since the array is fixed length
    for c in socks.chars() {
        let i = ('Z' as usize) - (c as usize);
        if pairs[i] == true {
            count = count + 1;
            pairs[i] = false;
        } else{
            pairs[i] = true;
        }
    }
    println!("pairs: {:?}", pairs);
    println!("count: {:?}", count);
    count
}

fn main() {
    assert_eq!(count_pairs("AA"),1);
    assert_eq!(count_pairs("ABABC"),2);
    assert_eq!(count_pairs("CABBACCC"),4);
}
