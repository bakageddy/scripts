use std::process;
fn main() {
    if let Ok(result) = process::Command::new("acpi").output() {
        let output = String::from_utf8(result.stdout).unwrap();
        let status;
        if output.contains("discharging") {
            status = "-"
        } else {
            status = "+"
        }

        let percent: Vec<_> = output
            .split_whitespace()
            .flat_map(|s| s.split(","))
            .filter(|s| s.contains("%"))
            .map(|s| s.replace("%", ""))
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let n = percent.len();
        let mut sum = 0;
        for i in percent {
            sum += i
        }

        let avg = sum / n as i32;
        println!("{status}{avg}");
    }
}
