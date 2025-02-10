const FONT_WEIGHT: &str = r#"font-weight="500""#;
const FONT_FAMILY: &str = r#"font-family="'Open Sans', arial, sans-serif""#;

pub fn text(x: f64, y: f64, color: & str, font_size: f64, anchor: &str, text: &str) -> String {
    // text-anchor can be "middle", "end" or "start"
    format!(
        r#"<text text-anchor="{}" x="{:.4}" y="{:.4}" fill="{}" font-size="{:.4}" {FONT_WEIGHT} {FONT_FAMILY}>{}</text>"#,
        anchor, x, y, color, font_size, text
    )
}

// fn text_vertical(x: f64, y: f64, color: & str, font_size: f64, anchor: &str, text: &str) -> String {
//     format!(
//         r#"<text style="writing-mode: tb;" text-anchor="{}" x="{:.4}" y="{:.4}" fill="{}" font-size="{:.4}" font-weight="500" font-family="{FONT_FAMILY}">{}</text>"#,
//         anchor, x, y, color, font_size, text
//     )
// }

pub fn rect(x: f64, y: f64, width: f64, height: f64, opacity: f64, color: &str) -> String {
    format!(
        r#"<rect x="{:.4}" y="{:.4}" width="{:.4}" height="{:.4}" style="fill-opacity: {:.4}; fill: {}; " />"#,
        x, y, width, height, opacity, color,
    )
}

pub fn line(x1: f64, x2: f64, y1: f64, y2: f64, color: &str, width: f64) -> String {
    format!(
        r#"<line x1="{:.4}" x2="{:.4}" y1="{:.4}" y2="{:.4}" stroke="{}" stroke-width="{:.4}" />"#,
        x1, x2, y1, y2, color, width
    )
}
