fn main() {
    // let num = 600851475143;
    let num = 36;
    println!("The prime factors of {} are {:?}", num, prime_factors(num));
    // prime_factors(num);
}

fn prime_factors(mut num: i64) -> Vec<i64> {
    let mut p = 2;
    let mut factors: Vec<i64> = Vec::new();
    
    if num == 0 {
        println!("This number is less than one!");
    }
    // 12>=4
    //12%2 

    while num >= p * p {
        if num % p == 0 {
            factors.push(p);
            num = num / p;
        } else {
            p = p + 1;
        }
    }
    factors.push(num);

    return factors;
}
