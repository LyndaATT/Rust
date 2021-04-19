// classic struct 
struct Color {
	red: u8, // u8 : 0-255
	green:u8,
	blue:u8
}

// tuple struct
struct Color2(u8,u8,u8);
fn main() {
	let bg = Color{red:230,green:10,blue:33};
    println!("{},{},{}",bg.red,bg.green,bg.blue);
    //the line below will generate an error
   // bg.blue = 9;
    // to change the value of the fields red,blue,green, we have
    // to declare the var as a mutable color
    let mut br = Color{red:1,green:30,blue:200};
    br.blue = 255;
    ///////////////
    let red = Color2(255,0,0);
    println!("{},{},{}",red.0,red.1,red.2); 
    // same as in classic struct, to change the value
    // of a var we should declare it as a mutable one
    print_color(&bg); //the var is passed by reference
}

//Pass by Reference
fn print_color(c:&Color){
	println!("Color : R={},G={},B={}",c.red,c.green,c.blue );
}
