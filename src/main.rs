
use std::io;
fn main() {
    println!("Enter any data");
    let mut inputdata=String::new(); //will be saved on heap
    io::stdin().read_line(&mut inputdata).expect("Failed to read line");
    let int_data:u32=inputdata.trim().parse().expect("Please type a number!");
    println!("Converted Integer:{:?}",int_data);

    table(int_data);
}

fn table(int_data:u32)
{
    
for count in 1..11
{
println!("{} * {}= {}",int_data, count,int_data*count);
}
}


