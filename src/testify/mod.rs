use std::time::Instant;

pub fn exec_mes(name: &str, f: fn() -> String) {
    let now = Instant::now();
    let res = f();
    println!("[{}] elapsed time: {:.2?}", name, now.elapsed());
    assert_ne!(res, "");
    if print() {
        println!("\t -> value of fn: {}", res);
    }
}

fn print() -> bool {
    true
}