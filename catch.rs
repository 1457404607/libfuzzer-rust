// RUN: rm -rf log out tmp save Output
// RUN: rustc catch.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis catch.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' catch.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' catch.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' catch.ll
// RUN: llvm-as catch.ll
// RUN: llvm-link lib-stable2.bc catch.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=10 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs" --nec=1 
// RUN: /home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=10 -seed=0 2>&1 | FileCheck %s
// CHECK: exit
// CHECK: number
// CHECK: none
// CHECK: code

#![no_main]

use std::fs::File;
use std::io::prelude::*;

#[no_mangle]
pub fn miner_top(data: *const u8, size: usize) -> i32 {
   let mut file =File::create("wednesday.txt").expect("create failed");
    // fuzzed code goes here
   let points_at=unsafe{*data};
    let x=points_at as char;
/*if let Ok(s) = String::from_utf8(data.to_vec()){
    let a=s.to_string();
    let bytes=a.as_bytes(); 
    let len= bytes.len();
     if len==0{
       return 
        }
    let x=bytes[0] as char;*/
  match x {
      'a'..='z'=> println!("exit"),
      '0'..='9'=> println!("number"),
      'A'..='U'=> println!("code"),
       _ =>println!("none"),
      };
    0
   }
  

