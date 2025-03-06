mod tag;

use super::{
    BinMarkerPosition,
    BarPlot,
    BarColors,
    Percentage,
    DEFAULT_BAR_COLOR,
    DEFAULT_TICK_LENGTH,
    DEFAULT_TEXT_SIDE_OFFSET,
    DEFAULT_LEGEND_POSITION,
    VERSION,
    REPOSITORY,
};

const LF: char = '\n';
const DEFAULT_SVG_SIZE: (u32, u32) = (1600, 1000);

enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

struct SvgGenerator<'a> {
    nodes: Vec<String>,
    min: f64,
    max: f64,
    mean: f64,
    svg_window: (u32, u32),
    plot_window: (f64, f64, f64, f64),
    scale_range: Option<(i64, i64, usize)>,
    line_color: &'a str,
    tick_color: &'a str,
    text_color: &'a str,
    bin_margin: Percentage,
    bar_margin: Percentage,
    font_size: Percentage,
    values: &'a Vec<&'a [f64]>,
    bar_color_variant: &'a BarColors<'a>,
    bar_colors_override: &'a Vec<(usize, usize, &'a str)>,
}

impl <'a>SvgGenerator<'a> {
    fn new(bp: &'a BarPlot) -> Self {
        let svg_window = bp.size.unwrap_or(DEFAULT_SVG_SIZE);

        // Unless changed, plot will take up the whole window.
        let plot_window = (svg_window.0 as f64, 0.0, svg_window.0 as f64, 0.0);

        let (mut max, mut min, mut sum) = (f64::MIN, f64::MAX, 0.0);
        let mut bar_count = 0;
        for arr in bp.values.iter() {
            bar_count += arr.len();
            for f in arr.iter() {
                max = f.max(max);
                min = f.min(min);
                sum += f;
            }
        }
        let mean = sum / bar_count as f64;

        Self {
            nodes: Vec::with_capacity(200),
            min,
            max,
            mean,
            svg_window,
            plot_window,
            scale_range: None,
            line_color: bp.line_color,
            tick_color: bp.tick_color,
            text_color: bp.text_color,
            bin_margin: bp.bin_margin,
            bar_margin: bp.bar_margin,
            font_size: bp.font_size,
            values: &bp.values,
            bar_color_variant: &bp.bar_color_variant,
            bar_colors_override: &bp.bar_colors_override,
        }
    }

    fn get_svg_width(&self) -> f64 {
        self.svg_window.0 as f64
    }

    fn get_svg_height(&self) -> f64 {
        self.svg_window.1 as f64
    }

    fn get_svg_window(&self) -> (f64, f64, f64, f64) {
        let (x1, x2, y1, y2) = (0.0, self.get_svg_width(), 0.0, self.get_svg_height());
        (x1, x2, y1, y2)
    }

    fn get_plot_window(&self) -> (f64, f64, f64, f64) {
        let (x_length, x_offset, y_length, y_offset) = self.plot_window;
        let (x1, x2, y1, y2) = (x_offset, x_offset + x_length, y_offset, y_length + y_offset);
        (x1, x2, y1, y2)
    }

    fn get_font_size(&self) -> f64 {
        (self.get_svg_width() * self.get_svg_height()).sqrt() / 50.0 * (self.font_size as f64 / 100.0)
    }

    fn get_base_line_width(&self) -> f64 {
        (self.get_svg_width() * self.get_svg_height()).sqrt() / 100.0
    }

    fn get_bar_color(&self, category_index: usize, bar_index: usize, bar_value: f64) -> &'a str {
        if !self.bar_colors_override.is_empty() {
            for (i, j, color) in self.bar_colors_override.iter() {
                if category_index == *i && bar_index == *j {
                    return color;
                }
            }
        }

        match self.bar_color_variant {
            BarColors::Category(arr) => {
                arr[category_index]
            }
            BarColors::Indexed(arr) => {
                arr[category_index][bar_index]
            }
            BarColors::Threshold((clr_min, clr_low, clr_high, clr_max)) => {
                if bar_value == self.max { clr_max }
                else if bar_value == self.min { clr_min }
                else if bar_value >= self.mean { clr_high }
                else { clr_low }
            }
            BarColors::Uniform(color) => {
                color
            }
        }
    }

    fn set_line_color(&mut self, color: &'a str) {
        self.line_color = color;
    }

    fn set_tick_color(&mut self, color: &'a str) {
        self.tick_color = color;
    }

    fn set_text_color(&mut self, color: &'a str) {
        self.text_color = color;
    }

    fn set_background_color(&mut self, color: &str) {
        let (width, height) = (self.get_svg_width(), self.get_svg_height());
        let rect = tag::rect(0.0, 0.0, width, height, 1.0, color);
        self.nodes.push(rect);
    }

    fn set_plot_window_size(&mut self, x_size: Percentage, x_offset: Percentage, y_size: Percentage, y_offset: Percentage) {
        // Calculate the plot window size and offset from given percentage.
        assert!(x_size <= 100 && x_offset <= 100, "x_size and or x_offset cannot exceed 100%");
        assert!(y_size <= 100 && y_offset <= 100, "y_size and or y_offset cannot exceed 100%");

        let (x_size, x_offset) = (x_size as f64, x_offset as f64);
        let (y_size, y_offset) = (y_size as f64, y_offset as f64);

        let x_length = self.get_svg_width() * x_size / 100.0;
        let x_offset = ((self.get_svg_width() * (1.0 - x_size / 100.0)) / 100.0) * x_offset;

        let y_length = self.get_svg_height() * y_size / 100.0;
        let y_offset = ((self.get_svg_height() * (1.0 - y_size / 100.0)) / 100.0) * y_offset;

        self.plot_window = (x_length, x_offset, y_length, y_offset);
    }

    fn generate_scale_range(
        &mut self,
        min: i64,
        max: i64,
        step: usize,
        axis_offset: Percentage,
        show_horizontal_lines: bool,
    ) {
        // Needed for when calculating bars.
        self.scale_range = Some((min, max, step));

        let (x1, x2, y1, y2) = self.get_plot_window();

        let plot_height = y2 - y1;
        let x3 = (x1 / 100.0) * (100 - axis_offset) as f64; // tick left end
        let range = (max - min) as f64;
        let vertical_move = plot_height / range;
        let line_width = self.get_base_line_width() / 10.0;
        let font_size = self.get_font_size();
        let font_fraction = font_size / 3.5;
        for n in (min..=max).step_by(step) {
            let cur_y = if min < 0 {
                y2 - ((n as f64 * vertical_move) - (min as f64 * plot_height / range))
            } else {
                y2 - (n as f64 * vertical_move)
            };

            if show_horizontal_lines {
                let tag = tag::line(x1, x2, cur_y, cur_y, &self.line_color, line_width);
                self.nodes.push(tag);
            }

            // If offset is 0, no point in rendering the tick.
            if axis_offset != 0 {
                let tag = tag::line(x3, x1, cur_y, cur_y, self.tick_color, line_width);
                self.nodes.push(tag);
            }

            let num = &n.to_string();
            let tag = tag::text(x3 - font_fraction, cur_y + font_fraction, self.text_color, font_size, "end", num);
            self.nodes.push(tag);
        }
    }

    fn generate_bin_markers(
            &mut self, markers: &[String],
            axis_offset: Percentage,
            bin_marker_position: &BinMarkerPosition,
            show_vertical_lines: bool
        ) {
        let (x1, x2, y1, y2) = self.get_plot_window();
        let y3 = y2 + ((self.get_svg_height() - y2) / 100.0 * axis_offset as f64);

        let line_width = self.get_base_line_width() / 10.0;
        let font_size = self.get_font_size();

        // FIXME: This is not thoroughly tested yet.
        let remainder = self.values[0].len() % markers.len();
        let nth_marker = (self.values[0].len() + remainder) / markers.len();

        let scale_unit = (x2 - x1) / self.values[0].len() as f64;
        let marker_shift = match bin_marker_position {
            BinMarkerPosition::Middle => scale_unit / 2.0,
            BinMarkerPosition::Left => 0.0,
            BinMarkerPosition::Right => scale_unit,
        };

        let mut mark_index = 0;
        for i in 0..self.values[0].len() {
            let cur_x = x1 + (scale_unit * i as f64) + marker_shift;

            if show_vertical_lines {
                let tag = tag::line(cur_x, cur_x, y1, y2, &self.line_color, line_width);
                self.nodes.push(tag);
            }

            // If offset is 0, no point in rendering the tick.
            if axis_offset != 0 {
                let tag = tag::line(cur_x, cur_x, y2, y3, self.tick_color, line_width);
                self.nodes.push(tag);
            }

            // We can have less markers/labels than bars. Will ignore every nth marker so that it all adds up.
            if i % nth_marker == 0 && markers.len() > mark_index {
                let text = &markers[mark_index];
                let tag = tag::text(cur_x, y3 + font_size, self.text_color, font_size, "middle", text);
                self.nodes.push(tag);
                mark_index += 1;
            }
        }
    }

    // Produce the bars in the plot window
    fn generate_bars(&mut self, negative_bars_go_down: bool) {
        let (y_min, y_max) = match self.scale_range {
            Some((min, max, _)) => (min as f64, max as f64),
            None => (self.min, self.max),
        };
        let (x1, x2, y1, y2) = self.get_plot_window();
        let plot_height = y2 - y1;

        let range = y_max - y_min;
        let top_offset = y_min * plot_height / range;
        let scale_unit = plot_height / range;

        let bin_width = (x2 - x1) / self.values[0].len() as f64;
        let bin_margin = bin_width * (self.bin_margin as f64 / 100.0);

        let bar_width = (bin_width - bin_margin) / self.values.len() as f64;
        let bar_margin = (bin_width - bin_margin) / self.values.len() as f64 * (self.bar_margin as f64 / 100.0);
        let cur_bar_width = bar_width - bar_margin;

        let x3 = x1 + bin_margin - (bin_margin/2.0) + (bar_margin/2.0);
        // FIXME: Let user set custom opacity.
        let opacity = 1.0;
        for (category_index, values) in self.values.iter().enumerate() {
            let x4 = x3 + (bar_width * category_index as f64);
            for (bar_index, bar_value) in values.iter().enumerate() {
                let cur_x = x4 + (bin_width * bar_index as f64);

                // FIXME: Can this be written in a more compact and simple way?
                let (cur_y, height) = if negative_bars_go_down && self.min < 0.0 {
                    if *bar_value >= 0.0 {
                        let height = bar_value * scale_unit;
                        let cur_y = y2 + top_offset - height;
                        (cur_y, height)
                    } else {
                        // If negative bars go down, we need to adjust "y" and "height" accordingly.
                        let height = (bar_value * scale_unit).abs();
                        let cur_y = y2 + top_offset;
                        (cur_y, height)
                    }
                } else {
                    let height = (bar_value * scale_unit) - top_offset;
                    let cur_y = y2 - height;
                    (cur_y, height)
                };
                let color = self.get_bar_color(category_index, bar_index, *bar_value);
                let tag = tag::rect(cur_x, cur_y, cur_bar_width, height, opacity, color);
                self.nodes.push(tag);
            }
            // return;
        }
    }

    fn generate_text(&mut self, text: &str, side: Side, offset: Percentage) {
        let (x1, x2, y1, y2) = self.get_plot_window();

        let plot_width = x2 - x1;
        let plot_height = y2 - y1;

        // FIXME: let user chose the shift offset.
        let small_offset = 1.1; // Moves text away from the plot corner by a small distance. (1 = no offset).
        let offset = offset as f64;
        let tag = match side {
            Side::Left => {
                let x = (x1 / 100.0 * offset) + (self.get_font_size() / 2.0);
                let y = y1 + (plot_height / small_offset);
                tag::text_bottom_up(x, y, self.text_color, self.get_font_size(), "start", text)
            }
            Side::Right => {
                let shift = self.get_svg_width() - x2;
                let x = x2 + shift - (shift / 100.0 * offset);
                let y = y2 - (plot_height / small_offset);
                tag::text_top_down(x, y, self.text_color, self.get_font_size(), "start", text)
            }
            Side::Top => {
                let x = x2 - (plot_width / small_offset);
                let y = (y1 / 100.0 * offset) + (self.get_font_size() / 2.0);
                tag::text(x, y, self.text_color, self.get_font_size(), "start", text)
            }
            Side::Bottom => {
                let x = x2 - (plot_width / small_offset);
                let shift = self.get_svg_height() - y2;
                let y = self.get_svg_height() - (shift / 100.0 * offset);
                tag::text(x, y, self.text_color, self.get_font_size(), "start", text)
            }
        };

        self.nodes.push(tag);
    }

    fn generate_legend(&mut self, titles: &[&str], x: Percentage, y: Percentage) {
        let colors: &Vec<&str> = match self.bar_color_variant {
            BarColors::Category(colors) => colors,
            _ => &vec![DEFAULT_BAR_COLOR; titles.len()],
        };

        let font_size = self.get_font_size();
        let x = self.get_svg_width() / 100.0 * x as f64;
        let mut y = self.get_svg_height() / 100.0 * y as f64;

        let step = self.get_font_size() * 1.2;
        for (title, color) in titles.iter().zip(colors.iter()) {
            let tag = tag::rect(x-(font_size*1.5), y-font_size, font_size, font_size, 1.0, color);
            self.nodes.push(tag);

            let tag = tag::text(x, y-(font_size*0.15), self.text_color, font_size, "start", title);
            self.nodes.push(tag);
            y += step;
        }
    }

    fn generate_plot_border(&mut self, color: &str) {
        let (x1, x2, y1, y2) = self.get_plot_window();
        self.nodes.push(tag::line(x1, x1, y1, y2, color, self.get_base_line_width()/10.0)); // left
        self.nodes.push(tag::line(x1, x2, y1, y1, color, self.get_base_line_width()/10.0)); // top
        self.nodes.push(tag::line(x2, x2, y1, y2, color, self.get_base_line_width()/10.0)); // right
        self.nodes.push(tag::line(x1, x2, y2, y2, color, self.get_base_line_width()/10.0)); // bottom
    }

    fn generate_svg_border(&mut self, color: &str) {
        let (x1, x2, y1, y2) = self.get_svg_window();
        self.nodes.push(tag::line(x1, x1, y1, y2, color, self.get_base_line_width()/5.0)); // left
        self.nodes.push(tag::line(x1, x2, y1, y1, color, self.get_base_line_width()/5.0)); // top
        self.nodes.push(tag::line(x2, x2, y1, y2, color, self.get_base_line_width()/5.0)); // right
        self.nodes.push(tag::line(x1, x2, y2, y2, color, self.get_base_line_width()/5.0)); // bottom
    }

    fn generate_svg(&self) -> String {
        let doc_declaration = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;
        let doc_comment = format!("<!-- Created with eb_bars v{VERSION} ({REPOSITORY}) -->");
        let svg_open = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">{LF}"#,
            self.get_svg_width(),
            self.get_svg_height(),
        );
        let svg_content = self.nodes.concat();
        let svg_close = "</svg>";

        format!("{doc_declaration}{LF}{doc_comment}{LF}{LF}{svg_open}{svg_content}{svg_close}{LF}")
    }
}

pub fn render(bp: &BarPlot) -> String {
    let mut svg = SvgGenerator::new(bp);

    if let Some(color) = bp.background_color {
        svg.set_background_color(color);
    }

    svg.set_line_color(bp.line_color);
    svg.set_text_color(bp.text_color);
    svg.set_tick_color(bp.tick_color);

    if let Some((x, x_offset, y, y_offset)) = bp.plot_window_scale {
        svg.set_plot_window_size(x, x_offset, y, y_offset);
    }

    if let Some(markers) = bp.bin_markers {
        assert!(!markers.is_empty());
        let axis_offset = bp.x_axis_tick_length.unwrap_or(DEFAULT_TICK_LENGTH);
        svg.generate_bin_markers(markers, axis_offset, &bp.bin_marker_position, bp.show_vertical_lines);
    }

    if let Some((min, max, step)) = bp.scale_range {
        let length = bp.y_axis_tick_length.unwrap_or(DEFAULT_TICK_LENGTH);
        svg.generate_scale_range(min, max, step, length, bp.show_horizontal_lines);
    }

    if let Some(text) = bp.plot_text.left {
        let offset = bp.plot_text.left_offset.unwrap_or(DEFAULT_TEXT_SIDE_OFFSET);
        svg.generate_text(text, Side::Left, offset);
    }

    if let Some(text) = bp.plot_text.right {
        let offset = bp.plot_text.right_offset.unwrap_or(DEFAULT_TEXT_SIDE_OFFSET);
        svg.generate_text(text, Side::Right, offset);
    }

    if let Some(text) = bp.plot_text.top {
        let offset = bp.plot_text.top_offset.unwrap_or(DEFAULT_TEXT_SIDE_OFFSET);
        svg.generate_text(text, Side::Top, offset);
    }

    if let Some(text) = bp.plot_text.bottom {
        let offset = bp.plot_text.bottom_offset.unwrap_or(DEFAULT_TEXT_SIDE_OFFSET);
        svg.generate_text(text, Side::Bottom, offset);
    }

    if bp.show_window_border {
        svg.generate_svg_border(bp.line_color);
    }

    svg.generate_bars(bp.negative_bars_go_down);

    if let Some(categories) = bp.legend.categories {
        let (x, y) = bp.legend.position.unwrap_or(DEFAULT_LEGEND_POSITION);
        svg.generate_legend(categories, x, y);
    }

    if bp.show_plot_border {
        svg.generate_plot_border(bp.line_color);
    }

    svg.generate_svg()
}
