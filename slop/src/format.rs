use std::ops::Deref;

use pretty::{Arena, DocAllocator, DocBuilder, Pretty};

use crate::ast::{Operand, Quantity, Recipe, SourceFile};

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
                quantities: quantity,
                unit,
                text: name,
                ..
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
            Operand::UnaryOp { operand, text, .. } => {
                let operand = operand.pretty(allocator).group();
                let operator = allocator
                    .softline()
                    .append(allocator.text("="))
                    .append(allocator.text(text))
                    .nest(4)
                    .group();
                operand.append(operator)
            }
            Operand::BinaryOp {
                first,
                second,
                text,
                ..
            } => {
                let operands = first.pretty(allocator).append(second.pretty(allocator));

                let operator = allocator
                    .softline()
                    .append(allocator.text("#"))
                    .append(allocator.text(text))
                    .nest(4)
                    .group();
                operands.group().append(operator)
            }
            Operand::MissingOperand { .. } => allocator.nil(),
            Operand::UnusedOperands { operands, .. } => allocator.concat(operands),
        }
    }
}

pub fn format(src: &SourceFile) -> String {
    let arena: Arena<()> = Arena::new();
    src.pretty(&arena).deref().pretty(80).to_string()
}
