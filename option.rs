// RUN: rm -rf log out tmp save Output
// RUN: rustc option.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis option.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' option.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' option.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' option.ll
// RUN: llvm-as option.ll
// RUN: llvm-link lib-stable2.bc option.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=10 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --nec=1 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs" 
// RUN: /home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=10 -seed=0 2>&1 | FileCheck %s
// CHECK: back
// CHECK: through
// CHECK: unless
// CHECK: turn to end


#![no_main]

use std::fs::File;
use std::io::prelude::*;

#[no_mangle]
pub fn miner_top(data: *const u8, mut size: usize) -> i32 {
   let mut file =File::create("one.txt").expect("create failed");
    // fuzzed code goes here
   let points_at =unsafe {*data};
    let s=points_at as char ;
 /*    if let Ok(sun)= std::str::from_utf8(data){
         let a=sun.to_string();
     let len = a.len();
     if len==0{
      return
      }
     let s=data[0] as char;*/
     if '9' < s && s < 'a'{
     println!("through");
   file.write(b"through").expect("write failed");
    }  else if '0'< s && s <'5' {
      println!("back");
  file.write(b"back").expect("write failed");
   }  else if 'a'< s && s < 'f' {
      println!("unless");
     file.write(b"unless").expect("write failed");
   }  else {
     println!("turn to end");
    file.write(b"turn to end").expect("write failed");
   };
  0
 }
   /* let u=convert(&data);
      pub fn convert(data: &[u8]){
    let ptr :*const u8 = data.as_ptr();
    let ptr :*const char = ptr as *const char;
     println!("{:?}",ptr);
   if let s = unsafe{ *ptr} {
      println!("==========")
     if '9' < s && s < 'a'{
      println!("cjrjeij");
    }  else if '0'< s && s <'5' {
      println!("b");
   }  else if 'a'< s && s < 'f' {
      println!("c");
   }  else {
     println!("d");
   }
    }
   }
  */
