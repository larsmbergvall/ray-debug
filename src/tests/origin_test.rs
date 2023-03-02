use crate::origin::Origin;
use gethostname::gethostname;

#[test]
fn it_captures_correct_data() {
    let line = line!() + 1;
    let origin = Origin::capture();
    let hostname = gethostname().to_str().unwrap().to_string();

    assert_eq!(line as usize, origin.line_number.clone());
    assert_eq!(&hostname, &origin.hostname);
    assert!(origin
        .function_name
        .contains("origin_test::it_captures_correct_data"));
}
