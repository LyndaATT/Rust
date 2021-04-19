struct  Rectangle {
	width : u32,
	height : u32
}
// impl specify any function we want to define for a struct
impl Rectangle {
	fn print_description(&self){ // it's like a method of class
		//through self wa access to the struct fields
		println!("Rectangle : {}x{}", self.width,self.height );
	}

	fn is_quare(&self)->bool {
		return self.width==self.height;
	}
}
fn main() {
    let my_rect = Rectangle{width:5,height:2};
    my_rect.print_description();
    println!("Rectangle is a square ?{}",my_rect.is_quare());

}
