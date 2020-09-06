
#[derive(Debug,PartialEq)]
pub enum Node {
    Ingredient(String),
    Operation(String, Vec<Node>),
}

peg::parser!(pub grammar parser() for str {
    pub rule node() -> Node
        = ingredient()
        / operation()
    rule ingredient() -> Node
        = "*" s:sentence() {Node::Ingredient(s)}
    rule operation() -> Node
        = n:(node())* "#" s:sentence() {Node::Operation(s),n}
    rule sentence() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' | '/' | ' ']*) { n.to_owned() } }
        / expected!("identifier")
});




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ingredient() {
        let src = "*1/3 cup sugar";
        let ast = parser::node(&src).expect("must parse");
        assert_eq!(
            Node::Ingredient("1/3 cup sugar".to_string()),
            ast
        )
    }
}
