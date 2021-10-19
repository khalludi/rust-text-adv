fn main() {
    home_screen();
}

fn home_screen() {
    println!("");
    println!("       WELCOME TO RTA");
    println!("");

    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
    } else {
        println!("Unable to get term size :(")
    }
}

