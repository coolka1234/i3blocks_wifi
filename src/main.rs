use std::process::Command;

fn main(){
    let wifi_level = get_wifi_level();
    println!("Wifi level: {}", wifi_level);
}

fn get_wifi_level() -> i32 {
    let output = Command::new("cat /proc/net/wireless | awk 'NR==3 {print $3}'")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.trim();

    let output = output.parse::<i32>().unwrap();
    output
}
