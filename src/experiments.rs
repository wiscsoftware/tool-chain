use crate::macho::MAGIC;

#[test]
fn hex_test() {
    let a: u8 = 0xfe;
    let b: u8 = 0xed;
    let c: u8 = 0xfa;
    let d: u8 = 0xce;

    let input: [u8; 4] = [a, b, c, d];

    let str = format!("{:x}{:x}{:x}{:x}", input[0], input[1], input[2], input[3]);

    if let Ok(MAGIC) = u32::from_str_radix(str.as_str(), 16) {
        println!("match1");
    } else {
        println!("not a match1")
    }

    if let Ok(MAGIC) = u32::from_str_radix("feedface", 16) {
        println!("match2")
    } else {
        println!("not a match2")
    }
}

#[test]
fn vec_test() {
    let mut s = Vec::new();
    s.push(String::from("fa"));
    s.push(String::from("cf"));
    s.push(String::from("fe"));
    s.push(String::from("ed"));
    s.push(String::from("00"));
    s.push(String::from("0c"));
    s.push(String::from("01"));
    s.push(String::from("00"));
    s.push(String::from("00"));
    s.push(String::from("00"));

    let mut chunks: Vec<Vec<String>> = Vec::new();
    for chunk in s.chunks(2) {
        chunks.push(chunk.to_vec());
    }

    let mut row: Vec<String> = Vec::new();
    for chunk in chunks {
        row.push(chunk.join(""));
    }

    let mut res: Vec<String> = Vec::new();
    for chunk in row.chunks(8) {
        res.push(chunk.join(" "));
    }

    println!("{}", res.join("\n"))
}
