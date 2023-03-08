use maud::{html, Markup};
use serde::Serialize;
use serde_json::{Map, Value};

pub fn value_to_html(value: &Value) -> Markup {
    html! {
        div style=(style("dark", "wrapper")) {
            (to_html("dark", value))
        }
    }
}

pub fn get_json<T: Serialize>(value: &T, pretty_print: bool) -> String {
    match pretty_print {
        true => match serde_json::to_string_pretty(&value) {
            Ok(json) => json,
            Err(e) => panic!("{}", e),
        },
        false => match serde_json::to_string(&value) {
            Ok(json) => json,
            Err(e) => panic!("{}", e),
        },
    }
}

fn to_html(theme: &str, value: &Value) -> Markup {
    match value {
        Value::Null => html!("null"),
        Value::Bool(b) => html!((with_raw(format!("{}", b)))),
        Value::Number(n) => number(theme, format!("{}", n)),
        Value::String(s) => string(theme, s),
        Value::Array(a) => array(theme, a),
        Value::Object(obj) => object(theme, obj),
    }
}

fn with_raw<S: ToString>(s: S) -> Markup {
    html! { (maud::PreEscaped(s.to_string())) }
}

fn string<S: ToString>(theme: &str, s: S) -> Markup {
    html! {
        { span style=(style(theme, "quote")) { (with_raw('"')) } }
        (text(theme, "string", s))
        { span style=(style(theme, "quote")) { (with_raw('"')) } }
    }
}

fn text<S: ToString>(theme: &str, style_key: &str, s: S) -> Markup {
    html! { span style=(style(theme, style_key)) { (with_raw(s)) } }
}

fn number<S: ToString>(theme: &str, s: S) -> Markup {
    html! { span style=(style(theme, "number")) { (with_raw(s)) } }
}

fn array(theme: &str, a: &[Value]) -> Markup {
    html! {
        div {
            @for (key, item) in a.iter().enumerate() {
                div style=(style(theme, "array_item_container")) {
                    div style="display: flex; align-items: center;" {
                        (with_raw('[')) (number(theme, key)) (with_raw("] => ")) (text(theme, "curly_brace", "&nbsp;{"))
                    }
                    div style=(style(theme, "indent")) { (to_html(theme, item)) }
                    div { (text(theme, "curly_brace", '}')) }
                }
            }
        }
    }
}

fn object(theme: &str, obj: &Map<String, Value>) -> Markup {
    html! {
        div {
            table {
                @for (key, value) in obj {
                    tr {
                        td style=(style(theme, "key")) { strong { (format!("{}: ", key)) } }
                        td { (to_html(theme, value)) }
                    }
                }
            }
        }
    }
}

// TODO: Extract this theming stuff to something more robust

fn style(theme: &str, key: &str) -> String {
    match theme {
        "dark" => dark(key),
        _ => panic!("Theme {theme} does not exist!"),
    }
}

fn dark(key: &str) -> String {
    match key {
        "wrapper" => "font: 12px Menlo, Monaco, Consolas, monospace;",
        "number" => "font-weight: bold; color: #1299DA;",
        "string" => "font-weight: bold; color: #56DB3A;",
        "quote" => "font-weight: bold; color: #aaa",
        "key" => "padding-right: 1rem; color: #ccc",
        "indent" => "padding-left: 1.5rem;",
        "array_item_container" => "padding-bottom: 1rem;",
        "curly_brace" => "color: cyan;",
        _ => "",
    }
    .to_string()
}
