extern crate gtk;
extern crate chrono;

use gtk::prelude::*;
use gtk::{Builder, Window};
use chrono::prelude::*;
use std::fs::File;
use std::io::{Read, Write};


fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let glade_src = include_str!("../resources/app.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.get_object("window").expect("Couldn't get window1");
    let calendar: gtk::Calendar = builder.get_object("calendar").expect("Couldn't get calendar");
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Connect the "activate" signal of the "Help > About" menu item
    let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").expect("Couldn't get about_dialog");
    let menu_about: gtk::MenuItem = builder.get_object("menu_about").expect("Couldn't get menu_about");
    let text_view: gtk::TextView = builder.get_object("text_view").expect("Couldn't get text_view");
    let text_view_clone = text_view.clone();
    menu_about.connect_activate(move |_| {
        about_dialog.run();
        about_dialog.hide();
    });

    // Set the date of the calendar to today's date
    let today = Local::today();
    
    let (year, month, day) = (today.year() as u32, today.month() as u32 + 1, today.day() as u32);
    calendar.select_month(month, year);
    calendar.select_day(day);


    // Connect the "day-selected" signal of the calendar
    calendar.connect_day_selected(move |calendar| {
        // let (year, month, day) = calendar.get_date();
        // let selected_date = NaiveDate::from_ymd(year as i32, month as u32, day as u32);
        let selected_date = get_selected_date(&calendar);
        println!("Selected date: {}", selected_date.format("%Y-%m-%d"));
        
        read_diary(&text_view, selected_date);
    });

    let text_buffer = text_view_clone.get_buffer().expect("Couldn't get text buffer");
    // Connect the "changed" signal of the text view
    text_buffer.connect_changed(move |_| {
        write_diary(&text_view_clone, get_selected_date(&calendar));
    });

    window.show_all();

    gtk::main();
}

fn get_selected_date(calendar: &gtk::Calendar) -> NaiveDate {
    let (year, month, day) = calendar.get_date();
    return NaiveDate::from_ymd(year as i32, month as u32, day as u32);
}

fn read_diary(text_view: &gtk::TextView, date: NaiveDate) {
    let text_buffer = text_view.get_buffer().expect("Couldn't get text buffer");
 
    let (year, month, day) = (date.year() as u32, date.month() as u32 + 1, date.day() as u32);
    let filename = format!("data/{}{}{}", year, month, day);

    match read_data_from_file(&filename) {
        Ok(text) => text_buffer.set_text(&text),
        Err(err) => text_buffer.set_text(""),
    }
}

fn write_diary(text_view: &gtk::TextView, date: NaiveDate) {
    let text_buffer = text_view.get_buffer().expect("Couldn't get text buffer");
    let start_iter = text_buffer.get_start_iter();
    let end_iter = text_buffer.get_end_iter();
    let text_to_save = text_buffer.get_text(&start_iter, &end_iter, false).expect("Text buffer is empty").to_string();


    let (year, month, day) = (date.year() as u32, date.month() as u32 + 1, date.day() as u32);
    let filename = format!("data/{}{}{}", year, month, day);

    save_text_to_file(&filename, &text_to_save).expect("Failed to save text");
    println!("Saved text: {} to {}{}{}", text_to_save, year, month, day);
}

fn save_text_to_file(filename: &str, text: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn read_data_from_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

