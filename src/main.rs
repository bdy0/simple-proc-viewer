use std::fs;

fn main() {
    let mem_info_path = String::from("/proc/meminfo");

    println!("From the file {}.\n", mem_info_path);

    let mem_info_content = fs::read_to_string(&mem_info_path)
        .expect("File read failed.");

    let Some(mem_available) = mem_info_content
        .lines()
        .find(|line| line.starts_with("MemAvailable:"))
        else { return };

    let mem_available = mem_available
        .strip_prefix("MemAvailable:")
        .expect("Could not remove the prefix of mem_available.");

    let mem_available = mem_available.strip_suffix("kB")
        .expect("Could not remove the suffix of mem_available.");

    let mem_available:i64 = mem_available
        .trim()
        .parse()
        .expect("Read value not int");

    let mem_available:f32 = mem_available as f32 / (1024.0 * 1024.0);

    print!("Memory Available: {mem_available:.2} GB\n");
}
