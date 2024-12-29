use std::process::{Command, Output, Stdio};
use colored::Colorize;

fn main(){
    let wifi_level = get_wifi_level();
    let wifi_string_level = match wifi_level {
        1..=30 => "None".blue(),
        31..=53 => "Low".yellow(),
        54..=70 => "Medium".yellow().green(),
        71..=100 => "High".green(),
        _ => "None".white(),
    };
    println!("Wifi level: 📶 {}", wifi_string_level);
}

fn get_wifi_level() -> i32 {
    let output = Command::new("/usr/bin/cat")
        .arg("/proc/net/wireless")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let awk= Command::new("awk")
        .arg("NR==3 {print $3}")
        .stdin(output.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let Output { stdout ,..} = awk.wait_with_output().unwrap();
    let result = String::from_utf8_lossy(&stdout);

    let cleaned_result= result.trim_end_matches(".\n");
    if cleaned_result.is_empty() {
        return -1;
    }
    let output = match cleaned_result.parse::<i32>()  {
        Ok(value) => value,
        Err(_) => -1,
    };
    output
}
