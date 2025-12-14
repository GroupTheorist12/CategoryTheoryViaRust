#[derive(Debug, Clone, PartialEq)]
struct FunctorBundle<T> {
    values: Vec<T>,
}

trait Functor<T> {
    fn fmap<U, F>(&self, f: F) -> FunctorBundle<U>
    where
        F: Fn(&T) -> U;
}

impl<T> Functor<T> for FunctorBundle<T> {
    fn fmap<U, F>(&self, f: F) -> FunctorBundle<U>
    where
        F: Fn(&T) -> U,
    {
        let mapped = self.values.iter().map(f).collect::<Vec<U>>();
        FunctorBundle { values: mapped }
    }
}

fn main() {
    let bundle = FunctorBundle { values: vec![1, 2, 3] };

    // Define two functions
    let f = |x: &i32| x * x;     // square
    let g = |x: &i32| x + 1;     // increment

    // Apply fmap sequentially: fmap(f) then fmap(g)
    let sequential = bundle.fmap(f).fmap(g);

    // Apply fmap with composition: fmap(g ∘ f)
    let composed = bundle.fmap(|x| g(&f(x)));

    // Print both results
    println!("Sequential: {:?}", sequential.values); // [2, 5, 10]
    println!("Composed:   {:?}", composed.values);   // [2, 5, 10]

    // Prove equality
    assert_eq!(sequential, composed);
    println!("✅ Composition law holds!");
}



