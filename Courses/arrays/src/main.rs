fn main() {
    let nums = [1,2,3,4];
    let nums2:[i32;4] = [1,2,3,4];
    /*
    in num2 we specified the type and the length of the array
    syntax : let arrayName:[datatype,length] 
    */
    for n in nums.iter() {
    	println!("{}",nums[n] );
    }
    // length
    let len = nums.len();

    //array with repeated elements
    let two = [2;100]; // an arrays of 100 items with the value 
}
