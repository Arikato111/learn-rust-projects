fn to_int(n: &String) -> i32 {
    match n.parse::<i32>() {
        Ok(v) => return v,
        Err(_) => return 0,
    };
}

pub fn add(n1: &String, n2: &String) -> i32 {
    return to_int(n1) + to_int(n2);
}

pub fn minus(n1: &String, n2: &String) -> i32 {
    let num1: i32 = to_int(n1);
    let num2: i32 = to_int(n2);
    return num1 - num2;
}

pub fn times(n1: &String, n2: &String) -> i32 {
    let num1: i32 = to_int(n1);
    let num2: i32 = to_int(n2);
    return num1 * num2;
}

pub fn divided(n1: &String, n2: &String) -> i32 {
    let num2: i32 = to_int(n2);
    if num2 == 0 {
        return 0;
    }
    let num1: i32 = to_int(n1);
    return num1 / num2;
}
