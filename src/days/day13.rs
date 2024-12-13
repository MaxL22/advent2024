use good_lp::{constraint, default_solver, variables, Solution, SolverModel};

extern crate good_lp;

#[derive(Clone, Debug)]
struct Claw {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

impl Claw {
    fn new(a: (f64, f64), b: (f64, f64), prize: (f64, f64)) -> Self {
        Self { a, b, prize }
    }
}

pub fn get_res(path: &str) -> (f64, f64) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut count = (0.0, 0.0);

    let mut all_prizes: Vec<Claw> = Vec::new();

    let mut tmp = Claw::new((0.0, 0.0), (0.0, 0.0), (0.0, 0.0));
    let mut counter = 1;

    // Generate vec of Claws
    for line in file.lines() {
        if line.trim().is_empty() {
            all_prizes.push(tmp.clone());
            counter = 1;
            continue;
        }

        let v: Vec<&str> = line.split(&['+', ',', '=']).collect();
        let vals: (f64, f64) = (v[1].parse().unwrap(), v[3].parse().unwrap());

        match counter {
            1 => {
                tmp.a.0 = vals.0;
                tmp.a.1 = vals.1;
            }
            2 => {
                tmp.b.0 = vals.0;
                tmp.b.1 = vals.1;
            }
            3 => {
                tmp.prize.0 = vals.0;
                tmp.prize.1 = vals.1;
            }
            _ => {}
        }

        counter += 1;
    }

    // Part 1
    /*
     * I need to solve the equations
     * prize.0 = A*a.0 + B*b.0
     * prize.1 = A*a.1 + B*b.1
     */

    for el in all_prizes.iter() {
        variables! {
            vars:
            0 <= a (integer) <= 100;
            0 <= b (integer) <= 100;
        }

        let c1 = constraint!(a * el.a.0 + b * el.b.0 == el.prize.0);
        let c2 = constraint!(a * el.a.1 + b * el.b.1 == el.prize.1);

        let sol = vars
            .minimise(3 * a + b)
            .using(default_solver)
            .with(c1)
            .with(c2)
            .solve();

        let mut v = (0.0, 0.0);

        if let Ok(t) = sol {
            v.0 = t.value(a);
            v.1 = t.value(b);
        }
        count.0 += v.0 * 3.0 + v.1;
    }

    // Part 2
    // Same thing, different boundaries?
    // Wrong? Idk why though, it should be right

    for el in all_prizes.iter_mut() {
        el.prize.0 += 10000000000000.0;
        el.prize.1 += 10000000000000.0;
    }

    for el in all_prizes.iter() {
        variables! {
            vars:
            a (integer);
            b (integer);
        }

        let c1 = constraint!(a * el.a.0 + b * el.b.0 == el.prize.0);
        let c2 = constraint!(a * el.a.1 + b * el.b.1 == el.prize.1);

        let sol = vars
            .minimise(3 * a + b)
            .using(default_solver)
            .with(c1)
            .with(c2)
            .solve();

        let mut v = (0.0, 0.0);

        if let Ok(t) = sol {
            v.0 = t.value(a);
            v.1 = t.value(b);
        }

        count.1 += v.0 * 3.0 + v.1;
    }

    count
}
// 32026, I just needed to keep the floats? Wtf
// 76412569284706 too low?
