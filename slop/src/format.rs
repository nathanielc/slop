use std::ops::Deref;

use pretty::{Arena, DocAllocator, DocBuilder, Pretty};

use crate::ast::{Operand, Quantity, Recipe, SourceFile};

const MAX_LINE_LEN: usize = 40;
struct Formatter {
    data: String,
    current_line_len: usize,
}

impl<'a, D, A> Pretty<'a, D, A> for &'a SourceFile
where
    A: 'a + Clone,
    D: DocAllocator<'a, A>,
    D::Doc: Clone,
{
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D, A> {
        allocator.intersperse(
            self.recipes.iter().map(|recipe| recipe.pretty(allocator)),
            allocator.hardline(),
        )
    }
}
impl<'a, D, A> Pretty<'a, D, A> for &'a Recipe
where
    A: 'a + Clone,
    D: DocAllocator<'a, A>,
    D::Doc: Clone,
{
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D, A> {
        allocator
            .nil()
            .append(if let Some(ref title) = self.title {
                allocator
                    .text("**")
                    .append(allocator.space())
                    .append(allocator.text(title))
                    .group()
            } else {
                allocator.nil()
            })
            .append(if let Some(ref preamble) = self.preamble {
                allocator
                    .softline()
                    .append(allocator.text("##"))
                    .append(allocator.space())
                    .append(allocator.text(preamble))
                    .group()
            } else {
                allocator.nil()
            })
            .append(self.root.pretty(allocator))
            .append(if let Some(ref comment) = self.comment {
                allocator
                    .hardline()
                    .append(allocator.text("#*"))
                    .append(allocator.text(comment))
            } else {
                allocator.nil()
            })
            .append(allocator.hardline())
            .angles()
            .append(allocator.hardline())
    }
}
impl<'a, D, A> Pretty<'a, D, A> for &'a Operand
where
    A: 'a + Clone,
    D: DocAllocator<'a, A>,
    D::Doc: Clone,
{
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D, A> {
        match self {
            Operand::Ingredient {
                derived,
                quantity,
                unit,
                name,
            } => allocator
                .hardline()
                .append(allocator.text("*"))
                .append(if *derived {
                    allocator.text("^")
                } else {
                    allocator.nil()
                })
                .append(allocator.intersperse(
                    quantity.iter().map(|q| match q {
                        Quantity::Number(s) => allocator.text(s),
                        Quantity::Fraction(s) => allocator.text(s),
                    }),
                    " ",
                ))
                .append(if let Some(u) = unit {
                    allocator.space().append(allocator.text(u))
                } else {
                    allocator.nil()
                })
                .append(if !quantity.is_empty() || unit.is_some() {
                    allocator.text(":").append(allocator.space())
                } else {
                    allocator.nil()
                })
                .append(allocator.text(name)),
            Operand::UnaryOp(operand, text) => {
                let operand = operand.pretty(allocator).group();
                let operator = allocator
                    .softline()
                    .append(allocator.text("="))
                    .append(allocator.text(text))
                    .nest(4)
                    .group();
                operand.append(operator)
            }
            Operand::BinaryOp(l, r, text) => {
                let operands = l.pretty(allocator).append(r.pretty(allocator));

                let operator = allocator
                    .softline()
                    .append(allocator.text("#"))
                    .append(allocator.text(text))
                    .nest(4)
                    .group();
                operands.group().append(operator)
            }
        }
    }
}

pub fn format(src: &SourceFile) -> String {
    let arena: Arena<()> = Arena::new();
    src.pretty(&arena).deref().pretty(80).to_string()
}

#[cfg(test)]
mod test {
    use expect_test::{expect, Expect};

    use super::*;
    use crate::parse;
    fn test_format(src: Expect) {
        let src_ast = parse(src.data()).unwrap();
        println!("AST: {:#?}", src_ast);
        let formatted = format(&src_ast);
        src.assert_eq(&formatted);
    }
    #[test]
    fn one_ingredient() {
        test_format(expect![[r#"
            <
            *sugar
            >
        "#]])
    }
    #[test]
    fn unary_list() {
        test_format(expect![[r#"
            <
            *a =one =two =three
            >
        "#]])
    }
    #[test]
    fn unary_list_long() {
        test_format(expect![[r#"
            <
            *a
                =one, this is a long line that has lots of words and it has more than 80 characters
                =two, this is a long line that has lots of words and it has more than 80 characters
                =three, this is a long line that has lots of words and it has more than 80 characters
            >
        "#]])
    }
    #[test]
    fn binary_tree() {
        test_format(expect![[r#"
            <
            *a
            *b #one
            *c
            *d #two #three
            >
        "#]])
    }
    #[test]
    fn binary_tree_long() {
        test_format(expect![[r#"
            <
            *a
            *b
                #one, this is a long line that has lots of words and it has more than 80 characters
            *c
            *d
                #two, this is a long line that has lots of words and it has more than 80 characters
                #three, this is a long line that has lots of words and it has more than 80 characters
            >
        "#]])
    }
    #[test]
    fn long_recipe() {
        test_format(expect![[r#"
            <** Hauloumi ## Sterilize all equipment, boil ~15m
            *4 L: unhomogenised milk =heat to 45C 113F
            *2 mL: calcium chloride #stir in
            *1/4 cup: non chlorinated water
            *2 tablets: rennet #dilute #stir in for no more than 1m
                =cover and rest for 45m or until the curd is set
                =cut curds into 1/2 inch cubes =allow to heal for 5m =stir for 10m
                =scoop curds into cheese cloth lined colander
                =press curds between two boards for 30m, large bowl of water as weight
                =cut curds into desired size blocks
            *^whey #boil in until they float =cool
            *salt to flavor (a few tbsp) #rub over =store for max 2 weeks
                =grill when ready to eat
            #*Yield 12
            >
        "#]]);
    }
    #[test]
    fn another_long_recipe() {
        test_format(expect![[r#"
            <** Souffle pancake with one egg
            *3 or 4 drops: lemon juice
            *1: egg =separate keep white #stir in =beat at medium speed, until foamy
            *1 1/2 tbsp: sugar #sprinkle in =beat at medium speed 3m until firm peaks form
            *^egg yolk
            *2 tbsp: flour #+
            *1 tbsp: milk #mix to combine
            *1/2 tsp: vanilla #stir in
            *^1/3 of: egg white mixture #mix with circular motion #fold in with flat spatula
            *1 tsp: oil =heat in pan 1m
            *^2/3 of: pancake mixture #scoop into pan as two pancakes
            *2 tsp: water #add to sides of pan =cover cook 2m on medium heat
            *1 tsp: water #add to sides of pan #place on top
                =cover cook 5m on medium low heat =flip =cover cook 5m
                =serve with fruit and syrup/powdered sugar
            #*Makes 2 pancakes
            >
        "#]]);
    }
}
