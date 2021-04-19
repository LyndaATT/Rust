//imports
use std::fs::File; //file struct
use std::io::prelude::* ; // help reading and writing ops


fn main() {
	// instance of file, related to our file, add exception text if not possible to read it
	let mut file = File::open("test.txt").expect("Can't open file!");

	// reading 
	let mut contents = String::new(); //empty string
	// we'll dump everything indise the file into a string (here contents)
	file.read_to_string(&mut contents)
			.expect("Can't open file!");

  //  println!("File contents : \n\n {}",contents);

    // writing in file
    // creating file : 
    let mut f = File::create("Output.txt")
    	.expect("Couldn't create file!");
    //writing
    file.write_all(b"A3yiiiiiiiiiiiigh")
    	.expect("Can't write on file!");
}
