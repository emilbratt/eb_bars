// #![allow(unused)]

mod svg;

type Percentage = i16;

// FIXME: add test to verify matching version in Cargo.toml and here.
const VERSION: &str = "0.3.0";
// FIXME: add test to verify matching repository in Cargo.toml and here.
const REPOSITORY: &str = "https://github.com/emilbratt/eb_bars";

const DEFAULT_BAR_COLOR: &str = "rgb(112, 153, 182)";
const DEFAULT_BASE_COLOR: &str = "rgb(197, 197, 197)";
const DEFAULT_BIN_MARGIN: Percentage = 10;
const DEFAULT_BAR_MARGIN: Percentage = 0;

#[derive(Debug, Default)]
struct PlotLegend<'a> {
    titles: Option<&'a[&'a str]>,
    position: Option<(Percentage, Percentage)>,
}

#[derive(Debug)]
enum BarColors<'a> {
    Category(Vec<&'a str>), // Each category has its own color.
    Indexed(Vec<Vec<&'a str>>), // Every bar has its own selected color.
    Threshold((&'a str, &'a str, &'a str, &'a str)), // Every bar is given its color based on its value.
    Uniform(&'a str), // All bars are the same color.
}

impl <'a>Default for BarColors<'a> {
    fn default() -> Self {
        Self::Uniform(DEFAULT_BAR_COLOR)
    }
}

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

// FIXME: add default macro for all fields".
#[derive(Debug)]
pub struct BarPlot<'a> {
    values: Vec<&'a [f64]>,
    bin_markers: Option<&'a [String]>,
    scale_range: Option<(i64, i64, usize)>,
    size: Option<(u32, u32)>,
    plot_window_scale: Option<(Percentage, Percentage, Percentage, Percentage)>,
    x_axis_tick_length: Option<Percentage>,
    bin_markers_at_middle: bool,
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
    bin_margin: Percentage,
    bar_margin: Percentage,
    plot_text: PlotText<'a>,
    enum_bar_colors: BarColors<'a>,
    bar_colors_override: Vec<(usize, usize, &'a str)>, // (bar category, bar index) => color
    legend: PlotLegend<'a>,
}

impl <'a>BarPlot<'a> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            bin_markers: None,
            scale_range: None,
            size: None,
            plot_window_scale: None,
            x_axis_tick_length: None,
            bin_markers_at_middle: false,
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
            bin_margin: DEFAULT_BIN_MARGIN,
            bar_margin: DEFAULT_BAR_MARGIN,
            plot_text: PlotText::default(),
            enum_bar_colors: BarColors::default(),
            bar_colors_override: Vec::new(),
            legend: PlotLegend::default(),
        }
    }

    pub fn add_values(&mut self, values: &'a [f64]) {
        if !self.values.is_empty() {
            // This makes sure that all new values are of same length e.g. same count as previous values.
            // This is needed so that all bins have the same amount of bars in them,
            // ..as long as we add more than 1 category, that is..
            let exp_count = self.values[0].len();
            let count = values.len();
            assert_eq!(
                exp_count,
                count,
                "Added values should be same count as old, expected {exp_count}, got {count}"
            );
        }
        self.values.push(values);
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

    pub fn set_bar_colors_by_uniform(&mut self, color: &'a str) {
        self.enum_bar_colors = BarColors::Uniform(color);
    }

    pub fn set_bar_colors_by_threshold(&mut self, min: &'a str, low: &'a str, high: &'a str, max: &'a str) {
        self.enum_bar_colors = BarColors::Threshold((min, low, high, max));
    }

    pub fn add_bar_colors_by_category(&mut self, color: &'a str) {
        if let BarColors::Category(ref mut v) = self.enum_bar_colors {
            v.push(color);
        } else {
            self.enum_bar_colors = BarColors::Category(vec![color]);
        }
    }

    pub fn add_bar_colors_from_vec(&mut self, colors: Vec<&'a str>) {
        if let BarColors::Indexed(ref mut v) = self.enum_bar_colors {
            v.push(colors);
        } else {
            self.enum_bar_colors = BarColors::Indexed(vec![colors]);
        }
    }

    pub fn add_bar_color_override(&mut self, bar: usize, color: &'a str) {
        // Will always select the bar from the last added category e.g. after most recent BarPlot.add_values() call.
        assert!(
            !self.values.is_empty(),
            "Can't override bar '{bar}' with color '{color}', because no bars (values) have been previously added."
        );

        // We always use the index of last added values, meaning we select last added category.
        let category = self.values.len() - 1;
        self.bar_colors_override.push((category, bar, color));
    }

    pub fn set_show_horizontal_lines(&mut self) {
        self.show_horizontal_lines = true;
    }

    pub fn set_show_vertical_lines(&mut self) {
        self.show_vertical_lines = true;
    }

    pub fn set_plot_window_size(
        &mut self,
        x_length: Percentage,
        x_offset: Percentage,
        y_length: Percentage,
        y_offset: Percentage
    ) {
        assert!(x_length <= 100 && x_offset <= 100, "plot window width cannot exceed 100%");
        assert!(y_length <= 100 && y_offset <= 100, "plot window height cannot exceed 100%");

        self.plot_window_scale = Some((x_length, x_offset, y_length, y_offset));
    }

    pub fn set_scale_range(&mut self, min: i64, max: i64, step: u64) {
        self.scale_range = Some((min, max, step as usize));
    }

    pub fn set_bin_markers(&mut self, bin_markers: &'a [String]) {
        self.bin_markers = Some(bin_markers);
    }

    pub fn set_bin_markers_at_middle(&mut self) {
        self.bin_markers_at_middle = true;
    }

    pub fn set_bar_margin(&mut self, margin: Percentage) {
        self.bar_margin = margin;
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

    pub fn set_text_left(&mut self, text: &'a str) {
        self.plot_text.left = Some(text);
    }

    pub fn set_text_left_offset(&mut self, offset: Percentage) {
        self.plot_text.left_offset = Some(offset);
    }

    pub fn set_text_right(&mut self, text: &'a str) {
        self.plot_text.right = Some(text);
    }

    pub fn set_text_right_offset(&mut self, offset: Percentage) {
        self.plot_text.right_offset = Some(offset);
    }

    pub fn set_text_bottom(&mut self, text: &'a str) {
        self.plot_text.bottom = Some(text);
    }

    pub fn set_text_bottom_offset(&mut self, offset: Percentage) {
        self.plot_text.bottom_offset = Some(offset);
    }

    pub fn set_text_top(&mut self, text: &'a str) {
        self.plot_text.top = Some(text);
    }

    pub fn set_text_top_offset(&mut self, offset: Percentage) {
        self.plot_text.top_offset = Some(offset);
    }

    pub fn set_legend(&mut self, categories: &'a [&'a str]) {
        self.legend.titles = Some(categories);
    }

    pub fn set_legend_position(&mut self, x: Percentage, y: Percentage) {
        self.legend.position = Some((x,y));
    }

    pub fn set_window_border(&mut self) {
        self.window_border = true;
    }

    pub fn set_plot_border(&mut self) {
        self.plot_border = true;
    }

    pub fn to_svg(&mut self, width: u32, height: u32) -> String {
        assert!(!self.values.is_empty(), "Can not generate plot without any values..");

        // FIXME: add more assertions about colors..
        if let BarColors::Category(colors) = &self.enum_bar_colors {
            let n = self.values.len();
            let n_colors = colors.len();
            assert_eq!(n, n_colors, "Categories dont match colors. Got {n} categories and {n_colors} colors");
        }

        self.size = Some((width, height));
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

        let values: Vec<f64> = vec![
            29.67, 41.99, 64.25, 73.07, 59.71, 42.71, 65.15, 58.86,
            9.52, 91.53, 77.34, 55.66, 11.30, 91.90, 43.09, 65.47,
            66.84, 18.63, 13.42, 30.13, 1.94, 10.38, 58.25, 44.29,
        ];
        let bin_markers: Vec<String> = (0..values.len()/3).map(|i| (i*3).to_string()).collect();

        let mut plot = BarPlot::new();
        plot.add_values(&values);
        plot.set_bin_markers(&bin_markers);
        plot.set_bar_colors_by_uniform("LightBlue");
        plot.set_background_color("Black");
        plot.set_plot_window_size(90, 65, 85, 40);
        plot.set_scale_range(0, 100, 10);
        plot.set_line_color("LightGreen");
        plot.set_tick_color("LightGreen");
        plot.set_x_axis_tick_length(10);
        plot.set_y_axis_tick_length(10);
        plot.set_window_border();
        plot.set_plot_border();
        plot.set_show_horizontal_lines();
        plot.set_text_color("LightGoldenRodYellow");
        plot.set_text_left("Left |_left Lorem Ipsum is simply dummy text of the..");
        plot.set_text_left_offset(20);
        plot.set_text_bottom("Bottom text | Lorem Ipsum is simply dummy text of the printing and typesetting industry.");
        plot.set_text_bottom_offset(25);
        plot.set_text_right("Right text | Lorem Ipsum is simply dummy text of the..");
        plot.set_text_right_offset(40);
        plot.set_text_top("Top text | Lorem Ipsum is simply dummy text of the printing and typesetting industry.");
        plot.set_text_top_offset(40);

        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn negative_bars_go_down() {
        let path = Path::new("negative_bars_go_down.svg");

        let mut plot = BarPlot::new();

        let values: Vec<f64> = vec![19.52, 91.53, -67.34, 55.66, -21.30, 43.09, -45.47];
        plot.add_values(&values);

        let colors = vec!["LightCoral", "LightBlue", "LightGreen", "LightYellow", "Pink", "Aqua", "Plum"];
        plot.add_bar_colors_from_vec(colors);

        let bin_markers: Vec<String> = (1..=values.len()).map(|i| i.to_string()).collect();
        plot.set_bin_markers(&bin_markers);

        plot.set_scale_range(-80, 100, 10);
        plot.set_background_color("Black");
        plot.set_plot_window_size(90, 80, 85, 30);
        plot.set_bin_markers_at_middle();
        plot.set_y_axis_tick_length(10);
        plot.set_negative_bars_go_down();
        plot.set_window_border();
        plot.set_show_vertical_lines();
        plot.set_plot_border();
        plot.set_bin_margin(80);

        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn shows_horizontal_lines_and_vertical_lines() {
        let path = Path::new("shows_horizontal_lines_and_vertical_lines.svg");

        let mut rng = rand::rng();
        let values: [f64; 17] = core::array::from_fn(|_| rng.random_range(-50.0..50.0));
        let bin_markers: Vec<String> = (0..values.len()).map(|i| (i).to_string()).collect();

        let mut plot = BarPlot::new();
        plot.add_values(&values);
        plot.set_bin_markers(&bin_markers);
        plot.set_background_color("Black");
        plot.set_plot_window_size(95, 90, 90, 35);
        plot.set_scale_range(-50, 50, 5);
        plot.set_x_axis_tick_length(30);
        plot.set_y_axis_tick_length(30);
        plot.set_plot_border();
        plot.set_show_horizontal_lines();
        plot.set_show_vertical_lines();
        plot.set_bar_margin(10);

        let blue = "rgb(107, 235, 255)";
        let green = "rgb(126, 255, 165)";
        let yellow = "rgb(255, 233, 133)";
        let red = "rgb(250, 107, 91)";
        plot.set_bar_colors_by_threshold(red, yellow, green, blue);

        let contents = plot.to_svg(840, 520);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn multiple_categories() {
        let path = Path::new("multiple_categories.svg");
        let weekdays: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Satirday", "Sunday"];

        let mut plot = BarPlot::new();

        // Category A.
        let values_a: Vec<f64> = vec![29.67, 41.99, 64.25, 83.07, 59.71, 42.71, 65.15];
        plot.add_values(&values_a);
        plot.add_bar_colors_by_category("Red");
        plot.add_bar_color_override(3, "Tomato"); // Second bar from last added values 'values_a'.

        // Category B.
        let values_b: Vec<f64> = vec![9.52, 51.53, 67.34, 55.66, 11.30, 93.90, 43.09];
        plot.add_values(&values_b);
        plot.add_bar_colors_by_category("LawnGreen");
        plot.add_bar_color_override(5, "PaleGreen"); // Sixth bar from last added values 'values_b'.

        // Category C.
        let values_c: Vec<f64> = vec![18.63, 86.84, 13.42, 30.13, 1.94, 10.38, 58.25];
        plot.add_values(&values_c);
        plot.add_bar_colors_by_category("Blue");
        plot.add_bar_color_override(1, "LightSkyBlue"); // Second bar from last added values 'values_c'.

        let categories = vec!["Tomatoes", "Apples", "Blueberries"];
        plot.set_legend(&categories);
        plot.set_legend_position(90, 22);

        let bin_markers: Vec<String> = weekdays.iter().map(|s| s.to_string()).collect();
        plot.set_bin_markers(&bin_markers);

        plot.set_background_color("Black");
        plot.set_plot_window_size(80, 30, 85, 40);
        plot.set_scale_range(0, 100, 10);
        plot.set_line_color("LightGreen");
        plot.set_text_color("LightGoldenRodYellow");
        plot.set_tick_color("LightGoldenRodYellow");
        plot.set_x_axis_tick_length(10);
        plot.set_y_axis_tick_length(10);
        plot.set_bin_markers_at_middle();
        plot.set_window_border();
        plot.set_plot_border();
        plot.set_show_horizontal_lines();
        plot.set_bin_margin(15);
        plot.set_text_top("The highest value from each category have its color 'overriden' with a brighter color");
        plot.set_text_top_offset(40);
        plot.set_text_bottom("Day of harvest");
        plot.set_text_bottom_offset(25);
        plot.set_text_left("Amount harvested in kg.");
        plot.set_text_left_offset(25);

        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }
}
