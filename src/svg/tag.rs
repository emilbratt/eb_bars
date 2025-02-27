const FONT_WEIGHT: &str = r#"font-weight="500""#;
const FONT_FAMILY: &str = r#"font-family="'Open Sans', arial, sans-serif""#;

pub fn text(x: f64, y: f64, color: & str, font_size: f64, anchor: &str, text: &str) -> String {
    // NOTE: valid text-anchor values are "middle", "end" and "start"
    format!(
        r#"<text text-anchor="{}" x="{:.3}" y="{:.3}" fill="{}" font-size="{:.3}" {FONT_WEIGHT} {FONT_FAMILY}>{}</text>"#,
        anchor, x, y, color, font_size, text
    )
}

pub fn text_top_down(x: f64, y: f64, color: & str, font_size: f64, anchor: &str, text: &str) -> String {
    format!(
        r#"<text style="writing-mode: tb;" text-anchor="{}" x="{:.3}" y="{:.3}" fill="{}" font-size="{:.3}" {FONT_WEIGHT} {FONT_FAMILY}>{}</text>"#,
        anchor, x, y, color, font_size, text
    )
}

pub fn text_bottom_up(x: f64, y: f64, color: & str, font_size: f64, anchor: &str, text: &str) -> String {
    format!(
        r#"<text text-anchor="{}" transform="translate({:.3}, {:.3}) rotate(270)" fill="{}" font-size="{:.3}" {FONT_WEIGHT} {FONT_FAMILY}>{}</text>"#,
        anchor, x, y, color, font_size, text
    )

    // A possible use of contac! and multiline for keeping code length < 120 collumns; might use it later.
    // format!(
    //     concat!(
    //         r#"<text text-anchor="{}" "#,
    //         r#"transform="translate({:.3}, {:.3}) rotate(270)" "#,
    //         r#"fill="{}" font-size="{:.3}" {} {}>""#,
    //         "{}</text>",
    //     )
    //     ,anchor, x, y, color, font_size, FONT_WEIGHT, FONT_FAMILY, text,
    // )
}

pub fn rect(x: f64, y: f64, width: f64, height: f64, opacity: f64, color: &str) -> String {
    format!(
        r#"<rect x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" style="fill-opacity: {:.3}; fill: {}; " />"#,
        x, y, width, height, opacity, color,
    )
}

pub fn line(x1: f64, x2: f64, y1: f64, y2: f64, color: &str, width: f64) -> String {
    format!(
        r#"<line x1="{:.3}" x2="{:.3}" y1="{:.3}" y2="{:.3}" stroke="{}" stroke-width="{:.3}" />"#,
        x1, x2, y1, y2, color, width
    )
}
