use crate::hashmap::map;
pub mod hashmap;

fn hash() {
    map();
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(1);
    v.push(1);

    for i in &v {
        println!("{i}");
    }
}

fn print_vec() {
    let vec = vec![10, 20, 30, 40, 50];
}
