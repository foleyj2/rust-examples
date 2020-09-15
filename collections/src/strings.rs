pub fn main_strings_examples() {
    println!("String examples");
    //utf8_examples();
    string_iterate();
}
pub fn utf8_examples() {
    let hello = String::from("Hola");
    let hello = String::from("Здравствуйте");
    let hello = "Здравствуйте";
    //let answer = &hello[0];
}

pub fn utf8_slice() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];
}

pub fn string_iterate() {
    for c in "Örðí".chars() {
        println!("{}", c);
    }
}
