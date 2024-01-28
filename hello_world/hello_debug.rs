#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Book<'a> {
    name: &'a str,
    price: u8
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`
    println!("{:?} number", 15352);
    println!("{1:?},{0:?} and {dev:?}.",
             "Tester",
             "QA",
             dev="Developer");
    // print Structure    
    println!("Structure = {:?}!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look?
    println!("{:?}", Deep(Structure(5)));    

    let name = "Odyssey";
    let price = 182;
    let book = Book { name, price };
    //Pretty Print
    println!("{:#?}", book);
}