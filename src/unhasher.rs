use std::env;

// range for the valid characters, do mind that we cast char as u8 anyway
const MIN_CHAR: u8 = 32;
const MAX_CHAR: u8 = 128;

const MAX_SIZE: u64 = 0xFFFF_FFFF_FFFF;
const OVERFLOW_CANCEL: u64 = 2 * MAX_SIZE + 2;


fn calculate_overflow_frequency(size: u8) -> u64 {
    let mut highest_hash: u64 = 0;
    let mut overflow_count: u64 = 0;

    for _ in 0..size {
        highest_hash = (highest_hash.wrapping_mul(47)) + MAX_CHAR as u64;
    }

    while highest_hash > MAX_SIZE {
        highest_hash -= MAX_SIZE;
        overflow_count = overflow_count.wrapping_add(1);
    }

    return overflow_count;
}

#[test]
fn test_vec_from_string() {
    assert_eq!(from_string_to_vec("test"), vec![116, 101, 115, 116])
}

#[allow(dead_code)]
fn from_string_to_vec(input: &str) -> Vec<u8> {
    String::from(input).into_bytes()
}

#[test]
fn test_string() {
    assert_eq!(print_string(&from_string_to_vec("test"), 2), "te")
}

fn print_string(input: &Vec<u8>, size: u8) -> String {
    let mut output: String = String::new();
    for i in 0u8..size {
        output.push(*input.get(i as usize).unwrap() as char);
    }
    output
}

fn recursive_search(max_depth: u8, hash: u64, current_depth: u8, current_hash: u64, current_string: &mut Vec<u8>) -> () {
    if current_depth == max_depth {
        let last_char: u8 = (hash.wrapping_sub(current_hash)) as u8;
        if last_char < MIN_CHAR || last_char > MAX_CHAR { return; }
        current_string[current_depth as usize] = last_char as u8;
        println!("{}", print_string(current_string, max_depth + 1));
        return;
    }
    let current_exponent = max_depth - current_depth;
    if current_exponent > 30 {
        return;
    }
    let current_multiplier: u64 = 47u128.pow(current_exponent as u32) as u64;

    let mut min_addend = 0u64;
    let mut max_addend = 0u64;
    for i in (0..current_exponent).rev() {
        let multiplier: u64 = 47u128.pow(i as u32) as u64;
        min_addend = min_addend.wrapping_add((MIN_CHAR as u64).wrapping_mul(multiplier));
        max_addend = max_addend.wrapping_add((MAX_CHAR as u64).wrapping_mul(multiplier));
    }

    let current_min_char: u8 = ((hash.wrapping_sub(current_hash).wrapping_sub(max_addend)) / current_multiplier as u64) as u8;
    let current_max_char: u8 = ((hash.wrapping_sub(current_hash).wrapping_sub(min_addend)) / current_multiplier as u64) as u8;

    if current_min_char < MIN_CHAR || current_max_char > MAX_CHAR {
        return;
    }
    for i in current_min_char..=current_max_char {
        current_string[current_depth as usize] = i;
        recursive_search(max_depth, hash, current_depth + 1, current_hash + (i as u64) * (current_multiplier as u64), current_string);
    }
}

fn lazy_unhasher(hash: u64, min_size: u8, max_size: u8) -> String {
    assert!(min_size > 0, "size should be at least 1");
    for size in min_size..=max_size {
        let overflow_freq: u64 = calculate_overflow_frequency(size);
        for index in 0u64..=overflow_freq {
            let mut collector: Vec<u8> = vec![0; size as usize];
            recursive_search(size - 1, hash + OVERFLOW_CANCEL.wrapping_mul(index as u64), 0u8, 0u64, &mut collector)
        }
    }
    String::new()
}

#[test]
fn test_reversing() {
    let hash: u64 = 111111111111111111;//lazy_hash("XXXXX".parse().unwrap());
    println!("{}", hash);
    assert!(hash < (1 << 48), "The hash is not in the correct range");
    //lazy_unhasher(hash, 11, 13);
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

// ./unhasher XXXX | grep "^Fg{.*}$" | head -n 1
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You should give a number as argument");
        return;
    }
    let mut flag: String = args[1].clone();
    trim_newline(&mut flag);
    println!("{}", flag);
    let hash: u64 = flag.parse::<u64>().expect("Not a number");
    lazy_unhasher(hash, 11, 32);
}