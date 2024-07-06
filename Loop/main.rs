fn main(){
    println!("Looping Mechanisms");

    // loop {
    //     println("Hello world");
    // }
    let mut x=0;
    loop{
        x=x+1;
        if x==10{
            break x;
        }
    };
    println!("The value of x after loop is {} ",x);


    //loops label
    let mut z=0;
    while  z!=10 {
        println!("z {z}"); 
        z=z+1;
    }
       
}