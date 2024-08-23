fn main() {
    println!("Hello, world!");
}

fn fermatsLittle( a: u64, p: u64) {
    if a % p == 0 {
        let mut powVal = u64::pow(a, p);
        if (powVal - p) % p == 0 {
            println!("Fermat's Little theorem Holds true!!");
        } else {
            println!("Fermat's little theorem holds false!!");
        }
    } else {
        
    }
}

// int fermatLittle(int a, int p) {
//    
//    int powVal;
//    if(a % p == 0){
//      
//       powVal = pow(a, p);
//       if((powVal - p) % p == 0){
//          cout<<"Fermat's little theorem holds true!";
//       }
//       else{
//          cout<<"Fermat's little theorem holds false!";
//       }
//    }  
//    else {
//       powVal = pow(a, (p - 1));
//       if((powVal - 1) % p == 0 ){
//        cout<<"Fermat's little theorem holds true!";
//       }
//       else{
//          cout<<"Fermat's little theorem holds false!";
//       }
//    }
//      
// }

// int main()
// {
//    int a = 3, m = 11;
//    fermatLittle(a, m);
//    return 0;
// }