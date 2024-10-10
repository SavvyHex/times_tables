use std::env;

fn main() {
    let args:Vec<String> = env::args().collect(); // type annotations required
    let n:i32 = args[1].trim().parse().unwrap();  // args[0] is the binary name
    for i in 1..11 {                         // end of range exclusive
        println!("{}\tx\t{}\t=\t{}",n,i,(n*i));
    }
}
