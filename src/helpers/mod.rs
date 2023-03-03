pub fn json_to_html(json: String) -> String {
    let mut html = String::from("<div>{html}</div>");

    html = html
        .replace("{html}", &json.replace('\n', "<br>"))
        .replace(' ', "&nbsp;");

    html
}
