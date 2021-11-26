//
// This is an excercise to teach myself how to use nested iterators 
// instead of a nested for loop, and then do some combined computation
// with each of the loop indexes. The idea is to map the outer iterator
// to the inner iterator. Since iterators are lazy, you must flatten
// and for_each the result. You must also move the data since the
// iterator can be executed after the parent is gone.
//
// Why do this? Iterators handle borrows more cleanly than expanded
// loops. 
//
// Weither or not this is idiomatic is for others to decide.
// 


fn main() {
    let vec1 = vec![0, 1, 2, 3, 4];
    let vec2 = vec![5, 6, 7, 8, 9];

    vec1.iter()
        .map( |y| vec2.iter()
                      .map(move |z| 10*y+z ))
        .flatten()
        .for_each(move |a| println!("Value {0}",a));
}
