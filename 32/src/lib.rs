#[macro_export]
macro_rules! choice {
    ($x: expr, $y: expr, $z: expr) => {
        if $x {$y} else {$z}
    };
}

#[macro_export]
macro_rules! median {
    ($x: expr, $y: expr, $z: expr) => {
        if ($x > $y) == ($x < $z) {$x}
        else if ($y > $x) == ($y < $z) {$y}
        else {$z}
    };
}

/* #[macro_export]
macro_rules! rotleft {
    ($x: expr, $y: expr) => {
    
    };
} */
