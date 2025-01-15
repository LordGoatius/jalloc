#![feature(arbitrary_self_types)]
#![feature(arbitrary_self_types_pointers)]
pub mod jallocator;

#[cfg(test)]
mod tests {
    use crate::jallocator::{self, Jallocator};

    #[test]
    fn test_alloc() {
        let jallocator = Jallocator::new();
        let thing: &mut [f32; 6] = jallocator.jalloc([6.1f32, 3.4, 2.2, 4.1, 5.3, f32::NAN]);
        let thing2: &mut [i32; 1] = jallocator.jalloc([5]);

        #[derive(Debug)]
        struct Str<'a> {
            a: i32,
            b: &'a str,
            c: [u8; 3]
        }

        eprintln!("{thing:?}");
        eprintln!("{thing2:?}");
        thing[5] = 6.1;
        eprintln!("{thing:?}");
        eprintln!("{thing2:?}");
        let a: &mut Str<'_> = jallocator.jalloc(Str { a: -5, b: "mytype", c: [6, 2, 54] });
        eprintln!("{a:?}");
        eprintln!("{jallocator:?}");
        eprintln!("{:?}", jallocator.size);
        eprintln!("{:?}", jallocator.capacity);
    }

    #[test]
    fn test_box() {
        use crate::jallocator::r#box::Box;
        let jallocator = Jallocator::new();
        let x = Box::new(jallocator, [7, 4]);
        eprintln!("{:?}", *x);
    }
}
