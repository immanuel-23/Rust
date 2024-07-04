fn main(){
    println!("Borrwoing and references");
    //immutable references
    let _x:i32=5;
    let _y: &i32=&_x;
   // _y=_y+1;
    println!("The value of _x is {}",_x );
    println!("The value of _y is {}",_y);

    //mutable references.

    let mut _a:i32=6;
    let _b:&mut i32=&mut _a;
    *_b+=1;
    println!("The value of a is {}",_a);
    //println!("The value of b is {}",_b);
    
   // println!("The value of b is {}",_s);

    

}