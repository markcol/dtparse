use ParseError;
use {parse, tokenize};

#[test]
fn test_fuzz() {

    assert_eq!(parse("\x2D\x38\x31\x39\x34\x38\x34"), Err(ParseError::InvalidMonth));
}

#[test]
fn test_tokenizer() {
    let v = tokenize("Sat Oct 11 17:13:46 UTC 2003");
    assert_eq!(v.len(), 15);
    assert_eq!(v[0], "Sat");
    assert_eq!(v[1], " ");
    assert_eq!(v[2], "Oct");
    assert_eq!(v[3], " ");
    assert_eq!(v[4], "11");
    assert_eq!(v[5], " ");
    assert_eq!(v[6], "17");
    assert_eq!(v[7], ":");
    assert_eq!(v[8], "13");
    assert_eq!(v[9], ":");
    assert_eq!(v[10], "46");
    assert_eq!(v[11], " ");
    assert_eq!(v[12], "UTC");
    assert_eq!(v[13], " ");
    assert_eq!(v[14], "2003");
}

#[test]
fn test_tokenizer_ws() {
    let v = tokenize("Sat  Oct  11  17:13:46  UTC  2003");
    assert_eq!(v.len(), 20);
    assert_eq!(v[0], "Sat");
    assert_eq!(v[1], " ");
    assert_eq!(v[2], " ");
    assert_eq!(v[3], "Oct");
    assert_eq!(v[4], " ");
    assert_eq!(v[5], " ");
    assert_eq!(v[6], "11");
    assert_eq!(v[7], " ");
    assert_eq!(v[8], " ");
    assert_eq!(v[9], "17");
    assert_eq!(v[10], ":");
    assert_eq!(v[11], "13");
    assert_eq!(v[12], ":");
    assert_eq!(v[13], "46");
    assert_eq!(v[14], " ");
    assert_eq!(v[15], " ");
    assert_eq!(v[16], "UTC");
    assert_eq!(v[17], " ");
    assert_eq!(v[18], " ");
    assert_eq!(v[19], "2003");
}
