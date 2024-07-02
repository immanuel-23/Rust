fn main(){
    println!("Functions in rust");
    hello_world();
    tell_height(1);
    emp_data(32,"M",5.9);
    println!("Addition of 2 numbers are {} ",add(3,4));
    //calaulatin bmi
    println!("Your BMI is {}" ,bmi(8.9,90.6));
}

fn hello_world(){
    println!("Hello World from function ");
}
fn tell_height(a :i32){
    println!("Height is  {}",a);
}
fn emp_data(age:i32,gender:&str,height:f32){
    println!("******* Emp data *******");
    println!("Age of emp is {}",age);
    println!("Gender of emp is {}",gender);
    println!("Height of emp is {}",height);
}
fn add(a:i32,b:i32)-> i32{
      a-b;
      a+b
}

fn bmi(weight:f32,height:f32)->f32{   
      weight/(height*height)
}

    


