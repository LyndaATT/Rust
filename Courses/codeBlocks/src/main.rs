fn main() {
	let x = 10;
	{
		//this is an codeBlock
		//it's isolated 
		// It has access to data stored outside the actual block
		// But the outside doesn't have access to what we have in the block
		let y = 6;
		//will print 10,6
		println!("{},{}",x,y );
	}
	//will generate an error
	//println!("{},{}",x,y );

	/*SHADOWING*/
	let mut y = 10;
	{
		y = 30;
	}
	// will print y=30
	println!("y = {}",y );
	
	let mut z = 10;
	{
		let z = 30; //this is a shadowing
	}
	// will print z=10
	println!("z = {}",z );

	//shadowing data types : 
	let z = "Z is a string";
	println!("z : {}", z );	
	let z = true;
	println!("z : {}", z );

   
}
