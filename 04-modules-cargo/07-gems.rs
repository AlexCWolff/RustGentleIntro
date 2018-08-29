// gems.rs

/* When processing anything except simple text, regular expressions make your life much easier. To use the regex crate, put 'regex = "0.2.1"' after "[dependencies]" in your Cargo.toml. */

extern crate regex;
use regex::Regex;

// Match exactly two digits, the character ':', and then any number of digits. Capture both sets of digits
let re = Regex::new(r"(\d{2}):(\d+)").unwrap();
println!("{:?}", re.captures("  10:230"));
println!("{:?}", re.captures("[22:2]"));
println!("{:?}", re.captures("10:x23"));
// Some(Captures({0: Some("10:230"), 1: Some("10"), 2: Some("230")}))
// Some(Captures({0: Some("22:2"), 1: Some("22"), 2: Some("2")}))
// None

/* The successful output actually has three captures: the whole match, and the two sets of digits. These regular expressions are not anchored by default, so regex will hunt for the first occurring match skipping anything that doesn't match. If you left out the '()' it would just give us the whole match.

It's possible to name those captures and spread the regular expression over several lines, even including comments. Compiling the regex might fail (the first expect) or the match might fail (the second expect). Here we can use the result as an associative array and look up captures by name. */

let re = Regex::new(r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
").expect("bad regex");
let caps = re.captures("2010-03-14").expect("match failed");

assert_eq!("2010", &caps["year"]);
assert_eq!("03", &caps["month"]);
assert_eq!("14", &caps["day"]);

/* Regular expressions can break up strings that match a pattern, but won't check whether they make sense. You can specify and match the syntax of ISO-style dates but semantically they may be nonsense, like "2014-24-52". For this you need dedicated date-time processing, which is provided by chrono. You need to decide on a time zone when doing dates. */

extern crate chrono;
use chrono::*;

fn main() {
    let date = Local.ymd(2010,3,14);
    println!("date was {}", date);
}
// date was 2010-03-14+02:00

// This isn't recommended because feeding it bad dates will cause a panic. The method you need here is 'ymd_opt' which returns 'LocalResult<Date>'

let date = Local.ymd_opt(2010,3,14);
println!("date was {:?}", date);
// date was Single(2010-03-14+02:00)

let date = Local.ymd_opt(2014,24,52);
println!("date was {:?}", date);
// date was None

/* You can also directly parse date-times, either in standard UTC format or using custom formats. These self-same formats allow you to print out dates in exactly the format you want.

I highlighted these two crates because they would be part of the standard library in most other languages. These beginnings of these crates were once part of the Rust stdlib, but were cut. This was a deliberate decision, the Rust team takes stdlib stability very seriously so features only arrive in stable once they have gone through incubation in unstable nightly versions, and only then beta and stable. For libraries that need experimentation and refinement, it's much better that they remain independent and get tracked with Cargo. For all practical purposes, these two crates are standard - they are not going away and may be folded back into the stdlib at some point. */