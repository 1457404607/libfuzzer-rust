// RUN: rm -rf log out tmp save Output
// RUN: rustc nine.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis nine.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' nine.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' nine.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' nine.ll
// RUN: llvm-as nine.ll
// RUN: llvm-link lib-stable2.bc nine.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=20 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs" --nec=1 
// RUN:  /home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=10 -seed=0 2>&1 | FileCheck %s
// CHECK: one
// CHECK: two
// CHECK: three
// CHECK: four

//cxxflags="--disable-verify -output-stats=0 --search=dfs"

#![no_main]

use std::fs::File;
use std::io::prelude::*;

#[no_mangle]
pub fn miner_top(data: *const u8, size: usize) -> i32 {
   let mut file =File::create("thursday.txt").expect("create failed");
    // fuzzed code goes here
    let point_at =unsafe{*data} as char; 
 /* if let Ok(s)=std::str::from_utf8(data){
      let len=s.len();
   if len ==0{
         return
      }
     let size = data[0]; 
    if len<=data[0] as usize{
       return 
     }*/
     let mut y =0;
   let mut i:usize =0;
   for i in 0..size {
      y+=1;
     // println!("{}",s[i]);
     // println!("{}",y);
      println!("one");
  // file.write(b"one").expect("write failed");
   if y>250 {
       println!("two");
    // file.write(b"two").expect("write failed");
      if point_at>'a'{
      println!("three");
    // file.write(b"three").expect("write failed");
      }else {
     println!("four");
   // file.write(b"four").expect("write failed");
       }
     }
   };
  0
 }
