mod message_streaming;

fn main() {
    let msg = "Hello, World!";
    let output = message_streaming::streaming(msg);
    println!("{}", output)
}


