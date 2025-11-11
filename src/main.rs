use peanut_butter::message_streaming::streaming;

fn main() {
    let msg: &str = "Hello, World!";
    let output = streaming(msg);
    println!("{}", output)
}
