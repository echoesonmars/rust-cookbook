const MAX_HP: u32 = 100;  // compile-time constant

fn main() {
    let x = 42;           // type inferred, i32 default for int literals
    let a: i8 = -128;     // signed 8-bit
    let b: u8 = 255;      // unsigned 8-bit
    let c: i16 = -32000;
    let d: u16 = 65000;
    let e: i32 = -2_000_000;  // underscores for readability
    let f: u32 = 4_000_000;
    let g: i64 = -9e18 as i64;
    let h: u64 = 18_000_000_000;
    let i: i128 = -1;
    let j: u128 = 1;
    let k: isize = -1;    // pointer-sized signed
    let l: usize = 1;     // pointer-sized unsigned, indices

    let m: f32 = 3.14;    // 32-bit float
    let n: f64 = 3.14159; // 64-bit float, default for decimals

    let o: bool = true;
    let p: char = 'z';    // single Unicode scalar

    let mut counter = 0;  // mutable binding
    counter += 1;

    let tup: (i32, f64, bool) = (1, 2.0, false);
    let (q, r, s) = tup;  // destructuring
    let first = tup.0;    // index access

    let arr: [i32; 5] = [1, 2, 3, 4, 5];  // fixed-size array
    let same: [i32; 3] = [0; 3];          // repeat N times

    let st: &str = "slice of string";           // string slice, UTF-8
    let owned: String = String::from("heap");  // owned heap string

    let maybe: Option<i32> = Some(42);  // optional value
    let nada: Option<i32> = None;

    let _ok: Result<i32, &str> = Ok(42);   // success
    let _err: Result<i32, &str> = Err("oops");  // error

    let _ = (x, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, counter, MAX_HP, q, r, s, first, arr, same, st, owned, maybe, nada);
}
