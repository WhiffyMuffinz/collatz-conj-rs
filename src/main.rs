use std::env;
use std::fs::File;
//use std::io::Read;
use std::io::Write;

fn main() {
    let data = hasse(2 << 10, 2 << 12);

    let mut file = create_file();
    for arr in data.clone() {
        write_to_file(&file, arr);
    }
    
    for i in &data {
        for j in i {
            print!("{}, ", j);
        }
        println!("");
    }
}

fn hasse(from: u128, to: u128) -> Vec<Vec<u128>> {
    //create a vec of vecs of u128, and a loop from from to to, creating a vec of all the values a tmp variable
    //passes through when the rules are applied. when the tmp == 1,
    //push the vec to the output and print the number of times the rules were applied to the tmp vaule bfore it settled on one
    let mut out = Vec::new();

    for i in from..=to {
        let mut j = apply_rules(i);
        let mut tmp = vec![j];
        let mut iter = 0;
        while j > 1 {
            j = apply_rules(j);
            tmp.push(j);
            iter += 1;
        }
        out.push(tmp);
        println!("{}->{}", i, iter);
    }

    return out;
}
//applies the rules of the collatz conjecture
fn apply_rules(n: u128) -> u128 {
    if n <= 1 {
        return n;
    } else if n % 2 == 0 {
        return n / 2;
    } else {
        return 3 * n + 1;
    }
}

//TODO: implement copy file then concatenate new values to existing string
//fn vec_to_string(n: Vec<u128>) -> String {
//    let mut out = String::new();
//    for i in 0..n.len() {
//        let m = n[i];
//        out += &m.to_string();
//        if i < n.len() - 1 {
//            out += ", "
//        }
//    }
//    out
//}


fn write_to_file(mut dest: &File, data: Vec<u128>) {
    let mut n = 0;
    for i in &data {
        if n < data.len() - 1 {
                write!(dest, "{}, ", i).unwrap();
        } else {
            write!(dest, "{}", i).unwrap();
        }
        n += 1;
    }
    write!(dest, "\n").unwrap();
}

fn create_file() -> File {
    let tmp_dir = env::temp_dir();
    let tmp_file = tmp_dir.join("file");

    let mut file = File::create("./data.txt").unwrap();

    return file;
}
