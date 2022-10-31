use std::cmp::{max, min};

use crate::semantic::{self, Ingredient, Operand, Recipe};
use svg::node::element::{Group, Rectangle, Style, TSpan, Text};
use svg::node::Text as RawText;
use svg::Document;

pub fn to_svg(src: &semantic::SourceFile) -> Vec<u8> {
    let recipe = src.recipes.first().unwrap();
    recipe_to_svg(recipe)
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
fn recipe_to_svg(r: &semantic::Recipe) -> Vec<u8> {
    let doc = build_doc(r);

    let mut out: Vec<u8> = Vec::new();

    svg::write(&mut out, &doc).unwrap();
    out
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
    fn enclose(self: Self, other: Self) -> Self {
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
                let text = ingredient_text(&i);
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
            Operand::Operator { text, operands } => {
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
    if let Some(title) = builder.build_title(&r) {
        doc = doc.add(title);
    }
    if let Some(preamble) = builder.build_preamble(&r) {
        doc = doc.add(preamble);
    }

    let (op_doc, mut bound) = builder.build_operand(&r.root);
    doc = doc.add(op_doc);

    if let Some(comment) = builder.build_comment(&r) {
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
    match (i.quantity.as_ref(), i.unit.as_ref()) {
        (Some(q), Some(u)) => format!("{}{} {} {}", derived, q.0, u, i.name),
        (Some(q), None) => format!("{}{} {}", derived, q.0, i.name),
        _ => format!("{}{}", derived, i.name),
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

#[cfg(test)]
mod tests {
    use crate::{parse, semantic::convert_source_file};
    use expect_test::expect;

    // useful tool for debugging https://www.svgviewer.dev/

    fn build_svg(src: &str) -> String {
        let src_ast = parse(&src).expect("parsing failed");
        let recipe = convert_source_file(src_ast);
        String::from_utf8(super::to_svg(&recipe)).expect("must be utf8")
    }
    #[test]
    fn one_ingredient() {
        let svg = build_svg(r#"<*apple>"#);
        expect![[r#"
            <svg height="40" width="210" xmlns="http://www.w3.org/2000/svg">
            <style>
            text {
                font-family: monospace;
                font-size: 16px;
            }
            rect {
                stroke: black;
                stroke-width: 1;
                fill: green;
                fill-opacity: 0;
                stroke-opacity: 1;
            }
            </style>
            <g>
            <text x="0" y="0">
            <tspan dy="20" x="5">
            apple 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="0"/>
            </g>
            </svg>"#]]
        .assert_eq(&svg);
    }
    #[test]
    fn multiline_ingredient() {
        let svg = build_svg(r#"<*crisp golden delicious apple>"#);
        expect![[r#"
            <svg height="60" width="210" xmlns="http://www.w3.org/2000/svg">
            <style>
            text {
                font-family: monospace;
                font-size: 16px;
            }
            rect {
                stroke: black;
                stroke-width: 1;
                fill: green;
                fill-opacity: 0;
                stroke-opacity: 1;
            }
            </style>
            <g>
            <text x="0" y="0">
            <tspan dy="20" x="5">
            crisp golden 
            </tspan>
            <tspan dy="20" x="5">
            delicious apple 
            </tspan>
            </text>
            <rect height="50" width="200" x="0" y="0"/>
            </g>
            </svg>"#]]
        .assert_eq(&svg);
    }
    #[test]
    fn two_ingredients() {
        let svg = build_svg(
            r#"<
*crisp golden delicious apple
*peanut butter #eat
>"#,
        );
        expect![[r#"
            <svg height="90" width="260" xmlns="http://www.w3.org/2000/svg">
            <style>
            text {
                font-family: monospace;
                font-size: 16px;
            }
            rect {
                stroke: black;
                stroke-width: 1;
                fill: green;
                fill-opacity: 0;
                stroke-opacity: 1;
            }
            </style>
            <g>
            <g>
            <text x="0" y="0">
            <tspan dy="20" x="5">
            crisp golden 
            </tspan>
            <tspan dy="20" x="5">
            delicious apple 
            </tspan>
            </text>
            <rect height="50" width="200" x="0" y="0"/>
            </g>
            <g>
            <text x="0" y="50">
            <tspan dy="20" x="5">
            peanut butter 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="50"/>
            </g>
            <text x="200" y="25">
            <tspan dy="20" x="205">
            eat 
            </tspan>
            </text>
            <rect height="80" width="250" x="0" y="0"/>
            </g>
            </svg>"#]]
        .assert_eq(&svg);
    }
    #[test]
    fn cookies() {
        let svg = build_svg(
            r#"<
*butter =soften
*sugar
*brown sugar #+
*vanilla #+ #beat
*eggs # beat one at a time
*flour
*soda #+
*salt #mix #beat slowly
*chocolate chips
*chopped nuts #+ #stir =form into balls =bake 375F 10m
#* Yield 1 dozen cookies
>"#,
        );
        expect![[r#"
            <svg height="340" width="950" xmlns="http://www.w3.org/2000/svg">
            <style>
            text {
                font-family: monospace;
                font-size: 16px;
            }
            rect {
                stroke: black;
                stroke-width: 1;
                fill: green;
                fill-opacity: 0;
                stroke-opacity: 1;
            }
            </style>
            <g>
            <g>
            <g>
            <g>
            <g>
            <g>
            <g>
            <g>
            <text x="0" y="0">
            <tspan dy="20" x="5">
            butter 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="0"/>
            </g>
            <text x="200" y="0">
            <tspan dy="20" x="205">
            soften 
            </tspan>
            </text>
            <rect height="30" width="280" x="0" y="0"/>
            </g>
            <g>
            <text x="0" y="30">
            <tspan dy="20" x="5">
            sugar 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="30"/>
            </g>
            <g>
            <text x="0" y="60">
            <tspan dy="20" x="5">
            brown sugar 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="60"/>
            </g>
            <g>
            <text x="0" y="90">
            <tspan dy="20" x="5">
            vanilla 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="90"/>
            </g>
            <text x="280" y="45">
            <tspan dy="20" x="285">
            beat 
            </tspan>
            </text>
            <rect height="120" width="340" x="0" y="0"/>
            </g>
            <g>
            <text x="0" y="120">
            <tspan dy="20" x="5">
            eggs 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="120"/>
            </g>
            <text x="340" y="50">
            <tspan dy="20" x="345">
            beat one at a 
            </tspan>
            <tspan dy="20" x="345">
            time 
            </tspan>
            </text>
            <rect height="150" width="490" x="0" y="0"/>
            </g>
            <g>
            <g>
            <text x="0" y="150">
            <tspan dy="20" x="5">
            flour 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="150"/>
            </g>
            <g>
            <text x="0" y="180">
            <tspan dy="20" x="5">
            soda 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="180"/>
            </g>
            <g>
            <text x="0" y="210">
            <tspan dy="20" x="5">
            salt 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="210"/>
            </g>
            <text x="200" y="180">
            <tspan dy="20" x="205">
            mix 
            </tspan>
            </text>
            <rect height="90" width="250" x="0" y="150"/>
            </g>
            <text x="490" y="105">
            <tspan dy="20" x="495">
            beat slowly 
            </tspan>
            </text>
            <rect height="240" width="620" x="0" y="0"/>
            </g>
            <g>
            <text x="0" y="240">
            <tspan dy="20" x="5">
            chocolate chips 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="240"/>
            </g>
            <g>
            <text x="0" y="270">
            <tspan dy="20" x="5">
            chopped nuts 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="270"/>
            </g>
            <text x="620" y="135">
            <tspan dy="20" x="625">
            stir 
            </tspan>
            </text>
            <rect height="300" width="680" x="0" y="0"/>
            </g>
            <text x="680" y="125">
            <tspan dy="20" x="685">
            form into 
            </tspan>
            <tspan dy="20" x="685">
            balls 
            </tspan>
            </text>
            <rect height="300" width="790" x="0" y="0"/>
            </g>
            <text x="790" y="135">
            <tspan dy="20" x="795">
            bake 375F 10m 
            </tspan>
            </text>
            <rect height="300" width="940" x="0" y="0"/>
            </g>
            <text x="5" y="325">
            Yield 1 dozen cookies
            </text>
            </svg>"#]]
        .assert_eq(&svg);
    }
    #[test]
    fn keiserschmarrn() {
        let svg = build_svg(
            r#"<**Keiserschmarrn
            ## Good Saturday breakfast
*6 : eggs =separate
*1 1/2 cups: milk #+
*1 tsp: vanilla #whisk
*salt #+
*1 cup: flour #whisk
*3 tbsp: sugar
*^egg whites #whip to form soft peaks #gently fold until white lumps are gone
*2 tbsp: butter =melt in large pan #in two batches cook until bottom side is firm flip and break into bit size pieces
*powdered sugar #+
*syrup #top and serve
#* Serves 4
>"#,
        );
        expect![[r#"
            <svg height="402" width="870" xmlns="http://www.w3.org/2000/svg">
            <style>
            text {
                font-family: monospace;
                font-size: 16px;
            }
            rect {
                stroke: black;
                stroke-width: 1;
                fill: green;
                fill-opacity: 0;
                stroke-opacity: 1;
            }
            </style>
            <text font-size="18px" font-style="bold" x="5" y="27">
            Keiserschmarrn
            </text>
            <text x="5" y="57">
            Good Saturday breakfast
            </text>
            <g>
            <g>
            <g>
            <g>
            <g>
            <g>
            <g>
            <text x="0" y="62">
            <tspan dy="20" x="5">
            6 eggs 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="62"/>
            </g>
            <text x="200" y="62">
            <tspan dy="20" x="205">
            separate 
            </tspan>
            </text>
            <rect height="30" width="300" x="0" y="62"/>
            </g>
            <g>
            <text x="0" y="92">
            <tspan dy="20" x="5">
            1 1/2 cups milk 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="92"/>
            </g>
            <g>
            <text x="0" y="122">
            <tspan dy="20" x="5">
            1 tsp vanilla 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="122"/>
            </g>
            <text x="300" y="92">
            <tspan dy="20" x="305">
            whisk 
            </tspan>
            </text>
            <rect height="90" width="370" x="0" y="62"/>
            </g>
            <g>
            <text x="0" y="152">
            <tspan dy="20" x="5">
            salt 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="152"/>
            </g>
            <g>
            <text x="0" y="182">
            <tspan dy="20" x="5">
            1 cup flour 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="182"/>
            </g>
            <text x="370" y="122">
            <tspan dy="20" x="375">
            whisk 
            </tspan>
            </text>
            <rect height="150" width="440" x="0" y="62"/>
            </g>
            <g>
            <g>
            <text x="0" y="212">
            <tspan dy="20" x="5">
            3 tbsp sugar 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="212"/>
            </g>
            <g>
            <text x="0" y="242">
            <tspan dy="20" x="5">
            ^egg whites 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="242"/>
            </g>
            <text x="200" y="217">
            <tspan dy="20" x="205">
            whip to form 
            </tspan>
            <tspan dy="20" x="205">
            soft peaks 
            </tspan>
            </text>
            <rect height="60" width="340" x="0" y="212"/>
            </g>
            <text x="440" y="122">
            <tspan dy="20" x="445">
            gently fold 
            </tspan>
            <tspan dy="20" x="445">
            until white 
            </tspan>
            <tspan dy="20" x="445">
            lumps are 
            </tspan>
            <tspan dy="20" x="445">
            gone 
            </tspan>
            </text>
            <rect height="210" width="570" x="0" y="62"/>
            </g>
            <g>
            <g>
            <text x="0" y="272">
            <tspan dy="20" x="5">
            2 tbsp butter 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="272"/>
            </g>
            <text x="200" y="272">
            <tspan dy="20" x="205">
            melt in large pan 
            </tspan>
            </text>
            <rect height="30" width="390" x="0" y="272"/>
            </g>
            <text x="570" y="97">
            <tspan dy="20" x="575">
            in two 
            </tspan>
            <tspan dy="20" x="575">
            batches cook 
            </tspan>
            <tspan dy="20" x="575">
            until bottom 
            </tspan>
            <tspan dy="20" x="575">
            side is firm 
            </tspan>
            <tspan dy="20" x="575">
            flip and 
            </tspan>
            <tspan dy="20" x="575">
            break into 
            </tspan>
            <tspan dy="20" x="575">
            bit size 
            </tspan>
            <tspan dy="20" x="575">
            pieces 
            </tspan>
            </text>
            <rect height="240" width="710" x="0" y="62"/>
            </g>
            <g>
            <text x="0" y="302">
            <tspan dy="20" x="5">
            powdered sugar 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="302"/>
            </g>
            <g>
            <text x="0" y="332">
            <tspan dy="20" x="5">
            syrup 
            </tspan>
            </text>
            <rect height="30" width="200" x="0" y="332"/>
            </g>
            <text x="710" y="197">
            <tspan dy="20" x="715">
            top and serve 
            </tspan>
            </text>
            <rect height="300" width="860" x="0" y="62"/>
            </g>
            <text x="5" y="387">
            Serves 4
            </text>
            </svg>"#]]
        .assert_eq(&svg);
    }
}
