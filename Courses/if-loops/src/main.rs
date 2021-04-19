fn main() {
	let mut n = 0;


	/* if statement */
	//same as in c
/*	if n<10 {
		println!("It's smaller than 10");
	}
	else if n==10 {
		println!("It's equal to 10");
	}
	else {
		println!("It's greater than 10");
	}
*/


	/*Infinite loop with the keyword loop*/
	// alows to execute a bunch of code infinitely we specify where 
	// to stop it

/*	loop{
		n+=1; // we can't use n++!!
		if n==3 {
			continue; // it means skip the iteration when n==3, we don't execute the below lines for n=3
		}
		//to specify the stop
		if n>10 {
			break;
		}
		println!("The value of n is {}",n ); 
	}
*/


  /*while loop*/
  // same as in C
/*  while n <10 {
  	println!("n = {}",n);
  	n+=1;
  } 
  */

  /*for loop*/
  	// the for loop uses an iterator! 
  	let nums = 1..10; // 1..10 mean a range from 1 to 9
  	let animals = vec!["Dog","Fish","Panda"];
	for i in nums { 
		println!("The num is {}", i);
	}
	// when we use vectors, we have to call the iter() method to prevent the ownership of the values inside the vector
	for a in animals.iter(){
		println!("The animal is {}",a );
	}

	for (index,a) in animals.iter().enumerate(){
		println!("The {} is in the index {}",a,index);
	}

}
