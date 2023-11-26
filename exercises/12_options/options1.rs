// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

type HourInt = u16;
type IcecreamCount = u16;

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: HourInt) -> Option<IcecreamCount> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!

    const MAX_HOUR: HourInt = 24;
    const NB_ICECREAM_BEFORE_EAT_TIME: HourInt = 5;
    const NB_ICECREAM_AFTER_EAT_TIME: HourInt = 0;
    const TIME_TO_EAT: HourInt = 22;

    // Guards
    if !(0..MAX_HOUR).contains(&time_of_day) {
        return None;
    }

    if (0..TIME_TO_EAT).contains(&time_of_day) {
        Some(NB_ICECREAM_BEFORE_EAT_TIME)
    } else {
        Some(NB_ICECREAM_AFTER_EAT_TIME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);
    }
}
