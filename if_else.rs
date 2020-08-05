// RUN: rm -rf log out tmp save Output
// RUN: rustc if_else.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis if_else.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' if_else.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' if_else.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' if_else.ll
// RUN: llvm-as if_else.ll
// RUN: llvm-link lib-stable2.bc if_else.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=0 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --nec=0 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs" 
// RUN: /home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=10 -seed=0 2>&1 | FileCheck %s
// CHECK: three
// CHECK: isnot dc


#![no_main]

use std::fs::File;
use std::io::prelude::*;
 //  let mut file =File::create("monday.txt").expect("create failed");
#[no_mangle]
pub fn miner_top(data: *const u8, size: usize) -> i32 {
    // fuzzed code goes here
 /*  if let Ok(s)= std::str::from_utf8(data){
     let bytes=s.as_bytes();
     let len= bytes.len();
     if len == 0 {
     return
   }*/
   let a=unsafe{*data};
  let some_u8_value = Some(a);
     match some_u8_value {
      Some(3) => println!("three"),
      _ => println!("isnot dc"),
      };
    0
    }

