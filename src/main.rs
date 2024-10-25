use aoc_toolkit::App;
mod y2023;
use y2023::*;

fn main() {
    let mut app = App::new();
    let key = std::env::args().nth(1).unwrap();

    app.add_day("2023-01".into(), d01::D1 {}, "inputs/1/input".into());
    app.add_day("2023-02".into(), d02::D2 {}, "inputs/2/input".into());
    app.add_day("2023-03".into(), d03::D3 {}, "inputs/3/input".into());
    app.add_day("2023-04".into(), d04::D4 {}, "inputs/4/input".into());
    app.add_day("2023-05".into(), d05::D5 {}, "inputs/5/input".into());
    app.add_day("2023-06".into(), d06::D6 {}, "inputs/6/input".into());
    app.add_day("2023-07".into(), d07::D7 {}, "inputs/7/input".into());
    app.add_day("2023-08".into(), d08::D8 {}, "inputs/8/input".into());
    app.add_day("2023-09".into(), d09::D9 {}, "inputs/9/input".into());
    app.add_day("2023-10".into(), d10::D10 {}, "inputs/10/input".into());
    app.add_day("2023-11".into(), d11::D11 {}, "inputs/11/input".into());
    app.add_day("2023-12".into(), d12::D12 {}, "inputs/12/input".into());
    app.add_day("2023-13".into(), d13::D13 {}, "inputs/13/input".into());
    app.add_day("2023-14".into(), d14::D14 {}, "inputs/14/input".into());
    app.add_day("2023-15".into(), d15::D15 {}, "inputs/15/input".into());
    app.add_day("2023-16".into(), d16::D16 {}, "inputs/16/input".into());
    app.add_day("2023-17".into(), d17::D17 {}, "inputs/17/input".into());

    app.run(key);
}
