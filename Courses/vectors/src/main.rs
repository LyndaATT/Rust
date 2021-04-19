fn main() {
	// declaration 1:
	// let name:Vec<itemsType>=Vec::new()
    let my_vec:Vec<i32> = Vec::new();
    //declaration 2 : 
    // let name = vec![items list]
    let mut my_vec2 = vec![9,2,3];

    //accessing elem in vec
    let a = my_vec2[2];

    //pushing
    my_vec2.push(8); //pushin(value), added at the end

    //removing
    my_vec2.remove(0); // remove(index)

    for n in my_vec2.iter(){
    	println!("{}",n );
    }


}
