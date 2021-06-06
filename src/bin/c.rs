use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        m: usize,
        ps: [(usize, String);m]
    };

    let mut bucket = vec![(false, 0i64);n+1];

    for (p, s) in ps.iter(){
        if s == "WA" && !bucket[*p].0{
            bucket[*p].1 += 1;
        }
        else {
            bucket[*p].0 = true;
        }
    }

    let mut num_wa = 0i64;
    let mut num_ac = 0i64;

    for &(ac, wa) in bucket.iter(){
        if ac {
            num_ac += 1;
            num_wa += wa;
        }
    }

    println!("{} {}", num_ac, num_wa)
}
