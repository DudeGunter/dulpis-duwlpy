#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("Start");

    println!("out: {:?}", Deep(Structure(3)));
}
