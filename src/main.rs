use std::fs;
use clap::Parser;

use gtk::{prelude::*, Window, WindowType};
use webkit2gtk::{WebContext,  WebView, WebViewExt};
use gtk::Button;
//use gtk::{prelude::*, Switch, Align};
//use gtk::{Application, ApplicationWindow};

use std::rc::Rc;
use std::cell::RefCell;

mod style;
use crate::style::DARK_STYLE;

fn build_ui(md_file: Option<String>, light: bool) -> Window {
    let window = Window::new(WindowType::Toplevel);

    window.set_default_size(1080, 1080);
    
    let context = WebContext::default().unwrap();

    let webview = Rc::new(RefCell::new(
        WebView::builder()
            .web_context(&context)
            .build()
    ));

    webview.borrow().set_hexpand(true);
    webview.borrow().set_halign(gtk::Align::Fill);
    webview.borrow().set_vexpand(true);
    webview.borrow().set_valign(gtk::Align::Fill);

    // webview.load_uri("https://crates.io/");
    let markdown: String = match md_file {
        Some(file) => {
            fs::read_to_string(file).expect("failed to read file")
        }
        None => {
            "# lol".to_string()
        }
    };
    let mut html: String = markdown::to_html(&markdown);
    if !light {
        html = format!("<html><head><style>{DARK_STYLE}</style></head><body>{html}</body></html>");
    }
    webview.borrow().load_html(html.as_str(), None);

    //let settings = WebViewExt::settings(&webview.clone().take()).unwrap();
    //settings.set_enable_developer_extras(true);

    /*let inspector = webview.get_inspector().unwrap();
    inspector.show();*/

    let file_chooser = gtk::FileChooserButton::builder()
        .title("select markdown file")
        .build();

    let back_button = Button::builder()
        .label("< Back")
        .margin(12)
        .build();

    let forward_button = Button::builder()
        .label("Forward >")
        .margin(12)
        .build();

    let gtk_box = gtk::Box::builder()
        .margin(0)
        .valign(gtk::Align::Fill)
        .halign(gtk::Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .spacing(12)
        .orientation(gtk::Orientation::Vertical)
        .build();

    let gtk_grid = gtk::Grid::builder()
        .margin_bottom(12)
        .hexpand(true)
        .vexpand(false)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .column_spacing(12)
        .orientation(gtk::Orientation::Horizontal)
        .build();

    gtk_box.add(&webview.borrow().clone());
    gtk_box.add(&gtk_grid);
    gtk_grid.add(&back_button);
    gtk_grid.add(&file_chooser);
    gtk_grid.add(&forward_button);
    window.add(&gtk_box);

    window.show_all();

    // webview.run_javascript("alert('Hello');", None::<&gio::Cancellable>, |_result| {});

    let webview_clone = webview.clone();
    back_button.connect_clicked(move |_| {
        println!("button");
        webview_clone.borrow_mut().run_javascript("history.back();", None::<&gtk::gio::Cancellable>, |_result| {});
    });

    let webview_clone = webview.clone();
    forward_button.connect_clicked(move |_| {
        println!("button");
        webview_clone.borrow_mut().run_javascript("history.forward();", None::<&gtk::gio::Cancellable>, |result| {println!("{:#?}", result.unwrap());});
    });

    let webview_clone = webview.clone();
    file_chooser.connect_file_set(move |idk| {
        let file_path = idk.file().expect("file error").path().unwrap();
        let markdown_text = fs::read_to_string(file_path).expect("failed to read file");
        let mut html_idk = markdown::to_html(&markdown_text);
        if !light {
            html_idk = format!("<html><head><style>{DARK_STYLE}</style></head><body>{html_idk}</body></html>");
        }
        webview_clone.borrow_mut().load_html(&html_idk, None);
    });

    window.connect_delete_event(|_, _| {
      gtk::main_quit();
      gtk::glib::Propagation::Proceed
    });

    window
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    pub file: Option<String>,

    #[arg(short, long)]
    pub light: bool,
}

fn main() {
    let args = Args::parse();

    gtk::init().unwrap();

    let _window = build_ui(args.file, args.light);

    gtk::main();
}

