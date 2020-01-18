#[macro_export]
macro_rules! s {
    ($e:expr) => {
        String::from($e)
    };
}

#[macro_export]
macro_rules! puts {
    ($e:expr) => {
        println!("{}", $e);
    };
}
