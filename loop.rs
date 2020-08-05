// RUN: rm -rf log out tmp save Output
// RUN: rustc loop.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis loop.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' loop.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' loop.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' loop.ll
// RUN: llvm-as loop.ll
// RUN: llvm-link lib-stable2.bc loop.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=20 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs" --nec=1
// RUN: /home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=5 -seed=0 2>&1 | FileCheck %s
// CHECK: here is right
// CHECK: none


#![no_main]

use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;
//#[no_sanitize(address)]
#[no_mangle]
pub fn miner_top(data: *mut u8, size: usize) -> i32 {
   //let mut file =File::create("friday.txt").expect("create failed");
    // fuzzed code goes here
   //data loop
//  if let Ok(s)= String::from_utf8(data.to_vec()){
  //let vec = data.to_vec();
    let mut s =String::new();
    //  let mut a=unsafe{*data};
 //   let i:isize=0;
     for i in 0..size {
  //     s.push(unsafe{*data} as char);
  // let ptr: *const u8 = s.as_ptr();

unsafe {
    s.push(*data.offset(i.try_into().unwrap()) as char);
  //  println!("{}", *ptr.offset(1) as char);
  //  println!("{}", *ptr.offset(2) as char);
}
     //    data;//dereferencing raw pointer 
//   println!("{:?}",s);
     }//data string 
  let bytes=s.as_bytes(); 
  for i in 0..size {
     if bytes[i]== 2 {
     println!("{}",i);
    println!("here is right")
   //  println!("{:?}",&s[0..i]);
  } else {
    println!("none");
   }
 }
  0
 }
// let mut points_at= unsafe{*data};
  // let b=points_at as char;
/*  let mut s=String::new();
    let mut i:usize=0; 
  for i in 0..size {
      s.push(b);
    a+=1;
   if a>64{
     break;
   }
 }
//   let mut len=s.len();
   if size<34 {
      return 0;
   }
    loop {
     size-=1;
   println!("{:?}",&s[0..7]);
   if size==4{
    println!("check");
   file.write(b"check").expect("write failed");
     break;
       }
    };*/
 
