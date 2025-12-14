#[allow(dead_code)]

// Category trait
trait Category {
    type Object;
    type Morphism;

    fn id() -> Self::Morphism;
    fn compose(f: Self::Morphism, g: Self::Morphism) -> Self::Morphism;
}

// IntCategory: objects are i32, morphisms are i32 -> i32
struct IntCategory;

impl Category for IntCategory {
    type Object = i32;
    type Morphism = Box<dyn Fn(i32) -> i32>;

    fn id() -> Self::Morphism {
        Box::new(|x| x)
    }

    fn compose(f: Self::Morphism, g: Self::Morphism) -> Self::Morphism {
        Box::new(move |x| f(g(x)))
    }
}

// StringCategory: objects are String, morphisms are String -> String
struct StringCategory;

impl Category for StringCategory {
    type Object = String;
    type Morphism = Box<dyn Fn(String) -> String>;

    fn id() -> Self::Morphism {
        Box::new(|x| x)
    }

    fn compose(f: Self::Morphism, g: Self::Morphism) -> Self::Morphism {
        Box::new(move |x| f(g(x)))
    }
}

// Functor trait: maps between categories
trait Functor<Source: Category, Target: Category> {
    fn map_object(obj: Source::Object) -> Target::Object;
    fn map_morphism(m: Source::Morphism) -> Target::Morphism;
}

// Example: Functor from IntCategory to StringCategory
struct IntToStringFunctor;

impl Functor<IntCategory, StringCategory> for IntToStringFunctor {
    fn map_object(obj: i32) -> String {
        format!("Number({})", obj)
    }

    fn map_morphism(m: Box<dyn Fn(i32) -> i32>) -> Box<dyn Fn(String) -> String> {
        Box::new(move |s: String| {
            // Parse string back to int, apply morphism, then wrap again
            let n: i32 = s.trim_start_matches("Number(")
                          .trim_end_matches(")")
                          .parse()
                          .unwrap_or(0);
            let result = m(n);
            format!("Number({})", result)
        })
    }
}

fn main() {
    // Object mapping
    let obj = 5;
    let mapped_obj = IntToStringFunctor::map_object(obj);
    println!("Mapped object: {}", mapped_obj); // "Number(5)"

    // Morphism mapping
    let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
    let mapped_f = IntToStringFunctor::map_morphism(f);

    let input = "Number(10)".to_string();
    println!("Mapped morphism: {}", mapped_f(input)); // "Number(11)"
}
