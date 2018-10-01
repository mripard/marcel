#[macro_use]
extern crate nom;

named!(timestamp_parser,
       delimited!(
           tag!("[ "),
           take_until!(" ]"),
           tag!(" ]")
       )
);

#[test]
fn timestamp_parser_test() {
    let string = "[ 1.002540 ] 1ca0000.dsi: reg=0x42424242 val=0x84848484";
    let timestamp = "1.002540";
    let rest = " 1ca0000.dsi: reg=0x42424242 val=0x84848484";

    assert_eq!(timestamp_parser(string.as_bytes()),
               Ok((rest.as_bytes(), timestamp.as_bytes())));

    let string = "[ 1.002540 ]";
    let rest = "";
    assert_eq!(timestamp_parser(string.as_bytes()),
               Ok((rest.as_bytes(), timestamp.as_bytes())));

    let string = "[ 1.002540 ";
    assert_eq!(timestamp_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(2))));

    let string = "[";
    assert_eq!(timestamp_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(2))));

    let string = "";
    assert_eq!(timestamp_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(2))));
}
