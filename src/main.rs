mod sb_clipboard;
mod sb_config;
mod sb_crypto;
mod sb_util;

fn main() {
    println!("SecBoard!");
    sb_clipboard::test();
    sb_crypto::test();
    sb_config::test();
    sb_util::test();
}
