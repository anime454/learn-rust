fn main() {
    // Scalar Types

    let _x8: i8 = 0;
    let _y8: u8 = 255;

    let _x16: i16 = 0;
    let _y16: u16 = 65535;

    let _x32: i32 = 0;
    let _y32: u32 = 4294967295;

    let _x64: i64 = 0;
    let _y64: u64 = 18446744073709551615;

    let _x128: i128 = 0;
    let _y128: u128 = 340282366920938463463374607431768211455;

    let _xsize: isize = 0;
    let _ysize: usize = 18446744073709551615;

    let _xfloat: f32 = 0.0;
    let _yfloat: f64 = 0.0;

    let _xbool: bool = true;
    let _ybool: bool = false;

    let _xchar: char = 'a';

    // Compound Types

    let _xtuple = (500, 6.4, 1);
    let _ytuple: (i32, &str, f64) = (500, "hello", 6.4);

    let _xarray = [1, 2, 3, 4, 5];
    let _yarray: [i32; 5] = [1, 2, 3, 4, 5];
    

}
