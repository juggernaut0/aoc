pub use paste::paste;
pub use seq_macro::seq;

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
    ($solutions:expr) => { generate_answer_tests!($solutions, 25) };
    ($solutions:expr, $num:literal) => {
        $crate::answers::seq!(N in 1..=$num {
            $crate::answer_tests!($solutions, N);
        });
    };
}
