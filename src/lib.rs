use std::{fs, process};

use arboard::Clipboard;

fn copy_to_clipboard(contents: &str) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(contents).unwrap();
}

pub fn run(mut args: impl Iterator<Item = String>) {
    args.next();

    let filepath = args.next().unwrap_or_else(|| {
        println!("ggcat; A simple tool to read text file contents to clipboard\n\n Usage: ggcat <filepath>");
        process::exit(1);
    });

    let contents = fs::read_to_string(filepath)
        .expect("Unable to read file!! Make sure the filepath is valid");

    copy_to_clipboard(&contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.clear().unwrap();

        let args = vec!["ggcat".to_string(), "test/1.txt".to_string()].into_iter();
        run(args);
        let clipboard_contents = clipboard.get_text().unwrap();
        assert_eq!(clipboard_contents, "This is one");

        let args = vec!["ggcat".to_string(), "test/2.txt".to_string()].into_iter();
        run(args);
        let clipboard_contents = clipboard.get_text().unwrap();
        assert_eq!(clipboard_contents, "This is two");
    }
}
