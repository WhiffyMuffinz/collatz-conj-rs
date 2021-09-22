use std::fs::File;
use std::io::Write;

fn main() {
    println!("Creating values...");
    let data = hasse2(1, 2 << 12);
    println!("done. \n");

    let file = create_file();
    println!("writing values to file...");
    for arr in &data {
        write_to_file(&file, arr);
    }
    
    println!("done! \n")
}


//the data that this creates is weird, goes 1,10,2,16,3,22...
fn hasse(from: u128, to: u128, debug: bool) -> Vec<Vec<u128>> {
    //create a vec of vecs of u128, and a loop from "from" to "to", creating a vec of all the values. a tmp variable
    //passes through when the rules are applied. when the tmp == 1,
    //push the vec to the output and print the number of times the rules were applied to the tmp vaule bfore it settled on 1
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
        if debug {
            println!("{}->{}", i, iter);
        }
    }

    return out;
}

fn hasse2 (from: u128, to: u128) -> Vec<Vec<u128>> {
    let mut out = Vec::new();

    for i in from..=to {
        let mut j = i;
        let mut tmp = vec![j];
        while j > 1 {
            j = apply_rules(j);
            tmp.push(j);
        }
        out.push(tmp);
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


//writes one vec to file 
fn write_to_file(mut dest: &File, data: &Vec<u128>) {
    let mut n = 0;//tmp variable used fir coma addition
    for i in data {
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
    let file = File::create("./data.txt").unwrap();
    return file;
}
