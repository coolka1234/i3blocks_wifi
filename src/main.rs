use std::process::{Command, Output, Stdio};

fn main(){
    let wifi_level = get_wifi_level();
    let mut wifi_level = match wifi_level {
        0..=30 => WifiLevel::None,
        31..=60 => WifiLevel::Low,
        61..=80 => WifiLevel::Medium,
        81..=100 => WifiLevel::High,
        _ => WifiLevel::None,
    };
    const WIFI_SYMBOL: &str= "📶";
    println!("Wifi level: {}", WIFI_SYMBOL.to_owned()+&wifi_level.to_string());
    // simple_test();
}

// fn simple_test() {
//     let output = Command::new("ls")
//         .output()
//         .expect("failed to execute process");

//     let output = String::from_utf8_lossy(&output.stdout);
//     println!("{}", output);
// }

enum WifiLevel {
    None,
    Low,
    Medium,
    High,
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
