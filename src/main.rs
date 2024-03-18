use pn;

fn main() {
    println!("Hello, world!");

    let x = pn::find_db("/private/var").unwrap();
    println!("{:?}", x)

}
