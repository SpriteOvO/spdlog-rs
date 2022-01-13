#[cfg(feature = "flexible-string")]
macro_rules! run_tests {
    ($($mod_name:ident => $cap:expr),*) => {
        $(
            mod $mod_name {
                const TEST_CAPACITY: usize = $cap;
                include!("common/flexible_string.in.rs");
            }
        )*
    };
}

#[cfg(feature = "flexible-string")]
run_tests! {
    capacity_0 => 0,
    capacity_1 => 1,
    capacity_2 => 2,
    capacity_3 => 3,
    capacity_4 => 4,
    capacity_5 => 5,
    capacity_8 => 8,
    capacity_10 => 10,
    capacity_16 => 16,
    capacity_32 => 32,
    capacity_64 => 64,
    capacity_128 => 128,
    capacity_250 => 250
}
