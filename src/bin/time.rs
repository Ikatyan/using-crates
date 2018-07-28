extern crate chrono;
extern crate time;

fn print_now() {
    println!("{}", time::now().asctime());
}

fn print_yyyymmdd_with_chrono() {
    println!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
}

fn main() {
    print_now();
    print_now_with_chrono()
}