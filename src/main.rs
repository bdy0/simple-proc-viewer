use std::fs;

fn main() {
    let proc_status_path = String::from("/proc/self/status");

    let proc_status_content = fs::read_to_string(&proc_status_path)
        .expect("Reading {proc_status_path} failed.");

    let Some(user_uid) = proc_status_content
        .lines()
        .find(|line| line.starts_with("Uid:"))
        else { return; };

    let user_uid:u32 = user_uid
        .split_whitespace()
        .nth(2) //0 - Uid:, 1 - first number
        .expect("second uid does not exist")
        .parse() 
        .unwrap();

    println!("User Uid is {user_uid}.");

    //memory info
    let mem_info_path = String::from("/proc/meminfo");

    //println!("From the file {}.\n", mem_info_path);

    let mem_info_content = fs::read_to_string(&mem_info_path)
        .expect("Reading {mem_info_path} failed.");

    let Some(mem_available) = mem_info_content
        .lines()
        .find(|line| line.starts_with("MemAvailable:"))
        else { return };

    //from kB string to GB float
    let mem_available:i64 = mem_available
        .strip_prefix("MemAvailable:")
        .expect("Could not remove the prefix of mem_available.")
        .strip_suffix("kB")
        .expect("Could not remove the suffix of mem_available.")
        .trim()
        .parse()
        .expect("Read value not int");

    let mem_available:f32 = mem_available as f32 / (1024.0 * 1024.0);

    print!("Memory Available: {mem_available:.2} GB\n");
}
