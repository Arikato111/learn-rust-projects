pub fn show(action: &str, num1: i32, num2: i32) {
    let result: i32;
    if action == "+" {
        result = num1 + num2;
    } else if action == "-" {
        result = num1 - num2;
    } else if action == "*" {
        result = num1 * num2;
    } else if action == "/" {
        result = num1 / num2;
    } else {
        println!("Error input");
        return;
    }
    println!("{} {} {} = {}", num1, action, num2, result);
}
