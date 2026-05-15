use use_escape::{
    escape_csv_field, escape_html, escape_json_string, escape_shell_basic, escape_xml,
    needs_csv_escape, needs_html_escape, needs_json_escape, unescape_html, unescape_json_string,
    unescape_xml,
};

#[test]
fn html_escaping_works() {
    assert_eq!(
        escape_html("<tag attr=\"1\">&</tag>"),
        "&lt;tag attr=&quot;1&quot;&gt;&amp;&lt;/tag&gt;"
    );
    assert!(needs_html_escape("<tag>"));
}

#[test]
fn html_unescaping_works() {
    assert_eq!(unescape_html("&lt;tag&gt;&#39;&amp;&#x41;"), "<tag>'&A");
}

#[test]
fn xml_escaping_works() {
    assert_eq!(
        escape_xml("<tag attr='1'>"),
        "&lt;tag attr=&apos;1&apos;&gt;"
    );
}

#[test]
fn xml_unescaping_works() {
    assert_eq!(
        unescape_xml("&lt;tag attr=&apos;1&apos;&gt;"),
        "<tag attr='1'>"
    );
}

#[test]
fn json_string_escaping_works() {
    assert_eq!(
        escape_json_string("line\n\"quote\""),
        "line\\n\\\"quote\\\""
    );
    assert!(needs_json_escape("line\n"));
}

#[test]
fn json_string_unescaping_works() {
    assert_eq!(
        unescape_json_string("line\\n\\u0041"),
        Some("line\nA".to_string())
    );
}

#[test]
fn csv_field_escaping_works() {
    assert_eq!(escape_csv_field("a,\"b\""), "\"a,\"\"b\"\"\"");
    assert!(needs_csv_escape("a,b"));
}

#[test]
fn shell_escaping_works() {
    assert_eq!(escape_shell_basic("a'b"), "'a'\"'\"'b'");
}

#[test]
fn empty_input_is_supported() {
    assert_eq!(escape_html(""), "");
    assert_eq!(unescape_html(""), "");
    assert_eq!(escape_xml(""), "");
    assert_eq!(unescape_xml(""), "");
    assert_eq!(escape_json_string(""), "");
    assert_eq!(unescape_json_string(""), Some(String::new()));
    assert_eq!(escape_csv_field(""), "");
    assert_eq!(escape_shell_basic(""), "''");
}
