fn main() {
    let a = String::from("I can't be used after L3.");
    take_ownership(a);

    let b = String::from("I can still be used after L6.");
    take_reference(&b);
    println!("{}", b);

    let c = String::from("In L10, my ownership goes to fn at L14, but then it's moved to d. Thus, I can be used it in L11.");
    let d = take_and_give_ownership(c);
    println!("{}", d);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn take_reference(s: &String) {
    println!("{}", s);
}

fn take_and_give_ownership(s: String) -> String {
    s
}
