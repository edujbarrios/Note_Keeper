extern crate gtk;

use gtk::prelude::*;
use std::fs::{self, File};
use std::path::Path;

fn main() {
    gtk::init().expect("GTK init failed");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Note App");
    window.set_default_size(400, 400);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.override_background_color(gtk::StateFlags::NORMAL, &gdk::RGBA {
        red: 0.9,
        green: 0.9,
        blue: 0.9,
        alpha: 1.0,
    });
    window.add(&vbox);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    hbox.set_border_width(5);
    vbox.pack_start(&hbox, false, false, 0);

    let create_button = gtk::Button::new_with_label("Crear Nota");
    create_button.connect_clicked(move |_| {
        let title_label = gtk::Label::new(Some("Título:"));
        vbox.pack_start(&title_label, false, false, 0);

        let title_entry = gtk::Entry::new();
        vbox.pack_start(&title_entry, false, false, 0);

        let note_label = gtk::Label::new(Some("Nota:"));
        vbox.pack_start(&note_label, false, false, 0);

        let note_text = gtk::TextView::new();
        vbox.pack_start(&note_text, false, false, 0);

        let save_button = gtk::Button::new_with_label("Guardar");
        save_button.connect_clicked(move |_| {
            let title = title_entry.get_text().unwrap();
            let buffer = note_text.get_buffer().unwrap();
            let start = buffer.get_start_iter();
            let end = buffer.get_end_iter();
            let note = buffer.get_text(&start, &end, true).unwrap();

            let path = Path::new("notas").join(format!("{}.txt", title));
            let file = File::create(path).expect("Error al crear el archivo");
            write!(file, "{}", note).expect("Error al escribir en el archivo");

            title_entry.set_text("");
            buffer.set_text("");
        });
        vbox.pack_start(&save_button, false, false, 0);
    });
    hbox.pack_start(&create_button, false, false, 0);

    let show_button = gtk::Button::new_with_label("Mostrar Notas");
    show_button.connect_clicked(move |_| {
    let notes_folder = Path::new("notas");
    if !notes_folder.exists() {
    fs::create_dir(notes_folder).expect("Error al crear la carpeta");
    }

  for entry in fs::read_dir(notes_folder).expect("Error al leer la carpeta") {
        let entry = entry.expect("Error al leer el archivo");
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let title = path.file_stem().unwrap().to_str().unwrap();
        let file = File::open(path).expect("Error al abrir el archivo");
        let note = std::io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("Error al leer la línea"))
            .collect::<Vec<_>>()
            .join("\n");

        let note_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        note_box.set_border_width(5);
        vbox.pack_start(&note_box, false, false, 0);

        let title_label = gtk::Label::new(Some(title));
        note_box.pack_start(&title_label, false, false, 0);

        let note_label = gtk::Label::new(Some(&note));
        note_box.pack_start(&note_label, false, false, 0);
    }
});
hbox.pack_start(&show_button, false, false, 0);

window.connect_delete_event(|_, _| {
    gtk::main_quit();
    Inhibit(false)
});
window.show_all();

gtk::main();
}
