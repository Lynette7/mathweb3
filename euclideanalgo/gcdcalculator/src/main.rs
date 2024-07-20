fn main() {
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

    let a = 60;
    let b = 33;
    let result = gcd(a, b);
    println!("GCD of {a} and {b} is: {}", result);
}
