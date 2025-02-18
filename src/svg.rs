mod tag;

use super::{BarPlot, Percentage};

const DEFAULT_RES: (usize, usize) = (1600, 1000);

const DEFAULT_Y_AXIS_TICK_LENGTH: Percentage = 10;
const DEFAULT_X_AXIS_TICK_LENGTH: Percentage = 10;

struct SvgGenerator<'a> {
    values: &'a [f64],
    min: f64,
    max: f64,
    mean: f64,
    svg_window: (f64, f64), // (x_length, y_length)
    plot_window: (f64, f64, f64, f64), // (x_length, x_offset, y_length, y_offset)
    scale_range: Option<(isize, isize, usize)>, // min, max, step
    nodes: Vec<String>,
    line_color: &'a str,
    tick_color: &'a str,
    text_color: &'a str,
    bar_color: &'a str,
    bar_threshold_colors: Option<(&'a str, &'a str, &'a str, &'a str)>,
    bin_margin: Percentage,
}

impl <'a>SvgGenerator<'a> {
    fn new(
        width: usize,
        height: usize,
        values: &'a [f64],
        line_color: &'a str,
        tick_color: &'a str,
        text_color: &'a str,
        bar_color: &'a str,
        bin_margin: Percentage,
    ) -> Self {
        let (width, height) = (width as f64, height as f64);
        let svg_window = (width, height);
        // Unless changed, plot will take up the whole window.
        let plot_window = (width, 0.0, height, 0.0);

        let (mut max, mut min, mut sum) = (values[0], values[0], values[0]);
        for i in 1..values.len() {
            min = min.min(values[i]);
            max = max.max(values[i]);
            sum = sum + values[i];
        }
        let mean = sum / values.len() as f64;

        Self {
            values,
            min,
            max,
            mean,
            svg_window,
            plot_window,
            scale_range: None,
            nodes: Vec::with_capacity(200),
            line_color,
            tick_color,
            text_color,
            bar_color,
            bar_threshold_colors: None,
            bin_margin,
        }
    }

    fn get_svg_width(&self) -> f64 {
        self.svg_window.0
    }

    fn get_svg_height(&self) -> f64 {
        self.svg_window.1
    }

    fn get_base_line_width(&self) -> f64 {
        (self.get_svg_width() * self.get_svg_height()).sqrt() / 100.0
    }

    fn set_bar_threshold_colors(&mut self, min: &'a str, low: &'a str, high: &'a str, max: &'a str) {
        self.bar_threshold_colors = Some((min, low, high, max));
    }

    fn get_bar_color(&self, val: f64) -> &'a str {
        match self.bar_threshold_colors {
            Some( (clr_min, clr_low, clr_high, clr_max) ) => {
                if val == self.max { clr_max }
                else if val == self.min { clr_min }
                else if val >= self.mean { clr_high }
                else { clr_low }
            }
            None => self.bar_color
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

    fn get_base_font_size(&self) -> f64 {
        (self.get_svg_width() * self.get_svg_height()).sqrt() / 50.0
    }

    fn set_plot_window(&mut self, x_size: Percentage, x_offset: Percentage, y_size: Percentage, y_offset: Percentage) {
        // Calculate the plot window size and offset from given percentage.
        assert!(x_size <= 100 && x_offset <= 100, "x_size and or x_offset cannot exceed 100%");
        assert!(y_size <= 100 && y_offset <= 100, "y_size and or y_offset cannot exceed 100%");

        let (x_size, x_offset) = (x_size as f64, x_offset as f64);
        let (y_size, y_offset) = (y_size as f64, y_offset as f64);

        let x_length = (self.get_svg_width() * x_size / 100.0);
        let x_offset = ((self.get_svg_width() * (1.0 - x_size / 100.0)) / 100.0) * x_offset;

        let y_length = (self.get_svg_height() * y_size / 100.0);
        let y_offset = ((self.get_svg_height() * (1.0 - y_size / 100.0)) / 100.0) * y_offset;

        self.plot_window = (x_length, x_offset, y_length, y_offset);
    }

    fn set_scale_range(&mut self, min: isize, max: isize, step: usize, axis_offset: Percentage, grid: bool) {
        // Needed when rendering bars.
        self.scale_range = Some((min, max, step));

        let (x_length, x_offset, y_length, y_offset) = self.plot_window;
        assert!(x_length < self.get_svg_width(), "no room for axis, x_length is to high");
        assert!(y_length < self.get_svg_width(), "no room for axis, y_length is to high");
        let line_width = self.get_base_line_width()/10.0;
        let font_size = self.get_base_font_size();

        let x1 = (x_offset / 100.0) * (100 - axis_offset) as f64; // tick left end
        let x2 = x_offset; // tick right end
        let range = (max - min) as f64;
        let vertical_move = y_length / range;

        for n in (min..=max).step_by(step) {
            let y = if min < 0 {
                y_offset + y_length - ((n as f64 * vertical_move) - (min as f64 * y_length / range))
            } else {
                y_offset + y_length - (n as f64 * vertical_move)
            };

            let mark = &n.to_string();
            let text = tag::text(x1-(font_size/3.5), y + (font_size/3.5), self.text_color, font_size, "end", mark);
            self.nodes.push(text);

            let tick = tag::line(x1, x2, y, y, self.tick_color, line_width);
            self.nodes.push(tick);

            if grid {
                let line = tag::line(x1, x_length + x_offset, y, y, &self.tick_color, line_width);
                self.nodes.push(line);
            }
        }
    }

    fn bar_markers(&mut self, x_markers: &[String], axis_offset: Percentage, x_marks_middle: bool, grid: bool) {
        let axis_offset = axis_offset as f64;
        let (x_length, x_offset, y_length, y_offset) = self.plot_window;

        let line_width = self.get_base_line_width()/10.0;
        let font_size = self.get_base_font_size();
        let y = y_length + y_offset;
        let y_marker = (self.get_svg_height() + y_length + y_offset) / 2.0;
        let y2 = y + ( (self.get_svg_height() - (y_length + y_offset)) / 100.0 * axis_offset);
        let y3 = y2 + self.get_base_font_size();

        // FIXME: This is not thoroughly tested yet.
        let remainder = (self.values.len() % x_markers.len());
        let nth_marker = (self.values.len() + remainder) / x_markers.len();

        let horizontal_move = x_length / self.values.len() as f64;
        let to_middle = if x_marks_middle {
            horizontal_move / 2.0
        } else {
            0.0
        };

        let mut mark_index = 0;
        for i in 0..self.values.len() {
            let x = x_offset + (horizontal_move * i as f64) + to_middle;

            let tick = tag::line(x, x, y, y2, self.tick_color, line_width);
            self.nodes.push(tick);

            // As stated above, we can have less markers than bars if we want to.
            if i % nth_marker == 0 && x_markers.len() > mark_index {
                let mark = &x_markers[mark_index];
                let text = tag::text(x, y3, self.text_color, font_size, "middle", mark);
                self.nodes.push(text);
                mark_index += 1;
            }

            if grid {
                let line = tag::line(x, x, y_offset, y, &self.tick_color, line_width);
                self.nodes.push(line);
            }
        }
    }

    // Produce the bars in the plot window
    fn set_bars(&mut self, negative_bars_go_down: bool) {
        let (mut max, mut min, mut sum) = (self.values[0], self.values[0], self.values[0]);
        for i in 1..self.values.len() {
            min = min.min(self.values[i]);
            max = max.max(self.values[i]);
            sum = sum + self.values[i];
        }
        let mean = sum / self.values.len() as f64;

        let (y_min, y_max) = match self.scale_range {
            Some((min, max, _)) => (min as f64, max as f64),
            None => (min, max),
        };

        let (x_length, x_offset, y_length, y_offset) = self.plot_window;

        let range = (y_max - y_min) as f64;
        let top_offset = y_min as f64 * y_length / range;
        let y_floor = y_length + y_offset; // Indicates the floor in plot window.

        let vertical_move: f64 = y_length / range;
        let bin_width = x_length / self.values.len() as f64;
        let margin = (self.bin_margin as f64 / 100_f64);
        let bin_margin = bin_width * margin;
        let bar_width = bin_width - bin_margin;

        for (i, bar) in self.values.iter().enumerate() {
            let x = (bin_width * i as f64) + x_offset + bin_margin - (bin_margin/2.0);
            let opacity = if true { 1.0 } else { 0.7 };

            let color = self.get_bar_color(*bar);

            // FIXME: Can this be written in a more compact way?
            let (y, height) = if negative_bars_go_down && min < 0.0 {
                if *bar >= 0.0 {
                    let height = bar * vertical_move;
                    let y = y_floor + top_offset - height;
                    ( y, height )
                } else {
                    // If negative bars go down, we need to adjust the y and height accordingly.
                    let height = (bar * vertical_move).abs();
                    let y = y_floor + top_offset;
                    ( y, height )
                }
            } else {
                let height = (bar * vertical_move) - top_offset;
                let y = y_floor - height;
                ( y, height )
            };

            self.nodes.push(tag::rect(x, y, bar_width, height, opacity, color));
        }
    }

    fn set_plot_border(&mut self, color: &str) {
        let (x_length, x_offset, y_length, y_offset) = self.plot_window;
        let (x1, x2, y1, y2) = (x_offset, x_offset + x_length, y_offset, y_length + y_offset);
        self.nodes.push(tag::line(x1, x1, y1, y2, color, self.get_base_line_width()/10.0)); // left
        self.nodes.push(tag::line(x1, x2, y1, y1, color, self.get_base_line_width()/10.0)); // top
        self.nodes.push(tag::line(x2, x2, y1, y2, color, self.get_base_line_width()/10.0)); // right
        self.nodes.push(tag::line(x1, x2, y2, y2, color, self.get_base_line_width()/10.0)); // bottom
    }

    fn set_svg_border(&mut self, color: &str) {
        let x1 = 0.0;
        let x2 = self.get_svg_width();
        let y1 = 0.0;
        let y2 = self.get_svg_height();

        self.nodes.push(tag::line(x1, x1, y1, y2, color, self.get_base_line_width()/5.0)); // left
        self.nodes.push(tag::line(x1, x2, y1, y1, color, self.get_base_line_width()/5.0)); // top
        self.nodes.push(tag::line(x2, x2, y1, y2, color, self.get_base_line_width()/5.0)); // right
        self.nodes.push(tag::line(x1, x2, y2, y2, color, self.get_base_line_width()/5.0)); // bottom
    }

    fn generate(&self) -> String {
        let svg = format!(
            r#"<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">"#,
            width = self.get_svg_width(),
            height = self.get_svg_height(),
        ) + &self.nodes.join("\n") + "\n</svg>";

        svg
    }
}

pub fn render(bp: &BarPlot) -> String {
    let (width, height) = match bp.res {
        Some((w, h)) => (w, h),
        None => DEFAULT_RES,
    };

    let mut svg = SvgGenerator::new(
        width,
        height,
        bp.values,
        bp.line_color,
        bp.tick_color,
        bp.text_color,
        bp.bar_color,
        bp.bin_margin
    );

    if let Some(color) = bp.background_color {
        svg.set_background_color(color);
    }

    svg.set_tick_color(bp.tick_color);

    svg.set_line_color(bp.line_color);

    if let Some((x, x_offset, y, y_offset)) = bp.plot_window_scale {
        svg.set_plot_window(x, x_offset, y, y_offset);
    }

    if let Some( (min, low, high, max) ) = bp.bar_threshold_colors {
        svg.set_bar_threshold_colors(min, low, high, max);
    }

    if let Some(markers) = bp.bar_markers {
        let length = bp.x_axis_tick_length.unwrap_or(DEFAULT_X_AXIS_TICK_LENGTH);
        svg.bar_markers(markers, length, bp.x_markers_at_middle, bp.show_vertical_lines);
    }

    if let Some((min, max, step)) = bp.scale_range {
        let length = bp.y_axis_tick_length.unwrap_or(DEFAULT_Y_AXIS_TICK_LENGTH);
        svg.set_scale_range(min, max, step, length, bp.show_horizontal_lines);
    }

    if bp.window_border {
        svg.set_svg_border(bp.line_color);
    }

    svg.set_bars(bp.negative_bars_go_down);

    if bp.plot_border {
        svg.set_plot_border(bp.line_color);
    }

    svg.generate()
}
