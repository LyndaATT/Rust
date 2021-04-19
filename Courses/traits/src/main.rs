// a trait in the other languages as Java represents an interface

struct Person{
	name : String ,
	age : u8
}

// trait for having the to string funct
impl ToString for Person{
	fn to_string(&self) -> String{
		return format!("My name's {} and I'm {} .",self.name,self.age);
	}
}
fn main() {
	let lyn = Person{name:String::from("Lyn"),age:21};
	println!("{}",lyn.to_string());
}
