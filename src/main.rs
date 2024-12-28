use std::process::{Command, Output, Stdio};

fn main(){
    let wifi_level = get_wifi_level();
    println!("Wifi level: {}", wifi_level);
    // simple_test();
}

// fn simple_test() {
//     let output = Command::new("ls")
//         .output()
//         .expect("failed to execute process");

//     let output = String::from_utf8_lossy(&output.stdout);
//     println!("{}", output);
// }

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

    println!("{}", cleaned_result);

    let output = cleaned_result.parse::<i32>().unwrap();
    output
}
