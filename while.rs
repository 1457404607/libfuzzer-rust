// RUN: rm -rf log out tmp save Output
// RUN: rustc while.rs --emit=llvm-bc --crate-type=lib
// RUN: llvm-dis while.bc
// RUN: sed -i 's/__rust_alloc/__rdl_alloc/g' while.ll
// RUN: sed -i 's/__rust_realloc/__rdl_realloc/g' while.ll
// RUN: sed -i 's/__rust_dealloc/__rdl_dealloc/g' while.ll
// RUN: llvm-as while.ll
// RUN: llvm-link lib-stable2.bc while.bc -o test.bc 
// RUN: miner --corp=in --output-dir=out --fcore-timeout=10 --dut=test.bc --checker=ALL --debug --refresh --lf-max-input-len=3000 --ctcore-limit=20 --ctcore-timeout=10 --ctcore-arguments="--disable-verify -output-stats=0 --search=dfs" --nec=1 
// RUN: /home/nssbsw/miner_rust/tmp/out_ctf /home/nssbsw/miner_rust/out  -max_len=3000 -jobs=0 -max_total_time=10 -seed=0 2>&1 | FileCheck %s
// CHECK: Greater than 9, quit!
// CHECK: Try again

//cxxflags="--disable-verify -output-stats=0 --search=dfs"

#![no_main]

use std::fs::File;
use std::io::prelude::*;

#[no_mangle]
pub fn miner_top(data: *const u8, size: usize) -> i32 {
   let mut file =File::create("two.txt").expect("create failed");
    // fuzzed code goes here
   let a=unsafe{*data};
/* if let Ok(s)= std::str::from_utf8(data){
     let bytes=s.as_bytes();
     let len= bytes.len();
     if len == 0 {
     return
     }*/
  let mut optional = Some(a);
    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(a) = optional {
        if a > 9 {
            println!("Greater than 9, quit!");
        file.write(b"Greater than 9, quit!").expect("write failed");
            optional = None;
        } else {
            println!("`a` is `{:?}`. Try again", a);
        file.write(b" Try again").expect("write failed");
            optional = Some(a + 1);
        }  
     };
   0
  }




/*
    let val=data[0] as u8 as i32;
pub struct ListNode {
    pub val,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
    let mut head=head::Option<Box<ListNode>>;
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    loop {
        if let Some(node) = fast {
            fast = node.next.as_ref();
           println!("nodefirst");
        } else {
            break;
        }
        if let Some(node) = fast {
            fast = node.next.as_ref();
            println!("nodesecond");
        } else {
            break;
        }
        if let Some(node) = slow {
            slow = node.next.as_ref();
           println!("nodethird");
        } else {
            break;
        }
    }

    let mid_addr = if let Some(node) = slow {
        node.as_ref() as *const ListNode
       
    } else {
        return ;
    };

    while let Some(node) = head.as_ref() {
        let addr = node.as_ref() as *const ListNode;
        if addr != mid_addr {
            head = head.unwrap().next;
           println!("nodefourth");
        } else {
            break;
        }
    }
}*/

