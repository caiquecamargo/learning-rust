fn main() {
    println!("{}", count_sheep(&[false]));
}

fn count_sheep(sheeps: &[bool]) -> u8 {
    let mut qty_sheeps = 0;
    for sheep in sheeps {
        let recp: Option<&bool> = Some(sheep);
        match recp {
            None => break,
            Some(value) => {
                if *value {
                    qty_sheeps += 1;
                }
            }
        }
    }

    qty_sheeps
}

/*
    ANOTHER SOLUTIONS

    fn count_sheep(sheep: &[bool]) -> u8 {
        sheep.iter().filter(|&&x|x).count() as u8
    }
*/

#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
  }