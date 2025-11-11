// Stream messages
//
use peanut_butter::message_streaming::streaming;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_streaming() {
        let msg = "Hello, World!";
        let res = streaming(msg);
        assert_eq!(res, "Hello, World!")
    }
}
