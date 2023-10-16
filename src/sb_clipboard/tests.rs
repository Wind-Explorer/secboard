#[test]
fn test_clipboard_io() {
    let data = "hello world qwq";
    crate::sb_clipboard::io::write(data).unwrap();
    let result = crate::sb_clipboard::io::read().unwrap();
    assert_eq!(result, data);
}
