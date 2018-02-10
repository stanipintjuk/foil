use grammar::html::{NodeKind, Content, Attribute};
use std::path::Path;

#[deprecated]
pub fn validate_paths<'a>(node: &'a NodeKind<'a>) 
-> Result<&'a NodeKind<'a>, Vec<(&'a str, &'a usize)>> {
    let paths = get_flattened_paths(node);
    let mut non_existent_paths = Vec::new();

    for (path, pos) in paths {
        if !Path::new(path).exists() {
            non_existent_paths.push((path, pos));
        }
    }

    if non_existent_paths.len() > 0 {
        Err(non_existent_paths)
    } else {
        Ok(node)
    }
}

pub fn validate_and_get_paths<'a>(node: &'a NodeKind<'a>) 
-> Result<Vec<(&'a str, &'a usize)>, Vec<(&'a str, &'a usize)>> {
    let paths = get_flattened_paths(node);
    let mut non_existent_paths = Vec::new();

    for (path, pos) in paths {
        if !Path::new(path).exists() {
            non_existent_paths.push((path, pos));
        }
    }

    if non_existent_paths.len() > 0 {
        Err(non_existent_paths)
    } else {
        Ok(paths)
    }
}

fn get_flattened_paths<'a>(node: &'a NodeKind<'a>) -> Vec<(&'a str, &'a usize)> {
    match node {
        &NodeKind::Content(ref content) => {
            if let Some(path) = content_to_path(content) {
                vec![path]
            } else {
                vec![]
            }
        },
        &NodeKind::ClosedNode(ref node) => 
            get_paths_from_attributes(&node.attributes),
        &NodeKind::OpenNode(ref node) => {
            let mut attr_paths = get_paths_from_attributes(&node.attributes);
            let mut child_paths = get_paths_from_nodelist(&node.children);
            attr_paths.append(&mut child_paths);
            attr_paths
        }
    }
}

fn content_to_path<'a>(content: &'a Content) -> Option<(&'a str, &'a usize)> {
    match content {
        &Content::Path(ref path, ref pos) => Some((path, pos)),
        _ => None,
    }
}

fn get_paths_from_attributes<'a>(attribs: &'a Vec<Attribute<'a>>) 
-> Vec<(&'a str, &'a usize)> {
    let mut paths = Vec::new();

    for &(_, ref content) in attribs {
        if let Some(path) = content_to_path(content) {
            paths.push(path);
        }
    }

    paths
}

fn get_paths_from_nodelist<'a>(nodes: &'a Vec<NodeKind<'a>>) -> Vec<(&'a str, &'a usize)> {
    let mut paths = Vec::new();
    for node in nodes {
        paths.append(&mut get_flattened_paths(node));
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_validator_works() {
        // This path should never exist
        let path1 = NodeKind::Content(Content::Path("./idontexist".to_string(), 2));
        // This path should exist
        let path2 = NodeKind::Content(Content::Path("/".to_string(), 3));
        // This path should exist if tests are run in the root of the project dir
        let path3 = NodeKind::Content(Content::Path("./Cargo.toml".to_string(), 4));

        let node = NodeKind::OpenNode(OpenNode{
            name: "",
            attributes: vec![],
            children: vec![path1, path2, path3],
        });

        let expected = vec![("./idontexist", &2)];
        assert_eq!(Err(expected), validate_paths(&node));
    }

}
