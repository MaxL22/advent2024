mod days;

fn main() {
    let path = "src/inputs/i1";
    let r = days::day1::get_res(path);
    println!("{:?}", r);

    let path = "src/inputs/i2";
    let r = days::day2::get_res(path);
    println!("{:?}", r);

    let path = "src/inputs/i3";
    let r = days::day3::get_res(path);
    println!("{:?}", r);

    let path = "src/inputs/i4";
    let r = days::day4::get_res(path);
    println!("{:?}", r);

    let path = ("src/inputs/i5.1", "src/inputs/i5.2");
    let r = days::day5::get_res(path);
    println!("{:?}", r);
}
