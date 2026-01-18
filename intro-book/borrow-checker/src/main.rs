fn main() {
    let r;
    {
        let x = 5;

        let r = &x;
    }
    println!("{x} , {r}");
}
