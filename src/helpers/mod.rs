use serde::Serialize;

pub fn json_to_html(json: String) -> String {
    let mut html = String::from("<div>{html}</div>");

    html = html
        .replace("{html}", &json.replace('\n', "<br>"))
        .replace(' ', "&nbsp;");

    html
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
