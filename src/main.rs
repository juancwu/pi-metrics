mod temperature;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let cpu_temp = temperature::get_cpu_temp().expect("Failed to get CPU temperature");

    println!("CPU Temperature: {:.1}C", cpu_temp);
}
