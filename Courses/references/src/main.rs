fn main() {
    // reference : another way to refer to a variable 
    let mut x = 10;
    let xr = &x; // & : reference, we use xr the same way we use x
    println!("x = {}", xr);
    let xrr = &x; 
    // we can have many refernces (immutable) to same var
    // but only One MUTABLE reference
  	/* error cz it's not a mutable reference, xr and xrr are immutable references
    xrr+=1;
    println!("increm xrr {}",xrr ); */

    let xx = &mut x; // here xx is a mutable reference
    *xx+=1; // we should put *
    println!("increm xrr {}",xx );

    // this line will generate an error : 
   // println!(" x {}", x);
    // it says that you can't define a mutable reference
    // to x and at the same time borrow it 
    // to solve that we need to wrap the 3 lines before 
    //modification of x by xx inside a code block

}