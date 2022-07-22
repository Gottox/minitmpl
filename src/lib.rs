#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
/// Error type for minitmpl
pub enum Error {
    #[error("Variable {0} not found")]
    /// Error type for variable not found
    UnresolvedVariable(String),
    #[error("SyntaxError")]
    /// Error type for syntax error
    SyntaxError,
}

/// templates `template` with `variables_fn` closure
pub fn minitmpl_fn<'a, F, S>(template: &'a str, variables_fn: F) -> Result<String, Error>
where
    F: Fn(&'a str) -> Option<S>,
    S: std::fmt::Display,
{
    if let Some((left, rest)) = template.split_once("{{") {
        if let Some((center, right)) = rest.split_once("}}") {
            let center = center.trim();
            Ok(format!(
                "{}{}{}",
                left,
                variables_fn(center)
                    .ok_or_else(|| Error::UnresolvedVariable(center.to_string()))?,
                minitmpl_fn(right, variables_fn)?
            ))
        } else {
            Err(Error::SyntaxError)
        }
    } else {
        Ok(template.to_string())
    }
}

/// templates `template` with `variables`
pub fn minitmpl<'a, V, S>(template: &'a str, variables: V) -> Result<String, Error>
where
    V: Into<HashMap<String, S>>,
    S: std::fmt::Display,
{
    let source = variables.into();
    minitmpl_fn(template, |key| source.get(key))
}

#[cfg(test)]
mod tests {
    use crate::minitmpl_fn;

    #[test]
    fn simple() {
        let template = "{{color}}";
        let closure = |x| match x {
            "color" => Some("red"),
            _ => None,
        };
        let result = minitmpl_fn(template, closure).unwrap();

        assert_eq!(result, "red");
    }
}
