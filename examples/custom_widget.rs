extern crate rustty;

use rustty::{
    Terminal,
    Event,
    Cell, 
    CellAccessor,
    Color,
    Attr,
    Size,
    HasSize
};

// Imports here are mostly for creating our own button
use rustty::ui::core::{
    Alignable,
    Painter,
    find_accel_char_index,
    Frame,
    Widget,
    Button,
    ButtonResult,
    HorizontalAlign,
    VerticalAlign
};

use rustty::ui::{
    Dialog,
};

// Declare new button, for red text. Structure cloned from stdbutton.rs
struct RedButton {
    frame: Frame,
    accel: char,
    result: ButtonResult,
    text: String
}

impl RedButton {
    // The only thing different about our red button is how we create the text
    fn new(text: &str, accel: char, result: ButtonResult) -> RedButton {
        let s = format!("< {} >", text);
        let width = s.chars().count();
        let mut button = 
            RedButton { frame: Frame::new(width, 1),
                        accel: accel.to_lowercase().next().unwrap_or(accel),
                        result: result,
                        text: s };
        // Print text to label with red cell
        button.frame.printline_with_cell(
            0, 0, &button.text[..],
            Cell::with_style(Color::Default, Color::Red, Attr::Default)
        );
        // Bold the character that acts as the key
        match find_accel_char_index(text, button.accel) {
            Some(i) => {
                button.frame.get_mut(i+2, 0).unwrap().set_attrs(Attr::Bold);
            },
            None => (),
        }
        button
    }
}

// Implement basic widget functions, also copied directly from
// stdbutton.rs
impl Widget for RedButton {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.frame.draw_into(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
            margin: (usize, usize)) {
        self.frame.align(parent, halign, valign, margin);
    }

    fn draw_box(&mut self) {
        self.frame.draw_box();
    }

    fn resize(&mut self, new_size: Size) {
        self.frame.resize(new_size);
    }

    fn frame(&self) -> &Frame {
        &self.frame
    }

    fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }
}

// RedButton is a widget, which implements button
impl Button for RedButton {
    fn accel(&self) -> char {
        self.accel
    }

    fn result(&self) -> ButtonResult {
        self.result
    }
}

fn create_maindlg() -> Dialog
{
    let mut maindlg = Dialog::new(60, 10);
    maindlg.draw_box();

    let mut b1 = RedButton::new("Quit", 'q', ButtonResult::Ok);
    b1.pack(&maindlg, HorizontalAlign::Middle, VerticalAlign::Middle, (4, 2));

    maindlg.add_button(b1);
    maindlg
}

fn main() {
    let mut term = Terminal::new().unwrap();
    let mut maindlg = create_maindlg();
    maindlg.pack(&term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match maindlg.result_for_key(ch) {
                Some(ButtonResult::Ok)  => break 'main,
                _ => {},
            }
        }

        maindlg.draw(&mut term);
        term.swap_buffers().unwrap();
    }
}

