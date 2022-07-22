#[macro_export]
macro_rules! result {
    (C) => {
        Correctness::Correct
    };
    (M) => {
        Correctness::Misplaced
    };
    (W) => {
        Correctness::Wrong
    };
    ($($item: tt)*) => {
        [
        $(result!($item)),*
        ]
    };
}
