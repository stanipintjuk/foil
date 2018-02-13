pub use self::html::{node, expression, ParseError};

peg! html(r#"
use super::*;
use super::super::node_tree::*;
use super::super::expression_tree::*;

#[pub]
node -> NodeKind<'input>
    = whitespace? n:node_kind whitespace? {n}

node_kind -> NodeKind<'input>
    = n:closed_node { NodeKind::ClosedNode(n) }
    / n:open_node { NodeKind::OpenNode(n) }
    / n:content { NodeKind::Content(n) }

closed_node -> ClosedNode<'input>
    = n:tag_name a:attributes whitespace ?";" { 
            ClosedNode{ name: n, attributes: a } 
        }

open_node -> OpenNode<'input>
    = n:tag_name a:attributes whitespace? c:children { 
        OpenNode{
            name: n, 
            attributes: a,
            children: c,
        }}

children -> Vec<NodeKind<'input>>
    = "{" whitespace? c:node* whitespace? "}" { c }
    / c:node_kind { vec![c] }

content -> Content
    = s:string {Content::Literal(s)}
    / pos:#position p:path {Content::Path(p, pos)}
    / "(" whitespace? expr:expression whitespace? ")" 
        { Content::Expression(Box::new(expr)) }

tag_name -> &'input str
    = $([a-zA-Z0-9]+)

attributes -> Vec<Attribute<'input>>
    = attribute*

attribute -> Attribute<'input>
    = whitespace n:tag_name "=" v:content { (n, v) }

string -> String
    = "\"" s:$(("\\\\"/"\\\""/[^\"])*) "\"" { strip_escape_chars(s, "\\", "\"") }

path -> String
    = "<" s:$(( "\\\\" / "\\>" /[^>])*) ">" { strip_escape_chars(s, "\\", ">") }

whitespace = #quiet<[ \n\t]+>


// Expression parser
#[pub]
expression -> Expression
    = l:atom whitespace? "+" whitespace? r:atom { Expression::Sum(l, r) }
    / a:atom { Expression::Atom(a) }

atom -> Atom
    = c:content { Atom::Content(c) }
    / "(" whitespace? expr:expression whitespace? ")" { Atom::Expression(Box::new(expr)) }

"#);

fn strip_escape_chars(s: &str, escape_char: &str, delimiter: &str) -> String {
    let escaped_escape_char = format!("{}{}", escape_char, escape_char);
    let escaped_delimiter = format!("{}{}", escape_char, delimiter);

    s.replace(&escaped_escape_char, &escape_char)
        .replace(&escaped_delimiter, &delimiter)
}
