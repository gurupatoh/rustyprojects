fn main() {
let results=summation(7,9);
    display_results(results);

}
fn summation(a:i32,b:i32)->i32{
    a+b
}
fn display_results(results:i32){
    println!("The results are:{:?}",results)
}