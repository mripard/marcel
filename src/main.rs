#[macro_use]
extern crate nom;

use nom::is_hex_digit;

struct RegisterWrite {
    register:	u32,
    timestamp:	String,
    value:	u32,
}

impl fmt::Debug for RegisterWrite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "timestamp: {:?} reg {:08x?} value {:08x?}",
               self.timestamp, self.register, self.value)
    }
}

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

named!(hex_parser<&[u8], u32>,
       do_parse!(
           tag!("0x") >>
           _hex: take_while_m_n!(1, 8, is_hex_digit) >>
           (u32::from_str_radix(str::from_utf8(_hex).unwrap(), 16).unwrap())
       )
);

#[test]
fn hex_parser_test() {
    let string = "0x42424242\n";
    let rest = "\n";
    assert_eq!(hex_parser(string.as_bytes()),
               Ok((rest.as_bytes(), 0x42424242)));

    let string = "0x42424242";
    let rest = "";
    assert_eq!(hex_parser(string.as_bytes()),
               Ok((rest.as_bytes(), 0x42424242)));

    let string = "0x";
    assert_eq!(hex_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(1))));
}

named!(register_parser<&[u8], u32>,
       do_parse!(
           tag!("reg=") >>
           _hex: hex_parser >>
           (_hex)
       )
);

#[test]
fn register_parser_test() {
    let string = "reg=0x42424242";
    let rest = "";
    assert_eq!(register_parser(string.as_bytes()),
               Ok((rest.as_bytes(), 0x42424242)));

    let string = "reg=0x";
    assert_eq!(register_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(1))));

    let string = "reg=";
    assert_eq!(register_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(2))));

    let string = "";
    assert_eq!(register_parser(string.as_bytes()),
               Err(nom::Err::Incomplete(nom::Needed::Size(4))));
}
