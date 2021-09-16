fn main() {
    let v = array_diff(vec![1,2,1], vec![1]);

    for item in v.iter() {
        println!("{}", item);
    }
}

fn array_diff<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    for element in b.iter() {
        loop {
            match a.iter().position(|e| e == element) {
                Some(index) => { a.remove(index); },
                None => {
                    break;
                }
            }
        }
    }

    a
}

// BETTER IMPLEMENTATION
// fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
//     a.into_iter().filter(|x| !b.contains(x)).collect()
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
        assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
        assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
        assert_eq!(array_diff(vec![1,2,3], vec![1,2]), vec![3]);
    }
}