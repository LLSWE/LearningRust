mod trait_example;

fn largest<T>(list: &[T]) -> &T {
    let largest = &list[0];

    for value in list {
        if value > largest {
            largest = value;
        }
    }

    largest
}

fn main() {
    let list_int = vec![200, 100, 670, 11, 22];
    let list_str = vec!["a", "b", "c", "d", "e"];
    let result_int = largest(&list_int);
    let result_str = largest(&list_str);
}

//Code does not compile, rust compiler suggest stricting generic type <T> with trait PartialOrd
