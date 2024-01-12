fn main() {
    convert_f_c();
}

fn convert_f_c() {
    let tem_c: f32 = 37.0;
    let tem_f: f32 = 98.6;
    let c = convert(tem_f, true);
    let f = convert(tem_c, false);
    println!("C{tem_c} is {f} in F");
    println!("F{tem_f} is {c} in C");
}

fn convert(tem: f32, to_c: bool) -> f32 {
    let x = if to_c {
        5.0 * (tem - 32.0) / 9.0
    } else {
        9.0 * tem / 5.0 + 32.0
    };
    return x
}