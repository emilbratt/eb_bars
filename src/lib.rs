// Copyright 2025 Developers of eb_bars.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! EB - Bars: Plotting library for Rust providing a simple way to create barcharts in svg format.
//!
//! # Quick Start
//!
//! The simplest usecase (which you never really might want) is written like so.
//! ```
//! use eb_bars::BarPlot;
//!
//! // Start out with an empty plot.
//! let mut plot = BarPlot::new();
//!
//! // Add a set of values.
//! plot.add_values(&[5.0, 16.4, 17.1, 13.7, 8.9, 3.9, 6.3, 9.6]);
//!
//! // Render to svg format.
//! let svg: String = plot.to_svg(1600, 1000);
//! ```
//!
//! As the above example stands, the largest number will have its bar take up the whole window.
//! Also, the lowest value will be a bar of zero length e.g. no bar at all.
//! The problem is that we have not set a specific scaling for the values.
//! This means that the bars are all scaled based on the minimum and maximum value.
//! Let's improve it a bit in the next section by adding a scale.
//!
//! ```
//! use eb_bars::BarPlot;
//!
//! let mut plot = BarPlot::new();
//!
//! // Add same values as before.
//! plot.add_values(&[5.0, 16.4, 17.1, 13.7, 8.9, 3.9, 6.3, 9.6]);
//!
//! // Here, we are setting a scale range for better visibility/scaling of bars.
//! plot.set_scale_range(0, 20, 2);
//! // The bars will look better, but the numbers on the scale won't be visible.
//! // We need to shrink the plot window relative to the full window.
//! // Keep in mind that all size and offset values are based of a percentage.
//! // Setting a width to 100 means it takes up the whole width.
//! // Same goes for the height.
//!
//! // Let's shrink the plot size.
//! plot.set_plot_window_size(95, 85, 93, 50);
//! // We have now set the width at 95% and moved it 85% right from the left side.
//! // We also set the height at 93% and moved it 50% down from the top.
//! // First two parameters affect the width and the left/right offset respectively.
//! // The last two parameters affect the height and the top/bottom offset respectively.
//!
//! // Let's render the svg.
//! let svg: String = plot.to_svg(1600, 1000);
//! ```


mod svg;

type Percentage = i16;

const VERSION: &str = "0.4.2";
const REPOSITORY: &str = "https://github.com/emilbratt/eb_bars";

const DEFAULT_BAR_COLOR: &str = "rgb(112, 153, 182)";
const DEFAULT_BASE_COLOR: &str = "rgb(197, 197, 197)";
const DEFAULT_BIN_MARGIN: Percentage = 10;
const DEFAULT_BAR_MARGIN: Percentage = 0;

const DEFAULT_TICK_LENGTH: Percentage = 10;
const DEFAULT_FONT_SIZE: Percentage = 100;
const DEFAULT_TEXT_SIDE_OFFSET: Percentage = 35;
const DEFAULT_LEGEND_POSITION: (Percentage, Percentage) = (90, 20);


#[derive(Debug, Default)]
enum BinMarkerPosition {
    Left,
    #[default]
    Middle,
    Right,
}

#[derive(Debug, Default)]
struct PlotLegend<'a> {
    categories: Option<&'a[&'a str]>,
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

#[derive(Debug)]
pub struct BarPlot<'a> {
    values: Vec<&'a [f64]>,
    bin_markers: Option<&'a [String]>,
    scale_range: Option<(i64, i64, usize)>,
    size: Option<(u32, u32)>,
    plot_window_scale: Option<(Percentage, Percentage, Percentage, Percentage)>,
    x_axis_tick_length: Option<Percentage>,
    y_axis_tick_length: Option<Percentage>,
    negative_bars_go_down: bool,
    show_window_border: bool,
    show_plot_border: bool,
    show_horizontal_lines: bool,
    show_vertical_lines: bool,
    background_color: Option<&'a str>,
    line_color: &'a str,
    text_color: &'a str,
    tick_color: &'a str,
    bin_margin: Percentage,
    bar_margin: Percentage,
    plot_text: PlotText<'a>,
    font_size: Percentage,
    bar_color_variant: BarColors<'a>,
    bar_colors_override: Vec<(usize, usize, &'a str)>,
    bin_marker_position: BinMarkerPosition,
    legend: PlotLegend<'a>,
}

impl <'a>BarPlot<'a> {

    /// Instantiate a new plot.
    /// 
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// 
    /// let mut plot = BarPlot::new();
    /// ```
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            bin_markers: None,
            scale_range: None,
            size: None,
            plot_window_scale: None,
            x_axis_tick_length: None,
            y_axis_tick_length: None,
            negative_bars_go_down: false,
            show_window_border: false,
            show_plot_border: false,
            show_horizontal_lines: false,
            show_vertical_lines: false,
            background_color: None,
            line_color: DEFAULT_BASE_COLOR,
            text_color: DEFAULT_BASE_COLOR,
            tick_color: DEFAULT_BASE_COLOR,
            bin_margin: DEFAULT_BIN_MARGIN,
            bar_margin: DEFAULT_BAR_MARGIN,
            plot_text: PlotText::default(),
            font_size: DEFAULT_FONT_SIZE,
            bar_color_variant: BarColors::default(),
            bar_colors_override: Vec::new(),
            bin_marker_position: BinMarkerPosition::default(),
            legend: PlotLegend::default(),
        }
    }

    /// Adding a set of values (bars) to the plot.
    ///
    /// # Takes an array slice of f64.
    /// 
    /// All valus must be f64.
    /// If you have a `Vec<u32>`, make sure to convert it to a `Vec<f64>` before passing it.
    /// Then you can pass the vector as a reference.
    /// 
    /// There must be at least one set of values to produce a plot. :)
    /// 
    /// # This method is required.
    /// 
    /// There must be at least one set of values to produce a plot. :)
    /// 
    /// # Grouped bars
    ///
    /// Calling this method more than once will create `groups` for the values.
    /// This means that the first index of the added values will be the first group,
    /// the second index will be the second group and so on..
    /// E.g. calling this method 5 times will add groups of 5 bars in each bin.
    /// 
    /// * Must be called at least once. A plot without values does not make any sense.. :)
    /// * If called multiple times, each bin will contain a group with values of the same index.
    /// * All arrays passed after first call must be of the `exact` same length as the first array.
    /// 
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// let mut plot = BarPlot::new();
    ///
    /// let apples: Vec<f64> = vec![5., 16., 17., 8., 3.];
    /// let oranges: Vec<f64> = vec![7., 6., 7., 16., 9.];
    /// assert_eq!(apples.len(), oranges.len());
    /// 
    /// // Add a set of values.
    /// plot.add_values(&apples);
    /// 
    /// // Adding a second set of values. Now each group contains two values.
    /// plot.add_values(&oranges);
    /// 
    /// // The first group contains 5 apples and 7 oranges.
    /// // The next one 16 and 6.
    /// // The last group contains 3 apples and 9 oranges.
    /// ```
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

    /// Set a fill color as background.
    ///
    /// By default, the image will be fully transparent where there is nothing drawn on it.
    /// Adding a background color might be a good idea for better visual presentation depending on usecase.
    /// 
    /// Accepted color conventions.
    /// * As it's name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    /// 
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// 
    /// let mut plot = BarPlot::new();
    /// plot.set_background_color("Black");
    /// ```
    pub fn set_background_color(&mut self, color: &'a str) {
        self.background_color = Some(color);
    }

    /// Set color for the lines.
    ///
    /// By default, all lines are drawn with a `default` color.
    /// You can `override` this by setting your own color.
    /// 
    /// Accepted color conventions.
    /// * As it's name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    /// 
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// 
    /// let mut plot = BarPlot::new();
    /// plot.set_line_color("Yellow");
    /// ```
    pub fn set_line_color(&mut self, color: &'a str) {
        self.line_color = color;
    }

    /// Set color for text/numbers.
    ///
    /// By default, all text are drawn with a `default` color.
    /// You can `override` this by setting your own color.
    /// 
    /// Accepted color conventions.
    /// * As it's name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    /// 
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// 
    /// let mut plot = BarPlot::new();
    /// plot.set_text_color("LightBlue");
    /// ```
    pub fn set_text_color(&mut self, color: &'a str) {
        self.text_color = color;
    }

    /// Set color for x-ticks and y-ticks.
    ///
    /// By default, all ticks are drawn with a `default` color.
    /// You can `override` this by setting your own color.
    /// 
    /// Accepted color conventions.
    /// * As it's name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    /// 
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// 
    /// let mut plot = BarPlot::new();
    /// plot.set_text_color("LightBlue");
    /// ```
    pub fn set_tick_color(&mut self, color: &'a str) {
        self.tick_color = color;
    }

    /// Set a single color for all bars.
    ///
    /// By default, all bars are drawn with a `default` color.
    /// You can `override` this by setting a new uniform color value.
    /// 
    /// Accepted color conventions.
    /// * As it's name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    /// 
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// 
    /// let mut plot = BarPlot::new();
    /// plot.set_bar_colors_by_uniform("Green");
    /// ```
    pub fn set_bar_colors_by_uniform(&mut self, color: &'a str) {
        self.bar_color_variant = BarColors::Uniform(color);
    }

    /// Set bar colors by threshold.
    ///
    /// By default, all bars are drawn with a `default` color.
    /// You can `override` this by setting different colors for different thresholds.
    /// The threshold is as follows: minumum value, less than average, greater than or equal to average and max.
    /// 
    /// Accepted color conventions.
    /// * As it's name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    /// 
    /// # Avoid if having more than one set of values.
    /// 
    /// When you add multiple sets of values e.g. calling [`BarPlot::add_values`] multiple times,
    /// then you might want to use [`BarPlot::add_bar_colors_by_category`] instead.
    /// 
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// 
    /// let mut plot = BarPlot::new();
    /// 
    /// // Each color represent how signifacant a value is.
    /// let min = "Red"; // The lowest value will have its bar colored red.
    /// let low = "Orange"; // Low values will have their bars be orange.
    /// let high = "Yellow"; // High values will be yellow.
    /// let max = "Green"; // Max value (tallest bar) will be green.
    /// 
    /// plot.set_bar_colors_by_threshold(min, low, high, max);
    /// ```
    pub fn set_bar_colors_by_threshold(&mut self, min: &'a str, low: &'a str, high: &'a str, max: &'a str) {
        self.bar_color_variant = BarColors::Threshold((min, low, high, max));
    }

    pub fn add_bar_colors_by_category(&mut self, color: &'a str) {
        if let BarColors::Category(v) = &mut self.bar_color_variant {
            v.push(color);
        } else {
            self.bar_color_variant = BarColors::Category(vec![color]);
        }
    }

    pub fn add_bar_colors_from_vec(&mut self, colors: Vec<&'a str>) {
        if let BarColors::Indexed(v) = &mut self.bar_color_variant {
            v.push(colors);
        } else {
            self.bar_color_variant = BarColors::Indexed(vec![colors]);
        }
    }

    pub fn add_bar_color_override(&mut self, bar: usize, color: &'a str) {
        // Will always select the bar from the last added category e.g. after most recent BarPlot.add_values() call.
        assert!(
            !self.values.is_empty(),
            "Can't override bar '{bar}' with color '{color}', because no bars (values) have been added yet."
        );

        // We always use the index of last added values, e.g. we select last added category.
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

    pub fn set_bin_markers_middle(&mut self) {
        self.bin_marker_position = BinMarkerPosition::Middle;
    }

    pub fn set_bin_markers_left(&mut self) {
        self.bin_marker_position = BinMarkerPosition::Left;
    }

    pub fn set_bin_markers_right(&mut self) {
        self.bin_marker_position = BinMarkerPosition::Right;
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
        self.legend.categories = Some(categories);
    }

    pub fn set_legend_position(&mut self, x: Percentage, y: Percentage) {
        self.legend.position = Some((x,y));
    }

    pub fn set_font_size(&mut self, p: Percentage) {
        self.font_size = p;
    }

    pub fn set_show_window_border(&mut self) {
        self.show_window_border = true;
    }

    pub fn set_show_plot_border(&mut self) {
        self.show_plot_border = true;
    }

    pub fn to_svg(&mut self, width: u32, height: u32) -> String {
        assert!(!self.values.is_empty(), "Can not generate plot without any values..");

        let n_categories = self.values.len();
        match &mut self.bar_color_variant {
            BarColors::Category(colors) => {
                let n_colors = colors.len();
                assert_eq!(
                    n_categories,
                    n_colors,
                    "Got {n_categories} categories and {n_colors} colors.",
                );
            }
            BarColors::Indexed(matrix) => {
                let n_color_vectors = matrix.len();
                assert_eq!(
                    n_categories,
                    n_color_vectors,
                    "Got {n_categories} categories and {n_color_vectors} color vectors.",
                );

                for i in 0..n_categories {
                    let values = self.values[i].len();
                    let n_colors = matrix[i].len();
                    assert_eq!(
                        values,
                        n_colors,
                        "Got {values} values and {n_colors} colors for category {i}.",
                    );
                }
            }
            // No need to do assertion on remaining variants..
            _ => (),
        }

        self.size = Some((width, height));
        svg::render(self)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs;

    use rand::Rng;
    use toml::Table;

    use super::*;

    fn _rand_range_f64(start: i32, end_incl: i32) -> f64 {
        rand::rng().random_range(start..=end_incl) as f64
    }

    #[test]
    fn side_text_and_predefined_colors() {
        let path = Path::new("side_text_and_predefined_colors.svg");

        let values = [3.67, 6.99, 6.25, 4.07];
        let labels = ["A", "B", "C", "D"];
        let markers = labels.into_iter().map(|s| s.to_owned()).collect::<Vec<String>>();

        // Different ways to express colors :)
        let red = "Red"; // color-name
        let yellow = "rgb(244, 244, 32)"; // rgb value
        let blue = r"#1111FA"; // hex-value
        let green = "hsl(115, 90.50%, 50.30%)"; // hsl value
        // Putting them in an array with same length as our values.
        let colors = [red, yellow, blue , green];

        let mut plot = BarPlot::new();
        plot.add_values(&values);
        plot.set_bin_markers(&markers);
        plot.add_bar_colors_from_vec(colors.to_vec());
        plot.set_background_color("Black");
        plot.set_plot_window_size(90, 65, 85, 40);
        plot.set_scale_range(0, 10, 2);
        plot.set_line_color("LightGreen");
        plot.set_tick_color("LightGreen");
        plot.set_x_axis_tick_length(10);
        plot.set_y_axis_tick_length(10);
        plot.set_show_window_border();
        plot.set_show_plot_border();
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
        let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let temperatures = [-11.5, -3.5, 1.3, 5.6, 12.3, 21.0, 23.7, 22.5, 12.5, 9.3, 5.6, -2.3];

        let mut plot = BarPlot::new();
        plot.set_negative_bars_go_down();
        plot.set_bin_markers_middle();
        plot.set_background_color("Black");
        plot.set_show_horizontal_lines();
        plot.set_bar_margin(40);
        plot.add_values(&temperatures);

        let min_color = "rgb(107, 235, 255)";
        let low_color = "rgb(126, 255, 165)";
        let high_color = "rgb(255, 233, 133)";
        let max_color = "rgb(250, 107, 91)";
        plot.set_bar_colors_by_threshold(min_color, low_color, high_color, max_color);

        let markers = months.into_iter().map(|s| s.to_owned()).collect::<Vec<String>>();
        plot.set_bin_markers(&markers);

        plot.set_text_top("Mean temperature °C every month in some particular place for some particular year :)");
        plot.set_text_top_offset(40);

        plot.set_plot_window_size(95, 80, 87, 55);
        plot.set_scale_range(-20, 30, 10);
        plot.set_y_axis_tick_length(0);

        let contents = plot.to_svg(1800, 1000);
        if let Err(e) = std::fs::write(path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn marker_on_left_with_horizontal_and_vertical_grid_lines() {
        let path = Path::new("marker_on_left_with_horizontal_and_vertical_grid_lines.svg");

        let mut rng = rand::rng();
        let values: [f64; 17] = core::array::from_fn(|_| rng.random_range(-50.0..50.0));
        let markers: Vec<String> = (0..values.len()).map(|i| (i).to_string()).collect();

        let mut plot = BarPlot::new();
        plot.add_values(&values);
        plot.set_bin_markers(&markers);
        plot.set_font_size(130);
        plot.set_background_color("Black");
        plot.set_plot_window_size(90, 80, 83, 50);
        plot.set_scale_range(-50, 50, 10);
        plot.set_x_axis_tick_length(30);
        plot.set_y_axis_tick_length(30);
        plot.set_show_plot_border();
        plot.set_show_horizontal_lines();
        plot.set_show_vertical_lines();
        plot.set_bar_margin(25);
        plot.set_bin_markers_left();
        plot.set_text_top("This plot shows random values :)");

        let max_color = "rgb(107, 235, 255)";
        let high_color = "rgb(126, 255, 165)";
        let low_color = "rgb(255, 233, 133)";
        let min_color = "rgb(250, 107, 91)";
        plot.set_bar_colors_by_threshold(min_color, low_color, high_color, max_color);

        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn multiple_categories() {
        let path = Path::new("multiple_categories.svg");
        let tomatoes = [29, 41, 64, 83, 59, 42, 65];
        let apples = [9, 51, 67, 55, 11, 93, 43];
        let eggplants = [18, 86, 13, 30, 1, 10, 58];
        let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Satirday", "Sunday"];

        let mut plot = BarPlot::new();

        // Category A.
        let values = tomatoes.into_iter().map(|i| i as f64).collect::<Vec<f64>>();
        plot.add_values(&values);
        plot.add_bar_colors_by_category("Red");
        plot.add_bar_color_override(3, "Tomato"); // Second bar from last added values 'values_a'.

        // Category B.
        let values = apples.into_iter().map(|i| i as f64).collect::<Vec<f64>>();
        plot.add_values(&values);
        plot.add_bar_colors_by_category("LawnGreen");
        plot.add_bar_color_override(5, "PaleGreen"); // Sixth bar from last added values 'values_b'.

        // Category C.
        let values = eggplants.into_iter().map(|i| i as f64).collect::<Vec<f64>>();
        plot.add_values(&values);
        plot.add_bar_colors_by_category("Blue");
        plot.add_bar_color_override(1, "LightSkyBlue"); // Second bar from last added values 'values_c'.

        let bin_markers: Vec<String> = weekdays.iter().map(|s| s.to_string()).collect();
        plot.set_bin_markers(&bin_markers);

        let categories = vec!["Tomatoes", "Apples", "Eggplants"];
        plot.set_legend(&categories);
        plot.set_legend_position(90, 22);

        plot.set_text_top("The highest value from each category have its color 'overriden' with a brighter color");
        plot.set_text_top_offset(40);
        plot.set_text_bottom("Day of harvest");
        plot.set_text_bottom_offset(25);
        plot.set_text_left("Total harvested.");
        plot.set_text_left_offset(25);

        plot.set_background_color("Black");
        plot.set_plot_window_size(80, 30, 85, 40);
        plot.set_scale_range(0, 100, 10);
        plot.set_line_color("LightGreen");
        plot.set_text_color("LightGoldenRodYellow");
        plot.set_tick_color("LightGoldenRodYellow");
        plot.set_x_axis_tick_length(10);
        plot.set_y_axis_tick_length(10);
        plot.set_bin_markers_middle();
        plot.set_show_window_border();
        plot.set_show_plot_border();
        plot.set_show_horizontal_lines();
        plot.set_bin_margin(15);


        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn version_and_repo() {
        // When updating Cargo.toml, make sure to update corresponding values in src files as well.
        let contents = fs::read_to_string("Cargo.toml").unwrap();
        let value = contents.parse::<Table>().unwrap();

        let version = value["package"]["version"].as_str().unwrap();
        assert_eq!(version, VERSION);

        let repository = value["package"]["repository"].as_str().unwrap();
        assert_eq!(repository, REPOSITORY);
    }

    #[test]
    fn a() {
        let mut plot = BarPlot::new();
        plot.add_values(&[5.0, 16.4, 17.1, 13.7, 8.9, 3.9, 6.3, 9.6]);
        plot.set_scale_range(0, 20, 2);
        plot.set_plot_window_size(95, 85, 93, 50);
        let svg = plot.to_svg(1600, 1000);
        let path = Path::new("a.svg");
        if let Err(e) = std::fs::write(&path, svg) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }
}
