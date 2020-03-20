fn main() {
    // # ENUMS WITH GENERICS
    // enum Option<T>{
    //     Some(T),
    //     None,
    // };
    println!("{:?}", Some(1));
    println!("{:?}", Some(1.0));
    // -> Option<i32>

    // enum Result<T, E>{
    //     Ok(T),
    //     Err(E),
    // }
    // -> Result<i32, io::Error>
    // Ok(())

    // # FXNS/IMPL WITH GENERICS
    fn largest_i32(list : &[i32]) -> i32 {
        let mut largest = list[0];
        for i in 0..list.len(){
            if list[i] > largest{
                largest = list[i];
            }
        }
        largest
    }
    fn largest_char(list : &[char]) -> char {
        let mut largest = list[0];
        for i in 0..list.len(){
            if list[i] > largest{
                largest = list[i];
            }
        }
        largest
    }
    // fn largest<T>(list : &[T]) -> T{
    //     let mut largest = list[0];
    //     for i in 0..list.len(){
    //         if list[i] > largest{
    //             largest = list[i];
    //         }
    //     }
    //     largest        
    // }
    // let list = &vec![1, 2, 3];
    // largest(list);

    // # STRUCS WITH GENERICS
    // struct Point<T>{
    //     x: T,
    //     y: T,
    // }
    #[derive(Debug)]
    struct Point<T, U>{
        x: T,
        y: U,
    }
    let p = Point{x:1.0, y: 1.0};
    
    impl<T,U> Point<T,U>{
        fn x(&self) -> &T{
            &self.x
        }
    }

    impl Point<f32, f32>{
        fn am(&self) {
            println!("f32: {:?}", &self);
        }
    }
    p.am();
    
    // impl<T, U> Point<T, U> {
    //     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    //         Point {
    //             x: self.x,
    //             y: other.y,
    //         }
    //     }
    // }

    // # PERFORMANCE -> monomorphization.
    // ## OUR CODE
    // ```
    // #derive[(Debug)]
    // enum whatever<T>{
    //     what(T),
    //     ever,
    // };
    // println!("{:?}", whatever::what(1));
    // println!("{:?}", whatever::what(1.0));
    // ```

    // ## OUR CODE POST COMPILATION W/ MONOMORPHIZATION
    // ```
    // #derive[(Debug)]
    // enum whatever_i32{
    //     what(i32),
    //     ever,
    // };
    // #derive[(Debug)]
    // enum whatever_f64{
    //     what(f64),
    //     ever,
    // };
    // println!("{:?}", whatever_i32::what(1));
    // println!("{:?}", whatever_f64::what(1.0));
    // ```
}
