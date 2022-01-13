// Copied and modified from:
// https://github.com/rust-lang/rust/blob/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/tests/string.rs
//
// `// "unimpl"` - unimplemented or unavailable
// `// "unsuit"` - unsuitable
// `// "altern"`  - alternative

// `TEST_CAPACITY` is defined at "../flexible_string.rs".
type FlexibleString = spdlog::string_buf::FlexibleString<TEST_CAPACITY>;

// "unimpl" pub trait IntoCow<'a, B: ?Sized>
// "unimpl" where
// "unimpl"     B: ToOwned,
// "unimpl" {
// "unimpl"     fn into_cow(self) -> Cow<'a, B>;
// "unimpl" }

// "unimpl" impl<'a> IntoCow<'a, str> for FlexibleString {
// "unimpl"     fn into_cow(self) -> Cow<'a, str> {
// "unimpl"         Cow::Owned(self)
// "unimpl"     }
// "unimpl" }

// "unimpl" impl<'a> IntoCow<'a, str> for &'a str {
// "unimpl"     fn into_cow(self) -> Cow<'a, str> {
// "unimpl"         Cow::Borrowed(self)
// "unimpl"     }
// "unimpl" }

#[test]
fn test_from_str() {
    let owned: Option<FlexibleString> = "string".parse().ok();
    assert_eq!(owned.as_deref(), Some("string"));
}

// "unimpl" #[test]
// "unimpl" fn test_from_cow_str() {
// "unimpl"     assert_eq!(FlexibleString::from(Cow::Borrowed("string")), "string");
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from(Cow::Owned(FlexibleString::from("string"))),
// "unimpl"         "string"
// "unimpl"     );
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_unsized_to_string() {
// "unimpl"     let s: &str = "abc";
// "unimpl"     let _: FlexibleString = (*s).to_string();
// "unimpl" }

#[test]
fn test_from_utf8() {
    let xs = b"hello".to_vec();
    assert_eq!(
        FlexibleString::from_utf8(xs).unwrap(),
        FlexibleString::from("hello")
    );

    let xs = "ศไทย中华Việt Nam".as_bytes().to_vec();
    assert_eq!(
        FlexibleString::from_utf8(xs).unwrap(),
        FlexibleString::from("ศไทย中华Việt Nam")
    );

    let xs = b"hello\xFF".to_vec();
    let err = FlexibleString::from_utf8(xs).unwrap_err();
    assert_eq!(err.as_bytes(), b"hello\xff");
    let err_clone = err.clone();
    assert_eq!(err, err_clone);
    assert_eq!(err.into_bytes(), b"hello\xff".to_vec());
    assert_eq!(err_clone.utf8_error().valid_up_to(), 5);
}

// "unimpl" #[test]
// "unimpl" fn test_from_utf8_lossy() {
// "unimpl"     let xs = b"hello";
// "unimpl"     let ys: Cow<'_, str> = "hello".into_cow();
// "unimpl"     assert_eq!(FlexibleString::from_utf8_lossy(xs), ys);
// "unimpl"
// "unimpl"     let xs = "ศไทย中华Việt Nam".as_bytes();
// "unimpl"     let ys: Cow<'_, str> = "ศไทย中华Việt Nam".into_cow();
// "unimpl"     assert_eq!(FlexibleString::from_utf8_lossy(xs), ys);
// "unimpl"
// "unimpl"     let xs = b"Hello\xC2 There\xFF Goodbye";
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf8_lossy(xs),
// "unimpl"         FlexibleString::from("Hello\u{FFFD} There\u{FFFD} Goodbye").into_cow()
// "unimpl"     );
// "unimpl"
// "unimpl"     let xs = b"Hello\xC0\x80 There\xE6\x83 Goodbye";
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf8_lossy(xs),
// "unimpl"         FlexibleString::from("Hello\u{FFFD}\u{FFFD} There\u{FFFD} Goodbye").into_cow()
// "unimpl"     );
// "unimpl"
// "unimpl"     let xs = b"\xF5foo\xF5\x80bar";
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf8_lossy(xs),
// "unimpl"         FlexibleString::from("\u{FFFD}foo\u{FFFD}\u{FFFD}bar").into_cow()
// "unimpl"     );
// "unimpl"
// "unimpl"     let xs = b"\xF1foo\xF1\x80bar\xF1\x80\x80baz";
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf8_lossy(xs),
// "unimpl"         FlexibleString::from("\u{FFFD}foo\u{FFFD}bar\u{FFFD}baz").into_cow()
// "unimpl"     );
// "unimpl"
// "unimpl"     let xs = b"\xF4foo\xF4\x80bar\xF4\xBFbaz";
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf8_lossy(xs),
// "unimpl"         FlexibleString::from("\u{FFFD}foo\u{FFFD}bar\u{FFFD}\u{FFFD}baz").into_cow()
// "unimpl"     );
// "unimpl"
// "unimpl"     let xs = b"\xF0\x80\x80\x80foo\xF0\x90\x80\x80bar";
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf8_lossy(xs),
// "unimpl"         FlexibleString::from("\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}foo\u{10000}bar").into_cow()
// "unimpl"     );
// "unimpl"
// "unimpl"     // surrogates
// "unimpl"     let xs = b"\xED\xA0\x80foo\xED\xBF\xBFbar";
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf8_lossy(xs),
// "unimpl"         FlexibleString::from("\u{FFFD}\u{FFFD}\u{FFFD}foo\u{FFFD}\u{FFFD}\u{FFFD}bar").into_cow()
// "unimpl"     );
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_from_utf16() {
// "unimpl"     let pairs = [
// "unimpl"         (
// "unimpl"             FlexibleString::from("𐍅𐌿𐌻𐍆𐌹𐌻𐌰\n"),
// "unimpl"             vec![
// "unimpl"                 0xd800, 0xdf45, 0xd800, 0xdf3f, 0xd800, 0xdf3b, 0xd800, 0xdf46, 0xd800, 0xdf39,
// "unimpl"                 0xd800, 0xdf3b, 0xd800, 0xdf30, 0x000a,
// "unimpl"             ],
// "unimpl"         ),
// "unimpl"         (
// "unimpl"             FlexibleString::from("𐐒𐑉𐐮𐑀𐐲𐑋 𐐏𐐲𐑍\n"),
// "unimpl"             vec![
// "unimpl"                 0xd801, 0xdc12, 0xd801, 0xdc49, 0xd801, 0xdc2e, 0xd801, 0xdc40, 0xd801, 0xdc32,
// "unimpl"                 0xd801, 0xdc4b, 0x0020, 0xd801, 0xdc0f, 0xd801, 0xdc32, 0xd801, 0xdc4d, 0x000a,
// "unimpl"             ],
// "unimpl"         ),
// "unimpl"         (
// "unimpl"             FlexibleString::from("𐌀𐌖𐌋𐌄𐌑𐌉·𐌌𐌄𐌕𐌄𐌋𐌉𐌑\n"),
// "unimpl"             vec![
// "unimpl"                 0xd800, 0xdf00, 0xd800, 0xdf16, 0xd800, 0xdf0b, 0xd800, 0xdf04, 0xd800, 0xdf11,
// "unimpl"                 0xd800, 0xdf09, 0x00b7, 0xd800, 0xdf0c, 0xd800, 0xdf04, 0xd800, 0xdf15, 0xd800,
// "unimpl"                 0xdf04, 0xd800, 0xdf0b, 0xd800, 0xdf09, 0xd800, 0xdf11, 0x000a,
// "unimpl"             ],
// "unimpl"         ),
// "unimpl"         (
// "unimpl"             FlexibleString::from("𐒋𐒘𐒈𐒑𐒛𐒒 𐒕𐒓 𐒈𐒚𐒍 𐒏𐒜𐒒𐒖𐒆 𐒕𐒆\n"),
// "unimpl"             vec![
// "unimpl"                 0xd801, 0xdc8b, 0xd801, 0xdc98, 0xd801, 0xdc88, 0xd801, 0xdc91, 0xd801, 0xdc9b,
// "unimpl"                 0xd801, 0xdc92, 0x0020, 0xd801, 0xdc95, 0xd801, 0xdc93, 0x0020, 0xd801, 0xdc88,
// "unimpl"                 0xd801, 0xdc9a, 0xd801, 0xdc8d, 0x0020, 0xd801, 0xdc8f, 0xd801, 0xdc9c, 0xd801,
// "unimpl"                 0xdc92, 0xd801, 0xdc96, 0xd801, 0xdc86, 0x0020, 0xd801, 0xdc95, 0xd801, 0xdc86,
// "unimpl"                 0x000a,
// "unimpl"             ],
// "unimpl"         ),
// "unimpl"         // Issue #12318, even-numbered non-BMP planes
// "unimpl"         (FlexibleString::from("\u{20000}"), vec![0xD840, 0xDC00]),
// "unimpl"     ];
// "unimpl"
// "unimpl"     for p in &pairs {
// "unimpl"         let (s, u) = (*p).clone();
// "unimpl"         let s_as_utf16 = s.encode_utf16().collect::<Vec<u16>>();
// "unimpl"         let u_as_string = FlexibleString::from_utf16(&u).unwrap();
// "unimpl"
// "unimpl"         assert!(core::char::decode_utf16(u.iter().cloned()).all(|r| r.is_ok()));
// "unimpl"         assert_eq!(s_as_utf16, u);
// "unimpl"
// "unimpl"         assert_eq!(u_as_string, s);
// "unimpl"         assert_eq!(FlexibleString::from_utf16_lossy(&u), s);
// "unimpl"
// "unimpl"         assert_eq!(FlexibleString::from_utf16(&s_as_utf16).unwrap(), s);
// "unimpl"         assert_eq!(u_as_string.encode_utf16().collect::<Vec<u16>>(), u);
// "unimpl"     }
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_utf16_invalid() {
// "unimpl"     // completely positive cases tested above.
// "unimpl"     // lead + eof
// "unimpl"     assert!(FlexibleString::from_utf16(&[0xD800]).is_err());
// "unimpl"     // lead + lead
// "unimpl"     assert!(FlexibleString::from_utf16(&[0xD800, 0xD800]).is_err());
// "unimpl"
// "unimpl"     // isolated trail
// "unimpl"     assert!(FlexibleString::from_utf16(&[0x0061, 0xDC00]).is_err());
// "unimpl"
// "unimpl"     // general
// "unimpl"     assert!(FlexibleString::from_utf16(&[0xD800, 0xd801, 0xdc8b, 0xD800]).is_err());
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_from_utf16_lossy() {
// "unimpl"     // completely positive cases tested above.
// "unimpl"     // lead + eof
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf16_lossy(&[0xD800]),
// "unimpl"         FlexibleString::from("\u{FFFD}")
// "unimpl"     );
// "unimpl"     // lead + lead
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf16_lossy(&[0xD800, 0xD800]),
// "unimpl"         FlexibleString::from("\u{FFFD}\u{FFFD}")
// "unimpl"     );
// "unimpl"
// "unimpl"     // isolated trail
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf16_lossy(&[0x0061, 0xDC00]),
// "unimpl"         FlexibleString::from("a\u{FFFD}")
// "unimpl"     );
// "unimpl"
// "unimpl"     // general
// "unimpl"     assert_eq!(
// "unimpl"         FlexibleString::from_utf16_lossy(&[0xD800, 0xd801, 0xdc8b, 0xD800]),
// "unimpl"         FlexibleString::from("\u{FFFD}𐒋\u{FFFD}")
// "unimpl"     );
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_push_bytes() {
// "unimpl"     let mut s = FlexibleString::from("ABC");
// "unimpl"     unsafe {
// "unimpl"         let mv = s.as_mut_vec();
// "unimpl"         mv.extend_from_slice(&[b'D']);
// "unimpl"     }
// "unimpl"     assert_eq!(s, "ABCD");
// "unimpl" }

#[test]
fn test_push_str() {
    let mut s = FlexibleString::new();
    s.push_str("");
    assert_eq!(&s[0..], "");
    s.push_str("abc");
    assert_eq!(&s[0..], "abc");
    s.push_str("ประเทศไทย中华Việt Nam");
    assert_eq!(&s[0..], "abcประเทศไทย中华Việt Nam");
}

#[test]
fn test_add_assign() {
    let mut s = FlexibleString::new();
    s += "";
    assert_eq!(s.as_str(), "");
    s += "abc";
    assert_eq!(s.as_str(), "abc");
    s += "ประเทศไทย中华Việt Nam";
    assert_eq!(s.as_str(), "abcประเทศไทย中华Việt Nam");
}

#[test]
fn test_push() {
    let mut data = FlexibleString::from("ประเทศไทย中");
    data.push('华');
    data.push('b'); // 1 byte
    data.push('¢'); // 2 byte
    data.push('€'); // 3 byte
    data.push('𤭢'); // 4 byte
    assert_eq!(data, "ประเทศไทย中华b¢€𤭢");
}

#[test]
fn test_pop() {
    let mut data = FlexibleString::from("ประเทศไทย中华b¢€𤭢");
    assert_eq!(data.pop().unwrap(), '𤭢'); // 4 bytes
    assert_eq!(data.pop().unwrap(), '€'); // 3 bytes
    assert_eq!(data.pop().unwrap(), '¢'); // 2 bytes
    assert_eq!(data.pop().unwrap(), 'b'); // 1 bytes
    assert_eq!(data.pop().unwrap(), '华');
    assert_eq!(data, "ประเทศไทย中");
}

// "unimpl" #[test]
// "unimpl" fn test_split_off_empty() {
// "unimpl"     let orig = "Hello, world!";
// "unimpl"     let mut split = FlexibleString::from(orig);
// "unimpl"     let empty: FlexibleString = split.split_off(orig.len());
// "unimpl"     assert!(empty.is_empty());
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_split_off_past_end() {
// "unimpl"     let orig = "Hello, world!";
// "unimpl"     let mut split = FlexibleString::from(orig);
// "unimpl"     let _ = split.split_off(orig.len() + 1);
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_split_off_mid_char() {
// "unimpl"     let mut shan = FlexibleString::from("山");
// "unimpl"     let _broken_mountain = shan.split_off(1);
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_split_off_ascii() {
// "unimpl"     let mut ab = FlexibleString::from("ABCD");
// "unimpl"     let orig_capacity = ab.capacity();
// "unimpl"     let cd = ab.split_off(2);
// "unimpl"     assert_eq!(ab, "AB");
// "unimpl"     assert_eq!(cd, "CD");
// "unimpl"     assert_eq!(ab.capacity(), orig_capacity);
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_split_off_unicode() {
// "unimpl"     let mut nihon = FlexibleString::from("日本語");
// "unimpl"     let orig_capacity = nihon.capacity();
// "unimpl"     let go = nihon.split_off("日本".len());
// "unimpl"     assert_eq!(nihon, "日本");
// "unimpl"     assert_eq!(go, "語");
// "unimpl"     assert_eq!(nihon.capacity(), orig_capacity);
// "unimpl" }

#[test]
fn test_str_truncate() {
    let mut s = FlexibleString::from("12345");
    s.truncate(5);
    assert_eq!(s, "12345");
    s.truncate(3);
    assert_eq!(s, "123");
    s.truncate(0);
    assert_eq!(s, "");

    let mut s = FlexibleString::from("12345");
    let p = s.as_ptr();
    s.truncate(3);
    s.push_str("6");
    let p_ = s.as_ptr();
    assert_eq!(p_, p);
}

#[test]
fn test_str_truncate_invalid_len() {
    let mut s = FlexibleString::from("12345");
    s.truncate(6);
    assert_eq!(s, "12345");
}

#[test]
#[should_panic]
fn test_str_truncate_split_codepoint() {
    let mut s = FlexibleString::from("\u{FC}"); // ü
    s.truncate(1);
}

#[test]
fn test_str_clear() {
    let mut s = FlexibleString::from("12345");
    s.clear();
    assert_eq!(s.len(), 0);
    assert_eq!(s, "");
}

#[test]
fn test_str_add() {
    let a = FlexibleString::from("12345");
    let b = a + "2";
    let b = b + "2";
    assert_eq!(b.len(), 7);
    assert_eq!(b, "1234522");
}

#[test]
fn remove() {
    let mut s = FlexibleString::from("ศไทย中华Việt Nam; foobar");
    assert_eq!(s.remove(0), 'ศ');
    assert_eq!(s.len(), 33);
    assert_eq!(s, "ไทย中华Việt Nam; foobar");
    assert_eq!(s.remove(17), 'ệ');
    assert_eq!(s, "ไทย中华Vit Nam; foobar");
}

#[test]
#[should_panic]
fn remove_bad() {
    FlexibleString::from("ศ").remove(1);
}

// "unimpl" #[test]
// "unimpl" fn test_remove_matches() {
// "unimpl"     let mut s = FlexibleString::from("abc");
// "unimpl"
// "unimpl"     s.remove_matches('b');
// "unimpl"     assert_eq!(s, "ac");
// "unimpl"     s.remove_matches('b');
// "unimpl"     assert_eq!(s, "ac");
// "unimpl"
// "unimpl"     let mut s = FlexibleString::from("abcb");
// "unimpl"
// "unimpl"     s.remove_matches('b');
// "unimpl"     assert_eq!(s, "ac");
// "unimpl"
// "unimpl"     let mut s = FlexibleString::from("ศไทย中华Việt Nam; foobarศ");
// "unimpl"     s.remove_matches('ศ');
// "unimpl"     assert_eq!(s, "ไทย中华Việt Nam; foobar");
// "unimpl"
// "unimpl"     let mut s = FlexibleString::from("");
// "unimpl"     s.remove_matches("");
// "unimpl"     assert_eq!(s, "");
// "unimpl"
// "unimpl"     let mut s = FlexibleString::from("aaaaa");
// "unimpl"     s.remove_matches('a');
// "unimpl"     assert_eq!(s, "");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_retain() {
// "unimpl"     let mut s = FlexibleString::from("α_β_γ");
// "unimpl"
// "unimpl"     s.retain(|_| true);
// "unimpl"     assert_eq!(s, "α_β_γ");
// "unimpl"
// "unimpl"     s.retain(|c| c != '_');
// "unimpl"     assert_eq!(s, "αβγ");
// "unimpl"
// "unimpl"     s.retain(|c| c != 'β');
// "unimpl"     assert_eq!(s, "αγ");
// "unimpl"
// "unimpl"     s.retain(|c| c == 'α');
// "unimpl"     assert_eq!(s, "α");
// "unimpl"
// "unimpl"     s.retain(|_| false);
// "unimpl"     assert_eq!(s, "");
// "unimpl"
// "unimpl"     let mut s = FlexibleString::from("0è0");
// "unimpl"     let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
// "unimpl"         let mut count = 0;
// "unimpl"         s.retain(|_| {
// "unimpl"             count += 1;
// "unimpl"             match count {
// "unimpl"                 1 => false,
// "unimpl"                 2 => true,
// "unimpl"                 _ => panic!(),
// "unimpl"             }
// "unimpl"         });
// "unimpl"     }));
// "unimpl"     assert!(std::str::from_utf8(s.as_bytes()).is_ok());
// "unimpl" }

#[test]
fn insert() {
    let mut s = FlexibleString::from("foobar");
    s.insert(0, 'ệ');
    assert_eq!(s, "ệfoobar");
    s.insert(6, 'ย');
    assert_eq!(s, "ệfooยbar");
}

#[test]
#[should_panic]
fn insert_bad1() {
    FlexibleString::from("").insert(1, 't');
}
#[test]
#[should_panic]
fn insert_bad2() {
    FlexibleString::from("ệ").insert(1, 't');
}

#[test]
fn test_slicing() {
    let s = FlexibleString::from("foobar");
    assert_eq!("foobar", &s[..]);
    assert_eq!("foo", &s[..3]);
    assert_eq!("bar", &s[3..]);
    assert_eq!("oob", &s[1..4]);
}

// "unimpl" #[test]
// "unimpl" fn test_simple_types() {
// "unimpl"     assert_eq!(1.to_string(), "1");
// "unimpl"     assert_eq!((-1).to_string(), "-1");
// "unimpl"     assert_eq!(200.to_string(), "200");
// "unimpl"     assert_eq!(2.to_string(), "2");
// "unimpl"     assert_eq!(true.to_string(), "true");
// "unimpl"     assert_eq!(false.to_string(), "false");
// "unimpl"     assert_eq!(("hi".to_string()).to_string(), "hi");
// "unimpl" }

#[test]
fn test_vectors() {
    let x: Vec<i32> = vec![];
    assert_eq!(format!("{:?}", x), "[]");
    assert_eq!(format!("{:?}", vec![1]), "[1]");
    assert_eq!(format!("{:?}", vec![1, 2, 3]), "[1, 2, 3]");
    assert!(format!("{:?}", vec![vec![], vec![1], vec![1, 1]]) == "[[], [1], [1, 1]]");
}

#[test]
fn test_from_iterator() {
    let s = FlexibleString::from("ศไทย中华Việt Nam");
    let t = "ศไทย中华";
    let u = "Việt Nam";

    let a: FlexibleString = s.chars().collect();
    assert_eq!(s, a);

    let mut b = FlexibleString::from(t);
    b.extend(u.chars());
    assert_eq!(s, b);

    let c: FlexibleString = vec![t, u].into_iter().collect();
    assert_eq!(s, c);

    let mut d = FlexibleString::from(t);
    d.extend(vec![u]);
    assert_eq!(s, d);
}

// "unimpl" #[test]
// "unimpl" fn test_drain() {
// "unimpl"     let mut s = FlexibleString::from("αβγ");
// "unimpl"     assert_eq!(s.drain(2..4).collect::<FlexibleString>(), "β");
// "unimpl"     assert_eq!(s, "αγ");
// "unimpl"
// "unimpl"     let mut t = FlexibleString::from("abcd");
// "unimpl"     t.drain(..0);
// "unimpl"     assert_eq!(t, "abcd");
// "unimpl"     t.drain(..1);
// "unimpl"     assert_eq!(t, "bcd");
// "unimpl"     t.drain(3..);
// "unimpl"     assert_eq!(t, "bcd");
// "unimpl"     t.drain(..);
// "unimpl"     assert_eq!(t, "");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_drain_start_overflow() {
// "unimpl"     let mut s = FlexibleString::from("abc");
// "unimpl"     s.drain((Excluded(usize::MAX), Included(0)));
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_drain_end_overflow() {
// "unimpl"     let mut s = FlexibleString::from("abc");
// "unimpl"     s.drain((Included(0), Included(usize::MAX)));
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_replace_range() {
// "unimpl"     let mut s = "Hello, world!".to_owned();
// "unimpl"     s.replace_range(7..12, "世界");
// "unimpl"     assert_eq!(s, "Hello, 世界!");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_replace_range_char_boundary() {
// "unimpl"     let mut s = "Hello, 世界!".to_owned();
// "unimpl"     s.replace_range(..8, "");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_replace_range_inclusive_range() {
// "unimpl"     let mut v = FlexibleString::from("12345");
// "unimpl"     v.replace_range(2..=3, "789");
// "unimpl"     assert_eq!(v, "127895");
// "unimpl"     v.replace_range(1..=2, "A");
// "unimpl"     assert_eq!(v, "1A895");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_replace_range_out_of_bounds() {
// "unimpl"     let mut s = FlexibleString::from("12345");
// "unimpl"     s.replace_range(5..6, "789");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_replace_range_inclusive_out_of_bounds() {
// "unimpl"     let mut s = FlexibleString::from("12345");
// "unimpl"     s.replace_range(5..=5, "789");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_replace_range_start_overflow() {
// "unimpl"     let mut s = FlexibleString::from("123");
// "unimpl"     s.replace_range((Excluded(usize::MAX), Included(0)), "");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[should_panic]
// "unimpl" fn test_replace_range_end_overflow() {
// "unimpl"     let mut s = FlexibleString::from("456");
// "unimpl"     s.replace_range((Included(0), Included(usize::MAX)), "");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_replace_range_empty() {
// "unimpl"     let mut s = FlexibleString::from("12345");
// "unimpl"     s.replace_range(1..2, "");
// "unimpl"     assert_eq!(s, "1345");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_replace_range_unbounded() {
// "unimpl"     let mut s = FlexibleString::from("12345");
// "unimpl"     s.replace_range(.., "");
// "unimpl"     assert_eq!(s, "");
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_replace_range_evil_start_bound() {
// "unimpl"     struct EvilRange(Cell<bool>);
// "unimpl"
// "unimpl"     impl RangeBounds<usize> for EvilRange {
// "unimpl"         fn start_bound(&self) -> Bound<&usize> {
// "unimpl"             Bound::Included(if self.0.get() {
// "unimpl"                 &1
// "unimpl"             } else {
// "unimpl"                 self.0.set(true);
// "unimpl"                 &0
// "unimpl"             })
// "unimpl"         }
// "unimpl"         fn end_bound(&self) -> Bound<&usize> {
// "unimpl"             Bound::Unbounded
// "unimpl"         }
// "unimpl"     }
// "unimpl"
// "unimpl"     let mut s = FlexibleString::from("🦀");
// "unimpl"     s.replace_range(EvilRange(Cell::new(false)), "");
// "unimpl"     assert_eq!(Ok(""), str::from_utf8(s.as_bytes()));
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_replace_range_evil_end_bound() {
// "unimpl"     struct EvilRange(Cell<bool>);
// "unimpl"
// "unimpl"     impl RangeBounds<usize> for EvilRange {
// "unimpl"         fn start_bound(&self) -> Bound<&usize> {
// "unimpl"             Bound::Included(&0)
// "unimpl"         }
// "unimpl"         fn end_bound(&self) -> Bound<&usize> {
// "unimpl"             Bound::Excluded(if self.0.get() {
// "unimpl"                 &3
// "unimpl"             } else {
// "unimpl"                 self.0.set(true);
// "unimpl"                 &4
// "unimpl"             })
// "unimpl"         }
// "unimpl"     }
// "unimpl"
// "unimpl"     let mut s = FlexibleString::from("🦀");
// "unimpl"     s.replace_range(EvilRange(Cell::new(false)), "");
// "unimpl"     assert_eq!(Ok(""), str::from_utf8(s.as_bytes()));
// "unimpl" }

#[test]
fn test_extend_ref() {
    let mut a = FlexibleString::from("foo");
    a.extend(&['b', 'a', 'r']);

    assert_eq!(&a, "foobar");
}

// "unimpl" #[test]
// "unimpl" fn test_into_boxed_str() {
// "unimpl"     let xs = FlexibleString::from("hello my name is bob");
// "unimpl"     let ys = xs.into_boxed_str();
// "unimpl"     assert_eq!(&*ys, "hello my name is bob");
// "unimpl" }

#[test]
fn test_reserve_exact() {
    // This is all the same as test_reserve

    let mut s = FlexibleString::new();
    // "unsuit" assert_eq!(s.capacity(), 0);
    assert_eq!(s.capacity(), TEST_CAPACITY); // "altern"

    s.reserve_exact(2);
    assert!(s.capacity() >= 2);

    for _i in 0..16 {
        s.push('0');
    }

    assert!(s.capacity() >= 16);
    s.reserve_exact(16);
    assert!(s.capacity() >= 32);

    s.push('0');

    s.reserve_exact(16);
    assert!(s.capacity() >= 33)
}

// "unimpl" #[test]
// "unimpl" #[cfg_attr(miri, ignore)] // Miri does not support signalling OOM
// "unimpl" #[cfg_attr(target_os = "android", ignore)] // Android used in CI has a broken dlmalloc
// "unimpl" fn test_try_reserve() {
// "unimpl"     // These are the interesting cases:
// "unimpl"     // * exactly isize::MAX should never trigger a CapacityOverflow (can be OOM)
// "unimpl"     // * > isize::MAX should always fail
// "unimpl"     //    * On 16/32-bit should CapacityOverflow
// "unimpl"     //    * On 64-bit should OOM
// "unimpl"     // * overflow may trigger when adding `len` to `cap` (in number of elements)
// "unimpl"     // * overflow may trigger when multiplying `new_cap` by size_of::<T> (to get bytes)
// "unimpl"
// "unimpl"     const MAX_CAP: usize = isize::MAX as usize;
// "unimpl"     const MAX_USIZE: usize = usize::MAX;
// "unimpl"
// "unimpl"     // On 16/32-bit, we check that allocations don't exceed isize::MAX,
// "unimpl"     // on 64-bit, we assume the OS will give an OOM for such a ridiculous size.
// "unimpl"     // Any platform that succeeds for these requests is technically broken with
// "unimpl"     // ptr::offset because LLVM is the worst.
// "unimpl"     let guards_against_isize = usize::BITS < 64;
// "unimpl"
// "unimpl"     {
// "unimpl"         // Note: basic stuff is checked by test_reserve
// "unimpl"         let mut empty_string: FlexibleString = FlexibleString::new();
// "unimpl"
// "unimpl"         // Check isize::MAX doesn't count as an overflow
// "unimpl"         if let Err(CapacityOverflow) = empty_string.try_reserve(MAX_CAP).map_err(|e| e.kind()) {
// "unimpl"             panic!("isize::MAX shouldn't trigger an overflow!");
// "unimpl"         }
// "unimpl"         // Play it again, frank! (just to be sure)
// "unimpl"         if let Err(CapacityOverflow) = empty_string.try_reserve(MAX_CAP).map_err(|e| e.kind()) {
// "unimpl"             panic!("isize::MAX shouldn't trigger an overflow!");
// "unimpl"         }
// "unimpl"
// "unimpl"         if guards_against_isize {
// "unimpl"             // Check isize::MAX + 1 does count as overflow
// "unimpl"             assert_matches!(
// "unimpl"                 empty_string.try_reserve(MAX_CAP + 1).map_err(|e| e.kind()),
// "unimpl"                 Err(CapacityOverflow),
// "unimpl"                 "isize::MAX + 1 should trigger an overflow!"
// "unimpl"             );
// "unimpl"
// "unimpl"             // Check usize::MAX does count as overflow
// "unimpl"             assert_matches!(
// "unimpl"                 empty_string.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
// "unimpl"                 Err(CapacityOverflow),
// "unimpl"                 "usize::MAX should trigger an overflow!"
// "unimpl"             );
// "unimpl"         } else {
// "unimpl"             // Check isize::MAX + 1 is an OOM
// "unimpl"             assert_matches!(
// "unimpl"                 empty_string.try_reserve(MAX_CAP + 1).map_err(|e| e.kind()),
// "unimpl"                 Err(AllocError { .. }),
// "unimpl"                 "isize::MAX + 1 should trigger an OOM!"
// "unimpl"             );
// "unimpl"
// "unimpl"             // Check usize::MAX is an OOM
// "unimpl"             assert_matches!(
// "unimpl"                 empty_string.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
// "unimpl"                 Err(AllocError { .. }),
// "unimpl"                 "usize::MAX should trigger an OOM!"
// "unimpl"             );
// "unimpl"         }
// "unimpl"     }
// "unimpl"
// "unimpl"     {
// "unimpl"         // Same basic idea, but with non-zero len
// "unimpl"         let mut ten_bytes: FlexibleString = FlexibleString::from("0123456789");
// "unimpl"
// "unimpl"         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).map_err(|e| e.kind()) {
// "unimpl"             panic!("isize::MAX shouldn't trigger an overflow!");
// "unimpl"         }
// "unimpl"         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).map_err(|e| e.kind()) {
// "unimpl"             panic!("isize::MAX shouldn't trigger an overflow!");
// "unimpl"         }
// "unimpl"         if guards_against_isize {
// "unimpl"             assert_matches!(
// "unimpl"                 ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
// "unimpl"                 Err(CapacityOverflow),
// "unimpl"                 "isize::MAX + 1 should trigger an overflow!"
// "unimpl"             );
// "unimpl"         } else {
// "unimpl"             assert_matches!(
// "unimpl"                 ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
// "unimpl"                 Err(AllocError { .. }),
// "unimpl"                 "isize::MAX + 1 should trigger an OOM!"
// "unimpl"             );
// "unimpl"         }
// "unimpl"         // Should always overflow in the add-to-len
// "unimpl"         assert_matches!(
// "unimpl"             ten_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
// "unimpl"             Err(CapacityOverflow),
// "unimpl"             "usize::MAX should trigger an overflow!"
// "unimpl"         );
// "unimpl"     }
// "unimpl" }

// "unimpl" #[test]
// "unimpl" #[cfg_attr(miri, ignore)] // Miri does not support signalling OOM
// "unimpl" #[cfg_attr(target_os = "android", ignore)] // Android used in CI has a broken dlmalloc
// "unimpl" fn test_try_reserve_exact() {
// "unimpl"     // This is exactly the same as test_try_reserve with the method changed.
// "unimpl"     // See that test for comments.
// "unimpl"
// "unimpl"     const MAX_CAP: usize = isize::MAX as usize;
// "unimpl"     const MAX_USIZE: usize = usize::MAX;
// "unimpl"
// "unimpl"     let guards_against_isize = usize::BITS < 64;
// "unimpl"
// "unimpl"     {
// "unimpl"         let mut empty_string: FlexibleString = FlexibleString::new();
// "unimpl"
// "unimpl"         if let Err(CapacityOverflow) = empty_string
// "unimpl"             .try_reserve_exact(MAX_CAP)
// "unimpl"             .map_err(|e| e.kind())
// "unimpl"         {
// "unimpl"             panic!("isize::MAX shouldn't trigger an overflow!");
// "unimpl"         }
// "unimpl"         if let Err(CapacityOverflow) = empty_string
// "unimpl"             .try_reserve_exact(MAX_CAP)
// "unimpl"             .map_err(|e| e.kind())
// "unimpl"         {
// "unimpl"             panic!("isize::MAX shouldn't trigger an overflow!");
// "unimpl"         }
// "unimpl"
// "unimpl"         if guards_against_isize {
// "unimpl"             assert_matches!(
// "unimpl"                 empty_string
// "unimpl"                     .try_reserve_exact(MAX_CAP + 1)
// "unimpl"                     .map_err(|e| e.kind()),
// "unimpl"                 Err(CapacityOverflow),
// "unimpl"                 "isize::MAX + 1 should trigger an overflow!"
// "unimpl"             );
// "unimpl"
// "unimpl"             assert_matches!(
// "unimpl"                 empty_string
// "unimpl"                     .try_reserve_exact(MAX_USIZE)
// "unimpl"                     .map_err(|e| e.kind()),
// "unimpl"                 Err(CapacityOverflow),
// "unimpl"                 "usize::MAX should trigger an overflow!"
// "unimpl"             );
// "unimpl"         } else {
// "unimpl"             assert_matches!(
// "unimpl"                 empty_string
// "unimpl"                     .try_reserve_exact(MAX_CAP + 1)
// "unimpl"                     .map_err(|e| e.kind()),
// "unimpl"                 Err(AllocError { .. }),
// "unimpl"                 "isize::MAX + 1 should trigger an OOM!"
// "unimpl"             );
// "unimpl"
// "unimpl"             assert_matches!(
// "unimpl"                 empty_string
// "unimpl"                     .try_reserve_exact(MAX_USIZE)
// "unimpl"                     .map_err(|e| e.kind()),
// "unimpl"                 Err(AllocError { .. }),
// "unimpl"                 "usize::MAX should trigger an OOM!"
// "unimpl"             );
// "unimpl"         }
// "unimpl"     }
// "unimpl"
// "unimpl"     {
// "unimpl"         let mut ten_bytes: FlexibleString = FlexibleString::from("0123456789");
// "unimpl"
// "unimpl"         if let Err(CapacityOverflow) = ten_bytes
// "unimpl"             .try_reserve_exact(MAX_CAP - 10)
// "unimpl"             .map_err(|e| e.kind())
// "unimpl"         {
// "unimpl"             panic!("isize::MAX shouldn't trigger an overflow!");
// "unimpl"         }
// "unimpl"         if let Err(CapacityOverflow) = ten_bytes
// "unimpl"             .try_reserve_exact(MAX_CAP - 10)
// "unimpl"             .map_err(|e| e.kind())
// "unimpl"         {
// "unimpl"             panic!("isize::MAX shouldn't trigger an overflow!");
// "unimpl"         }
// "unimpl"         if guards_against_isize {
// "unimpl"             assert_matches!(
// "unimpl"                 ten_bytes
// "unimpl"                     .try_reserve_exact(MAX_CAP - 9)
// "unimpl"                     .map_err(|e| e.kind()),
// "unimpl"                 Err(CapacityOverflow),
// "unimpl"                 "isize::MAX + 1 should trigger an overflow!"
// "unimpl"             );
// "unimpl"         } else {
// "unimpl"             assert_matches!(
// "unimpl"                 ten_bytes
// "unimpl"                     .try_reserve_exact(MAX_CAP - 9)
// "unimpl"                     .map_err(|e| e.kind()),
// "unimpl"                 Err(AllocError { .. }),
// "unimpl"                 "isize::MAX + 1 should trigger an OOM!"
// "unimpl"             );
// "unimpl"         }
// "unimpl"         assert_matches!(
// "unimpl"             ten_bytes.try_reserve_exact(MAX_USIZE).map_err(|e| e.kind()),
// "unimpl"             Err(CapacityOverflow),
// "unimpl"             "usize::MAX should trigger an overflow!"
// "unimpl"         );
// "unimpl"     }
// "unimpl" }

// "unimpl" #[test]
// "unimpl" fn test_from_char() {
// "unimpl"     assert_eq!(FlexibleString::from('a'), 'a'.to_string());
// "unimpl"     let s: FlexibleString = 'x'.into();
// "unimpl"     assert_eq!(s, 'x'.to_string());
// "unimpl" }

#[test]
fn test_str_concat() {
    let a: FlexibleString = FlexibleString::from("hello");
    let b: FlexibleString = FlexibleString::from("world");
    let s: FlexibleString = format!("{}{}", a, b).into();
    assert_eq!(s.as_bytes()[9], b'd');
}
