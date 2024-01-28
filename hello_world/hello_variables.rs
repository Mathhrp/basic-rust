fn main(){
    //The '{}' will be automatically replaced with arguments stringified;
    println!("{} days", 31);

    //Positional araguments
    println!("{0}, {1}. {1}, {0}", "Cappybara", "Wombats");

    //Formating caracters
    println!("Base 10: {}", 452687);
    println!("Base 2(binary): {:b}",452687);
    println!("Base 8(octal): {:o}",452687);
    println!("Base 16(hex): {:x}",452687);
    println!("Base 16(hex/upper): {:X}",452687);

    //Formating right-justify
    println!("{number:>7}", number=2);
    //Pad extra zeroes
    println!("{number:0>7}", number=2);
    //left-ajust flipping
    println!("{number:<7}", number=2);

    //Add arguments in format
    println!("{number:0>width$}", number=2, width=7);
}