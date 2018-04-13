#[macro_use]
extern crate rand;

use rand::Rng;

/// 1. Generate random numbers
fn gen_random() {
    // each thread has an automatically-initialised random number generator
    let mut rng = rand::thread_rng();
    let rn1: u8 = rng.gen();
    let rn2: u16 = rng.gen();
    println!("Random u8: {}", rn1);
    println!("Random u16: {}", rn2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random u64: {}", rng.gen::<u64>());
}

/// 2. Generate random numbers within a range
/// Generates a random value within [0,10)
fn gen_random_in_range() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));
    func()
}

/// 2.1. Also can use Range to obtain values with uniform distribution.
/// This func has the same effect, but may be faster when repeatedly generating
/// numbers in the same range
use rand::distributions::{Range, IndependentSample};

fn func() {
    let mut rng = rand::thread_rng();
    let die = Range::new(0.0, 10.0);
    loop {
        let throw = die.ind_sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6.0 {
            break;
        }
    }
}

/// 3. Generate random numbers with given distribution
/// By default, random numbers are generated with uniform distribution.
/// To generate numbers with other distribution you instantiate a distribution,
/// then sample from that distribution using IndependentSample::ind_sample
// use rand::distributions::IndependentSample;
use rand::distributions::Normal;

fn gen_random_with_distribution() {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0);
    let v = normal.ind_sample(&mut rng);
}

/// 4. Generate random values of a custom type
use rand::Rand;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Rand for Point {
    fn rand<R: Rng>(rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn rand_point() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point = rng.gen::<Point>();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random point: {:?}", rand_point);
}

/// Create random passwords from a set of alphanumeric characters
fn rand_passwd() {
    let mut rng = rand::thread_rng();
    let rand_string: String = rng.gen_ascii_chars().take(30).collect();
    println!("{}", rand_string);
}

fn main() {
    gen_random();
    gen_random_in_range();
    gen_random_with_distribution();
    rand_point();
    rand_passwd();
}
