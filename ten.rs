// RUN: rm -rf log out tmp save Output
// RUN: rustc ten.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis ten.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' ten.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' ten.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' ten.ll
// RUN: llvm-as ten.ll
// RUN: llvm-link lib-stable2.bc ten.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=0 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --nec=0 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs"
// RUN: /home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=10 -seed=0 2>&1 | FileCheck %s
// CHECK: one
// CHECK: two
// CHECK: three
// CHECK: four
// CHECK: five
// CHECK: six
// CHECK: seven
// CHECK: eight
// CHECK: nine
// CHECK: ten




#![no_main]

use std::fs::File;
use std::io::prelude::*; 

#[no_mangle]
pub fn miner_top(data:*const u8, size :usize) ->i32 {
   let points_at = unsafe { *data };
     let a = points_at as char; 
    let mut file =File::create("tuesday.txt").expect("create failed");
   if a >'0' && a < 'z'  {
        if a > '0' && a < '9'{
            if a > '6'{
             println!("one");
            file.write(b"one").expect("write failed");
                  
           } else if a < '6'{
               println!("two");
            file.write(b"two").expect("write failed");
                   
           } else {
               println!("three");
            file.write(b"three").expect("write failed");
                  
          }
       }
        if a > 'A' && a < 'Z'{
            if a > 'O'{
              println!("four");
           file.write(b"four").expect("write failed");
                   
            } else if a < 'O'{
              println!("five");
            file.write(b"five").expect("write failed");
                    
            } else {
             println!("six");
           file.write(b"six").expect("write failed");
                   
           }
       }
        if a > 'a' && a < 'z'{
            if a > 'o'{
               println!("seven");
           file.write(b"seven").expect("write failed");
                  
            } else if a < 'o'{
                println!("eight");
          file.write(b"eight").expect("write failed");
                   
            } else {
            println!("nine");
       file.write(b"nine").expect("write failed");
                
           }
        }
    }  else{
      println!("ten");
   file.write(b"ten").expect("write failed");
      
     };
  0
 }

