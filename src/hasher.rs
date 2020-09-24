use std::io;

#[test]
fn test_hashing() {
    let hash: u64 = lazy_hash("Made_with_l0ve_in_Netherlands!".parse().unwrap());
    assert!(hash < (1 << 48), "The hash is not in the correct range");
}

fn lazy_hash(input: String) -> u64 {
    let mut h: u64 = 0;
    for char in input.chars() {
        h = h.wrapping_mul(47).wrapping_add((char as u8) as u64);
        h = h % (1 << 48);
    }
    return h;
}

fn main(){
    println!("Enter the flag ");
    let mut flag = String::new();
    io::stdin().read_line(&mut flag).expect("Failed to read line");
    println!("{}", lazy_hash(flag))
}