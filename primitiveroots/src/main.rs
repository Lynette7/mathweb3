fn gcd (num1: u64, num2: u64) -> u64 {
    if num1 == 0 {
        return num2;
    } else if num2 == 0 {
        return num1;
        //a = b.x + r
    } else if num1 > num2 {
        let mut a = num1;
        let mut b = num2;

        while a % b != 0 {
            let temp = a;
            a = b;
            b = temp % a;
        }
        return b;
    } else {
        let mut b = num1;
        let mut a = num2;

        while a % b != 0 {
            let temp = a;
            a = b;
            b = temp % a;
        }
        return b;
    }
}

fn relatively_prime (n: u64) -> Vec<u64> {
    let mut a: u64 = 2;
    let mut coprimes=Vec::new();
    if a < n{
        let mut result = gcd(a,n);
        if result==1{
            coprimes.push(result);
        }
        a+=1;
    }
    return coprimes;
}

fn find_powers (n: u64) -> Vec<u64> {
    let phi = n-1;
    let mut powers=Vec::new();
    for x in range 1..=phi{
        if phi % x == 0{
            powers.push(x);
        }   
    }
    return powers;
}

fn find_primitive(comprimes:Vec<u64>,powers:Vec<u64>) -> HashMap{
    let phi = n-1;
    let primitives=HashMap::new();
    for i in coprimes{
        for j in powers{
            let mut remainder= pow(i,j);
            if remainder == 1 && j==phi{
                primitives.insert(i,j);
            }    
        }
    }
    return primitives;


}


// fn gcd(a: i64, b: i64) -> i64 {
//     if b == 0 {
//         a
//     } else {
//         gcd(b, a % b)
//     }
// }

// fn phi(n: i64) -> i64 {
//     (1..n).filter(|&i| gcd(i, n) == 1).count() as i64
// }

// fn is_primitive_root(g: i64, n: i64) -> bool {
//     if g < 2 || g >= n {
//         return false;
//     }

//     let factors = {
//         let mut factors = vec![];
//         let mut phi_n = phi(n);

//         for i in 2..=(phi_n as f64).sqrt() as i64 {
//             if phi_n % i == 0 {
//                 factors.push(i);
//                 while phi_n % i == 0 {
//                     phi_n /= i;
//                 }
//             }
//         }

//         if phi_n > 1 {
//             factors.push(phi_n);
//         }

//         factors
//     };

//     for &f in &factors {
//         if g.pow((phi(n) / f) as u32) % n == 1 {
//             return false;
//         }
//     }

//     true
// }

// fn find_primitive_roots(n: i64) -> Vec<i64> {
//     (2..n).filter(|&g| is_primitive_root(g, n)).collect()
// }

// fn main() {
//     let n = 20;
//     let primitive_roots = find_primitive_roots(n);
//     println!("Primitive roots modulo {}: {:?}", n, primitive_roots);
// }