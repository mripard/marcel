#[macro_use]
extern crate nom;

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
