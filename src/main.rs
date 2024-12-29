use std::process::{Command, Output, Stdio};

fn main(){
    let wifi_level = get_wifi_level();
    let wifi_string_level = match wifi_level {
        0..=30 => "None",
        31..=53 => "Low",
        54..=70 => "Medium",
        71..=100 => "High",
        _ => "None",
    };
    const WIFI_SYMBOL: &str= "📶";
    println!("Wifi level: {}", WIFI_SYMBOL.to_owned()+&wifi_string_level);
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
    //this may be empty btw if there is now wifi
    let result = String::from_utf8_lossy(&stdout);

    let cleaned_result= result.trim_end_matches(".\n");
    if cleaned_result.is_empty() {
        return -1;
    }

    let output = cleaned_result.parse::<i32>().unwrap();
    output
}
