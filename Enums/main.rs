fn main() {
    println!("Enumns");

    enum IpAdress {
        V4,
        V6,
    };
    let four: IpAdress = IpAdress::V4;
    let six: IpAdress = IpAdress::V6;

    println!("Enum four is {} ",IpAdress::V4);
    println!("Enum Six is {} ",six);

}
