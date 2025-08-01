mod tag;

use crate::{
    BarColorLayout,
    BarColors,
    BarPlot,
    BinMarkerPosition,
    Colors,
    LinesAt,
    Percentage,
    DEFAULT_BAR_COLOR,
    DEFAULT_LEGEND_POSITION,
    DEFAULT_TEXT_SIDE_OFFSET,
    REPOSITORY,
    VERSION,
};

enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

struct BarValues<'a> {
    values: &'a [&'a [f64]],
    min: f64,
    max: f64,
    mean: f64,
}

impl <'a>BarValues<'a> {
    fn from(values: &'a [&'a [f64]]) -> Self {
        assert!(!values.is_empty());

        let (mut max, mut min, mut sum) = (f64::MIN, f64::MAX, 0.0);
        let mut bar_count = 0;
        for arr in values.iter() {
            bar_count += arr.len();
            for f in arr.iter() {
                max = f.max(max);
                min = f.min(min);
                sum += f;
            }
        }
        let mean = sum / bar_count as f64;

        Self { values, min, max, mean }
    }

    fn len(&self) -> usize {
        self.values[0].len()
    }
}

struct SvgGenerator {
    svg_window: (f64, f64, f64, f64),
    plot_window: Option<(f64, f64, f64, f64)>,
    nodes: Vec<String>,
}

impl SvgGenerator {
    fn new(svg_width: f64, svg_height: f64) -> Self {
        Self {
            svg_window: (0.0, svg_width, 0.0, svg_height),
            plot_window: None,
            nodes: Vec::with_capacity(200),
        }
    }

    fn get_svg_width(&self) -> f64 {
        self.svg_window.1
    }

    fn get_svg_height(&self) -> f64 {
        self.svg_window.3
    }

    fn get_plot_width(&self) -> f64 {
        let (x1, x2, _, _) = self.plot_window.unwrap_or(self.svg_window);
        x2-x1
    }

    fn get_plot_height(&self) -> f64 {
        let (_, _, y1, y2) = self.plot_window.unwrap_or(self.svg_window);
        y2-y1
    }

    fn get_font_size(&self, font_size: Percentage) -> f64 {
        (self.get_svg_width() * self.get_svg_height()).sqrt() / 50.0 * (font_size / 100.0)
    }

    fn get_base_line_width(&self) -> f64 {
        (self.get_svg_width() * self.get_svg_height()).sqrt() / 100.0
    }

    fn set_background_color(&mut self, color: &str) {
        let (width, height) = (self.get_svg_width(), self.get_svg_height());
        let rect = tag::rect(0.0, 0.0, width, height, 1.0, color);
        self.nodes.push(rect);
    }

    fn set_plot_window(&mut self, x_size: Percentage, x_offset: Percentage, y_size: Percentage, y_offset: Percentage) {
        // Calculate the plot window size and offset from given percentage.
        assert!(x_size <= 100.0 && x_offset <= 100.0, "x_size and or x_offset cannot exceed 100%");
        assert!(y_size <= 100.0 && y_offset <= 100.0, "y_size and or y_offset cannot exceed 100%");

        let x_length = self.get_svg_width() * x_size / 100.0;
        let x_offset = ((self.get_svg_width() * (1.0 - x_size / 100.0)) / 100.0) * x_offset;

        let y_length = self.get_svg_height() * y_size / 100.0;
        let y_offset = ((self.get_svg_height() * (1.0 - y_size / 100.0)) / 100.0) * y_offset;

        let (x1, x2, y1, y2) = (x_offset, x_offset + x_length, y_offset, y_length + y_offset);

        self.plot_window = Some((x1, x2, y1, y2));
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_scale_range(
        &mut self,
        min: i64,
        max: i64,
        step: usize,
        axis_offset: Percentage,
        show_horizontal_lines: bool,
        line_color: &str,
        tick_color: &str,
        text_color: &str,
        font_size: Percentage,
    ) {
        let (x1, x2, _, y2) = self.plot_window.unwrap_or(self.svg_window);
        let x3 = (x1 / 100.0) * (100.0 - axis_offset); // tick left end
        let range = max as f64 - min as f64;
        let vertical_move = self.get_plot_height() / range;
        let line_width = self.get_base_line_width() / 10.0;
        let font_size = self.get_font_size(font_size);
        let font_fraction = font_size / 3.5;
        for n in (min..=max).step_by(step) {
            let cur_y = if min < 0 {
                y2 - ((n as f64 * vertical_move) - (min as f64 * self.get_plot_height() / range))
            } else {
                y2 - (n as f64 * vertical_move)
            };

            if show_horizontal_lines {
                let tag = tag::line(x1, x2, cur_y, cur_y, line_color, line_width);
                self.nodes.push(tag);
            }

            // If offset is 0, no point in rendering the tick.
            if axis_offset != 0.0 {
                let tag = tag::line(x3, x1, cur_y, cur_y, tick_color, line_width);
                self.nodes.push(tag);
            }

            let num = &n.to_string();
            let tag = tag::text(x3 - font_fraction, cur_y + font_fraction, text_color, font_size, "end", num);
            self.nodes.push(tag);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_bin_markers(
            &mut self,
            bar_values: &BarValues,
            markers: &[&str],
            x_axis_tick_length: Percentage,
            bin_marker_position: &BinMarkerPosition,
            show_vertical_lines: bool,
            colors: &Colors,
            font_size: Percentage,
        ) {
        let (x1, _, y1, y2) = self.plot_window.unwrap_or(self.svg_window);
        let y3 = y2 + ((self.get_svg_height() - y2) / 100.0 * x_axis_tick_length);

        let line_width = self.get_base_line_width() / 10.0;
        let font_size = self.get_font_size(font_size);

        let scale_unit = self.get_plot_width() / bar_values.len() as f64;
        let marker_shift = match bin_marker_position {
            BinMarkerPosition::Middle => scale_unit / 2.0,
            BinMarkerPosition::Left => 0.0,
            BinMarkerPosition::Right => scale_unit,
        };

        for i in 0..markers.len() {
            let cur_x = x1 + (scale_unit * i as f64) + marker_shift;

            if show_vertical_lines {
                let tag = tag::line(cur_x, cur_x, y1, y2, colors.line, line_width);
                self.nodes.push(tag);
            }

            // If offset is 0, no point in rendering the tick.
            if x_axis_tick_length != 0. {
                let tag = tag::line(cur_x, cur_x, y2, y3, colors.tick, line_width);
                self.nodes.push(tag);
            }

            let text = &markers[i];
            let tag = tag::text(cur_x, y3 + font_size, colors.text, font_size, "middle", text);
            self.nodes.push(tag);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_bars(
        &mut self,
        bar_values: &BarValues,
        scale_range: Option<(i64, i64, usize)>,
        negative_bars_go_down: bool,
        bin_gap: Percentage,
        bar_gap: Percentage,
        bar_colors: &BarColors,
    ) {
        let (y_min, y_max) = match scale_range {
            Some((min, max, _)) => (min as f64, max as f64),
            None => (bar_values.min, bar_values.max),
        };
        let (x1, _, _, y2) = self.plot_window.unwrap_or(self.svg_window);

        let range = y_max - y_min;
        let top_offset = y_min * self.get_plot_height() / range;
        let scale_unit = self.get_plot_height() / range;

        let bin_width = self.get_plot_width() / bar_values.len() as f64;
        let bin_margin = bin_width * (bin_gap / 100.0);

        let margined_bin_width = (bin_width - bin_margin) / bar_values.values.len() as f64;
        let bar_margin = (bin_width - bin_margin) / bar_values.values.len() as f64 * (bar_gap / 100.0);
        let bar_width = margined_bin_width - bar_margin;

        let x3 = x1 + bin_margin - (bin_margin / 2.0) + (bar_margin / 2.0);
        // FIXME: Let user set custom opacity.
        let opacity = 1.0;
        for (category_index, values) in bar_values.values.iter().enumerate() {
            let x4 = x3 + (margined_bin_width * category_index as f64);
            for (bar_index, bar_value) in values.iter().copied().enumerate() {
                let bar_x = x4 + (bin_width * bar_index as f64);

                // FIXME: Can this be written in a more compact and simple way?
                let (bar_y, bar_height) = if negative_bars_go_down {
                    if bar_value >= 0.0 {
                        let bar_height = bar_value * scale_unit;
                        let bar_y = y2 + top_offset - bar_height;
                        (bar_y, bar_height)
                    } else {
                        // If negative bars go down, we need to adjust "y" and "height" accordingly.
                        let bar_height = (bar_value * scale_unit).abs();
                        let bar_y = y2 + top_offset;
                        (bar_y, bar_height)
                    }
                } else {
                    let bar_height = (bar_value * scale_unit) - top_offset;
                    let bar_y = y2 - bar_height;
                    (bar_y, bar_height)
                };

                let mut bar_color = match &bar_colors.layout {
                    BarColorLayout::Category(arr) => {
                        arr[category_index]
                    }
                    BarColorLayout::Indexed(arr) => {
                        arr[category_index][bar_index]
                    }
                    BarColorLayout::Threshold((clr_min, clr_low, clr_high, clr_max)) => {
                        if bar_value == bar_values.max { clr_max }
                        else if bar_value == bar_values.min { clr_min }
                        else if bar_value >= bar_values.mean { clr_high }
                        else { clr_low }
                    }
                    BarColorLayout::Uniform(color) => {
                        color
                    }
                };

                if !bar_colors.overrides.is_empty() {
                    for (i, j, color) in bar_colors.overrides.iter().copied() {
                        if category_index == i && bar_index == j {
                            bar_color = color;
                            break;
                        }
                    }
                }

                let tag = tag::rect(bar_x, bar_y, bar_width, bar_height, opacity, bar_color);
                self.nodes.push(tag);
            }
        }
    }

    fn generate_text(&mut self, text: &str, side: Side, offset: Percentage, color: &str, font_size: Percentage) {
        let (x1, x2, y1, y2) = self.plot_window.unwrap_or(self.svg_window);

        // FIXME: let user choose the shift offset.
        let small_offset = 1.1; // Moves text away from the plot corner by a small distance. (1 = no offset).
        let tag = match side {
            Side::Left => {
                let x = (x1 / 100.0 * offset) + (self.get_font_size(font_size) / 2.0);
                let y = y1 + (self.get_plot_height() / small_offset);
                tag::text_bottom_up(x, y, color, self.get_font_size(font_size), "start", text)
            }
            Side::Right => {
                let shift = self.get_svg_width() - x2;
                let x = x2 + shift - (shift / 100.0 * offset);
                let y = y2 - (self.get_plot_height() / small_offset);
                tag::text_top_down(x, y, color, self.get_font_size(font_size), "start", text)
            }
            Side::Top => {
                let x = x2 - (self.get_plot_width() / small_offset);
                let y = (y1 / 100.0 * offset) + (self.get_font_size(font_size) / 2.0);
                tag::text(x, y, color, self.get_font_size(font_size), "start", text)
            }
            Side::Bottom => {
                let x = x2 - (self.get_plot_width() / small_offset);
                let shift = self.get_svg_height() - y2;
                let y = self.get_svg_height() - (shift / 100.0 * offset);
                tag::text(x, y, color, self.get_font_size(font_size), "start", text)
            }
        };

        self.nodes.push(tag);
    }

    fn generate_legend(
        &mut self,
        categories: &[&str],
        x: Percentage,
        y: Percentage,
        text_color: &str,
        font_size: Percentage,
        bar_colors: &BarColors,
    ) {
        let colors: &[&str] = match bar_colors.layout {
            BarColorLayout::Category(ref colors) => colors,
            _ => &vec![DEFAULT_BAR_COLOR; categories.len()],
        };

        let font_size_len = self.get_font_size(font_size);
        let x = self.get_svg_width() / 100.0 * x;
        let mut y = self.get_svg_height() / 100.0 * y;

        let step = self.get_font_size(font_size) * 1.2;
        for (title, color) in categories.iter().zip(colors.iter()) {
            let tag = tag::rect(x-(font_size_len*1.5), y-font_size_len, font_size_len, font_size_len, 1.0, color);
            self.nodes.push(tag);

            let tag = tag::text(x, y-(font_size_len*0.15), text_color, font_size_len, "start", title);
            self.nodes.push(tag);
            y += step;
        }
    }

    fn generate_plot_border(&mut self, color: &str) {
        let (x1, x2, y1, y2) = self.plot_window.unwrap_or(self.svg_window);
        let width = self.get_base_line_width() / 10.0;
        self.nodes.push(tag::line(x1, x1, y1, y2, color, width)); // left
        self.nodes.push(tag::line(x1, x2, y1, y1, color, width)); // top
        self.nodes.push(tag::line(x2, x2, y1, y2, color, width)); // right
        self.nodes.push(tag::line(x1, x2, y2, y2, color, width)); // bottom
    }

    fn generate_svg_border(&mut self, color: &str) {
        let (x1, x2, y1, y2) = self.svg_window;
        let width = self.get_base_line_width() / 5.0;
        self.nodes.push(tag::line(x1, x1, y1, y2, color, width)); // left
        self.nodes.push(tag::line(x1, x2, y1, y1, color, width)); // top
        self.nodes.push(tag::line(x2, x2, y1, y2, color, width)); // right
        self.nodes.push(tag::line(x1, x2, y2, y2, color, width)); // bottom
    }

    fn generate_lines_at(&mut self, lines_at: &Vec<LinesAt>) {
        let width = self.get_base_line_width() / 5.0;
        let base_x = self.get_plot_width() / 100.0;
        let base_y = self.get_plot_height() / 100.0;
        let (x1, x2, y1, y2) = self.plot_window.unwrap_or(self.svg_window);
        for line in lines_at {
            let tag = match line {
                LinesAt::Horizontal(f, color) => {
                    let y = f * base_y + y1;
                    tag::line(x1, x2, y, y, color, width)
                },
                LinesAt::Vertical(f, color) => {
                    let x = f * base_x + x1;
                    tag::line(x, x, y1, y2, color, width)
                },
            };
            self.nodes.push(tag);
        }
    }

    fn generate_svg(&self) -> String {
        let mut svg = String::with_capacity(200*200);

        svg.push_str(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#
        );
        svg.push('\n');

        svg.push_str(
            format!("<!-- Created with eb_bars v{VERSION} ({REPOSITORY}) -->").as_str()
        );
        svg.push_str("\n\n");

        svg.push_str(
            format!(
                r#"<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">"#,
                width = self.get_svg_width(),
                height = self.get_svg_height(),
            ).as_str()
        );
        svg.push('\n');

        svg.push_str(self.nodes.concat().as_str());

        svg.push_str("</svg>\n");

        svg
    }
}

pub fn render(bp: &BarPlot) -> String {
    let (svg_width, svg_height) = (bp.size.0 as f64, bp.size.1 as f64);

    let mut svg = SvgGenerator::new(svg_width, svg_height);

    let bar_values = BarValues::from(&bp.values);

    if let Some((x_size, x_offset, y_size, y_offset)) = bp.layout.plot_window_scale {
        svg.set_plot_window(x_size, x_offset, y_size, y_offset);
    }

    if let Some(color) = bp.colors.background {
        svg.set_background_color(color);
    }

    if let Some(markers) = bp.markers {
        assert!(!markers.is_empty());
        svg.generate_bin_markers(
            &bar_values,
            markers,
            bp.layout.x_axis_tick_length,
            &bp.layout.bin_marker_position,
            bp.show.vertical_lines,
            &bp.colors,
            bp.layout.font_size,
        );
    }

    if let Some((min, max, step)) = bp.layout.scale_range {
        svg.generate_scale_range(
            min,
            max,
            step,
            bp.layout.y_axis_tick_length,
            bp.show.horizontal_lines,
            bp.colors.line,
            bp.colors.tick,
            bp.colors.text,
            bp.layout.font_size,
        );
    }

    if let Some(text) = bp.plot_text.left {
        let offset = bp.plot_text.left_offset.unwrap_or(DEFAULT_TEXT_SIDE_OFFSET);
        svg.generate_text(text, Side::Left, offset, bp.colors.text, bp.layout.font_size);
    }

    if let Some(text) = bp.plot_text.right {
        let offset = bp.plot_text.right_offset.unwrap_or(DEFAULT_TEXT_SIDE_OFFSET);
        svg.generate_text(text, Side::Right, offset, bp.colors.text, bp.layout.font_size);
    }

    if let Some(text) = bp.plot_text.top {
        let offset = bp.plot_text.top_offset.unwrap_or(DEFAULT_TEXT_SIDE_OFFSET);
        svg.generate_text(text, Side::Top, offset, bp.colors.text, bp.layout.font_size);
    }

    if let Some(text) = bp.plot_text.bottom {
        let offset = bp.plot_text.bottom_offset.unwrap_or(DEFAULT_TEXT_SIDE_OFFSET);
        svg.generate_text(text, Side::Bottom, offset, bp.colors.text, bp.layout.font_size);
    }

    if bp.show.window_border {
        svg.generate_svg_border(bp.colors.line);
    }

    svg.generate_bars(
        &bar_values,
        bp.layout.scale_range,
        bp.layout.negative_bars_go_down,
        bp.layout.bin_gap,
        bp.layout.bar_gap,
        &bp.colors.bars,
    );

    if let Some(categories) = bp.legend.categories {
        let (x, y) = bp.legend.position.unwrap_or(DEFAULT_LEGEND_POSITION);
        svg.generate_legend(
            categories,
            x,
            y,
            bp.colors.text,
            bp.layout.font_size,
            &bp.colors.bars
        );
    }

    if bp.show.plot_border {
        svg.generate_plot_border(bp.colors.line);
    }

    if !bp.lines_at.is_empty() {
        svg.generate_lines_at(&bp.lines_at);
    }

    svg.generate_svg()
}
