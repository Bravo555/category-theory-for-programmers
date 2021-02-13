fn main() {
    println!("testing identity with str to int");

    let _ida = compose(str2int, id);
    let _idb = compose(id, str2int);
}

fn id<A>(a: A) -> A {
    a
}

fn compose<F1, F2, A, B, C>(f1: F1, f2: F2) -> impl FnOnce(A) -> C
where
    F1: FnOnce(A) -> B,
    F2: FnOnce(B) -> C,
{
    move |x| f2(f1(x))
}

fn str2int(_: &str) -> i32 {
    5
}
