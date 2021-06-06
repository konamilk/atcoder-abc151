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
        k: i64,
        m: i64,
        a: [i64; n-1]
    };

    let require = m * (n as i64) - a.iter().sum::<i64>();

    if require <= 0 {
        println!("0");
        return
    }

    if require > k{
        println!("-1");
        return
    }

    println!("{}", require)

}
