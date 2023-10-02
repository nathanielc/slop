use crate::ast::{Operand, Quantity, Recipe, SourceFile};

const MAX_LINE_LEN: usize = 40;
struct Formatter {
    data: String,
    current_line_len: usize,
}

impl Formatter {
    fn push(&mut self, c: char) {
        self.data.push(c);
        self.current_line_len += 1;
        if c == '\n' {
            self.current_line_len = 0;
        }
    }
    fn push_str(&mut self, s: &str) {
        self.data.push_str(s);
        if let Some(idx) = self.data.rfind('\n') {
            self.current_line_len = self.data[idx..].chars().count();
        }
    }
    fn space(&mut self) {
        if self.current_line_len > MAX_LINE_LEN {
            self.indent();
            return;
        } else if let Some(idx) = self.data.rfind('\n') {
            let start = self.data[idx + 1..].chars().next().unwrap();
            if start != '*' {
                self.indent();
                return;
            }
        }
        self.pad();
    }
    fn indent(&mut self) {
        self.push('\n');
        self.push_str("    ");
    }
    fn pad(&mut self) {
        self.push(' ');
    }
}

pub fn format(src: &SourceFile) -> String {
    let mut f = Formatter {
        data: String::new(),
        current_line_len: 0,
    };
    for r in &src.recipes {
        format_recipe(&mut f, r);
        f.push('\n');
    }
    f.data
}

fn format_recipe(f: &mut Formatter, r: &Recipe) {
    f.push('<');
    if let Some(ref title) = r.title {
        f.push('*');
        f.push('*');
        f.push(' ');
        f.push_str(title.as_str());
    }
    if let Some(ref preamble) = r.preamble {
        f.push('\n');
        f.push('#');
        f.push('#');
        f.push(' ');
        f.push_str(preamble.as_str());
    }
    format_operand(f, &r.root);
    if let Some(ref comment) = r.comment {
        f.push('\n');
        f.push('#');
        f.push('*');
        f.push(' ');
        f.push_str(comment.as_str());
    }
    f.push('\n');
    f.push('>');
}

fn format_operand(f: &mut Formatter, o: &Operand) {
    match o {
        Operand::Ingredient {
            derived,
            quantity,
            unit,
            name,
        } => {
            f.push('\n');
            f.push('*');
            if *derived {
                f.push('^');
            }
            let mut has_measure = false;
            for q in quantity.iter() {
                has_measure = true;
                match q {
                    Quantity::Number(s) => f.push_str(s),
                    Quantity::Fraction(s) => f.push_str(s),
                };
                f.push(' ');
            }
            if let Some(u) = unit {
                f.push_str(u);
                has_measure = true;
            }
            if has_measure {
                f.push_str(": ");
            }
            f.push_str(name);
        }
        Operand::UnaryOp(op, text) => {
            format_operand(f, op);
            f.space();
            f.push('=');
            f.push_str(text);
        }
        Operand::BinaryOp(l, r, text) => {
            format_operand(f, l);
            format_operand(f, r);
            f.space();
            f.push('#');
            f.push_str(text);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::parse;
    fn test_format(src: &str) {
        let src_ast = parse(src).unwrap();
        assert_eq!(src, format(&src_ast));
    }
    #[test]
    fn one_ingredient() {
        test_format(
            "<
*sugar
>
",
        )
    }
    #[test]
    fn long_recipe() {
        test_format(
            "<** Hauloumi
## Sterilize all equipment, boil ~15m
*4 L: unhomogenised milk =heat to 45C 113F
*2 mL: calcium chloride #stir in
*1/4 cup: non chlorinated water
*2 tablets: rennet #dilute #stir in for no more than 1m
    =cover and rest for 45m or until the curd is set
    =cut curds into 1/2 inch cubes
    =allow to heal for 5m
    =stir for 10m
    =scoop curds into cheese cloth lined colander
    =press curds between two boards for 30m, large bowl of water as weight
    =cut curds into desired size blocks
*^whey #boil in until they float =cool
*salt to flavor (a few tbsp) #rub over =store for max 2 weeks
    =grill when ready to eat
#* Yield 12
>
",
        );
    }
    #[test]
    fn another_long_recipe() {
        test_format(
            "<** Souffle pancake with one egg
*3 or 4 drops: lemon juice
*1 : egg =separate keep white #stir in =beat at medium speed, until foamy
*1 1/2 tbsp: sugar #sprinkle in =beat at medium speed 3m until firm peaks form
*^egg yolk
*2 tbsp: flour #+
*1 tbsp: milk #mix to combine
*1/2 tsp: vanilla #stir in
*^1/3 of: egg white mixture #mix with circular motion
    #fold in with flat spatula
*1 tsp: oil =heat in pan 1m
*^2/3 of: pancake mixture #scoop into pan as two pancakes
*2 tsp: water #add to sides of pan =cover cook 2m on medium heat
*1 tsp: water #add to sides of pan #place on top
    =cover cook 5m on medium low heat
    =flip
    =cover cook 5m
    =serve with fruit and syrup/powdered sugar
#* Makes 2 pancakes
>
",
        );
    }
}
