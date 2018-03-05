use compiler::parser::ast::{Ast, SetField};
use compiler::evaluator::{Evaluator, EvalResult, EvalError, Output};

pub fn evaluate_html<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, tag_name: &str, attributes: &Vec<SetField>, children: &Vec<Ast>) -> EvalResult {
    let children = children
        .iter()
        .map(|child|{ 
            Evaluator{
                expr: &child, 
                scope: eval.scope.clone(), 
                file_path: eval.file_path.clone(), 
                out_path: eval.out_path.clone()
            }
            .eval()
            .and_then(Output::to_string)
        })
    .fold(Ok(String::new()), fold_html);

    let attributes = eval_attributes(eval, attributes);

    match (children, attributes) {
        (Ok(children), Ok(attributes)) => Ok(Output::String(format!("<{}{}>{}</{}>", tag_name, attributes, children, tag_name))),
        (_, Err(err)) => Err(err),
        (Err(err), _) => Err(err),
    }
}

pub fn evaluate_html_closed<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, tag_name: &str, attributes: &Vec<SetField>) -> EvalResult {
    let attributes = eval_attributes(eval, attributes);
    match attributes {
        Ok(attributes) => Ok(Output::String(format!("<{}{}/>", tag_name, attributes))),
        Err(err) => Err(err),
    }
}

type StrRes = Result<String,  EvalError>;

fn eval_attributes<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, attributes: &Vec<SetField>) -> StrRes {
    let attributes = attributes
        .iter()
        .map(|field|{
            field_to_attribute_string(eval, field)
        })
        .fold(Ok(String::new()), 
              fold_attribute_strings);
    attributes
}

fn fold_html(out_str: StrRes, next_string: StrRes) -> StrRes {
    fold_string_result(out_str, next_string, |s1, s2|{format!("{}{}", s1, s2)})
}

fn fold_attribute_strings(out_str: StrRes, next_string: StrRes) -> StrRes {
    fold_string_result(out_str, next_string, |s1, s2|{format!("{} {}", s1, s2)})
}

fn fold_string_result<F>(out_str: StrRes, next_string: StrRes, combinator: F) -> StrRes where F: FnOnce(String, String) -> String{
    if let Err(err) = out_str {
        Err(err)
    } else if let Err(err) = next_string {
        Err(err)
    } else {
        let next_string = next_string.unwrap();
        let out_str = out_str.unwrap();
        Ok(combinator(out_str, next_string))
    }
}

fn field_to_attribute_string<'scope, 'ast: 'scope>(eval: &Evaluator<'scope, 'ast>, field: &SetField) -> StrRes {
    let evaluator = 
        Evaluator{
            expr: &field.value,
            scope: eval.scope.clone(),
            file_path: eval.file_path.clone(),
            out_path: eval.out_path.clone()
        };
    let result = evaluator.eval();
    let result = result.and_then(Output::to_string);

    if let Ok(value) = result {
        let attr = format!("{}=\"{}\"", field.name, value);
        Ok(attr)
    } else {
        result
    }
}
