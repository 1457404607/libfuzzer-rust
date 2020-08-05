// RUN: rm -rf log out tmp save Output
// RUN: rustc overflow.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis overflow.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' overflow.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' overflow.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' overflow.ll
// RUN: llvm-as overflow.ll
// RUN: llvm-link lib-stable2.bc overflow.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=0 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --nec=1 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs" 
// RUN:/home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=10 -seed=0 2>&1 | FileCheck %s
// CHECK: Change the color to hue 



#![no_main]

use std::fs::File;
use std::io::prelude::*;
 //  let mut file =File::create("monday.txt").expect("create failed");
#[no_mangle]
pub fn miner_top(data: *const u8, size: usize) -> i32 {
  let mut file =File::create("monday.txt").expect("create failed");
    // fuzzed code goes here
//data u8 tuple 
     //address sanitizer
 //  let s=unsafe {*data};
   unsafe {
    let a=*data.offset(1);
    let b=*data.offset(2);
    let c=*data.offset(3);
  //  println!("{}", *ptr.offset(1) as char);
  //  println!("{}", *ptr.offset(2) as char);


enum Color {
    Rgb(i32, i32, i32),
    Hsv(u8, u8, u8),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
    let msg = Message::ChangeColor(Color::Hsv(a, b, c));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
 };
  0
}
  /* let mut v:Vec<u8> =vec![0;10];
   if let Ok(s)= std::str::from_utf8(data){
      let a=s.to_string();
      let bytes=a.as_bytes();
      let len=bytes.len();
    if len<10{
  return 
    }
   //let ptr = s.as_ptr();
   for i in 0..10 {
       v[i] = bytes[i]; 
     println!("100%");

      }
  
   }*/
