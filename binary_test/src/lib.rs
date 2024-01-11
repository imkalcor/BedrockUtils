///
/// This test tests the encoding and decoding of conditional fields with the `[skip]` attribute.
///
#[test]
fn test_conditional() {
    use binary::datatypes::U16;
    use binary::Binary;
    use binary_derive::Binary;
    use byteorder::LE;
    use bytes::BytesMut;
    use std::io::{Cursor, Write};

    #[derive(Debug, Binary)]
    struct Test {
        #[skip]
        short: U16<LE>,
    }

    let mut bytes = BytesMut::new();

    let test = Test {
        short: U16::new(100),
    };

    test.serialize(&mut bytes);

    let mut reader = Cursor::new(&bytes[..]);
    let test2 = Test::deserialize(&mut reader).unwrap();

    assert_eq!(test2.short.0, 0);
    assert_eq!(test.short.0, 100);
}

///
/// This test tests the encoding and decoding of some fields.
///
#[test]
fn test_serde() {
    use binary::datatypes::{U16, U24, U8};
    use binary::prefixed::Str;
    use binary::Binary;
    use binary_derive::Binary;
    use byteorder::{BE, LE};
    use bytes::BytesMut;
    use std::env;
    use std::io::{Cursor, Write};

    env::set_var("RUST_BACKTRACE", "1");

    #[derive(Debug, Binary)]
    struct Test<'a> {
        byte: U8,
        short: U16<LE>,
        str: Str<'a, U16<LE>>,
        u24: U24<BE>,
    }

    let mut bytes = BytesMut::new();

    let ser = Test {
        byte: U8::new(10),
        short: U16::new(100),
        str: Str::new("Hello world"),
        u24: U24::new(102),
    };

    ser.serialize(&mut bytes);

    println!("{:?}", bytes.to_vec());

    let mut reader = Cursor::new(&bytes[..]);
    let de = Test::deserialize(&mut reader).unwrap();

    assert_eq!(ser.short, de.short);
    assert_eq!(ser.byte, de.byte);
    assert_eq!(ser.str, de.str);
    assert_eq!(ser.u24, de.u24);
}
