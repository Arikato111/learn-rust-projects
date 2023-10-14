fn p(m: &mut i32) {
    *m = 128;
    println!("m {:p} : {}", m, *m);
}
fn main() {
    let mut i: i32 = 30;
    let j = &mut i;
    *j = 59;

    p(j);

    println!("j {:p} : {}", j, *j);
    println!("i {:p} : {}", &i, i);
}
