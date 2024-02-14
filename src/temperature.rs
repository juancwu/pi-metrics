use std::fs;
use std::io::Read;

pub fn get_cpu_temp() -> Result<f32, Box<dyn std::error::Error>> {
    let mut temp_file = fs::File::open("/sys/class/thermal/thermal_zone0/temp")?;
    let mut temp_str = String::new();
    temp_file.read_to_string(&mut temp_str)?;

    let cpu_temp: f32 = temp_str.trim().parse::<f32>()? / 1000.0;
    Ok(cpu_temp)
}
