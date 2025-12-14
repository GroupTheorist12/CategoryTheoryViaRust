struct Functor<T> {
    values: Vec<T>,
}

trait Map<T, U> {
    fn map<F>(&self, f: F) -> Functor<U>
    where
        F: Fn(&T) -> U;
}

impl<T, U> Map<T, U> for Functor<T> {
    fn map<F>(&self, f: F) -> Functor<U>
    where
        F: Fn(&T) -> U,
    {
        let mapped = self.values.iter().map(f).collect::<Vec<U>>();
        Functor { values: mapped }
    }
}

fn main() {
    let numbers = Functor { values: vec![1, 2, 3] };
    let squared = numbers.map(|x| x * x);
    for v in squared.values {
        println!("{}", v); // Output: 1, 4, 9
    }
}
