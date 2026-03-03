fn takes_ownership(s: String) { // moves value in, drops here
    let _ = s;
}

fn borrows_value(s: &String) { // shared borrow, no move
    let _ = s;
}

fn borrows_value_mut(s: &mut String) { // exclusive borrow, can mutate
    s.push('!');
}

fn main() {
    let s1 = String::from("one"); // owned String on heap
    takes_ownership(s1);

    let s2 = String::from("two");
    borrows_value(&s2);

    let mut s3 = String::from("three");
    borrows_value_mut(&mut s3);

    let n = 10;
    let r = &n; // shared reference to n

    let mut m = 20;
    {
        let mr = &mut m; // mutable reference to m
        *mr += 1;
    }

    let _ = (s2, s3, n, r, m);
}

