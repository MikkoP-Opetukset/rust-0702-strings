#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 8.2: Storing UTF-8 Encoded Text with Strings
///
/// `String` is a growable, owned, UTF-8 encoded text type. `str` (usually seen
/// as `&str`) is the borrowed string slice type.
fn main() {
    creating_strings();
    updating_strings();
    concatenation();
    indexing_is_not_allowed();
    slicing_strings();
    iterating_over_strings();
}

/// # Creating Strings
/// - String literals are `&str` slices stored in the program binary.
/// - `String::new()`, `String::from`, and `.to_string()` create owned Strings.
/// - Strings are UTF-8 encoded, so any language or emoji works.
fn creating_strings() {
    println!("\n{:=>80}", "");
    println!("creating_strings()\n");

    let empty = String::new();
    let from_literal = String::from("Hello, hero!");
    let to_string = "Hello again".to_string();
    println!("empty = {empty:?}");
    println!("{from_literal} / {to_string}");

    let greetings = [
        String::from("Mörjens maailma"),
        String::from("Henlo wrold"),
        String::from("你好，世界"),
        String::from("👋🌍🌎🌏"),
    ];
    // The array holds owned `String`s, not references, so each element owns
    // its text. Looping over `greetings` itself would move the array (and
    // every String in it) into the loop. Borrowing it with `&greetings`
    // keeps the array usable afterwards, and each `greeting` is then a
    // `&String` that only reads the element.
    for greeting in &greetings {
        println!("{greeting}");
    }
}

/// # Updating Strings
/// - `push_str` appends a `&str` and does not take ownership of it. It takes
///   a borrowed slice because it only needs to read the text and copy it into
///   the String's own buffer, so the caller keeps its value.
/// - `push` appends a single `char`.
/// - `insert` and `insert_str` add text at a byte index.
fn updating_strings() {
    println!("\n{:=>80}", "");
    println!("updating_strings()\n");

    let mut message = String::from("The hero");
    message.push_str(" enters");
    message.push('!');
    println!("{message}");

    let suffix = String::from(" Everyone cheers.");
    message.push_str(&suffix);
    println!("{message}");
    println!("`suffix` is still usable: {suffix}");

    message.insert(0, '>');
    message.insert_str(1, "> ");
    println!("{message}");
}

/// # Concatenation
/// - `+` takes ownership of the left `String` and borrows the right side as
///   `&str`. The compiler turns `&String` into `&str` with deref coercion.
/// - `format!` builds a new String from several parts without taking
///   ownership of any of them. It works like `println!`.
fn concatenation() {
    println!("\n{:=>80}", "");
    println!("concatenation()\n");

    let first = String::from("Dragon");
    let second = String::from("slayer");
    let combined = first + "-" + &second;
    println!("{combined}");

    // `first` was moved into `combined` by `+`.
    // println!("{first}"); // Uncomment to see the error.

    let name = String::from("Aria");
    let class = String::from("ranger");
    let level = 7;
    let description = format!("{name} the {class}, level {level}");
    println!("{description}");
    println!("`name` and `class` are still usable: {name}, {class}");
}

/// # Indexing Is Not Allowed
/// - Rust does not allow `s[0]`, because a character can use several bytes.
/// - `len()` returns the length in bytes, not in characters.
/// - Bytes, scalar values (`char`), and grapheme clusters are three different
///   views of the same text.
fn indexing_is_not_allowed() {
    println!("\n{:=>80}", "");
    println!("indexing_is_not_allowed()\n");

    let ascii = String::from("Hello");
    let greek = String::from("Καλημέρα");
    println!("{ascii} has {} bytes", ascii.len());
    println!("{greek} has {} bytes", greek.len());

    // Indexing a String is a compile error.
    // let first = ascii[0]; // Uncomment to see the error.

    // The Hindi word "नमस्ते" is stored as 18 bytes and 6 chars, but a person
    // sees four graphemes: "न", "म", "स्" and "ते". Bytes, chars, and
    // graphemes do not match one to one.
    let hindi = "नमस्ते";
    println!("bytes: {:?}", hindi.bytes().collect::<Vec<u8>>());
    println!("chars: {:?}", hindi.chars().collect::<Vec<char>>());
}

/// # Slicing Strings
/// - A range can slice a string: `&s[0..4]` takes the first four bytes.
/// - The range must fall on character boundaries, otherwise the program
///   panics at runtime.
/// - `get` takes a range and returns an `Option<&str>`, which is `None`
///   instead of a panic when the range is not on character boundaries.
fn slicing_strings() {
    println!("\n{:=>80}", "");
    println!("slicing_strings()\n");

    let text = String::from("Καλημέρα");
    let first_two_letters = &text[0..4];
    println!("The first 4 bytes: {first_two_letters}");

    // Each Greek letter uses two bytes, so byte 1 is inside a character.
    // let broken = &text[0..1]; // Uncomment and run the project to see the panic.

    // `get` checks the boundaries and returns an Option instead of panicking.
    match text.get(0..1) {
        Some(slice) => println!("get(0..1) found {slice}."),
        None => println!("get(0..1) is not on a character boundary."),
    }
    match text.get(0..4) {
        Some(slice) => println!("get(0..4) found {slice}."),
        None => println!("get(0..4) is not on a character boundary."),
    }
}

/// # Iterating over Strings
/// - `chars()` iterates over Unicode scalar values.
/// - `bytes()` iterates over raw bytes.
/// - Choose the method that matches what the program needs.
fn iterating_over_strings() {
    println!("\n{:=>80}", "");
    println!("iterating_over_strings()\n");

    for c in "Κα".chars() {
        println!("char: {c}");
    }

    for b in "Κα".bytes() {
        println!("byte: {b}");
    }

    let word = "stressed";
    let reversed: String = word.chars().rev().collect();
    println!("{word} reversed is {reversed}");
}
