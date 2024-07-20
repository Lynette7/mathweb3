fn main() {
    fn extendedeuclidean(mut a: i32, mut b: i32) -> (i32, i32, i32) {
        // q is the quotient and r is the remainder
        //The other variables are used to store and track the values of the previous iteration
        // a = b.q + r
        let mut q = 0;
        let mut r = 1;
        let mut x1 = 1;
        let mut x2 = 0;
        let mut x3 = 1;
        let mut y1 = 0;
        let mut y2 = 1;
        let mut y3 = 0;

        //add the condition b > 0 to handle the situation where b = 0
        //Otherwise, the program will return an error: thread 'main' panicked at src/main.rs: attempt to divide by zero
        while r != 0 && b > 0{
            q = a / b;
            r = a % b;
            x3 = x1 - q * x2;
            y3 = y1 - q * y2;

            if r != 0 {
                a = b;
                b = r;
                x1 = x2;
                x2 = x3;
                y1 = y2;
                y2 = y3;
            }
        }

        return(b, x2, y2)
    }

    let a = 60;
    let b = 33;
    let (g, x, y) = extendedeuclidean(a, b);
    println!("The extended euclidean algorithm of {a} and {b} is: gcd = {}, s = {}, t = {}", g, x, y)
}
