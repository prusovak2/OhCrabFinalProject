// The debug version
#[cfg(feature = "visualizer_verbose")]
#[macro_export]
macro_rules! println_d {
    ($( $args:expr ),*) => { println!( $( $args ),* );  }
}

// Non-debug version
#[cfg(not(feature = "visualizer_verbose"))]
#[macro_export]
macro_rules! println_d {
    ($( $args:expr ),*) => {}
}
