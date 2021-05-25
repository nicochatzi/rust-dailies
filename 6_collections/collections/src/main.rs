fn sum(arr: &[f32]) -> f32 {
    // arr.iter().sum()

    let mut sum = 0.0;
    // for x in arr {
    //     sum += x;
    // }
    let mut iter = arr.iter();
    while let Some(item) = iter.next() {
        sum += item;
    }
    sum
}

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
//
// struct MyCollec<T> {
//     inner: Vec<T>
// }
//
// impl<T> Iterator for MyCollec<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn with_x(x: f32) -> Self {
        Self { x, y: 0.0 }
    }
}
fn main() {
    // let vec: Vec<f32> = Vec::new();
    // let vec = Vec::<f32>::new();
    let mut vec = vec![1.0, 2.0];
    for x in &vec {
        println!("{}", x);
    }
    println!("sum: {}", sum(&vec));
    vec.push(100.0);

    println!("sum: {}", sum(&vec));
    vec.remove(0);
    for x in &vec {
        println!("{}", x);
    }

    let arr: [f32; 4] = [0.0; 4];
    println!("sum: {}", sum(&arr));

    println!(
        "{:#?}",
        vec.iter()
            .filter(|&x| *x < 100.0)
            .map(|x| Point::with_x(*x))
            .collect::<Vec<Point>>()
    );
}
