fn main() {
    println!("Hello, world!");
    use dreaps::math::{fact_f64, fact_u64, fact_u128};
    // if let Some(n) = fact_u128(34) {println!("{}", n)};
    // println!("{}", factorial(34));
    for i in 2..34 {
        let abs_fact: u128 = fact_u128(i).unwrap();
        let f64_fact: f64 = fact_f64(i);
        match abs_fact == f64_fact as u128 {
            true => println!("Equal at: {}", i),
            false => {
                println!(
                    "Not Equal at: {}\n\tf = {}\n\tu = {}",
                    i, f64_fact, abs_fact
                )
            }
        }
    }
}
