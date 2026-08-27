// copywrite Marat Purnyn 2026
fn is_veg(skewer: &str) -> bool {
    for c in skewer.chars() {
        if c == 'x' {
            return false;
        }
    }
    
    true
}

fn count_skewers(skewers: &str) -> (i32, i32) {
    let mut veg_count = 0;
    let mut non_veg_count = 0;

    for skewer in skewers.split(',') {
        if is_veg(skewer) {
            veg_count += 1;
        } else {
            non_veg_count += 1;
        }
    }

    (veg_count, non_veg_count)
}

fn main() {
    let test_grill1 = [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];
    let test_grill2 = [
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"
    ];

    assert_eq!(count_skewers(&test_grill1.join(",")), (2, 3));
    assert_eq!(count_skewers(&test_grill2.join(",")), (3, 2));
}
