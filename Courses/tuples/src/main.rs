fn main() {
    let tup1 = (1,2,0,4,0);
    // we will have 0
    println!("{}", tup1.2);

    let tup2 = (3,"Hey",2.2,true);
    println!("{}", tup2.3);

    let tup3 = (1,3,tup1);
    //we will have 4
    println!("{}",(tup3.2).3);
    // (tup3.2) --> access to tup1
    // (tup3.2).3 --> access to the 4th elem of tup1,ie 4

    //separating a tuple into variables
    let tup4 = ("bim","bam",true);
    let (a,b,c) = tup4;
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);

}
