// Trait for a Category
trait Category {
    type Object;
    type Morphism;

    fn id() -> Self::Morphism;
    fn compose(f: Self::Morphism, g: Self::Morphism) -> Self::Morphism;
}

// Category of integers with morphisms i32 -> i32
struct IntCategory;

impl Category for IntCategory {
    type Object = i32;
    // 'static so morphisms can be stored in closures safely
    type Morphism = Box<dyn Fn(i32) -> i32 + 'static>;

    fn id() -> Self::Morphism {
        Box::new(|x| x)
    }

    fn compose(f: Self::Morphism, g: Self::Morphism) -> Self::Morphism {
        // Move owned boxes into the closure; no borrowed lifetimes captured
        Box::new(move |x| f(g(x)))
    }
}

fn main() {
    let obj = 5;

    // Define morphisms
    let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
    let g: Box<dyn Fn(i32) -> i32> = Box::new(|x| x * 2);

    // Identity law checks
    let id = IntCategory::id();
    assert_eq!(id(obj), obj);                 // id acts as identity
    assert_eq!(f(id(obj)), f(obj));           // f ∘ id = f
    assert_eq!(id(f(obj)), f(obj));           // id ∘ f = f

    // Composition
    let composed = IntCategory::compose(f, g); // returns a new morphism
    println!("Compose(f,g)(5) = {}", composed(obj)); // (5*2)+1 = 11

    // Composition law validation
    let f2: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
    let g2: Box<dyn Fn(i32) -> i32> = Box::new(|x| x * 2);
    let composed2 = IntCategory::compose(f2, g2);

    // Evaluate both sides on a few objects
    for x in [0, 1, 2, 5, 10] {
        // Left: (f ∘ g)(x)
        let left = composed2(x);
        // Right: f(g(x))
        let right = (|y| (|z| z + 1)((|w| w * 2)(y)))(x);
        assert_eq!(left, right);
    }

    println!("✅ Category identity and composition laws hold for IntCategory.");
}
