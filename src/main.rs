use std::fs;

fn main() {
    let mem_info_path = String::from("/proc/meminfo");

    println!("From the file {}.\n", mem_info_path);

    let mem_info_content = fs::read_to_string(mem_info_path)
        .expect("File read failed.");

    let Some(mem_available) = mem_info_content
        .lines()
        .find(|line| line.starts_with("MemAvailable:"))
        else { return };

    print!("{}\n", mem_available);
}
