fn main() {
    let name = format!("functional programming");
    let r = &name;
    helper(r);
    helper(r);
    //println!("Hello, world!");
}

fn helper(name: &String) {
    println!("{}", name)
}
