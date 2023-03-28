use extest::group;

#[group(single, not_single)]
#[test]
fn single_test() {
    assert_eq!(1, 1);
}

fn main() {}
