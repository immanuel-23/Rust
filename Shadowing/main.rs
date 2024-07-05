fn main (){
    println!("Shadowning in rust");
    let x=5;
    println!("The value of x before shadowing is {} ",x);
    let x=x+1;// shadowing is done here 
    // use of shadowing is u don't need to create a new variable always with different names u can even change the type of it 
    println!("The value of x after  shadowing is {} ",x);

    let x = "      ";

    println!("Changing the type of x is  {} ",x);

    let x =x.len();
    println!("Chanded the type of x {} from spaces to int",x);
}