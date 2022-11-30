use cursive::{Cursive, CursiveExt};
use cursive::backend::Dummy;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout};


struct Interface {
    name: String
}

struct AppData {
    interfaces: Vec<Interface>,
}

fn main() {
    let mut siv = Cursive::default();
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(LinearLayout::horizontal()
        .child(Dialog::around(
            LinearLayout::vertical()
                .child(Button::new("Add", add_interface_dialog).full_width())
                .full_width()
                .full_height()
        )
            .fixed_width(50)
            .full_height())
        .child(Dialog::new()
            .full_width()
            .full_height())
        .full_screen());
    siv.run();
}

fn add_interface_dialog(s: &mut Cursive) {
    s.add_layer(Dialog::around(LinearLayout::vertical()
        .child(EditView::default()
            .with_name("interface_name"))
        .child(LinearLayout::horizontal()
            .child(DummyView)
            .child(DummyView)
            .child(Button::new("Ok", add_interface))
            .child(Button::new("Cancel", |s| {
                s.pop_layer();
            }))))
        .title("New Interface"));
}


fn add_interface(s: &mut Cursive) {
    s.pop_layer();
    let interface_name =
            s.call_on_name("interface_name", |edit: &mut EditView| {
                edit.get_content()
            }).unwrap();
}
