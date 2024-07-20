fn main() {
    fn congruence(mut a: i32, mut c: i32, m: i32) -> i32 {
        a = a % m;
        c = c % m;
        // let u = 0;
        // let v = 0;
        let (d, u, _v) = extendedeuclidean(a, m);
        if c % d == 0 {
            let x0 = (u * (c / d)) % m;
            if x0 < 0 {
                return x0 + m;
            } else {
                return x0;
            }
        } else {
            return -1;
        }
    }

    fn extendedeuclidean(mut a: i32, mut b: i32) -> (i32, i32, i32) {
        // The extended Euclidean algorithm will be used to find the gcd of a and m
        // let mut q = 0;
        let mut r = 1;
        let mut x1 = 1;
        let mut x2 = 0;
        // let mut x3 = 1;
        let mut y1 = 0;
        let mut y2 = 1;
        // let mut y3 = 0;

        while r != 0 && b > 0{
            let q = a / b;
            r = a % b;
            let x3 = x1 - q * x2;
            let y3 = y1 - q * y2;

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

    let x = congruence(15, 9, 57);
    println!("{}", x);
}
