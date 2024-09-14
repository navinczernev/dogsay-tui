use cursive::views::{Dialog};
use cursive::event::Key;

fn main() {
    let mut siv = cursive::default();
    let dog_text = "Woof!
\\
 \\
  /^ ^\\
 / 0 0 \\
 V\\ Y /V
  / - \\
  |    \\
  || (__V";

    siv.add_layer(
        Dialog::text(dog_text).button("OK", |s| s.quit())
    );
    siv.add_global_callback(Key::Esc, |s| s.quit());

    siv.run();
}
