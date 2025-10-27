fn main() {
    let mut prime_numbers: Vec<u32> = vec![2];
    let number:u32 = 100;
    for i in 3..number {
        for n in &prime_numbers.clone() {
            if &i % n == 0 {
                break;
            }
            else if *n == prime_numbers[prime_numbers.len()-1].clone(){
                prime_numbers.push(i);
            }
        }
    }
    for i in prime_numbers {
        println!("{}", i);
    }
}
