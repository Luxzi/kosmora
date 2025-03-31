use crate::path::{KosmoraPath, KosmoraPathBuf, PathComponent};
use pretty_assertions::assert_eq;

#[test]
fn test_absolute_path_components() {
    let path = match KosmoraPathBuf::from_str("/usr/bin") {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };
    let expected_components = vec![
        PathComponent::Root,
        PathComponent::Ident("usr".to_string()),
        PathComponent::Ident("bin".to_string()),
    ];
    println!("{path}");
    assert!(path.is_absolute);
    assert_eq!(path.components, expected_components);
}

#[test]
fn test_relative_path_components() {
    let path = match KosmoraPathBuf::from_str("folder/subfolder") {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };
    let expected_components = vec![
        PathComponent::Ident("folder".to_string()),
        PathComponent::Ident("subfolder".to_string()),
    ];
    assert!(!path.is_absolute);
    assert_eq!(path.components, expected_components);
}

#[test]
fn test_special_path_components() {
    match KosmoraPathBuf::from_str("folder/./subfolder/../file.txt") {
        Ok(path) => {
            let expected_components = vec![
                PathComponent::Ident("folder".to_string()),
                PathComponent::Current,
                PathComponent::Ident("subfolder".to_string()),
                PathComponent::Parent,
                PathComponent::Ident("file.txt".to_string()),
            ];
            assert!(!path.is_absolute);
            assert_eq!(path.components, expected_components);
        }
        Err(e) => {
            panic!("{e}")
        }
    }
}

#[test]
fn test_path_output() {
    let input = "folder/./subfolder/../file.txt";
    let path = match KosmoraPathBuf::from_str(input) {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };
    let output = path.to_string();
    assert_eq!(input, output);
}

#[test]
fn test_invalid_path() {
    let input = "C:\\folder/./subfolder/../file.txt";
    match KosmoraPathBuf::from_str(input) {
        Ok(path) => {
            panic!("Expected an error but got path: {path}");
        }
        Err(e) => {
            println!("{e}");
            println!("You passed, congratulations")
        }
    }
}
