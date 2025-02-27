// #![allow(unused)]
mod svg;

type Percentage = u16;

const DEFAULT_BIN_MARGIN: Percentage = 2; // Distance/gap between bins.
const DEFAULT_BASE_COLOR: &str = "rgb(197, 197, 197)";
const DEFAULT_BAR_COLOR: &str = "rgb(112, 153, 182)";

#[derive(Debug, Default)]
struct PlotText<'a> {
    left: Option<&'a str>,
    left_offset: Option<Percentage>,

    right: Option<&'a str>,
    right_offset: Option<Percentage>,

    top: Option<&'a str>,
    top_offset: Option<Percentage>,

    bottom: Option<&'a str>,
    bottom_offset: Option<Percentage>,
}

// FIXME: add default macro for all fields except "values".
#[derive(Debug)]
pub struct BarPlot<'a> {
    values: &'a [f64],
    bar_markers: Option<&'a [String]>,
    scale_range: Option<(i64, i64, usize)>,
    res: Option<(u32, u32)>,
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
    text_color: &'a str,
    tick_color: &'a str,
    bar_color: &'a str,
    bar_threshold_colors: Option<(&'a str, &'a str, &'a str, &'a str)>,
    bin_margin: Percentage,
    plot_text: PlotText<'a>,
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
            text_color: DEFAULT_BASE_COLOR,
            tick_color: DEFAULT_BASE_COLOR,
            bar_color: DEFAULT_BAR_COLOR,
            bar_threshold_colors: None,
            bin_margin: DEFAULT_BIN_MARGIN,
            plot_text: PlotText::default(),
        }
    }

    pub fn set_background_color(&mut self, color: &'a str) {
        self.background_color = Some(color);
    }

    pub fn set_line_color(&mut self, color: &'a str) {
        self.line_color = color;
    }

    pub fn set_text_color(&mut self, color: &'a str) {
        self.text_color = color;
    }

    pub fn set_tick_color(&mut self, color: &'a str) {
        self.tick_color = color;
    }

    pub fn set_bar_threshold_colors(&mut self, min: &'a str, low: &'a str, high: &'a str, max: &'a str) {
        self.bar_threshold_colors = Some((min, low, high, max));
    }

    pub fn set_show_horizontal_lines(&mut self) {
        self.show_horizontal_lines = true;
    }

    pub fn set_show_vertical_lines(&mut self) {
        self.show_vertical_lines = true;
    }

    pub fn set_plot_window_scale(
        &mut self, x_length: Percentage, x_offset: Percentage, y_length: Percentage, y_offset: Percentage
    ) {
        assert!(x_length <= 100 && x_offset <= 100, "plot window width cannot exceed 100%");
        assert!(y_length <= 100 && y_offset <= 100, "plot window height cannot exceed 100%");

        self.plot_window_scale = Some((x_length, x_offset, y_length, y_offset));
    }

    pub fn set_scale_range(&mut self, min: i64, max: i64, step: u64) {
        self.scale_range = Some((min, max, step as usize));
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

    pub fn set_y_axis_tick_length(&mut self, p: Percentage) {
        self.y_axis_tick_length = Some(p);
    }

    pub fn set_x_axis_tick_length(&mut self, p: Percentage) {
        self.x_axis_tick_length = Some(p);
    }

    pub fn set_negative_bars_go_down(&mut self) {
        self.negative_bars_go_down = true;
    }

    pub fn set_left_text(&mut self, text: &'a str) {
        self.plot_text.left = Some(text);
    }
    pub fn set_left_text_offset(&mut self, offset: Percentage) {
        self.plot_text.left_offset = Some(offset);
    }

    pub fn set_right_text(&mut self, text: &'a str) {
        self.plot_text.right = Some(text);
    }

    pub fn set_right_text_offset(&mut self, offset: Percentage) {
        self.plot_text.right_offset = Some(offset);
    }

    pub fn set_bottom_text(&mut self, text: &'a str) {
        self.plot_text.bottom = Some(text);
    }
    pub fn set_bottom_text_offset(&mut self, offset: Percentage) {
        self.plot_text.bottom_offset = Some(offset);
    }

    pub fn set_top_text(&mut self, text: &'a str) {
        self.plot_text.top = Some(text);
    }
    pub fn set_top_text_offset(&mut self, offset: Percentage) {
        self.plot_text.top_offset = Some(offset);
    }

    pub fn set_window_border(&mut self) {
        self.window_border = true;
    }

    pub fn set_plot_border(&mut self) {
        self.plot_border = true;
    }

    pub fn to_svg(&mut self, width: u32, height: u32) -> String {
        self.res = Some((width, height));
        svg::render(self)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rand::Rng;

    use super::*;

    fn _rand_f64(start: i32, end_incl: i32) -> f64 {
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
        plot.set_background_color("Black");
        plot.set_plot_window_scale(90, 65, 85, 40);
        plot.set_scale_range(0, 100, 10);
        plot.set_line_color("LightGreen");
        plot.set_tick_color("LightGreen");
        plot.set_x_axis_tick_length(10);
        plot.set_y_axis_tick_length(10);
        plot.set_window_border();
        plot.set_plot_border();
        plot.set_show_horizontal_lines();
        plot.set_bar_markers(&bar_markers);
        plot.set_text_color("LightGoldenRodYellow");
        plot.set_left_text("Left text | Lorem Ipsum is simply dummy text of the..");
        plot.set_left_text_offset(20);
        plot.set_bottom_text("Bottom text | Lorem Ipsum is simply dummy text of the printing and typesetting industry.");
        plot.set_bottom_text_offset(25);
        plot.set_right_text("Right text | Lorem Ipsum is simply dummy text of the..");
        plot.set_right_text_offset(40);
        plot.set_top_text("Top text | Lorem Ipsum is simply dummy text of the printing and typesetting industry.");
        plot.set_top_text_offset(40);

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
            29.67, 41.99, 64.25, 73.07, -59.71, 65.15, 58.86,
            9.52, 91.53, -77.34, 55.66, -11.30, 43.09, 65.47,
        ];

        // Add every third value from 0 to value.len() as a bar marker.
        let bar_markers: Vec<String> = (1..=values.len()).map(|i| i.to_string()).collect();

        let mut plot = BarPlot::new(&values);
        plot.set_background_color("rgb(30, 35, 45)");
        plot.set_tick_color("LightGreen");
        plot.set_line_color("LightBlue");
        plot.set_plot_window_scale(90, 80, 85, 30);
        plot.set_scale_range(-80, 100, 10);
        plot.set_x_markers_at_middle();
        plot.set_y_axis_tick_length(10);
        plot.set_negative_bars_go_down();
        plot.set_window_border();
        plot.set_show_vertical_lines();
        plot.set_plot_border();
        plot.set_bin_margin(80);

        let light_blue = "rgb(130, 250, 255)";
        let light_green = "rgb(150, 250, 180)";
        let yellow = "rgb(250, 210, 150)";
        let red = "rgb(250, 144, 120)";
        plot.set_bar_threshold_colors(light_blue, light_green, yellow, red);
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
        plot.set_background_color("rgb(30, 35, 45)");
        plot.set_plot_window_scale(95, 80, 90, 35);
        plot.set_scale_range(-10, 10, 2);
        plot.set_bar_markers(&bar_markers);
        plot.set_x_axis_tick_length(30);
        plot.set_y_axis_tick_length(30);
        plot.set_plot_border();
        plot.set_show_horizontal_lines();
        plot.set_show_vertical_lines();

        let contents = plot.to_svg(840, 520);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }
}
