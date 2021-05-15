use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Hello, ");
    v.push("World!\n");

    for msg in &v {
        print!("{}", msg);
    }
}