pub fn fibonacci(nterms:i64){
    let mut list: Vec<i64>=vec![0,1];
    while nterms>list.len() as i64{
        let  num=list.iter().rev().take(2).sum();
        list.push(num);
    }
    println!("Fibonacci Series with {} terms:\n{:?}",nterms,list);
}
