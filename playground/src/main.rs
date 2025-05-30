#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::{BufRead, BufReader};
use std::fs::File;

use core::time;
use std::thread::sleep;

use p3_mersenne_31::*;
use p3_monolith::*;
use rand::Rng;

use hw_monolith::*;

/// The Mersenne31 prime
const P: u32 = (1 << 31) - 1;

// State size
const STATE_SIZE: usize = 16;

const MATRIX_CIRC_MDS_16_MERSENNE31_MONOLITH: [u64; 16] = [
    61402, 17845, 26798, 59689, 12021, 40901, 41351, 27521, 56951, 12034, 53865, 43244, 7454,
    33823, 28750, 1108,
];

const RUNS: usize = 1_000_000;

fn check_mul() {
    let mut rng = rand::rng();
    let gen1: u32 = rng.random_range(0..P);
    let gen2: u32 = rng.random_range(0..P);
    println!("Generated: {gen1}");
    println!("Generated: {gen2}");
    
    let a: Mersenne31 = Mersenne31::new_checked(gen1).unwrap();
    let b: Mersenne31 = Mersenne31::new_checked(gen2).unwrap();
    let c = a * b;
    
    println!("{a} * {b} = {c}");
}

fn check_mvm() {
    let random_numbers: [u32; STATE_SIZE] = rand::random();
    let mut vec: [Mersenne31; STATE_SIZE] = Mersenne31::new_array(random_numbers);
    println!("Using MDS matrix with first row: {:x?}", MATRIX_CIRC_MDS_16_MERSENNE31_MONOLITH);
    println!("Generated random vec: {:x?}", vec);
    
    let mds = MonolithMdsMatrixMersenne31::<6>;
    let monolith: MonolithMersenne31<16, 6> = MonolithMersenne31::new(mds);
    monolith.concrete(&mut vec);
    
    println!("Vec after mul: {:x?}", vec);
}

fn check_monolith_round() {
    let some_input: [u32; STATE_SIZE] = [ 0x222c88bf, 0x5948b323, 0x5244cda7, 0x5ba9b516, 0x33982c58, 0x5ccd4124, 0x1c4e3fab, 0x583a607e, 0x3863f49c, 0x174fe8d3, 0x5f88cc61, 0x280ec0f2, 0x703f1d50, 0xed88d37, 0x5bc5f8f2, 0x2f19df17 ];
    let mut state: [Mersenne31; STATE_SIZE] = Mersenne31::new_array(some_input);
    println!("Using some vec: {:x?}", state);
    println!();

    let mds = MonolithMdsMatrixMersenne31::<6>;
    let monolith: MonolithMersenne31<16, 6> = MonolithMersenne31::new(mds);
    
    monolith.bars(&mut state);
    println!("Vec after bars: {:x?}", state); println!();
    p3_monolith::MonolithMersenne31::<16, 5>::bricks(&mut state);
    println!("Vec after bricks: {:x?}", state); println!();
    monolith.concrete(&mut state);

    println!("Vec after one round: {:x?}", state);
}

fn check_monolith_hash() {
    let some_input: [u32; STATE_SIZE] = [ 0x222c88bf, 0x5948b323, 0x5244cda7, 0x5ba9b516, 0x33982c58, 0x5ccd4124, 0x1c4e3fab, 0x583a607e, 0x3863f49c, 0x174fe8d3, 0x5f88cc61, 0x280ec0f2, 0x703f1d50, 0xed88d37, 0x5bc5f8f2, 0x2f19df17 ];
    let mut state: [Mersenne31; STATE_SIZE] = Mersenne31::new_array(some_input);
    println!("Using some vec: {:x?}", state);
    println!();

    let mds = MonolithMdsMatrixMersenne31::<6>;
    let monolith: MonolithMersenne31<16, 6> = MonolithMersenne31::new(mds);

    monolith.concrete(&mut state);
    println!("After pre round: {:x?}", state);
    println!();
    for rc in monolith.round_constants {
        monolith.bars(&mut state);
        MonolithMersenne31::<16, 6>::bricks(&mut state);
        monolith.concrete(&mut state);
        println!("After concrete: {:x?}", state);
        monolith.add_round_constants(&mut state, &rc);
        
        println!("After round: {:x?}", state);
        println!();
    }

    // monolith.permutation(&mut state);

    println!("Used round constants: {:x?}", monolith.round_constants);
    println!();

    println!("Monolith hash of vec: {:x?}", state);
    println!();
}

fn benchmark_monolith() {
    let mds = MonolithMdsMatrixMersenne31::<6>;
    let monolith: MonolithMersenne31<16, 6> = MonolithMersenne31::new(mds);

    use std::time::Instant;
    
    loop {
        let mut inputs: Vec<[Mersenne31; STATE_SIZE]> = vec![[Mersenne31::new_checked(0).unwrap(); STATE_SIZE]; RUNS];
        for i in 0..RUNS {
            let random_numbers: [u32; STATE_SIZE] = rand::random();
            inputs[i] = Mersenne31::new_array(random_numbers);
        }

        let now = Instant::now();
        for i in 0..RUNS {
            monolith.permutation(&mut inputs[i]);
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        println!("Throughput: {:.2} hash/sec", (RUNS as f64)/elapsed.as_secs_f64());
    }
}

fn check_one_input(smth: u32) {
    let mut some_input: [u32; STATE_SIZE] = [0; STATE_SIZE];
    some_input[0] = smth;
    let mut state: [Mersenne31; STATE_SIZE] = Mersenne31::new_array(some_input);
    // println!("Using some vec: {:x?}", state);
    // println!();
 
    let mds = MonolithMdsMatrixMersenne31::<6>;
    let monolith: MonolithMersenne31<16, 6> = MonolithMersenne31::new(mds);
 
    monolith.permutation(&mut state);

    // println!("Monolith hash of vec: {:x?}", state);
    println!("Monolith hash(0x{:x?}) = 0x{:x?}", smth, state[0]);
    // println!();
}

fn benchmark_and_check_hw_monolith() {
    use std::time::Instant;
    
    let mut mapped_monolith = HWMonolith::new();

    let mds = MonolithMdsMatrixMersenne31::<6>;
    let monolith: MonolithMersenne31<16, 6> = MonolithMersenne31::new(mds);

    let mut inputs: Vec<u32> = vec![0; RUNS];
    let mut outputs: Vec<u32> = vec![0; RUNS];
    for i in 0..RUNS {
        let rand_input: u32 = (rand::random::<u32>() << 1) >> 1; // Remove MSB to pass check when unwrapping Mersenne31 below.
        inputs[i] = rand_input;
    }

    let now = Instant::now();
    for i in 0..RUNS {
        outputs[i] = mapped_monolith.hash(inputs[i]);
    }
    let elapsed = now.elapsed();
    
    println!("Elapsed: {:.2?}", elapsed);
    println!("Throughput: {:.2} hash/sec", (RUNS as f64)/elapsed.as_secs_f64());
    println!("Checking results...");

    let mut misses = 0;
    for i in 0..RUNS {
        let mut some_input: [u32; STATE_SIZE] = [0; STATE_SIZE];
        some_input[0] = inputs[i];
        let mut state: [Mersenne31; STATE_SIZE] = Mersenne31::new_array(some_input);
        monolith.permutation(&mut state);

        let hw_out = Mersenne31::new_checked(outputs[i]).unwrap();

        if state[0] != hw_out {
            println!("For input {:x?}: (correct) {:x?} != {:x?} (computed)", inputs[i], state[0], outputs[i]);
            misses += 1;
        }
    }
    println!("Misses: {misses}.");

    unsafe { mapped_monolith.unmap(); }

    // Hardware Accelerator is automatically unmapped when HWMonolith struct is dropped.
}

fn main() {
    // Check M31 multiplier, thus modular reduction.
    // check_mul();

    // Check Matrix-Vector-Multiplication.
    // check_mvm();

    // Check one Monolith Hash round (without constans)
    // check_monolith_round();

    // Check one Monolith Hash round (with constants)
    // check_monolith_hash();

    // Benchmark one milion Monolith hashes
    // benchmark_monolith();

    // Check on simple input
    // check_one_input(0x267260e1);
    
    // Benchmakrs one milion Monolith hashes executed using accelerator and check its outputs afterwards.
    benchmark_and_check_hw_monolith();
}