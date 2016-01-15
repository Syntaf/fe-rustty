# ruik: a rustty UI kit

[![Build Status](https://travis-ci.org/Syntaf/ruik.svg?branch=widget_redesign)](https://travis-ci.org/Syntaf/ruik)

- [API Documentation][1]
- [Intro](#intro)
- [Installation](#installation)
- [Concepts](#concepts)
	- [Terminal](#terminal)
	- [Cells](#cells)
	- [Events](#events)
- [Usage Guide](#usage-guide)
- [Contact](#contact)

## Intro

ruik is a fork of [rustty](https://github.com/cpjreynolds/rustty) that provides a widget based API for building terminal UI applications.

-------------

Ruik's backend, Rustty, is based on the concepts of cells and events. A terminal display is an array of cells,
each holding a character and a set of foreground and background styles. Events are how a
terminal communicates changes in its state; events are received from a terminal, processed, and
pushed onto an input stream to be read and responded to.

## Installation

To use `ruik` within your project, first add this to your `Cargo.toml`:

```toml
[dependencies]
ruik = { git = "https://github.com/Syntaf/ruik.git" }
```

Then, add this to your crate root:

```rust
extern crate ruik;
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

## Usage Guide

Examples and usage suggestions can be found in the [API
documentation][1].

## Contact

[email me](mailto:syntaf@gmail.com)
I am able.

[1]: http://syntaf.github.io/ruik
