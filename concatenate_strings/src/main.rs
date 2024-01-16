fn main() {
    let string1 = "Hello, ";
    let string2 = "World!";

    let concatenated_string = concatenate_strings(string1, string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result
}
