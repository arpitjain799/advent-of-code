use std::io::Write;

use crate::{Coordinate, SvgColor, SvgElement, SvgId};

#[derive(Default)]
pub struct SvgPath {
    pub shape: SvgPathShape,
    pub stroke: Option<SvgColor>,
    pub stroke_width: Option<f64>,
    pub fill: Option<SvgColor>,
}

enum SvgPathElement {
    LineAbsolute((Coordinate, Coordinate)),
    LineRelative((Coordinate, Coordinate)),
    MoveAbsolute((Coordinate, Coordinate)),
    MoveRelative((Coordinate, Coordinate)),
    /// The "Close Path" command, called with Z. This command draws a straight line from the current
    /// position back to the first point of the path. It is often placed at the end of a path node,
    /// although not always
    ///
    /// The SVG syntax for this is 'z' or 'Z'.
    Close,
}

impl From<SvgPath> for SvgElement {
    fn from(value: SvgPath) -> Self {
        Self::Path(value)
    }
}

impl SvgPath {
    pub const fn stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = Some(width);
        self
    }

    pub const fn stroke(mut self, color: SvgColor) -> Self {
        self.stroke = Some(color);
        self
    }

    pub(crate) fn write<W: Write>(&self, id: Option<SvgId>, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        writer.write_all(b"<path").unwrap();
        if let Some(id) = id {
            id.write(writer);
        }
        if let Some(stroke) = &self.stroke {
            stroke.write_stroke(writer);
        }
        if let Some(stroke_width) = &self.stroke_width {
            writer
                .write_all(format!(" stroke-width=\"{}\"", stroke_width).as_bytes())
                .unwrap();
        }
        if let Some(fill) = &self.fill {
            fill.write_fill(writer);
        }
        writer.write_all(b" d=\"").unwrap();
        self.shape.write(writer);
        writer.write_all(b"\"").unwrap();
        writer.write_all(b"/>\n").unwrap();
    }
}

#[derive(Default)]
pub struct SvgPathShape {
    elements: Vec<SvgPathElement>,
}

impl SvgPathShape {
    pub const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn at(x: Coordinate, y: Coordinate) -> Self {
        Self {
            elements: vec![SvgPathElement::MoveAbsolute((x, y))],
        }
    }

    pub fn is_empty(&self) -> bool {
        // If it contains the single initial move command.
        self.elements.is_empty()
    }

    pub fn line_to_absolute(mut self, x: Coordinate, y: Coordinate) -> Self {
        self.elements.push(SvgPathElement::LineAbsolute((x, y)));
        self
    }

    pub fn line_to_relative<C: Into<Coordinate>>(mut self, x: C, y: C) -> Self {
        self.elements
            .push(SvgPathElement::LineRelative((x.into(), y.into())));
        self
    }

    pub fn move_to_absolute<C: Into<Coordinate>>(mut self, x: C, y: C) -> Self {
        self.elements
            .push(SvgPathElement::MoveAbsolute((x.into(), y.into())));
        self
    }

    pub fn move_to_relative(mut self, x: Coordinate, y: Coordinate) -> Self {
        self.elements.push(SvgPathElement::MoveRelative((x, y)));
        self
    }

    pub fn close(mut self) -> Self {
        self.elements.push(SvgPathElement::Close);
        self
    }

    pub fn data_string(&self) -> String {
        #![allow(clippy::unwrap_used)]
        let mut buffer = Vec::new();
        self.write(&mut buffer);
        String::from_utf8(buffer).unwrap()
    }

    pub(crate) fn write<W: Write>(&self, writer: &mut W) {
        #![allow(clippy::unwrap_used)]
        for element in &self.elements {
            match element {
                SvgPathElement::MoveAbsolute((x, y)) => {
                    writer
                        .write_all(format!("M {} {}", x, y).as_bytes())
                        .unwrap();
                }
                SvgPathElement::MoveRelative((x, y)) => {
                    writer
                        .write_all(format!("m {} {}", x, y).as_bytes())
                        .unwrap();
                }
                SvgPathElement::LineAbsolute((x, y)) => {
                    writer
                        .write_all(format!("L {} {}", x, y).as_bytes())
                        .unwrap();
                }
                SvgPathElement::LineRelative((x, y)) => {
                    writer
                        .write_all(format!("l {} {}", x, y).as_bytes())
                        .unwrap();
                }
                SvgPathElement::Close => {
                    writer.write_all(b"Z").unwrap();
                }
            }
        }
    }
}
