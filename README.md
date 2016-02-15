# fe-rustty: rustty with widgets

[![Build Status](https://travis-ci.org/Syntaf/fe-rustty.svg?branch=master)](https://travis-ci.org/Syntaf/fe-rustty)

- [API Documentation][1]
- [Intro](#intro)
- [Installation](#installation)
- [Concepts](#concepts)
	- [Terminal](#terminal)
	- [Cells](#cells)
	- [Events](#events)
	- [Widgets](#widgets)
- [Usage Guide](#usage-guide)
- [Contact](#contact)

## Intro

fe is a fork of [rustty](https://github.com/cpjreynolds/rustty) that provides a widget based API for building terminal UI applications. fe only aims at a better implementation of rustty's `ui` module, the backend will always remain the same
.
-------------

fe's backend, Rustty, is based on the concepts of cells and events. A terminal display is an array of cells,
each holding a character and a set of foreground and background styles. Events are how a
terminal communicates changes in its state; events are received from a terminal, processed, and
pushed onto an input stream to be read and responded to.

## Installation

To use `fe-rustty` within your project, first add this to your `Cargo.toml`:

```toml
[dependencies]
rustty = { git = "https://github.com/Syntaf/fe-rustty.git" }
```

Then, add this to your crate root:

```rust
extern crate rustty;
```

### Terminal

The terminal representation can be thought of as such:

```
0-------------------------------cols (x)
|
|
|
|
|
|
|
|
rows (y)
```

Along the x-axis are the columns and along the y-axis are the rows. The
upper-left corner is the origin, which begins at index (0, 0) and extends to
(cols, rows). Each point (x, y) represents a single cell, which is the next
topic.

### Cells

A cell is a single point on a character display, representing a single
character and its foreground and background styles.

### Events

Events are how changes in a terminal's state are represented. 
A terminal has an associated event stream which acts much like a UNIX pipe,
or a FIFO queue. When events occur they are pushed on to
the back of the stream; when events are read they are taken
from the front of the stream.

### Widgets


fe has a couple key concepts: generalized widgets and user customization. The design of the widgets are inspired by 
Tkinter and aims to have a similar form and function. fe supplies a basic number of widgets that are useful for 
designing application, but you can easily write your own widgets using the traits available.

To get started with widgets, your most basic and most widely used container will be a `Dialog`. A dialog's job is to
mesh widgets together and act as a aggregator, a widget which *takes in* other widgets for easier management. Creation
of a dialog is simple:

```
let mut dlg = Dialog::new(60, 10);	// create dialog 60 columns wide, 10 rows long
dlg.draw_box();				// draw border
```

This dialog will now allow us to aggregate widgets we wish to bundle together, say labels or buttons:


```
let mut b1 = StdButton::new("Quit", 'q', ButtonResult::Ok);		// Create button
b1.pack(&maindlg, HorizontalAlign::Left, VerticalAlign::Bottom, (4,2));	// Align button within dialog

dlg.add_button(b1);	// dlg now takes ownership of b1
```

Great! now when we want to poll events, we can use dialogs to forward events to our buttons

```
// Poll events
while let Some(Event::Key(ch)) = terminal.get_event(0).unwrap() {
    match dlg.result_for_key(ch) {
        Some(ButtonResult::Ok) => break 'main,
	_ => {},
    }
}
```

Widgets can still function as independent objects, but Dialogs helps bring everything together and let you
encapsulate your data for better abstraction.  

A good way to understand fe widgets are that they are simply specialized frames that *own* an area of cells, 
and perform actions based on that specliazation. At their core, widgets implement a frame and some basic trait 
specialization. Take for example a label:


```
pub struct Label {
    frame: Frame,
    text: Vec<String>
    // ...
}
```

Our widget in this case has a frame, and uses `text` for drawing into that frame. `Frames` simply represent an area 
that a widget owns. Multiple widgets can own the same cells, but each widget is only concered with itself. In the
case of a Label, we want to write text to an area of cells. Like other widgets, this Label can be owned by a dialog,
packed, and drawn to the screen. 

Any widget implements basic functionality: drawing, packing, outlining, resizing, and returning the *frame*. In
most cases the actual widget is the frame, and structs like `Label` or `Button` *wrap* a frame to provide special
functionality.

## Usage Guide

Examples and usage suggestions can be found in the [API
documentation][1].

## Contact

[email me](mailto:syntaf@gmail.com)

[1]: http://syntaf.github.io/ruik
