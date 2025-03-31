use std::{
    fmt::{Display, format},
    io::BufRead,
    ops::Deref,
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, PartialEq, Default)]
pub struct KosmoraPathBuf {
    pub(crate) is_absolute: bool,
    pub(crate) components: Vec<PathComponent<String>>,
}

fn format_path_components<S: AsRef<str> + ToString + Display>(
    components: &Vec<PathComponent<S>>,
) -> String {
    let mut output_string = String::new();

    for (idx, cmp) in components.iter().enumerate() {
        match cmp {
            PathComponent::Root => output_string.push_str("/"),
            PathComponent::Ident(i) => {
                let middle_path = format!("{i}/");
                let last_path = format!("{i}");

                if idx != components.len() - 1 {
                    output_string.push_str(&middle_path)
                } else {
                    output_string.push_str(&last_path)
                }
            }
            PathComponent::Parent => output_string.push_str("../"),
            PathComponent::Current => output_string.push_str("./"),
        }
    }

    output_string
}

impl From<KosmoraPathBuf> for std::path::PathBuf {
    fn from(value: KosmoraPathBuf) -> Self {
       PathBuf::from_str(&format_path_components(&value.components)).unwrap()
    }
}

impl std::fmt::Display for KosmoraPathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = format_path_components(&self.components);
        write!(f, "{}", output_string)
    }
}

impl<'path> std::fmt::Display for KosmoraPath<'path> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output_string = format_path_components(&self.components);
        write!(f, "{}", output_string)
    }
}

impl Into<std::path::PathBuf> for KosmoraPathBuf {
    fn into(self) -> std::path::PathBuf {
        PathBuf::from_str(&format!("{self}")).unwrap()
        // todo!()
    }
}

impl<'path> Into<&'path std::path::Path> for KosmoraPath<'path> {
    fn into(self) -> &'path std::path::Path {
        todo!()
    }
}

impl KosmoraPathBuf {
    pub fn new() -> Self {
        Self::default()
    }

    fn as_kpath<'path>(&self) -> KosmoraPath {
        KosmoraPath { buf: &self }
    }

    pub fn from_str<S: AsRef<str>>(path: S) -> Result<Self, crate::Error> {
        let path_str = path.as_ref();
        let components: Vec<&str> = path_str.split(|c| c == '/' || c == '\\').collect();
        let is_absolute = components.first().map(|&s| s.is_empty()).unwrap_or(false);
        let mut parsed = Vec::new();
        if is_absolute {
            parsed.push(PathComponent::Root);
        }

        let mut invalid_chars = Vec::new();
        for part in components.into_iter().skip(if is_absolute { 1 } else { 0 }) {
            if part.is_empty() {
                continue;
            }
            for c in part.chars() {
                if matches!(c, ':' | '*' | '?' | '"' | '<' | '>' | '|' | '/' | '\\')
                    && !invalid_chars.contains(&c)
                {
                    invalid_chars.push(c);
                }
            }
            parsed.push(match part {
                "." => PathComponent::Current,
                ".." => PathComponent::Parent,
                _ => PathComponent::Ident(part.to_string()),
            });
        }
        if !invalid_chars.is_empty() {
            let err = crate::Error {
                kind: crate::ErrorKind::InvalidPath,
                label: "The inputted path contains invalid characters".to_string(),
                msg: Some(format!("Invalid characters found: {:?}", invalid_chars)),
                source: None,
            };
            return Err(err);
        }

        Ok(Self {
            is_absolute,
            components: parsed,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct KosmoraPath<'path> {
    buf: &'path KosmoraPathBuf,
}

impl<'path> Deref for KosmoraPath<'path> {
    type Target = KosmoraPathBuf;

    fn deref(&self) -> &Self::Target {
        self.buf
    }
}

#[derive(Debug, PartialEq, Default)]
pub(crate) enum PathComponent<T> {
    #[default]
    Root,
    Ident(T),
    Parent,
    Current,
}
