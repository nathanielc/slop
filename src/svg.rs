use std::char;
use std::cmp::{max, min};

use crate::semantic;
use svg::node::element::{Group, Polygon, Rectangle, Text};
use svg::node::Text as RawText;
use svg::Document;

const rect_height: i32 = 25;
const char_width: i32 = 12;
const x_margin: i32 = 20;
const y_margin: i32 = 20;

pub fn to_svg(op: semantic::Operand) ->Vec<u8> {
    let max_ing_width = compute_max_ing_width(&op);
    let doc = build_doc(op, max_ing_width);

    let mut out :Vec<u8>= Vec::new();

    svg::write(&mut out, &doc).unwrap();
    out
}

fn compute_max_ing_width(op: &semantic::Operand) -> i32 {
    match op {
        semantic::Operand::Ingredient { text } => compute_width(text),
        semantic::Operand::Operator { text, operands } => operands
            .iter()
            .fold(0, |acc, op| max(acc, compute_max_ing_width(op))),
    }
}

fn build_doc(op: semantic::Operand, max_ing_width: i32) -> Document {
    let doc = Document::new();
    let mut state = BuildState {
        max_ing_width: max_ing_width,
        ing_count: 0,
    };
    let rstate = add_operand(doc, op, &mut state);
    rstate
        .doc
        .set("width", rstate.x + x_margin * 2)
        .set("height", state.ing_count * rect_height + y_margin * 2)
}

struct BuildState {
    max_ing_width: i32,
    ing_count: i32,
}

struct ReturnState {
    // the document being constructed
    doc: Document,
    // minumum y value of any parent ingredient
    min_y: i32,
    // last x position of bottom right corner
    x: i32,
    // last y position of bottom right corner
    y: i32,
}

// add the operand to the document, return the tuple of
// (new doc, minumum y value of all parent operands, x position of op, y, position of op)
fn add_operand(doc: Document, op: semantic::Operand, state: &mut BuildState) -> ReturnState {
    match op {
        semantic::Operand::Ingredient { text } => {
            state.ing_count = state.ing_count + 1;
            let ing_idx = state.ing_count - 1;
            let x = 0;
            let y = ing_idx * rect_height;
            let w = state.max_ing_width;
            let h = rect_height;
            let doc = doc.add(text_group(&text, x, y, w, h));
            return ReturnState {
                doc: doc,
                min_y: y,
                x: w,
                y: y+rect_height,
            };
        }
        semantic::Operand::Operator { text, mut operands } => {
            let mut corners: Vec<(i32, i32)> = Vec::with_capacity(operands.len());
            let init = add_operand(doc, operands.remove(0), state);
            corners.push((init.x, init.y));
            let rstate = operands.drain(..).fold(init, |acc_state, op| {
                let rstate = add_operand(acc_state.doc, op, state);
                corners.push((rstate.x, rstate.y));
                ReturnState {
                    doc: rstate.doc,
                    min_y: min(acc_state.min_y, rstate.min_y),
                    x: max(acc_state.x, rstate.x),
                    y: rstate.y,
                }
            });
            let mut points: Vec<(i32, i32)> = Vec::with_capacity(corners.len() * 2);
            points.push((rstate.x, rstate.min_y));
            for (x, y) in corners {
                let (prev_x, prev_y) = points[points.len() - 1];
                if prev_x != x {
                    points.push((x, prev_y));
                }
                points.push((x, y));
            }
            let w = compute_width(&text);
            points.push((rstate.x + w, rstate.y));
            points.push((rstate.x + w, rstate.min_y));


            ReturnState {
                doc: rstate.doc.add(text_group_with_points(
                    &text,
                    points,
                    rstate.x,
                    rstate.min_y,
                    rstate.y,
                )),
                min_y: rstate.min_y,
                x: rstate.x + w,
                y: rstate.y,
            }
        }
    }
}

fn text_group(s: &String, x: i32, y: i32, width: i32, height: i32) -> Group {
    Group::new()
        .add(
            Rectangle::new()
                .set("onclick",ON_CLICK)
                .set("x", x)
                .set("y", y)
                .set("height", height)
                .set("width", width)
                .set(
                    "style",
                    "stroke:black;stroke-width:1;fill:green;fill-opacity:0;stroke-opacity:1",
                ),
        )
        .add(
            Text::new()
                .set("x", x + x_margin)
                .set("y", y + y_margin)
                .add(RawText::new(s)),
        )
}
fn text_group_with_points(
    s: &String,
    points: Vec<(i32, i32)>,
    min_x: i32,
    min_y: i32,
    max_y: i32,
) -> Group {
    let w = compute_width(s);
    let mut points_str = String::new();
    for (x, y) in points {
        points_str.push_str(&format!("{},{} ", x, y).to_string());
    }
    Group::new()
        .add(
            Polygon::new()
                .set("onclick",ON_CLICK)
                .set("points", points_str)
                .set(
                    "style",
                    "stroke:black;stroke-width:1;fill:green;fill-opacity:0;stroke-opacity:1",
                ),
        )
        .add(
            Text::new()
                .set("x", min_x + x_margin)
                .set("y", (min_y  + (max_y  - min_y ) / 2 + 5))
                .add(RawText::new(s)),
        )
}

fn compute_width(s: &String) -> i32 {
    s.len() as i32 * char_width + x_margin + x_margin
}

const ON_CLICK:&str =  "function toggle_fill(e) { if (e.style['fill-opacity'] == 0) { e.style['fill-opacity'] = 0.5 } else {e.style['fill-opacity'] = 0}}; toggle_fill(this);";
