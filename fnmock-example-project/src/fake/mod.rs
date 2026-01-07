#[cfg(not(test))]
use crate::fake::add_two_calc::add_two;
#[cfg(test)]
use crate::fake::add_two_calc::add_two_fake as add_two;

mod add_two_calc;

fn calc(x: i32) -> i32 {

    add_two(x) + add_two(x)
}

#[cfg(test)]
mod test {
    use crate::fake::add_two_calc::add_two_fake;
    use crate::fake::calc;

    #[test]
    fn it_works() {
        add_two_fake::fake_implementation(|_| {
            8
        });

        let res = calc(2);

        assert_eq!(res, 16);
    }
}