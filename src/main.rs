fn do_sth(){
    // C style comments
    let x=90;
    println!("I'm doing something with x = {}",x);
    /* Multiline
     * comments
     */
    //cannot assign x=34 because x is immutable by default
    //to make mutable, do
    let mut y=39;
    println!("Creating mutable var y with {}",y);
    y=43;
    println!("Mutating var y to {}",y);
}
fn main() {
    println!("Hello, world!");
    do_sth();
}
