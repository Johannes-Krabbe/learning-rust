fn main() {
    let mut s = a_function();

    a_third_function(&mut s);

    println!("{s}")
}

fn _func1() {
    let s = a_function();
    let s2 = a_second_function(s);

    // cant use s anymore, because it got moved to a_second_function
    // println!("{s}, World!");
    println!("{s2}, World!");
}

fn a_function() -> String {
    let s = String::from("Hello");
    return s;
}

fn a_second_function(my_string: String) -> String {
    let s = my_string.clone();
    return s;
}

fn a_third_function(my_string: &mut String) {
    my_string.push_str(", World!")
}
