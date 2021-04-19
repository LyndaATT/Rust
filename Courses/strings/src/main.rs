fn main() {
    let mut my_str = String::from("Wingardium Leviosa");

    //length
    println!("Length {}",my_str.len() );

    //Is empty?
    println!("String is empty?{}",my_str.is_empty());

    //Split string by whitespace
    //THIS RETURNS : Wingardium
    //               Leviosa
    for token in my_str.split_whitespace(){
    	println!("{}",token);
    }

    //testing whether the string contains a certain token
    println!("Does the string contains 'magic'?{}",my_str.contains("magic") );

    //Pushing a token to the string 
    // but to do that we should have a mutable var!
    my_str.push_str(" !!");
    println!("The string is now : {}",my_str );

    // these are the basic methods, there are others!
}