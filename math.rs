fn main() {
    let price = 129;
    let tax = 23.22;
    let total = f64::from(price) + tax;
    println!("Total: {} + {} = {}", price, tax, total)
}