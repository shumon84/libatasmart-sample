use std::path::Path;
use libatasmart;

fn main() {
    let p = Path::new("/dev/sdb");
    let mut disk = libatasmart::Disk::new(p).unwrap();
    let size = disk.get_disk_size().unwrap();
    let temperature = disk.get_temperature().unwrap();
    let cycle_count = disk.get_power_cycle_count().unwrap();

    println!(" size = {}",size);
    println!(" temp = {}",temperature);
    println!("count = {}", cycle_count);
}
