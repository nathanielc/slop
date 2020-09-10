use crate::ast;
use std::cell::{RefCell, RefMut};
use std::cmp::max;
use std::rc::Rc;

use svg::node::element::{Group, Rectangle, Text};
use svg::node::Text as RawText;
use svg::Document;

enum Operand {
    Ingredient {
        text: String,
        width: i32,
    },
    Operator {
        text: String,
        width: i32,
        operands: Vec<Operand>,
    },
}

const rect_height: i32 = 25;
const char_width: i32 = 12;
const x_margin: i32 = 20;
const y_margin: i32 = 5;

pub fn to_svg(op: ast::Operand) {
    let (max_ing_width, ing_count, total_width) = compute_meta(&op);
    let graph = build_graph(op, max_ing_width);
    let w = total_width + max_ing_width;
    let h = rect_height * ing_count;
    let doc = build_doc(
        Document::new()
            .set("width", w + x_margin * 2)
            .set("height", h + y_margin * 2),
        graph,
        w,
        h,
    );

    svg::save("image.svg", &doc).unwrap();
}

fn compute_meta(op: &ast::Operand) -> (i32, i32, i32) {
    match op {
        ast::Operand::Ingredient(text) => (compute_width(text), 1, 0),
        ast::Operand::UnaryOp(o, text) => {
            let (len, count, width) = compute_meta(o);
            return (len, count, width + compute_width(text));
        }
        ast::Operand::BinaryOp(l, r, text) => {
            let (l_len, l_count, l_width) = compute_meta(l);
            let (r_len, r_count, r_width) = compute_meta(r);
            return (
                max(r_len, l_len),
                l_count + r_count,
                l_width + r_width + compute_width(text),
            );
        }
    }
}

fn build_graph(op: ast::Operand, ingredient_width: i32) -> Operand {
    match op {
        ast::Operand::Ingredient(text) => Operand::Ingredient {
            text: text,
            width: ingredient_width,
        },
        ast::Operand::UnaryOp(op, text) => Operand::Operator {
            width: compute_width(&text),
            text: text,
            operands: vec![build_graph(*op, ingredient_width)],
        },
        ast::Operand::BinaryOp(left, right, text) => Operand::Operator {
            width: compute_width(&text),
            text: text,
            operands: vec![
                build_graph(*left, ingredient_width),
                build_graph(*right, ingredient_width),
            ],
        },
    }
}

fn build_doc(doc: Document, graph: Operand, width: i32, height: i32) -> Document {
    let mut builder = Builder {
        doc: doc,
        x: width,
        y: height,
    };
    walk(&Rc::new(RefCell::new(&mut builder)),  &graph);
    return builder.doc;
}
//fn build_doc(doc: Document, graph: Operand, x: i32, y: i32) -> Document {
//    match graph {
//        Operand::Ingredient { text, width } => {
//            doc.add(text_group(&text, 0, y - rect_height, width, rect_height))
//        }
//        Operand::Operator {
//            text,
//            width,
//            mut operands,
//        } => {
//            let mut doc = doc.add(text_group(&text, 0, 0, x, y));
//            let mut y = y;
//            while operands.len() > 0 {
//                doc = build_doc(doc, operands.pop().unwrap(), x - width, y);
//                y = y - rect_height;
//            }
//            return doc;
//        }
//    }
//}

fn text_group(s: &String, x: i32, y: i32, width: i32, height: i32) -> Group {
    let w = compute_width(s);
    Group::new()
        .add(
            Rectangle::new()
                .set("onclick", "function toggle_fill(e) { if (e.style['fill-opacity'] == 0) { e.style['fill-opacity'] = 0.5 } else {e.style['fill-opacity'] = 0}}; toggle_fill(this);")
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
                .set("x", x + width - w + x_margin)
                .set("y", y + height/2 + y_margin)
                .add(RawText::new(s)),
        )
}

fn compute_width(s: &String) -> i32 {
    s.len() as i32 * char_width + x_margin + x_margin
}

/// Visitor defines a visitor pattern for walking the AST.
///
/// When used with the walk function, Visit will be called for every node
/// in depth-first order. After all children for a Node have been visted,
/// Done is called on that Node to signal that we are done with that Node.
///
/// If Visit returns None, walk will not recurse on the children.
///
/// Note: the Rc in visit and done is to allow for multiple ownership of a node, i.e.
///       a visitor can own a node as well as the walk funciton. This allows
///       for nodes to persist outside the scope of the walk function and to
///       be cleaned up once all owners have let go of the reference.
///
/// Implementors of the Visitor trait will typically wrap themselves in Rc and RefCell
/// in order to allow for:
///   - mutable state, accessed from `Rc::borrow_mut()`
///   - multiple ownership (required so that walking can share ownership with caller)
///
/// See example with `FuncVisitor` below in this file.
trait Visitor: Sized {
    /// Visit is called for a node.
    /// The returned visitor will be used to walk children of the node.
    /// If visit returns None, walk will not recurse on the children.
    fn visit(&self, operand: Rc<&Operand>) -> Option<Self>;
    /// Done is called for a node once it has been visited along with all of its children.
    fn done(&self, _: Rc<&Operand>) {} // default is to do nothing
}

/// Walk recursively visits children of a node.
/// Nodes are visited in depth-first order.
fn walk<T>(v: &T, operand: &Operand)
where
    T: Visitor,
{
    walk_rc(v, Rc::new(operand));
}

fn walk_rc<T>(v: &T, operand: Rc<&Operand>)
where
    T: Visitor,
{
    if let Some(w) = v.visit(operand.clone()) {
        match *operand {
            Operand::Ingredient { text, width } => {}
            Operand::Operator {
                text,
                width,
                operands,
            } => {
                for op in operands {
                    walk(&w, op);
                }
                //while operands.len() >0{
                //   walk(&w, operands.pop().unwrap());
                //}
            }
        }
    }

    v.done(operand.clone())
}

type FuncVisitor<'a> = Rc<RefCell<&'a mut dyn FnMut(Rc<&Operand>)>>;

impl<'a> Visitor for FuncVisitor<'a> {
    fn visit(&self, operand: Rc<&Operand>) -> Option<Self> {
        let mut func: RefMut<_> = self.borrow_mut();
        (&mut *func)(operand);
        Some(Rc::clone(self))
    }
}

/// Create Visitor will produce a visitor that calls the function for all nodes.
fn create_visitor(func: &mut dyn FnMut(Rc<&Operand>)) -> FuncVisitor {
    Rc::new(RefCell::new(func))
}

struct Builder {
    doc: Document,
    x: i32,
    y: i32,
}


impl<'a> Visitor for Rc<RefCell<&'a mut Builder>> {
    fn visit(&self, operand: Rc<&Operand>) -> Option<Self> {
        let mut builder: RefMut<_> = self.borrow_mut();
        match *operand {
            Operand::Ingredient { text, width } => {
                builder.doc =
                    builder
                        .doc
                        .add(text_group(&text, 0, builder.y - rect_height, *width, rect_height))
            }
            Operand::Operator {
                text,
                width,
                operands,
            } => {
                builder.doc = builder.doc.add(text_group(&text, 0, 0, builder.x, builder.y));
            }
        };
        Some(Rc::clone(self))
    }
}
