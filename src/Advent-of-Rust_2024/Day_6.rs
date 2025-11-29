// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let s1_t = s1.trim();
    let s2_t = s2.trim();

    let s1_l = s1_t.chars().count();
    let s2_l = s2_t.chars().count();

    if s1_l > s2_l {
        Some(s1_t)
    } else if s1_l < s2_l {
        Some(s2_t)
    } else {
        None
    }
}

fn main() {
    let t1 = "你好";
    let t2 = "world";
    println!("{:?}", longer_wish(t1, t2));
    println!("Hello world");
}
