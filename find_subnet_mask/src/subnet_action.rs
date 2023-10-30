pub fn bin_to_int(bin: &String) -> String {
    let sbin: Vec<&str> = bin.split(".").collect();
    let mut c_sbin: String = "".to_string();
    for i in sbin.iter() {
        let j = isize::from_str_radix(&i, 2).expect("not binary");
        if c_sbin.len() == 0 {
            c_sbin = format!("{}", j.to_string());
        } else {
            c_sbin = format!("{}.{}", c_sbin, j.to_string());
        }
    }
    c_sbin
}
pub fn subnet_format(subnet: String) -> String {
    subnet
        .as_bytes()
        .rchunks(8)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(".")
}
pub fn get_subnet(mask: u8) -> String {
    let mut count_limit = 32;
    let mut subnet: String = "".to_string();
    let mut count = mask;
    while count > 0 {
        subnet += "1";
        count -= 1;
        count_limit -= 1;
    }
    while count_limit > 0 {
        subnet += "0";
        count_limit -= 1;
    }
    subnet
}
