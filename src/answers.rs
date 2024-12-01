pub use paste::paste;

pub fn read_answer(day: usize, puzzle: usize) -> Option<String> {
    std::fs::read_to_string(format!("answers/{day}.txt"))
        .ok()?
        .lines()
        .map(str::to_string)
        .nth(puzzle)
}

#[macro_export]
macro_rules! answer_tests {
    ($solutions:expr, $day:literal) => {
        $crate::answers::paste! {
            #[test]
            fn [<day_ $day _part_1>]() {
                use $crate::Solution;
                $crate::init_test_logging();
                let Some(expected) = $crate::answers::read_answer($day, 0) else { return };
                let input = $crate::read_input($day, "1");
                let ans = $solutions[$day - 1].solve_1(input);
                assert_eq!(expected, ans);
            }

            #[test]
            fn [<day_ $day _part_2>]() {
                use $crate::Solution;
                $crate::init_test_logging();
                let Some(expected) = $crate::answers::read_answer($day, 1) else { return };
                let input = $crate::read_input($day, "2");
                let ans = $solutions[$day - 1].solve_2(input);
                assert_eq!(expected, ans);
            }
        }
    };
}

#[macro_export]
macro_rules! generate_answer_tests {
    ($solutions:expr) => {
        $crate::answer_tests!($solutions, 01);
        $crate::answer_tests!($solutions, 02);
        $crate::answer_tests!($solutions, 03);
        $crate::answer_tests!($solutions, 04);
        $crate::answer_tests!($solutions, 05);
        $crate::answer_tests!($solutions, 06);
        $crate::answer_tests!($solutions, 07);
        $crate::answer_tests!($solutions, 08);
        $crate::answer_tests!($solutions, 09);
        $crate::answer_tests!($solutions, 10);
        $crate::answer_tests!($solutions, 11);
        $crate::answer_tests!($solutions, 12);
        $crate::answer_tests!($solutions, 13);
        $crate::answer_tests!($solutions, 14);
        $crate::answer_tests!($solutions, 15);
        $crate::answer_tests!($solutions, 16);
        $crate::answer_tests!($solutions, 17);
        $crate::answer_tests!($solutions, 18);
        $crate::answer_tests!($solutions, 19);
        $crate::answer_tests!($solutions, 20);
        $crate::answer_tests!($solutions, 21);
        $crate::answer_tests!($solutions, 22);
        $crate::answer_tests!($solutions, 23);
        $crate::answer_tests!($solutions, 24);
        $crate::answer_tests!($solutions, 25);
    };
}
