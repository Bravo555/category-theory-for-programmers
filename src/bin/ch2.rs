use rand::prelude::*;
use std::fmt::Debug;
use std::{collections::HashMap, hash::Hash};

fn main() {
    let random = move |seed| {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        rng.gen::<u32>()
    };
    let mut rand_memo = memoize(random);

    for i in 0..10 {
        println!("{}", rand_memo(i as u64));
    }
}

fn memoize<A, B, F>(f: F) -> impl FnMut(A) -> B
where
    F: Fn(A) -> B,
    A: Clone + Hash + Eq + Debug,
    B: Clone + Debug,
{
    let mut cacher = Box::new(Cacher {
        cache: HashMap::new(),
        function: f,
    });

    move |x| cacher.run(x)
}

struct Cacher<A, B, F>
where
    F: Fn(A) -> B,
{
    cache: HashMap<A, B>,
    function: F,
}

impl<A: Eq + Hash + Clone, B: Clone, F> Cacher<A, B, F>
where
    A: Eq + Hash + Clone + Debug,
    B: Clone + Debug,
    F: Fn(A) -> B,
{
    fn run(&mut self, a: A) -> B {
        let f = &self.function;
        self.cache
            .entry(a.clone())
            .or_insert_with(|| f(a.clone()))
            .clone()
    }
}
