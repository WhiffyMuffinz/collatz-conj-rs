fn main() {
    let m = hasse(100, 1001);

    for i in m {
        for j in i{
            print!("{}, ", j);
        }
        println!("");
    }

}


fn hasse(from: i128, to: i128) ->Vec<Vec<i128>> {
    //create a vec of vecs of i128, and a loop from from to to, creating a vec of all the values a tmp variable 
    //passes through when the rules are applied. when the tmp == 1,
    //push the vec to the output and print the number of times the rules were applied to the tmp vaule bfore it settled on one
    let mut out = Vec::new();

    for i in from..=to {
        let mut j = apply_rules(i);
        let mut tmp = vec![j];
        let mut iter  = 0;
        while j > 1 {
            j = apply_rules(j);
            tmp.push(j);
            iter += 1;
        }
        out.push(tmp);
        println!("{}->{}",i, iter);
    }
    
    return out;
    
}

fn apply_rules(n: i128) -> i128 {
    if n <= 1 {
        return n;
    } else if n % 2 == 0 {
        return n / 2;
    } else {
        return 3 * n + 1;
    }
}