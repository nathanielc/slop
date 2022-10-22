use std::cmp::{max, min};

use crate::semantic::{self, Ingredient};
use svg::node::element::{Group, Polygon, Rectangle, Text};
use svg::node::Text as RawText;
use svg::Document;

const RECT_HEIGHT: i32 = 25;
const CHAR_WIDTH: i32 = 8;
const X_MARGIN: i32 = 5;
const Y_MARGIN: i32 = 20;

pub fn to_svg(mut src: semantic::SourceFile) -> Vec<u8> {
    let recipe = src.recipes.pop().unwrap();
    recipe_to_svg(recipe)
}

fn recipe_to_svg(r: semantic::Recipe) -> Vec<u8> {
    let max_ing_width = compute_max_ing_width(&r.root);
    let doc = build_doc(r, max_ing_width);

    let mut out: Vec<u8> = Vec::new();

    svg::write(&mut out, &doc).unwrap();
    out
}

fn compute_max_ing_width(op: &semantic::Operand) -> i32 {
    match op {
        semantic::Operand::Ingredient(ing) => compute_width(ingredient_text(&ing).as_str()),
        semantic::Operand::Operator { text: _, operands } => operands
            .iter()
            .fold(0, |acc, op| max(acc, compute_max_ing_width(op))),
    }
}

fn build_doc(r: semantic::Recipe, max_ing_width: i32) -> Document {
    let mut state = BuildState {
        max_ing_width,
        ing_count: 0,
        id: 0,
    };
    let mut doc = Document::new();
    if let Some(ref title) = r.title {
        doc = doc.add(
            Text::new()
                .add(RawText::new(title))
                .set("font-family", "monospace")
                .set("font-style", "bold")
                .set("font-size", "14")
                .set("x", X_MARGIN)
                .set("y", Y_MARGIN),
        );
        state.ing_count = state.ing_count + 1
    }
    let mut preamble_y = 0;
    match &r.preamble {
        Some(_) => {
            preamble_y = state.ing_count * RECT_HEIGHT;
            state.ing_count = state.ing_count + 1;
        }
        None => {}
    };

    let mut rstate = add_operand(doc, r.root, &mut state);
    if let Some(ref preamble) = r.preamble {
        rstate.doc = rstate.doc.add(text_group(
            state.id,
            preamble,
            0,
            preamble_y,
            rstate.x,
            RECT_HEIGHT,
        ));
        state.id += 1;
    }
    if let Some(ref comment) = r.comment {
        rstate.doc = rstate.doc.add(
            Text::new()
                .add(RawText::new(comment))
                .set("font-family", "monospace")
                .set("font-size", "14")
                .set("x", X_MARGIN)
                .set("y", state.ing_count * RECT_HEIGHT + Y_MARGIN * 2),
        );
        state.ing_count = state.ing_count + 1;
    }
    rstate
        .doc
        .set("width", rstate.x + X_MARGIN * 2)
        .set("height", state.ing_count * RECT_HEIGHT + Y_MARGIN * 2)
}

struct BuildState {
    max_ing_width: i32,
    ing_count: i32,
    id: i32,
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

// render the complete text for an ingredient
fn ingredient_text(i: &Ingredient) -> String {
    let derived = if i.derived { "^" } else { "" };
    match (i.quantity.as_ref(), i.unit.as_ref()) {
        (Some(q), Some(u)) => format!("{}{} {} {}", derived, q.0, u, i.name),
        (Some(q), None) => format!("{}{} {}", derived, q.0, i.name),
        _ => format!("{}{}", derived, i.name),
    }
}

// add the operand to the document, return the tuple of
// (new doc, minumum y value of all parent operands, x position of op, y, position of op)
fn add_operand(doc: Document, op: semantic::Operand, state: &mut BuildState) -> ReturnState {
    match op {
        semantic::Operand::Ingredient(ing) => {
            state.ing_count = state.ing_count + 1;
            let ing_idx = state.ing_count - 1;
            let x = 0;
            let y = ing_idx * RECT_HEIGHT;
            let w = state.max_ing_width;
            let h = RECT_HEIGHT;
            let ing_text = ingredient_text(&ing);
            let doc = doc.add(text_group(state.id, ing_text.as_str(), x, y, w, h));
            state.id += 1;
            return ReturnState {
                doc,
                min_y: y,
                x: w,
                y: y + RECT_HEIGHT,
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

            let s = ReturnState {
                doc: rstate.doc.add(text_group_with_points(
                    state.id,
                    &text,
                    points,
                    rstate.x,
                    rstate.min_y,
                    rstate.y,
                )),
                min_y: rstate.min_y,
                x: rstate.x + w,
                y: rstate.y,
            };
            state.id += 1;
            return s;
        }
    }
}

fn text_group(id: i32, s: &str, x: i32, y: i32, width: i32, height: i32) -> Group {
    Group::new()
        .add(
            Rectangle::new()
                .set("id", id)
                .set("onclick", on_click(id))
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
                .set("onclick", on_click(id))
                .set("font-family", "monospace")
                .set("font-size", "12")
                .set("x", x + X_MARGIN)
                .set("y", y + Y_MARGIN)
                .add(RawText::new(s)),
        )
}
fn text_group_with_points(
    id: i32,
    s: &String,
    points: Vec<(i32, i32)>,
    min_x: i32,
    min_y: i32,
    max_y: i32,
) -> Group {
    let mut points_str = String::new();
    for (x, y) in points {
        points_str.push_str(&format!("{},{} ", x, y).to_string());
    }
    Group::new()
        .add(
            Polygon::new()
                .set("id", id)
                .set("onclick", on_click(id))
                .set("points", points_str)
                .set(
                    "style",
                    "stroke:black;stroke-width:1;fill:green;fill-opacity:0;stroke-opacity:1",
                ),
        )
        .add(
            Text::new()
                .set("onclick", on_click(id))
                .set("font-family", "monospace")
                .set("font-size", "12")
                .set("x", min_x + X_MARGIN)
                .set("y", min_y + (max_y - min_y) / 2 + 5)
                .add(RawText::new(s)),
        )
}

fn compute_width(s: &str) -> i32 {
    s.len() as i32 * CHAR_WIDTH + X_MARGIN + X_MARGIN
}

fn on_click(id: i32) -> String {
    format!("function toggle_fill(e) {{ if (e.style['fill-opacity'] == 0) {{ e.style['fill-opacity'] = 0.5 }} else {{e.style['fill-opacity'] = 0}}}}; toggle_fill(document.getElementById('{}'));", id)
}
