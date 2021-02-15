#[derive(PartialEq, Debug)]
enum Optional<A> {
    Some(A),
    None
}

fn ret<A>(a: A) -> Optional<A> {
    Optional::Some(a)
}

fn compose<A, B, C>(f1: impl Fn(A) -> Optional<B>, f2: impl Fn(B) -> Optional<C>) -> impl Fn(A) -> Optional<C> {
    move |x| {
        if let Optional::Some(b) = f1(x) {
            return f2(b);
        }
        Optional::None
    }
}

fn main() {
    let safe_root = |x| {
        if x >= 0.0 {
            Optional::Some(f32::sqrt(x))
        } else {
            Optional::None
        }
    };

    let safe_reciprocal = |x: i32| {
        if x == 0 {
            return Optional::None;
        }
        Optional::Some(1.0/(x as f32))
    };

    let safe_root_reciprocal = compose(safe_reciprocal, safe_root);

    println!("{:?}", safe_root_reciprocal(4));
    assert_eq!(safe_root_reciprocal(4), Optional::Some(0.5))
}