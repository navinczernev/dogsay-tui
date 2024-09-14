use cursive::views::TextView;

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
    siv.run();
}
