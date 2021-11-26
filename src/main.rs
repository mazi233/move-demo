fn main() {
    // let vec0 = Vec::new();

    let mut vec1 = full_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn full_vec() -> Vec<i32> {
    let mut vec = vec![];

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
