fn main(){
	//declaration et initialisation
	let x = 45; // here the type of x is an integer-32,
	           // it's assumed in rust 
	println!("The value of x is {}",x);
    // x = 23, we get an error !!
	/*
	The variable in rust are immutable, like ocaml, none 
	affectation like this : x = 60 is permitted. 
	But : 
	to do an affectation we should add the keyword "mut"
	while declaring the variable, look at the example below
	*/

	let mut y = 2;
	println!("The value of y is {}",y);
	y = 33;
	println!("The value of is now {}",y);

	////////type////////
    //to specify the type of a variable, rust may assume it 
    //but to have a clearer code, just specify it like this : 
    let i :i64 = 21; //integer-64
    let u :u64 = 3; // unsigned integer-64 (none neg)
    let f : f32 = 9.2; // floar-32
    let b : bool = false;
}