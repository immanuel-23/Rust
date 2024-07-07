fn main (){
    println!("Structs in Rust");

    //tuples 
    let rect:(i32,i32)=(200,300);

    struct Books{
        title :String,
        author:String,
        noofpages:u32,
        price:f32,
    }

    struct User{
        active:bool,
        username:String,
        email:String,
    }

    let mut user1:User =User{
        active:true,
        username:String::from("Test"),
        email:String::from("test@gmail.com"),
    };


    println!("Struct user Example ");
    println!("Is user Active {} ",user1.active);
    println!("user Name {} ",user1.username);
    println!("Emial Id {} ",user1.email);


    let mut  books1:Books=Books{
        title:String::from("Peace With GOD"),
        author:String::from("DR. Billy Graham"),
        noofpages:190,
        price:210.00,
    };

    println!("Struct Books Example ");
    println!("Title of the book is : {} ",books1.title);
    println!("Name of the author is : {} ",books1.author);
    println!("Total No of pages :{} ",books1.noofpages);
    println!("Price of the Books is {}",books1.price);


}