use calc_rs::solve_funcs;
use calc_rs::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    for line in io::stdin().lock().lines() {
        let l = line?;
        println!("{l} => {:?}", solve_funcs(vec![&l], 0, 25));
    }

    Ok(())
}
