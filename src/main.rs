fn sorted(s: String) -> Vec<char> {
    let mut out: Vec<char> = Vec::new();
    for c in s.chars() {
        if c.is_ascii_lowercase() { out.push(c) }
    }
    out.sort();
    out
}

fn unique(v: Vec<char>) -> Vec<char> {
    let mut out: Vec<char> = Vec::new();
    for c in v {
        match out.binary_search(&c) {
            Ok(_) => {},
            Err(_) => { out.push(c) },
        }
    }
    out
}

fn mix(s1: &str, s2: &str) -> String {
    let s1 = s1.to_string();
    let s2 = s2.to_string();
    let ss1 = sorted(s1);
    let ss2 = sorted(s2);
    let u1 = unique(ss1);
    let u2 = unique(ss2);
    let count1: ()
    println!("{:?}", u1);
    String::new()
}

fn main() {
    println!("{:?}", mix("A aaaa bb c", "& aaa bbb c d"));
}
