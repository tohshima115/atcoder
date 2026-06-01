use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        mut a: [usize; n]
    }
    // let p_draft: usize = a[p-1];
    // let q_draft: usize = a[q-1];
    // a[p-1] = q_draft;
    // a[q-1] = p_draft;
    a.swap(p-1, q-1);
    let s: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", s.join(" "));
 }
