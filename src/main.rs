mod process;

use process::ProcessBuilder;

fn main() {
    println!(
        "{}\t{:-40}\t{:-15}\t{}\t{}",
        "PID", "Name", "Sched Policy", "RT Priority", "*RT"
    );
    let tasks = ProcessBuilder::full();
    for task in tasks {
        println!("{task}");
    }
}
