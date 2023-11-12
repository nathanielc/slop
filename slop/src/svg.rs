use std::cmp::{max, min};

use crate::semantic::{self, Ingredient, Operand, Recipe};
use svg::node::element::{Group, Rectangle, Style, TSpan, Text};
use svg::node::Text as RawText;
use svg::Document;

pub fn to_svgs(src: &semantic::SourceFile) -> Vec<String> {
    src.recipes.iter().map(recipe_to_svg).collect()
}

// Draw a recipe card as an SVG.
//
// Basic algorithm:
//
// Everything is a rectangle, how the rectangles intersect draws the
// complete card.
//
// When drawing a rectangle for an ingredient or an operator
// there are two aspects, first the rectangle that encloses
// all depedencies, second the rectangle that encloses the text
// of the current element. The rectangle that is drawn is the
// rectangle the encloses both of these rectangle.
//
// The text rectangle is different depending on ingredient or operator.
// For ingredients the width is static and known but the height is variable.
// Its the opposite for operators.
//
// Walking the recipe once is sufficient to draw all the rectangles.
// First we traverse depth first and draw the rectangles as we pop
// back up the tree.
fn recipe_to_svg(r: &semantic::Recipe) -> String {
    let doc = build_doc(r);

    let mut out: Vec<u8> = Vec::new();

    svg::write(&mut out, &doc).unwrap();
    String::from_utf8(out).expect("svg should be valid utf-8")
}

#[derive(Clone, Debug, Default)]
struct Point {
    x: usize,
    y: usize,
}
impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Default)]
struct BoundingBox {
    upper_left: Point,
    bottom_right: Point,
}

impl BoundingBox {
    fn height(&self) -> usize {
        self.bottom_right.y - self.upper_left.y
    }
    fn width(&self) -> usize {
        self.bottom_right.x - self.upper_left.x
    }
    fn enclose(self, other: Self) -> Self {
        Self {
            upper_left: Point {
                x: min(self.upper_left.x, other.upper_left.x),
                y: min(self.upper_left.y, other.upper_left.y),
            },
            bottom_right: Point {
                x: max(self.bottom_right.x, other.bottom_right.x),
                y: max(self.bottom_right.y, other.bottom_right.y),
            },
        }
    }
}

// all constants in pixels, based off a 16pt monospace font
const CHAR_WIDTH: usize = 10;
const LINE_HEIGHT: usize = 20;
const INGREDIENT_WIDTH: usize = 200; // 20 characters
const OPERATOR_WIDTH: usize = 150; // 15 characters
const WIDTH_INCREMENT: usize = 10; // 1 characters
const X_MARGIN: usize = 5;
const Y_MARGIN: usize = 5;

struct Builder {
    bottom: usize,
    max_width: usize,
}

impl Builder {
    fn build_title(&mut self, r: &Recipe) -> Option<Text> {
        if let Some(ref title) = r.title {
            let y = self.bottom + Y_MARGIN + LINE_HEIGHT + 2;
            self.bottom = y + Y_MARGIN;
            Some(
                Text::new()
                    .add(RawText::new(title))
                    .set("font-style", "bold")
                    .set("font-size", "18px")
                    .set("x", X_MARGIN)
                    .set("y", y),
            )
        } else {
            None
        }
    }
    fn build_preamble(&mut self, r: &Recipe) -> Option<Text> {
        if let Some(ref preamble) = r.preamble {
            let y = self.bottom + LINE_HEIGHT + Y_MARGIN;
            self.bottom = y + Y_MARGIN;
            Some(
                Text::new()
                    .add(RawText::new(preamble))
                    .set("x", X_MARGIN)
                    .set("y", y),
            )
        } else {
            None
        }
    }
    fn build_comment(&mut self, r: &Recipe) -> Option<Text> {
        if let Some(ref comment) = r.comment {
            render_text(
                comment,
                Point {
                    x: 0,
                    y: self.bottom,
                },
                usize::max_value(),
                self.max_width,
                0,
                false,
            );
            let y = self.bottom + LINE_HEIGHT + Y_MARGIN;
            self.bottom = y + Y_MARGIN;
            Some(
                Text::new()
                    .add(RawText::new(comment))
                    .set("x", X_MARGIN)
                    .set("y", y),
            )
        } else {
            None
        }
    }
    fn build_operand(&mut self, op: &Operand) -> (Group, BoundingBox) {
        let (mut g, b) = match op {
            Operand::Ingredient(i) => {
                let text = ingredient_text(i);
                let (t, mut txt_bounds) = render_text(
                    text.as_str(),
                    Point {
                        x: 0,
                        y: self.bottom,
                    },
                    usize::max_value(),
                    INGREDIENT_WIDTH,
                    0,
                    false,
                );
                txt_bounds.bottom_right.x = max(txt_bounds.bottom_right.x, INGREDIENT_WIDTH);
                self.bottom = txt_bounds.bottom_right.y;
                (Group::new().add(t), txt_bounds)
            }
            Operand::Operator { text, operands, .. } => {
                let mut g = Group::new();
                let mut b: Option<BoundingBox> = None;
                for op in operands {
                    let (grp, bounds) = self.build_operand(op);
                    g = g.add(grp);
                    if let Some(bs) = b {
                        b = Some(bs.enclose(bounds));
                    } else {
                        b = Some(bounds);
                    }
                }
                let mut bounds = b.unwrap();
                let (txt_grp, txt_bounds) = render_text(
                    text,
                    Point {
                        x: bounds.bottom_right.x,
                        y: bounds.upper_left.y,
                    },
                    bounds.height(),
                    OPERATOR_WIDTH,
                    WIDTH_INCREMENT,
                    true,
                );
                bounds = bounds.enclose(txt_bounds);
                g = g.add(txt_grp);
                (g, bounds)
            }
            Operand::MissingOperand { position } => {
                self.build_operand(&Operand::Ingredient(Ingredient {
                    position: position.clone(),
                    derived: false,
                    quantities: Default::default(),
                    unit: None,
                    text: "*MISSING*".to_string(),
                }))
            }
            Operand::UnusedOperands { position, operands } => {
                self.build_operand(&Operand::Operator {
                    position: position.clone(),
                    text: "*UNUSED*".to_string(),
                    operands: operands.clone(),
                })
            }
        };

        self.max_width = max(self.max_width, b.bottom_right.y);

        g = g.add(
            Rectangle::new()
                .set("x", b.upper_left.x)
                .set("y", b.upper_left.y)
                .set("height", b.height())
                .set("width", b.width()),
        );
        (g, b)
    }
}

fn build_doc(r: &semantic::Recipe) -> Document {
    let mut builder = Builder {
        bottom: 0,
        max_width: 0,
    };
    let mut doc = Document::new().add(Style::new(
        r#"text {
    font-family: monospace;
    font-size: 16px;
}
rect {
    stroke: black;
    stroke-width: 1;
    fill: green;
    fill-opacity: 0;
    stroke-opacity: 1;
}"#,
    ));
    if let Some(title) = builder.build_title(r) {
        doc = doc.add(title);
    }
    if let Some(preamble) = builder.build_preamble(r) {
        doc = doc.add(preamble);
    }

    let (op_doc, mut bound) = builder.build_operand(&r.root);
    doc = doc.add(op_doc);

    if let Some(comment) = builder.build_comment(r) {
        doc = doc.add(comment);
        // Add room for comment line
        // TODO: Render text to max width of enitre card
        bound.bottom_right.y += LINE_HEIGHT + Y_MARGIN * 2;
    }
    doc.set("width", bound.bottom_right.x + X_MARGIN * 2)
        .set("height", bound.bottom_right.y + Y_MARGIN * 2)
}

// format the complete text for an ingredient
fn ingredient_text(i: &Ingredient) -> String {
    let derived = if i.derived { "^" } else { "" };
    match (i.quantities.as_ref(), i.unit.as_ref()) {
        (Some(q), Some(u)) => format!("{}{} {} {}", derived, q.0, u, i.text),
        (Some(q), None) => format!("{}{} {}", derived, q.0, i.text),
        _ => format!("{}{}", derived, i.text),
    }
}
fn render_text(
    text: &str,
    upper_left: Point,
    max_height: usize,
    width: usize,
    width_increment: usize,
    center: bool,
) -> (Text, BoundingBox) {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    let mut max_line_width = 0;
    let mut height = Y_MARGIN * 2;
    for word in text.split_whitespace() {
        if (line.len() + word.len() + 1) * CHAR_WIDTH < width {
            line.push_str(word);
            line.push(' ');
        } else {
            if !line.is_empty() {
                let line_width = line.len() * CHAR_WIDTH;
                if line_width > max_line_width {
                    max_line_width = line_width;
                }
                lines.push(line);
                height += LINE_HEIGHT;
                if height > max_height {
                    // Text doesn't fit in the required height,
                    // increase width and try again
                    return render_text(
                        text,
                        upper_left,
                        max_height,
                        width + width_increment,
                        width_increment,
                        center,
                    );
                }
            }
            line = String::new();
            line.push_str(word);
            line.push(' ');
        }
    }
    if !line.is_empty() {
        let line_width = line.len() * CHAR_WIDTH;
        if line_width > max_line_width {
            max_line_width = line_width;
        }
        lines.push(line);
        height += LINE_HEIGHT;
        if height > max_height {
            // Text doesn't fit in the required height,
            // increase width and try again
            return render_text(
                text,
                upper_left,
                max_height,
                width + width_increment,
                width_increment,
                center,
            );
        }
    }
    let bottom_right = upper_left.clone()
        + Point {
            x: max_line_width + X_MARGIN * 2,
            y: height,
        };
    let bounds = BoundingBox {
        upper_left,
        bottom_right,
    };

    let dy = if center { (max_height - height) / 2 } else { 0 };

    let mut text_node = Text::new()
        .set("x", bounds.upper_left.x)
        .set("y", bounds.upper_left.y + dy);
    for line in lines {
        text_node = text_node.add(
            TSpan::new()
                .set("x", bounds.upper_left.x + X_MARGIN)
                .set("dy", LINE_HEIGHT)
                .add(RawText::new(line)),
        );
    }

    (text_node, bounds)
}
