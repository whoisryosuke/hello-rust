fn main() {
    println!("Hello, world! \n");

    // String
    let message = String::from("Yo!");
    println!("{}", message);

    // Booleans
    let mut show_message = false;
    show_message = true;
    if show_message {
        println!("Showing message from boolean {} \n", show_message)
    }

    // Char
    let my_first_initial = 'C';

    if my_first_initial.is_alphabetic() {
        println!("Alphabetical char!");
    }

    // Arrays
    let a = [0; 101];

    if a.len() > 100 {
        println!("Array is larger than 100 - {} \n", a.len());
    } else {
        println!("Array too small - {} \n", a.len());
    }

    // Array slicing
    let b = [1, 2, 3, 4, 5];

    // We can slice arrays by using the pointer `&var_name`
    // and using this special array syntax `[start_index..last_index]`
    let nice_slice = &b[1..4];
    println!("{}", nice_slice.len());
    assert_eq!([2, 3, 4], nice_slice)
}
