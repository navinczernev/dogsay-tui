use cursive::traits::Nameable;
use cursive::views::{Checkbox, Dialog, EditView, ListView};
use cursive::Cursive;

struct DogsayOptions<'a> {
    message: &'a str,
    dead: bool,
}

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    .child(
                        "Message: ", EditView::new().with_name("message")
                    )
                    .child(
                        "Dead?", Checkbox::new().with_name("dead")
                    ),
            )
            .button("OK", |s| {
                let message = s.call_on_name(
                    "message", |t: &mut EditView| t.get_content()
                ).unwrap();
                let is_dead = s.call_on_name(
                    "dead", |t: &mut Checkbox| t.is_checked()
                ).unwrap();
                let options = DogsayOptions {
                    message: &message,
                    dead: is_dead,
                };
                result_step(s, &options)
            })
    )
}

fn result_step(siv: &mut Cursive, options: &DogsayOptions) {
    let eye = if options.dead { "x" } else { "0" };
    let dog_text = format!(
        "{msg}
        \\
         \\
          /^ ^\\
         / {eye} {eye} \\
         V\\ Y /V
          / - \\
          |    \\
          || (__V", msg = options.message, eye=eye);
    siv.pop_layer();
    siv.add_layer(
        Dialog::text(dog_text)
        .title("The dog says...")
        .button("OK", |s| s.quit()),
    );
}

fn main() {
    let mut siv = cursive::default();
    input_step(&mut siv);
    siv.run();
}
