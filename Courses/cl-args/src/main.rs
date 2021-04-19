//lecture des outputs terminal
//imports
use std::env;
fn main() {
	// we store the command-line args collected in a vect of strings
    let args:Vec<String> = env::args().collect();

    for arg in args.iter(){
    	println!("{}",arg );
    }
    // the 1st arg will print the name of the executable file
}
