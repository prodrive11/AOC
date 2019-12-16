extern crate chrono;
use chrono::prelude::*;
// use math::round;
use std::f64;


use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> std::io::Result<()> {
    let t0: DateTime<Utc> = Utc::now();
 
    let file = File::open("input/1")?;
    let reader = BufReader::new(file);
    let mut sum: u64 = 0;
    let mut a:f64 = 0.0;

    for line in reader.lines(){
      if let std::result::Result::Ok(ln) = line {
        //a=ln.trim().parse().expect("wanted a num num") as f64;
        a=ln.trim().parse::<u64>().expect("wanted a num num") as f64;
        sum+=((a/3.0).floor()-2.0)as u64;
        //sum+=round::floor(a/3,0);
      }
    }


    println!("Res:  {}", sum);
    let t1: DateTime<Utc> = Utc::now();
    println!("Duration: {}", t1-t0);

    Ok(())
}


/*
 *
 * Res:  3353880
 * Duration: PT0.000837100S
 * ]
