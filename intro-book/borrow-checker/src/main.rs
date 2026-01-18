fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("pilas");
    let string2 = "wilass";

    let result = longest(&string1, &string2);
    println!("{result}");
}
