use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, Orientation};
use std::process::Command;

fn main() {
    let app = Application::new(
        Some("com.spacehowen.xamppcontrol"),
        Default::default(),
    );

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("XAMPP Control");
    window.set_default_size(300, 200);
    window.set_resizable(false);

    let vbox = Box::new(Orientation::Vertical, 10);
    window.add(&vbox);

    let status_label = Label::new(Some("Estado: Esperando acción..."));
    status_label.set_justify(gtk::Justification::Center);
    vbox.pack_start(&status_label, false, false, 10);

    let start_button = Button::with_label("Iniciar XAMPP");
    vbox.pack_start(&start_button, true, true, 0);
    let status_label_clone = status_label.clone();
    start_button.connect_clicked(move |_| {
        run_command("/opt/lampp/lampp start", &status_label_clone, "XAMPP iniciado correctamente.");
    });

    let stop_button = Button::with_label("Detener XAMPP");
    vbox.pack_start(&stop_button, true, true, 0);
    let status_label_clone = status_label.clone();
    stop_button.connect_clicked(move |_| {
        run_command("/opt/lampp/lampp stop", &status_label_clone, "XAMPP detenido correctamente.");
    });

    let restart_button = Button::with_label("Reiniciar XAMPP");
    vbox.pack_start(&restart_button, true, true, 0);
    let status_label_clone = status_label.clone();
    restart_button.connect_clicked(move |_| {
        run_command("/opt/lampp/lampp restart", &status_label_clone, "XAMPP reiniciado correctamente.");
    });

    window.show_all();
}

fn run_command(command: &str, status_label: &Label, success_message: &str) {
    use std::path::Path;

    if !Path::new("/opt/lampp/lampp").exists() {
        update_status(status_label, "Error: XAMPP no está instalado.", "red");
        return;
    }

    let full_command = format!("pkexec {}", command);
    match Command::new("sh")
        .arg("-c")
        .arg(&full_command)
        .output()
    {
        Ok(output) if output.status.success() => {
            update_status(status_label, success_message, "green");
        }
        Ok(output) => {
            let error_message = String::from_utf8_lossy(&output.stderr);
            update_status(
                status_label,
                &format!("Error: {}", error_message),
                "red",
            );
        }
        Err(e) => {
            update_status(
                status_label,
                &format!("Error al ejecutar el comando: {}", e),
                "red",
            );
        }
    }
}

fn update_status(status_label: &Label, message: &str, color: &str) {
    let styled_message = format!("<span foreground='{}'><b>{}</b></span>", color, message);
    status_label.set_markup(&styled_message);
}
