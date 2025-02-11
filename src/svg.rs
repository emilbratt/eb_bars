mod tag;

use std::path::Path;

use super::{BarPlot, Percentage};

const DEFAULT_RES: (usize, usize) = (1600, 1000);

const BAR_MARGIN_PERMILLIE: f64 = 5.0; // introduce a gap between cans (bars).

const DEFAULT_Y_AXIS_TICK_LENGTH: Percentage = 10;
const DEFAULT_X_AXIS_TICK_LENGTH: Percentage = 10;

const X_MARKER_IS_MIDDLE: bool = false; // wether each marker (label) is between two bars or directly unerneath one bar
const X_TICK_IS_MIDDLE: bool = false; // wether each tick is between two bars or directly unerneath one bar

struct SvgGenerator<'a> {
    values: &'a [f64],
    svg_window: (f64, f64), // (x_length, y_length)
    plot_window: (f64, f64, f64, f64), // (x_length, x_offset, y_length, y_offset)
    scale_range: Option<(isize, isize, usize)>, // min, max, step
    nodes: Vec<String>,
}

impl <'a>SvgGenerator<'a> {
    fn new(width: usize, height: usize, values: &'a [f64]) -> Self {
        let (width, height) = (width as f64, height as f64);
        let svg_window = (width, height);
        // Unless changed, plot will take up the whole window.
        let plot_window = (width, 0.0, height, 0.0);

        Self {
            values,
            svg_window,
            plot_window,
            scale_range: None,
            nodes: Vec::with_capacity(200),
        }
    }

    fn svg_width(&self) -> f64 {
        self.svg_window.0
    }

    fn svg_height(&self) -> f64 {
        self.svg_window.1
    }

    fn get_base_line_width(&self) -> f64 {
        (self.svg_width() * self.svg_height()).sqrt() / 100_f64
    }

    fn get_base_color(&self) -> &str {
        "rgb(197, 197, 197)"
    }

    fn get_base_font_size(&self) -> f64 {
        (self.svg_width() * self.svg_height()).sqrt() / 50_f64
    }

    fn set_plot_window(&mut self, x_size: Percentage, x_offset: Percentage, y_size: Percentage, y_offset: Percentage) {
        // Calculate the plot window size and offset for x and y in percentage.
        assert!(x_size <= 100 && x_offset <= 100, "x_size and x_offset cannot exceed 100%");
        assert!(y_size <= 100 && y_offset <= 100, "y_size and x_offset cannot exceed 100%");

        let (x_size, x_offset) = (x_size as f64, x_offset as f64);
        let (y_size, y_offset) = (y_size as f64, y_offset as f64);

        let x_length = (self.svg_width() * x_size / 100_f64);
        let x_offset = ((self.svg_width() * (1.0 - x_size / 100_f64)) / 100_f64) * x_offset;

        let y_length = (self.svg_height() * y_size / 100_f64);
        let y_offset = ((self.svg_height() * (1.0 - y_size / 100_f64)) / 100_f64) * y_offset;

        self.plot_window = (x_length, x_offset, y_length, y_offset);
    }

    // Produce axis containing the range of values
    fn set_scale_range(&mut self, min: isize, max: isize, step: usize, axis_offset: Percentage) {
        let axis_offset = (100 - axis_offset) as f64;
        self.scale_range = Some((min, max, step));
        let (x_length, x_offset, y_length, y_offset) = self.plot_window;
        assert!(x_length < self.svg_width(), "no room for y-axis");
        assert!(y_length < self.svg_width(), "no room for y-axis");

        let range = (max - min) as f64;

        let bar_window_offset = if min >= 0 {
            0.0
        } else {
            min as f64 * y_length / range
        };

        let unit = y_length / range;

        for n in (min..=max).step_by(step) {
            let (n, max, min) = (n as f64, max as f64, min as f64);

            let x1 = (x_offset / 100.0) * axis_offset; // left end
            let x2 = x_offset; // right end

            let y = y_offset + y_length - ((n * unit) - bar_window_offset);

            let text = tag::text(x1, y, self.get_base_color(), self.get_base_font_size(), "end", &n.to_string());
            self.nodes.push(text);

            let tick = tag::line(x1, x2, y, y, self.get_base_color(), self.get_base_line_width()/10.0);
            self.nodes.push(tick);
        }
    }

    fn bar_markers(&mut self, x_markers: &[String], axis_offset: Percentage, x_marks_middle: bool) {
        let axis_offset = axis_offset as f64;
        let (x_length, x_offset, y_length, y_offset) = self.plot_window;

        let y = y_length + y_offset;
        let y_marker = (self.svg_height() + y_length + y_offset) / 2.0;
        let horizontal_move = x_length / self.values.len() as f64;
        let y2 = y + ( (self.svg_height() - (y_length + y_offset)) / 100.0 * axis_offset);

        // We can have less markers than bars if we want to.
        let remainder = self.values.len() % x_markers.len();
        let nth_marker = if remainder == 0 {
            (self.values.len() / x_markers.len())
        } else {
            (self.values.len() / x_markers.len()) + 1
        };

        let mut mark_index = 0;
        for i in 0..self.values.len() {
            let x = if X_TICK_IS_MIDDLE {
                x_offset + (horizontal_move * i as f64) + (horizontal_move / 2.0)
            } else {
                x_offset + (horizontal_move * i as f64)
            };

            let tick = tag::line(x, x, y, y2, self.get_base_color(), self.get_base_line_width()/10.0);
            self.nodes.push(tick);

            // As stated above, we can have less markers than bars if we want to.
            if i % nth_marker == 0 {
                let x = if x_marks_middle {
                    x_offset + (horizontal_move * i as f64) + (horizontal_move / 2.0)
                } else {
                    x_offset + (horizontal_move * i as f64)
                };
                let mark = tag::text(x, y2 + self.get_base_font_size(), self.get_base_color(), self.get_base_font_size(), "middle", &x_markers[mark_index]);
                self.nodes.push(mark);
                mark_index += 1;
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
        let bar_window_offset = y_min as f64 * y_length / range;
        let base_y = y_length + y_offset; // X-AXIS e.g. the floor in bar window

        let unit = y_length / range;
        let bin_width = x_length / self.values.len() as f64;
        let bar_margin = bin_width * (BAR_MARGIN_PERMILLIE / 1000_f64);
        let width = bin_width - (bar_margin * 2.0);

        for (i, bar) in self.values.iter().enumerate() {
            let x = (bin_width * i as f64) + x_offset + bar_margin;
            let opacity = if true { 1.0 } else { 0.7 };

            let color =
                if bar == &max { "rgb(250, 144, 120)" }
                else if bar == &min { "rgb(130, 250, 255)" }
                else if bar >= &mean { "rgb(250, 210, 150)" }
                else { "rgb(150, 250, 180)" };

            // If negative bars go down, we need to adjust the y and height accordingly.
            let (y, height) = if negative_bars_go_down {
                if bar >= &0.0 {
                    let height = bar * unit;
                    let y = base_y + bar_window_offset - height;
                    ( y, height )
                } else {
                    let height = (bar * unit).abs();
                    let y = base_y + bar_window_offset;
                    ( y, height )
                }
            } else {
                let height = (bar * unit) - bar_window_offset;
                let y = base_y - height;
                ( y, height )
            };

            self.nodes.push(tag::rect(x, y, width, height, opacity, color));
        }
    }

    fn set_plot_border(&mut self) {
        let (x_length, x_offset, y_length, y_offset) = self.plot_window;
        let (x1, x2, y1, y2) = (x_offset, x_offset + x_length, y_offset, y_length + y_offset);
        self.nodes.push(tag::line(x1, x1, y1, y2, self.get_base_color(), self.get_base_line_width()/10.0)); // left
        self.nodes.push(tag::line(x1, x2, y1, y1, self.get_base_color(), self.get_base_line_width()/10.0)); // top
        self.nodes.push(tag::line(x2, x2, y1, y2, self.get_base_color(), self.get_base_line_width()/10.0)); // right
        self.nodes.push(tag::line(x1, x2, y2, y2, self.get_base_color(), self.get_base_line_width()/10.0)); // bottom
    }

    fn set_svg_border(&mut self) {
        let x1 = 0.0;
        let x2 = self.svg_width();
        let y1 = 0.0;
        let y2 = self.svg_height();

        self.nodes.push(tag::line(x1, x1, y1, y2, self.get_base_color(), self.get_base_line_width())); // left
        self.nodes.push(tag::line(x1, x2, y1, y1, self.get_base_color(), self.get_base_line_width())); // top
        self.nodes.push(tag::line(x2, x2, y1, y2, self.get_base_color(), self.get_base_line_width())); // right
        self.nodes.push(tag::line(x1, x2, y2, y2, self.get_base_color(), self.get_base_line_width())); // bottom
    }

    fn generate(&self) -> String {
        let svg = format!(
            r#"<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">"#,
            width = self.svg_width(),
            height = self.svg_height(),
        ) + &self.nodes.join("\n") + "\n</svg>";

        svg
    }
}

pub fn render(bar_plot: &BarPlot) -> String {
    let (width, height) = match bar_plot.res {
        Some((w, h)) => (w, h),
        None => DEFAULT_RES,
    };

    let mut svg = SvgGenerator::new(width, height, bar_plot.values);

    if let Some((x, x_offset, y, y_offset)) = bar_plot.plot_window_scale {
        svg.set_plot_window(x, x_offset, y, y_offset);
    }

    if let Some(markers) = bar_plot.bar_markers {
        match bar_plot.x_axis_tick_length {
            Some(length) => svg.bar_markers(markers, length, bar_plot.x_markers_set_middle),
            None => svg.bar_markers(markers, DEFAULT_X_AXIS_TICK_LENGTH, bar_plot.x_markers_set_middle),
        }
    }

    if let Some((min, max, step)) = bar_plot.scale_range {
        match bar_plot.y_axis_tick_length {
            Some(length) => svg.set_scale_range(min, max, step, length),
            None => svg.set_scale_range(min, max, step, DEFAULT_Y_AXIS_TICK_LENGTH),
        }
    }

    if bar_plot.window_border {
        svg.set_svg_border();
    }

    if bar_plot.plot_border {
        svg.set_plot_border();
    }

    svg.set_bars(bar_plot.negative_bars_go_down);

    svg.generate()
}
