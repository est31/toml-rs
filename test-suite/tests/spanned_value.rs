extern crate toml;

use std::collections::HashMap;
use toml::SpannedValue;
use toml::value::Datetime;

/// A set of good datetimes.
pub fn good_datetimes() -> Vec<&'static str> {
    let mut v = Vec::new();
    v.push("1997-09-09T09:09:09Z");
    v.push("1997-09-09T09:09:09+09:09");
    v.push("1997-09-09T09:09:09-09:09");
    v.push("1997-09-09T09:09:09");
    v.push("1997-09-09");
    v.push("09:09:09");
    v.push("1997-09-09T09:09:09.09Z");
    v.push("1997-09-09T09:09:09.09+09:09");
    v.push("1997-09-09T09:09:09.09-09:09");
    v.push("1997-09-09T09:09:09.09");
    v.push("09:09:09.09");
    v
}

#[test]
fn test_spanned_field() {
    fn good<'de, T>(s: &'de str, expected: &str)
    where
        T: serde::Deserialize<'de>,
    {
        let foo: SpannedValue = toml::from_str(s).unwrap();

        assert_eq!(6, foo.start());
        assert_eq!(s.len(), foo.end());
        assert_eq!(expected, &s[foo.start()..foo.end()]);
    }

    good::<String>("foo = \"foo\"", "\"foo\"");
    good::<u32>("foo = 42", "42");
    // leading plus
    good::<u32>("foo = +42", "+42");
    // table
    good::<HashMap<String, u32>>(
        "foo = {\"foo\" = 42, \"bar\" = 42}",
        "{\"foo\" = 42, \"bar\" = 42}",
    );
    // array
    good::<Vec<u32>>("foo = [0, 1, 2, 3, 4]", "[0, 1, 2, 3, 4]");
    // datetime
    good::<String>("foo = \"1997-09-09T09:09:09Z\"", "\"1997-09-09T09:09:09Z\"");

    for expected in good_datetimes() {
        let s = format!("foo = {}", expected);
        good::<Datetime>(&s, expected);
    }
}
