fn main(){ 
    println!("Owner Ship in Rust ");

    //1) each value has one owner example
    //2) Only one owner at a time

    let mut s1=String::from("Rust");
    //borrowed from s1 so now the owner is s2 
    let s2=s1;
    // s2 will print rust where as s1 will have compilation error because s1 dont have the ownership 
    println!("Tets {}",s2 );
    // now taking back the owner ship from s2 again
    s1=s2;
    println!("Tets {}",s1 );
    // println!("The length of string {} is {} ",s1,len);

    // now borrwoing

    let s3=&s1; // here we are not actually taking the owner ship  just reference it 
// & s1 here is reference 
    println!("Test s3 {} ",s3);
    println!("Test s1 {} ",s1);// owner ship still is with  s1


    // 3) when owner is out of scope value wil  be droped

    let s4=String::from("Rust Owner Ship");

    let len=calculate_length(&s4);

    println!("S4 lenght is {}",len);
    
    
    printLost();
    printLostTest(&s4);

    println!("Owner Ship {}" ,&s4)

}
fn calculate_length(s:&String) ->usize{
    s.len()
}

fn printLost(){
  //  println!("Lost value {} ",&s4);
}
fn printLostTest(s :&String){
    println!("Lost value Test {} ",s);
}
