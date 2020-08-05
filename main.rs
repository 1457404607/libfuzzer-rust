// RUN: rm -rf log out tmp save Output
// RUN: rustc main.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis main.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' main.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' main.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' main.ll
// RUN: llvm-as main.ll
// RUN: llvm-link lib-stable2.bc main.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=0 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --nec=0 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs"
// RUN: /home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=3 -seed=0 2>&1 | FileCheck %s
// CHECK: hello world
// CHECK: pass successful 


#![no_main]

use std::fs::File;
use std::io::prelude::*;
 //  let mut file =File::create("monday.txt").expect("create failed");
#[no_mangle]
pub fn miner_top(data: *const u8, size: usize) -> i32 { 
  let mut file =File::create("monday.txt").expect("create failed");   
   let x = size;
   let mut y: usize =0;
    if x == 5 {
       y=3;
     file.write(b"hello world").expect("write failed");
    println!("hello world");
    } else {
       y=7;
     file.write(b"pass successful").expect("Write failed");  
   println!("pass successful");
   };
// println!("{}",y);

   0
}
/*
fn main() {
    let u =0;
    let a=0u8;
    let c=&a;
  miner_top(c, u);
 }*/

