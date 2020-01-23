mod macros;
mod module1;
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
    println!("Doing sth with arrays");
    let arry=[1,3,4,5,6,7];
    println!("{}",arry[3]);
    println!("{:?}",arry);
    println!("Doing sth with tuples unpacking");
    let tupl=("hello",23.2,100);
    let (t1,t2,t3)=tupl; //unpacking
    println!("{0}\n {1}\n {2}\n",t1,t2,t3);
    println!("{v1}\n {v2}\n {v3}\n",v2=t1,v1=t2,v3=t3);
    println!("{} to Types bin: {:b}, oct: {:o} hex: {:x}",10,10,10,10);
    println!("Variable debug for tupl is {:?}",tupl);
    println!("Doing sth with loops over arrays");
    for i in arry.iter(){
        println!("{}",i);
    }

}
fn main() {
    println!("Hello, world!");
    do_sth();
    fx_args("Tralah");
    println!("Using modules...");
    module1::run();
    welcome!();
}

fn fx_args(name: &str){
    println!("Argument is {}",name);
}
