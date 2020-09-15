pub fn lifetime_main() {
    danglingref();
    find_longer();
    struct_lifetime();
}

fn danglingref() {
    let r;
    // { // dangling reference if we restrict the scope
        let x = 5;
        r = &x;
    // }
    println!("r: {}", r);
}

fn find_longer() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_lifetime() {
    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}