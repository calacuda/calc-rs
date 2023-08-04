// use calc_rs::{solve_equ, solve_func};
use calc_rs::Result;
use calc_rs::{solve_equ, solve_funcs};
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    // get funcs/equations
    // if func:
    //     println!("{}", solve_func(function, -100, 100)?);
    // else:
    //     println!("{}", solve_equ(equation)?);
    for line in io::stdin().lock().lines() {
        let l = line?;
        println!("{l} => {:?}", solve_funcs(vec![&l], 0, 25));
        // for i in 0..25 {
        //     let mut map = HashMap::new();
        //     map.insert("x".to_string(), i as f64);
        //     println!("f({i}) = {} = {}", &l, solve_funcs([&l], &map)?);
        // }
    }

    Ok(())
}
