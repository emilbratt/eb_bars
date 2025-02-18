#![allow(unused)]

use std::path::Path;

mod svg;

type Percentage = usize;

const DEFAULT_BIN_MARGIN: Percentage = 2; // introduce a gap between bins.
const DEFAULT_BASE_COLOR: &str = "rgb(197, 197, 197)";
const DEFAULT_BAR_COLOR: &str = "rgb(112, 153, 182)";

// FIXME: add default macro for all fields except "values".
#[derive(Debug)]
pub struct BarPlot<'a> {
    values: &'a [f64],
    bar_markers: Option<&'a [String]>,
    scale_range: Option<(isize, isize, usize)>,
    res: Option<(usize, usize)>,
    plot_window_scale: Option<(Percentage, Percentage, Percentage, Percentage)>,
    x_axis_tick_length: Option<Percentage>,
    x_markers_at_middle: bool,
    y_axis_tick_length: Option<Percentage>,
    negative_bars_go_down: bool,
    window_border: bool,
    show_horizontal_lines: bool,
    show_vertical_lines: bool,
    plot_border: bool,
    background_color: Option<&'a str>,
    line_color: &'a str,
    tick_color: &'a str,
    text_color: &'a str,
    bar_color: &'a str,
    bar_threshold_colors: Option<(&'a str, &'a str, &'a str, &'a str)>,
    bin_margin: Percentage,
}

impl <'a>BarPlot<'a> {
    pub fn new(values: &'a [f64]) -> Self {
        Self {
            values,
            bar_markers: None,
            scale_range: None,
            res: None,
            plot_window_scale: None,
            x_axis_tick_length: None,
            x_markers_at_middle: false,
            y_axis_tick_length: None,
            negative_bars_go_down: false,
            window_border: false,
            plot_border: false,
            show_horizontal_lines: false,
            show_vertical_lines: false,
            background_color: None,
            line_color: DEFAULT_BASE_COLOR,
            tick_color: DEFAULT_BASE_COLOR,
            text_color: DEFAULT_BASE_COLOR,
            bar_color: DEFAULT_BAR_COLOR,
            bar_threshold_colors: None,
            bin_margin: DEFAULT_BIN_MARGIN,
        }
    }

    pub fn background_color(&mut self, color: &'a str) {
        self.background_color = Some(color);
    }

    pub fn line_color(&mut self, color: &'a str) {
        self.line_color = color;
    }

    pub fn tick_color(&mut self, color: &'a str) {
        self.tick_color = color;
    }

    pub fn text_color(&mut self, color: &'a str) {
        self.text_color = color;
    }

    pub fn bar_threshold_colors(&mut self, min: &'a str, low: &'a str, high: &'a str, max: &'a str) {
        self.bar_threshold_colors = Some((min, low, high, max));
    }

    pub fn show_horizontal_lines(&mut self) {
        self.show_horizontal_lines = true;
    }

    pub fn show_vertical_lines(&mut self) {
        self.show_vertical_lines = true;
    }

    pub fn plot_window_scale(
        &mut self, x_length: Percentage, x_offset: Percentage, y_length: Percentage, y_offset: Percentage
    ) {
        assert!(x_length <= 100 && x_offset <= 100, "plot window width cannot exceed 100%");
        assert!(y_length <= 100 && y_offset <= 100, "plot window height cannot exceed 100%");

        self.plot_window_scale = Some((x_length, x_offset, y_length, y_offset));
    }

    pub fn scale_range(&mut self, min: isize, max: isize, step: usize) {
        self.scale_range = Some((min, max, step));
    }

    pub fn set_bar_markers(&mut self, bar_markers: &'a [String]) {
        self.bar_markers = Some(bar_markers);
    }

    pub fn set_x_markers_at_middle(&mut self) {
        self.x_markers_at_middle = true;
    }

    pub fn set_bin_margin(&mut self, margin: Percentage) {
        self.bin_margin = margin;
    }

    pub fn y_axis_tick_length(&mut self, p: Percentage) {
        self.y_axis_tick_length = Some(p);
    }

    pub fn x_axis_tick_length(&mut self, p: Percentage) {
        self.x_axis_tick_length = Some(p);
    }

    pub fn negative_bars_go_down(&mut self) {
        self.negative_bars_go_down = true;
    }

    pub fn window_border(&mut self) {
        self.window_border = true;
    }

    pub fn plot_border(&mut self) {
        self.plot_border = true;
    }

    pub fn to_svg(&mut self, width: usize, height: usize) -> String {
        self.res = Some((width, height));
        svg::render(self)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    fn rand_f64(start: i32, end_incl: i32) -> f64 {
        rand::rng().random_range(start..=end_incl) as f64
    }

    #[test]
    fn positive_values() {
        let path = Path::new("positive_values.svg");

        // Values for the bars.
        let values: Vec<f64> = vec![
            29.67, 41.99, 64.25, 73.07, 59.71, 42.71, 65.15, 58.86,
            9.52, 91.53, 77.34, 55.66, 11.30, 91.90, 43.09, 65.47,
            66.84, 18.63, 13.42, 30.13, 1.94, 10.38, 58.25, 44.29,
        ];

        // Add every value from 0 to value.len() as a bar marker.
        let bar_markers: Vec<String> = (0..values.len()/3).map(|i| (i*3).to_string()).collect();

        let mut plot = BarPlot::new(&values);
        plot.background_color("Black");
        plot.plot_window_scale(95, 80, 90, 40);
        plot.scale_range(0, 100, 10);
        plot.line_color("LightGreen");
        plot.text_color("LightGreen");
        plot.tick_color("LightGreen");
        plot.x_axis_tick_length(10);
        plot.y_axis_tick_length(10);
        plot.window_border();
        plot.plot_border();
        plot.show_horizontal_lines();
        plot.set_bar_markers(&bar_markers);

        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn includes_negative_values() {
        let path = Path::new("includes_negative_values.svg");

        // Values for the bars.
        let values: Vec<f64> = vec![
            29.67, 41.99, 64.25, 73.07, -59.71, 42.71, 65.15, 58.86,
            9.52, 91.53, -77.34, 55.66, -11.30, 91.90, 43.09, 65.47,
            66.84, 18.63, 13.42, 30.13, 1.94, 10.38, -58.25, 44.29,
        ];

        // Add every third value from 0 to value.len() as a bar marker.
        let bar_markers: Vec<String> = (1..=values.len()).map(|i| i.to_string()).collect();

        let mut plot = BarPlot::new(&values);
        plot.background_color("rgb(30, 35, 45)");
        plot.tick_color("LightGreen");
        plot.line_color("LightBlue");
        plot.plot_window_scale(90, 80, 85, 30);
        plot.scale_range(-80, 100, 10);
        plot.set_x_markers_at_middle();
        plot.y_axis_tick_length(10);
        plot.negative_bars_go_down();
        plot.window_border();
        plot.show_vertical_lines();
        plot.plot_border();
        plot.set_bin_margin(80);

        let light_blue = "rgb(130, 250, 255)";
        let light_green = "rgb(150, 250, 180)";
        let yellow = "rgb(250, 210, 150)";
        let red = "rgb(250, 144, 120)";
        plot.bar_threshold_colors(light_blue, light_green, yellow, red);
        plot.set_bar_markers(&bar_markers);

        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn shows_horizontal_lines_and_vertical_lines() {
        let path = Path::new("shows_horizontal_lines_and_vertical_lines.svg");

        // Values for the bars.
        let mut rng = rand::rng();
        let values: [f64; 17] = core::array::from_fn(|_| rng.random_range(-10.0..10.0));

        // Add every third value from 0 to value.len() as a bar marker.
        let bar_markers: Vec<String> = (0..=values.len()/3).map(|i| (i*3).to_string()).collect();

        let mut plot = BarPlot::new(&values);
        plot.background_color("rgb(30, 35, 45)");
        plot.plot_window_scale(95, 80, 90, 35);
        plot.scale_range(-10, 10, 2);
        plot.set_bar_markers(&bar_markers);
        plot.x_axis_tick_length(30);
        plot.y_axis_tick_length(30);
        plot.plot_border();
        plot.show_horizontal_lines();
        plot.show_vertical_lines();

        let contents = plot.to_svg(840, 520);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }
}
