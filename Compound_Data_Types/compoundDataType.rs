fn main(){
    println!("Compund Data Types Test File");
    //arrays
    let numbers :[i32;5]=[1,2,3,4,5];
    println!("The number of arrays are {:?}",numbers);

    let fruits:[&str;3]=["Apple","Orange","Mango"];
    println!("Fuits are : {:?}",fruits);
    println!("Fuits at o index is : {}",fruits[0]);
    println!("Fuits at 1 index is : {}",fruits[1]);
    println!("Fuits at 2 index is  : {}",fruits[2]);

    //tupels
    let human =("Alice",30,false);
    let human2:(String ,i32,bool) =("Alice".to_string(),30,false);
    println!("Human Tuple:{:?}",human);
    println!("Human2 Tuple:{:?}",human2);

    //slices:
    let number_slices :&[i32]=&[1,2,3,4];
    println!("Number Slices: {:?}",number_slices);


    // string vs String slices
    //1)String are expandable 
    //2)String slices 

    let  mut stone_col:String=String::from("Hello,");
    println!("Stone Col says :{}",stone_col);
    stone_col.push_str("World");
    println!("Stone Col says :{:?}",stone_col.push_str("World"));
    
    println!("Stone Col says :{:?}",stone_col);

    //




}