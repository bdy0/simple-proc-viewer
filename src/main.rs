use std::fs;

fn main() {
    {
        //getting the users name and printing it
        let proc_status_path = String::from("/proc/self/status");

        let proc_status_content =
            fs::read_to_string(&proc_status_path).expect("Reading a file failed.");

        let Some(user_uid) = proc_status_content
            .lines()
            .find(|line| line.starts_with("Uid:"))
        else {
            return;
        };

        let user_uid: u32 = user_uid
            .split_whitespace()
            .nth(2) //0 - Uid:, 1 - first number
            .expect("second uid does not exist")
            .parse()
            .unwrap();

        let user_name_path = String::from("/etc/passwd");

        let user_name_content =
            fs::read_to_string(&user_name_path).expect("Reading a file failed.");

        let Some(user_name) = user_name_content
            .lines()
            .find(|line| line.split(":").nth(2) == Some(&user_uid.to_string()))
        else {
            return;
        };

        let user_name = user_name.split(":").nth(0).unwrap();

        println!("Hi {user_name}!");
    }

    //memory info
    let mem_info_path = String::from("/proc/meminfo");

    let mem_info_content = fs::read_to_string(&mem_info_path).expect("Reading a file failed.");

    let Some(mem_available) = mem_info_content
        .lines()
        .find(|line| line.starts_with("MemAvailable:"))
    else {
        return;
    };

    //from kB string to GB float
    let mem_available: i64 = mem_available
        .strip_prefix("MemAvailable:")
        .expect("Could not remove the prefix of mem_available.")
        .strip_suffix("kB")
        .expect("Could not remove the suffix of mem_available.")
        .trim()
        .parse()
        .expect("Read value not int");

    let mem_available: f32 = mem_available as f32 / (1024.0 * 1024.0);

    //memory total
    let Some(mem_total) = mem_info_content
        .lines()
        .find(|line| line.starts_with("MemTotal:"))
    else {
        return;
    };

    //from kB string to GB float
    let mem_total: i64 = mem_total
        .strip_prefix("MemTotal:")
        .expect("Could not remove the prefix of mem_available.")
        .strip_suffix("kB")
        .expect("Could not remove the suffix of mem_available.")
        .trim()
        .parse()
        .expect("Read value not int");

    let mem_total: f32 = mem_total as f32 / (1024.0 * 1024.0);

    let mem_used: f32 = mem_total - mem_available;

    print!("Memory Used: {mem_used:.2} GB / {mem_total:.2} GB\n");
}
