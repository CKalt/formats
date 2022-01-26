fn main() {
    let num = 778i32;
    println!("num = {:x^width$}", num, width = 10);

    let how_wide = 30;
    println!("num = {:.>how_wide$}", num);

    let num1 = 778i32;
    let num2 = 123432;
    println!("num1={:.<num1_width$} num2={:x>num2_width$}",
        num1, num2, num1_width = 20, num2_width = 30);
}
