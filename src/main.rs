use cursive::views::TextView;
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

    siv.add_layer(TextView::new(dog_text));
    siv.add_global_callback(Key::Esc, |s| s.quit());

    siv.run();
}
