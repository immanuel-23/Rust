fn main(){
    println!("Error Handling in rust");
    //APPROACH 1
    // enum Option<T>{
    //     Some(T),// represents a value
    //     None,// represents no value
    // }

    // //Approach 2
    // enum Result<T,E>{
    //     Ok(T),// Represents a value
    //     Err(E),// Represents an error
    // }

    // approach 1 example 
    let result =divide(8.0,0.0);
   match result{
    Some(x)=>println!("Result {} ",x),
    None=>println!("Cant divide by zero"),
   }
}

fn divide(numerator:f64,denominator:f64)->Option<f64>{
    if(denominator==0.0){
        None
    }else{
        Some(numerator/denominator)
    }
}