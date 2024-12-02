mod days;

fn main() {
    let d1path = "src/inputs/i1";
    let r1 = days::day1::get_res(d1path);
    println!("{:?}", r1);

    let d2path = "src/inputs/i2";
    let r2 = days::day2::get_res(d2path);
    println!("{:?}", r2);
}
