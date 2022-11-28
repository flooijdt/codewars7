fn main() {
    in_array(&["sd"], &["a"]);
}
fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();

    for i in arr_b.iter() {
        for j in arr_a.iter() {
            if i.contains(j) && !answer.contains(&j.to_string()) {
                answer.push(j.to_string());
            }
        }
    }
    answer.sort();
    answer
}
