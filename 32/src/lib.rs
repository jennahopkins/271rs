
#[macro_export]
macro_rules! choice {
    ($x: expr, $y: expr, $z: expr) => {
        if $x == 1 {$y} else {$z}
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

#[macro_export]
macro_rules! rotright {
    ($x: expr, $y: expr) => {
        {(($x as u64) >> ($y % 64)) | (($x as u64) << (64 - ($y % 64)))}
    };
} 

#[macro_export]
macro_rules! rotleft {
    ($x: expr, $y: expr) => {
        {(($x as u64) << ($y % 64)) | (($x as u64) >> (64 - ($y % 64)))}
    };
}
