use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let data = vec![1,2,3,4,5,6,7];
    print!("{}", stringify!(data));
}

fn test1() {
    let tmp_dir = env::temp_dir();
    let tmp_file = tmp_dir.join("file");

    let mut file = File::create("./tmp_file.txt").unwrap();

    let mut x = vec![12, 12, 12, 12, 12, 12, 12];
    writeln!(&mut file, "{}", vec_to_string(x)).unwrap();

    //file.write(b"bytes\n").unwrap();
}

fn vec_to_string(n: Vec<i32>) -> String {
    let mut out = String::new();
    for i in 0..n.len() {
        let mut m = n[i];
        out += &m.to_string();
        if i < n.len() - 1 {
            out += ", "
        }
    }
    out
}
