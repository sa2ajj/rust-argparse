use parser::ArgumentParser;
use super::StoreOption;
use test_parser::{check_ok,check_err};

fn opt(args: &[&str]) -> Option<isize> {
    let mut val = None;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-s", "--set"], box StoreOption::<isize>,
            "Set int value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_opt() {
    assert_eq!(opt(&["./argparse_test"]), None);
    assert_eq!(opt(&["./argparse_test", "-s", "10"]), Some(10));
    assert_eq!(opt(&["./argparse_test", "--set", "11"]), Some(11));
}

#[test]
#[should_fail]
fn test_opt_no_arg() {
    opt(&["./argparse_test", "--set"]);
}

fn optstr(args: &[&str]) -> Option<String> {
    let mut val = None;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-s", "--set"], box StoreOption::<String>,
            "Set string value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_str() {
    assert_eq!(optstr(&["./argparse_test"]), None);
    assert_eq!(optstr(&["./argparse_test", "-s", "10"]), Some(10.to_string()));
    assert_eq!(optstr(&["./argparse_test", "--set", "11"]),
        Some(11.to_string()));
}

#[test]
#[should_fail]
fn test_str_no_art() {
    optstr(&["./argparse_test", "--set"]);
}
