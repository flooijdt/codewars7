fn main() {
    in_array(["sd"], ["a"]);
}
fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let toda = String::from("tifoide");
    let part = String::from("oide");

    if toda.contains(part) {
        println!("contem carai");
    }
    Vec::new([toda])
}
