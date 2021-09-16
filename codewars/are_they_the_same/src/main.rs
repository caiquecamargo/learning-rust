fn main() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    comp(a1,a2);
}

fn comp(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    if a.len() == 0 || b.len() == 0 || a.len() != b.len() {
        return false;
    }

    a.sort();
    b.sort();

    for i in 0..a.len() {
        println!("{} == {}", a[i] * a[i], b[i]);
        if a[i] * a[i] != b[i] {
            return false;
        }
    }

    true
}

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn tests_comp() {

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);

}
