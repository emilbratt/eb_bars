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
//! # But the above "Quick Start" looks bad and boring
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
//!
//! It is still kinda boring, so please checkout all the tweaks available in [`BarPlot`].
//!
//! # Important note
//!
//! If the method name is prefixed `add_`, then calling it multiple times will _add_ stuff to the plot.
//! This goes for adding multiple sets of values (categories) and adding colors to those values etc..
//!
//! If the method name is prefixed `set_`, then calling it multiple times will _override_ the previous one.
//! You most certainly never want to call these more than once, unless there is a good reason to.
//!
//! Check out [`BarPlot`] for all implementations.
//!
//! # Panics and error handling.
//!
//! This library has very limited error handling at the moment. Actually, it has none.
//! There are some assertions here and there that will provoke a panic on invalid input.
//! That way, you can try-and-re-try your code until it works.
//!
//! Once everything works, it is very unlikely that it will panic on continous use in your application.
//! However, if you pass values that are generated from a source that you do not have full control over,
//! then the task of making sure the input is sanitized and double checked lies on your end and your code.

mod svg;

type Percentage = i16;

const VERSION: &str = "0.4.2";
const REPOSITORY: &str = "https://github.com/emilbratt/eb_bars";

const DEFAULT_BAR_COLOR: &str = "rgb(112, 153, 182)";
const DEFAULT_BASE_COLOR: &str = "rgb(197, 197, 197)";
const DEFAULT_BIN_GAP: Percentage = 10;
const DEFAULT_BAR_GAP: Percentage = 0;

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
    bin_gap: Percentage,
    bar_gap: Percentage,
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
            bin_gap: DEFAULT_BIN_GAP,
            bar_gap: DEFAULT_BAR_GAP,
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
    /// This means that the first datapoint of each added dataset will be the first group,
    /// the second datapoint of each added dataset will be the second group and so on..
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
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
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
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
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
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_background_color("Black");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_background_color(&mut self, color: &'a str) {
        self.background_color = Some(color);
    }

    /// Set color for the lines.
    ///
    /// By default, all lines are drawn with a `default` color.
    /// You can `override` this by setting your own color.
    ///
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
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
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_line_color("Yellow");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_line_color(&mut self, color: &'a str) {
        self.line_color = color;
    }

    /// Set color for text/numbers.
    ///
    /// By default, all text are drawn with a `default` color.
    /// You can `override` this by setting your own color.
    ///
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
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
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_text_color("LightBlue");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_color(&mut self, color: &'a str) {
        self.text_color = color;
    }

    /// Set color for x-ticks and y-ticks.
    ///
    /// By default, all ticks are drawn with a `default` color.
    /// You can `override` this by setting your own color.
    ///
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
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
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_text_color("LightBlue");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_tick_color(&mut self, color: &'a str) {
        self.tick_color = color;
    }

    /// Set a single color for all bars.
    ///
    /// By default, all bars are drawn with a `default` color.
    /// You can `override` this by setting a new uniform color value.
    ///
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
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
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_bar_colors_by_uniform("Green");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
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
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
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
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// // Each color represent how signifacant a value is.
    /// let min = "Red"; // The lowest value will have its bar colored red.
    /// let low = "Orange"; // Low values will have their bars be orange.
    /// let high = "Yellow"; // High values will be yellow.
    /// let max = "Green"; // Max value (tallest bar) will be green.
    ///
    /// plot.set_bar_colors_by_threshold(min, low, high, max);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bar_colors_by_threshold(&mut self, min: &'a str, low: &'a str, high: &'a str, max: &'a str) {
        self.bar_color_variant = BarColors::Threshold((min, low, high, max));
    }

    /// Add color to last added values.
    ///
    /// By default, all bars are drawn with a `default` color.
    /// You can `override` this by setting different colors for different categories.
    /// Keep in mind that this only make sens to use if you have called [`BarPlot::add_values`]
    /// at least 2 times. The two datasets are treated as two distinct categories.
    /// Note: see [`BarPlot::set_legend`] for displaying the names with their respective color.
    ///
    /// # How it operates
    /// The category added first will have its bars colored first.
    /// Then, any consecutive call will apply proportional to all consecutive calls to [`BarPlot::add_values`].
    ///
    /// In simple terms: if you call [`BarPlot::add_values`] 5 times, then it is required to call this method
    /// `exactly` 5 times. Each time with its own particular color.
    ///
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    ///
    /// # Avoid if having only one set of values.
    ///
    /// If you only add one set of values e.g. calling [`BarPlot::add_values`] one time,
    /// then it makes no sense using this method. Use any of the other ways to set colors.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// let mut plot = BarPlot::new();
    ///
    /// let apples: Vec<f64> = vec![5., 16., 17., 8., 3.];
    /// let oranges: Vec<f64> = vec![7., 6., 7., 16., 9.];
    ///
    /// // Add first set of values.
    /// plot.add_values(&apples);
    /// // Adding a second set of values.
    /// plot.add_values(&oranges);
    ///
    /// // First call adds a color to the first category. Red bacuse apples are often red.
    /// plot.add_bar_colors_by_category("Red");
    /// // Second call adds a color to the second category. Orange because; oranges are orange. :)
    /// plot.add_bar_colors_by_category("Orange");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn add_bar_colors_by_category(&mut self, color: &'a str) {
        if let BarColors::Category(v) = &mut self.bar_color_variant {
            v.push(color);
        } else {
            self.bar_color_variant = BarColors::Category(vec![color]);
        }
    }

    /// Add a set of colors for every bar in last added category.
    ///
    /// By default, all bars are drawn with a `default` color.
    /// You can `override` this by adding an array of colors which have same length as the values.
    ///
    /// # How it operates
    /// The category added first will have its bars colored first.
    /// Then, any consecutive call will apply proportional to all consecutive calls to [`BarPlot::add_values`].
    ///
    /// In simple terms: if you call [`BarPlot::add_values`] 5 times, then it is required to call this method
    /// `exactly` 5 times. Each time with its own particular set of colors.
    ///
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    ///
    /// This method is meant for users that want full control over the bar colors.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// let mut plot = BarPlot::new();
    ///
    /// let apples: Vec<f64> = vec![5., 16., 17., 8., 3.];
    /// let oranges: Vec<f64> = vec![7., 6., 7., 16., 9.];
    ///
    /// // Add first set of values.
    /// plot.add_values(&apples);
    /// // Adding a second set of values.
    /// plot.add_values(&oranges);
    ///
    /// // First call adds a color to the first category. Red bacuse apples are often red.
    /// let clr_red = vec!["Red", "Red", "Red", "Red", "Red"];
    /// plot.add_bar_colors_from_vec(clr_red);
    /// // Second call adds a color to the second category. Orange because; oranges are orange. :)
    /// let clr_orange = vec!["Orange", "Orange", "Orange", "Orange", "Orange"];
    /// plot.add_bar_colors_from_vec(clr_orange);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn add_bar_colors_from_vec(&mut self, colors: Vec<&'a str>) {
        if let BarColors::Indexed(v) = &mut self.bar_color_variant {
            v.push(colors);
        } else {
            self.bar_color_variant = BarColors::Indexed(vec![colors]);
        }
    }

    /// Override any bar with any color.
    ///
    /// This one will override the category index and the value (bar) index with a color.
    /// Say you added 3 sets of values by calling [`BarPlot::add_values`] 3 times.
    /// This means you have 3 categories times `X` values where `X` is the length of array holding
    /// each of set of values.
    ///
    /// # How it operates
    /// The category added first will have its category and bars colored zero indexed.
    /// The first bar in the first category will have index 0-0. The next bar will have 0-1.
    /// You call this method as many times as you need, passing one color at the time.
    ///
    /// Calling [`BarPlot::add_values`] 5 times with 3 values for each set of values,
    /// then the index for category will be 0-4 (5 indexes ) and the index for values will be 0-2 (3 indexes).
    ///
    /// # Accepted color conventions.
    ///
    /// * As its name such as "Red".
    /// * As an RGB value such as "rgb(29, 28, 27)".
    /// * As a HEX value such as "#1111FA".
    /// * As an HSL value such as "hsl(30, 3.80%, 10.20%)".
    ///
    /// # Avoid if having only one set of values.
    ///
    /// If you only add one set of values e.g. calling [`BarPlot::add_values`] one time,
    /// then it makes no sense using this method. Use any of the other ways to set colors.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// let mut plot = BarPlot::new();
    ///
    /// let apples: Vec<f64> = vec![5., 16., 17., 8., 3.];
    /// let oranges: Vec<f64> = vec![7., 6., 7., 16., 9.];
    ///
    /// // Add first set of values.
    /// plot.add_values(&apples);
    /// // Adding a second set of values.
    /// plot.add_values(&oranges);
    ///
    /// // First call adds a color to the first category. Red bacuse apples are often red.
    /// let clr_red = vec!["Red", "Red", "Red", "Red", "Red"];
    /// plot.add_bar_colors_from_vec(clr_red);
    /// // Second call adds a color to the second category. Orange because; oranges are orange. :)
    /// let clr_orange = vec!["Orange", "Orange", "Orange", "Orange", "Orange"];
    /// plot.add_bar_colors_from_vec(clr_orange);
    ///
    /// // Setting the first apple = green.
    /// plot.add_bar_color_override(0, 0, "Green");
    /// // Setting the last orange to blue.. :)
    /// plot.add_bar_color_override(1, 4, "Blue");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn add_bar_color_override(&mut self, category: usize, bar: usize, color: &'a str) {
        // Will always select the bar from the last added category e.g. after most recent BarPlot.add_values() call.
        assert!(
            !self.values.is_empty(),
            "Can't override bar '{bar}' with color '{color}', because no bars (values) have been added yet."
        );
        self.bar_colors_override.push((category, bar, color));
    }

    /// Show horizontal grid lines.
    ///
    /// NOTE: Call [`BarPlot::set_scale_range`] first, otherwise there are no values to base the grid on.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    /// plot.add_values(&[5.0, 16.4, 17.1, 13.7, 8.9, 3.9, 6.3, 9.6]);
    ///
    /// // Needed for horizontal (y-grid) lines.
    /// plot.set_scale_range(0, 20, 2);
    ///
    /// plot.set_show_horizontal_lines();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_show_horizontal_lines(&mut self) {
        self.show_horizontal_lines = true;
    }

    /// Show vertical grid lines.
    ///
    /// NOTE: Call [`BarPlot::set_bin_markers`] first, otherwise there are no values to base the grid on.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// let values = [5.0, 16.4, 17.1, 13.7, 8.9, 3.9, 6.3, 9.6];
    /// plot.add_values(&values);
    ///
    /// // Needed for vertical (x-grid) lines.
    /// let markers: Vec<String> = (0..values.len()).map(|i| (i).to_string()).collect();
    /// plot.set_bin_markers(&markers);
    ///
    /// plot.set_show_vertical_lines();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_show_vertical_lines(&mut self) {
        self.show_vertical_lines = true;
    }

    /// Set size of the barplot size (relative to the canvas/frame).
    ///
    /// By default, the barchart part of the image will take up the full width and length of the frame.
    /// However, by adding literally anything to the barplot, you most likely want to change the size of the plot.
    /// If you skip this method, the text, ticks, markers, legend etc. will not be inside the viewbox of the canvas.
    ///
    /// # How it operates
    ///
    /// We only work with percentage values when defining sizes. A width of 100 means it takes up 100% of the width.
    /// A height of 100 will do the same, but for height.
    /// Same goes for offset. A horizontal offset of 10 means that the offset is pushed 10% from the left.
    /// A vertical offset of 10 means that the offset is pushed 10% from the top.
    /// An offset with value 100 means the offset is pushed all the way to the right (horizontal) or bottom (vertical).
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// let width = 90; // Set width to 90%,
    /// let horizontal_offset = 65; // Move 65% right
    /// let height = 85; // Set height to 85%
    /// let vertical_offset = 40; // Move 40% down.
    /// plot.set_plot_window_size(width, horizontal_offset, height, vertical_offset);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
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

    /// Set a scale for the barchart.
    ///
    /// By default, the scale is calculated using the maximum and minimum value throughout all data.
    /// However, by setting the scale manually we get a more presentable plot.
    /// Pass whole numbers (can be negative) as the scale minimum and scale maximum.
    /// The 3rd parameter is the gap between each number on the scale e.g. the step.
    /// The step must be a positive number as it is an increasing number starting from minimum.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[5.0, 16.4, 17.1, 13.7, 8.9, 3.9, 6.3, 9.6]);
    ///
    /// let min = 0;
    /// let max = 20;
    /// let step = 2;
    ///
    /// plot.set_scale_range(min, max, step);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_scale_range(&mut self, min: i64, max: i64, step: u64) {
        self.scale_range = Some((min, max, step as usize));
    }

    /// Set the labels and markers for each bin/bucket on the x-axis.
    ///
    /// Note: Passing an array with fewer bin markers than added values will cause some bins to be un-labeled.
    /// To make sure everything is correct, it is recommended to pass the same amount of markers as values.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// let absence_boys = [5., 3., 8., 4., 7.];
    /// let absence_girls = [4., 1., 2., 3., 1.];
    /// let weekdays = vec![
    ///     "Monday".to_string(),
    ///     "Tuesday".to_string(),
    ///     "Wednesday".to_string(),
    ///     "Thursday".to_string(),
    ///     "Friday".to_string(),
    /// ];
    ///
    /// plot.add_values(&absence_boys);
    /// plot.add_values(&absence_girls);
    /// plot.set_bin_markers(&weekdays);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_markers(&mut self, bin_markers: &'a [String]) {
        self.bin_markers = Some(bin_markers);
    }

    /// Place the bin markers at the middle of each bin/bucket instead of to the left.
    ///
    /// By default, the bin markers are placed on the left side in between each bin.
    /// You can have the markers placed either `left`, `middle` or `right` depending on usecase.
    ///
    /// Note: check out [`BarPlot::set_bin_markers_left`] and [`BarPlot::set_bin_markers_right`].
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// let values = [1., 2., 3.];
    /// plot.add_values(&values);
    ///
    /// let markers: Vec<String> = (0..values.len()).map(|i| (i).to_string()).collect();
    /// plot.set_bin_markers(&markers);
    ///
    /// plot.set_bin_markers_middle();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_markers_middle(&mut self) {
        self.bin_marker_position = BinMarkerPosition::Middle;
    }

    /// Place the bin markers to the left of each bin/bucket.
    ///
    /// By default, the bin markers are already placed on the left side in between each bin.
    /// This method can be used to _reset_ an eventual change.
    /// You can have the markers placed either `left`, `middle` or `right` depending on usecase.
    ///
    /// Note: check out [`BarPlot::set_bin_markers_middle`] and [`BarPlot::set_bin_markers_right`].
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// let values = [1., 2., 3.];
    /// plot.add_values(&values);
    ///
    /// let markers: Vec<String> = (0..values.len()).map(|i| (i).to_string()).collect();
    /// plot.set_bin_markers(&markers);
    ///
    /// // Setting markers at middle.
    /// plot.set_bin_markers_middle();
    /// // Then back to left side.
    /// plot.set_bin_markers_left();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_markers_left(&mut self) {
        self.bin_marker_position = BinMarkerPosition::Left;
    }

    /// Place the bin markers to the right of each bin/bucket instead of to the left.
    ///
    /// By default, the bin markers are placed on the left side in between each bin.
    /// You can have the markers placed either `left`, `middle` or `right` depending on usecase.
    ///
    /// Note: check out [`BarPlot::set_bin_markers_left`] and [`BarPlot::set_bin_markers_middle`].
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// let values = [1., 2., 3.];
    /// plot.add_values(&values);
    ///
    /// let markers: Vec<String> = (0..values.len()).map(|i| (i).to_string()).collect();
    /// plot.set_bin_markers(&markers);
    ///
    /// plot.set_bin_markers_right();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_markers_right(&mut self) {
        self.bin_marker_position = BinMarkerPosition::Right;
    }

    /// Introduce a `gap` between every bar.
    ///
    /// The gap is calculated using a percentage.
    /// A gap of 0 means there is no gap/air between bars.
    /// A gap of 50 means that the bar and the gap will take up the same width.
    /// A gap of 100 means that the gap will take up all space and so the bar becomes invisible.
    ///
    /// Note: check out [`BarPlot::set_bin_gap`] for gap only betweem bins.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    /// plot.add_values(&[4., 5., 6.]);
    ///
    /// let gap = 30; // The gap will take up 30% of the space, leaving 70% for bar.
    /// plot.set_bar_gap(gap);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bar_gap(&mut self, gap: Percentage) {
        self.bar_gap = gap;
    }

    /// Introduce a `gap` between every bin/bucket.
    ///
    /// The gap is calculated using a percentage.
    /// A gap of 0 means there is no gap/air between bins.
    /// A gap of 50 means that the bin and the gap will take up the same width.
    /// A gap of 100 means that the gap will take up all space and so the bin squashed into nothing.
    ///
    /// Note: check out [`BarPlot::set_bar_gap`] for gap betweem bars.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    /// plot.add_values(&[4., 5., 6.]);
    ///
    /// let gap = 30; // The gap will take up 30% of the space, leaving 70% for bin.
    /// plot.set_bin_gap(gap);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_gap(&mut self, gap: Percentage) {
        self.bin_gap = gap;
    }

    /// Set length for ticks on the y axis.
    ///
    /// The length is calculated using a percentage.
    /// A length of 0 means the tick will not be generated.
    /// A length of 10 means that the tick will be of a _somewhat_ _normal_ length.
    /// A length of greater than 10 means that the tick will be long.
    /// Try different lengths to find the best length for your usecase.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// let len = 20; // The tick length will be of considerate length.
    /// plot.set_y_axis_tick_length(len);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_y_axis_tick_length(&mut self, p: Percentage) {
        self.y_axis_tick_length = Some(p);
    }

    /// Set length for ticks on the x axis.
    ///
    /// The length is calculated using a percentage.
    /// A length of 0 means the tick will not be generated.
    /// A length of 10 means that the tick will be of a _somewhat_ _normal_ length.
    /// A length of greater than 10 means that the tick will be long.
    /// Try different lengths to find the best length for your usecase.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// let len = 20; // The tick length will be of considerate length.
    /// plot.set_x_axis_tick_length(len);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_x_axis_tick_length(&mut self, p: Percentage) {
        self.x_axis_tick_length = Some(p);
    }

    /// Anchor bars at zero so that all negative values point downwards.
    ///
    /// By default, bars are anchored at the floor of the barchart.
    /// However, you might want negative values to stand out by having them point downwards.
    /// This method will apply that configuration for you.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[5.0, -16.4, 17.1, 13.7, 8.9, 3.9, -6.3, 9.6]);
    ///
    /// // Adding some extra tweaks that makes this example more clear.
    /// let min = 0;
    /// let max = 20;
    /// let step = 2;
    /// plot.set_scale_range(min, max, step);
    /// plot.set_plot_window_size(90, 80, 83, 50);
    ///
    /// // Negative bars will now grow downwards instead of upwards. :)
    /// plot.set_negative_bars_go_down();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_negative_bars_go_down(&mut self) {
        self.negative_bars_go_down = true;
    }

    /// Apply text on the left side of the plot window.
    ///
    /// By default, the plot window takes up the whole window.
    /// For the text to be visible, we need to scale down the plot first with [`BarPlot::set_plot_window_size`].
    ///
    /// The text will by default be offset from the plot window with a sane value.
    /// If you want to override this value, you can set a custom offset with [`BarPlot::set_text_left_offset`].
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// plot.set_text_left("This is some text.");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_left(&mut self, text: &'a str) {
        self.plot_text.left = Some(text);
    }

    /// Apply offset for text on the left side of the plot window.
    ///
    /// The offset is calculated percent wise from the frame towards the plot window.
    /// The higher the percentage the closer the text moves towards the plot window.
    ///
    /// Note: you need to explicitly apply text first with [`BarPlot::set_text_left`] first.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// plot.set_text_left("This is some text.");
    ///
    /// let percent = 30;
    /// plot.set_text_left_offset(percent);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_left_offset(&mut self, offset: Percentage) {
        self.plot_text.left_offset = Some(offset);
    }

    /// Apply text on the right side of the plot window.
    ///
    /// By default, the plot window takes up the whole window.
    /// For the text to be visible, we need to scale down the plot first with [`BarPlot::set_plot_window_size`].
    ///
    /// The text will by default be offset from the plot window with a sane value.
    /// If you want to override this value, you can set a custom offset with [`BarPlot::set_text_right_offset`].
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// plot.set_text_right("This is some text.");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_right(&mut self, text: &'a str) {
        self.plot_text.right = Some(text);
    }

    /// Apply offset for text on the right side of the plot window.
    ///
    /// The offset is calculated percent wise from the frame towards the plot window.
    /// The higher the percentage the closer the text moves towards the plot window.
    ///
    /// Note: you need to explicitly apply text first with [`BarPlot::set_text_right`] first.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// plot.set_text_right("This is some text.");
    ///
    /// let percent = 30;
    /// plot.set_text_right_offset(percent);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_right_offset(&mut self, offset: Percentage) {
        self.plot_text.right_offset = Some(offset);
    }

    /// Apply text underneath the plot window.
    ///
    /// By default, the plot window takes up the whole window.
    /// For the text to be visible, we need to scale down the plot first with [`BarPlot::set_plot_window_size`].
    ///
    /// The text will by default be offset from the plot window with a sane value.
    /// If you want to override this value, you can set a custom offset with [`BarPlot::set_text_bottom_offset`].
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// plot.set_text_bottom("This is some text.");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_bottom(&mut self, text: &'a str) {
        self.plot_text.bottom = Some(text);
    }

    /// Apply offset for text underneath the plot window.
    ///
    /// The offset is calculated percent wise from the frame towards the plot window.
    /// The higher the percentage the closer the text moves towards the plot window.
    ///
    /// Note: you need to explicitly apply text first with [`BarPlot::set_text_bottom`] first.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// plot.set_text_bottom("This is some text.");
    ///
    /// let percent = 30;
    /// plot.set_text_bottom_offset(percent);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_bottom_offset(&mut self, offset: Percentage) {
        self.plot_text.bottom_offset = Some(offset);
    }

    /// Apply text above the plot window.
    ///
    /// By default, the plot window takes up the whole window.
    /// For the text to be visible, we need to scale down the plot first with [`BarPlot::set_plot_window_size`].
    ///
    /// The text will by default be offset from the plot window with a sane value.
    /// If you want to override this value, you can set a custom offset with [`BarPlot::set_text_top_offset`].
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// plot.set_text_top("This is some text.");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_top(&mut self, text: &'a str) {
        self.plot_text.top = Some(text);
    }

    /// Apply offset for text above the plot window.
    ///
    /// The offset is calculated percent wise from the frame towards the plot window.
    /// The higher the percentage the closer the text moves towards the plot window.
    ///
    /// Note: you need to explicitly apply text first with [`BarPlot::set_text_top`] first.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// plot.set_text_top("This is some text.");
    ///
    /// let percent = 30;
    /// plot.set_text_top_offset(percent);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_top_offset(&mut self, offset: Percentage) {
        self.plot_text.top_offset = Some(offset);
    }

    /// Apply a legend with category names and their corresponding colors.
    ///
    /// When calling [`BarPlot::add_values`] multiple times each time adding a set of values (categories),
    /// you most likely want a legend to label each category by name and color.
    ///
    /// Note: The legend will use the colors added with [`BarPlot::add_bar_colors_by_category`],
    /// remember to call this method once for each category added.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// let mut plot = BarPlot::new();
    ///
    /// let apples: Vec<f64> = vec![5., 16., 17., 8., 3.];
    /// let oranges: Vec<f64> = vec![7., 6., 7., 16., 9.];
    ///
    /// // Add first set of values.
    /// plot.add_values(&apples);
    /// // Adding a second set of values.
    /// plot.add_values(&oranges);
    ///
    /// // First call adds a color to the first category. Red bacuse apples are often red.
    /// plot.add_bar_colors_by_category("Red");
    /// // Second call adds a color to the second category. Orange because; oranges are orange. :)
    /// plot.add_bar_colors_by_category("Orange");
    ///
    /// // Adding the labels (colors are already added) for legend.
    /// let categories = ["Apples", "Oranges"];
    /// // Applying the legend.
    /// plot.set_legend(&categories);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_legend(&mut self, categories: &'a [&'a str]) {
        self.legend.categories = Some(categories);
    }

    /// Set legend position.
    ///
    /// When calling [`BarPlot::set_legend`], its position is calculated using percentage.
    /// By default, the legend is positioned on the far right side and close to the top.
    /// You can override the default position by passing two values.
    /// First value is the absolute offset from left to right,
    /// the second value is the absolute offset from top to bottom.
    ///
    /// An offset X = 50 and offset Y = 50 will roughly position the legend in the center of the canvas.
    /// More precisely, the top left corner of the legend itself will be pinned on that position.
    ///
    /// NOTE: you might want to resize the plot window to accomodate for the legend if you want it
    /// to be drawn outside the plot, otherwise it will be drawn on top of the plot figure.
    /// Check out [`BarPlot::set_plot_window_size`] for that.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    /// let mut plot = BarPlot::new();
    ///
    /// let apples: Vec<f64> = vec![5., 16., 17., 8., 3.];
    /// let oranges: Vec<f64> = vec![7., 6., 7., 16., 9.];
    ///
    /// // Add first set of values.
    /// plot.add_values(&apples);
    /// // Adding a second set of values.
    /// plot.add_values(&oranges);
    ///
    /// // First call adds a color to the first category. Red bacuse apples are often red.
    /// plot.add_bar_colors_by_category("Red");
    /// // Second call adds a color to the second category. Orange because; oranges are orange. :)
    /// plot.add_bar_colors_by_category("Orange");
    ///
    /// // Adding the labels (colors are already added) for legend.
    /// let categories = ["Apples", "Oranges"];
    /// // Applying the legend.
    /// plot.set_legend(&categories);
    ///
    /// // Make room for legend on the right side by shrinking plot window.
    /// plot.set_plot_window_size(80, 30, 85, 40);
    ///
    /// let offset_x = 90;
    /// let offset_y = 22;
    /// plot.set_legend_position(offset_x, offset_y);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_legend_position(&mut self, x: Percentage, y: Percentage) {
        self.legend.position = Some((x,y));
    }

    /// Apply a custom font-size for all text.
    ///
    /// By default, all text and numbers are rendered with a default font-size. This can be overriden.
    /// The font-size is calculated using a percentage value.
    /// A font-size of size 100 (100%) will not affect the text as it is the default.
    /// You can either increase the size by passing a >100 value or decrease it with <100.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85, 65, 80, 40);
    ///
    /// // Apply some text.
    /// plot.set_text_left("This is some text.");
    ///
    /// // Increase font-size using a percentage greater than 100%.
    /// let size = 130;
    /// plot.set_font_size(size);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_font_size(&mut self, p: Percentage) {
        self.font_size = p;
    }

    /// Apply a border around the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Shrink plot so that it becomes clear that the border goes around the canvas.
    /// plot.set_plot_window_size(80, 30, 85, 40);
    ///
    /// plot.set_show_window_border();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_show_window_border(&mut self) {
        self.show_window_border = true;
    }

    /// Apply a border around the plot.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// // Shrink plot so that it becomes clear that the border goes around the plot window.
    /// plot.set_plot_window_size(80, 30, 85, 40);
    ///
    /// plot.set_show_plot_border();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_show_plot_border(&mut self) {
        self.show_plot_border = true;
    }

    /// Generate the final svg image of all applied content.
    ///
    /// When you are satisfied with all the tweaks in above methods, it is time to generate the final svg.
    /// Keep in mind that you can still add or re-apply changes on the same object after calling this method.
    /// Changes will be applied and included on the following call to this method.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// plot.add_values(&[1., 2., 3.]);
    ///
    /// let mut svg: String = plot.to_svg(1600, 1000);
    ///
    /// // Add a new configuration (just resize plot window).
    /// plot.set_plot_window_size(80, 30, 85, 40);
    ///
    /// // Lets overwrite the old plot with this new configuration.
    /// svg = plot.to_svg(1600, 1000);
    /// ```
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
        plot.set_text_left("Left text | Lorem Ipsum is simply dummy text of the..");
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
        plot.set_bar_gap(40);
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
        plot.set_bar_gap(25);
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
        let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

        let mut plot = BarPlot::new();

        // Category A.
        let values = tomatoes.into_iter().map(|i| i as f64).collect::<Vec<f64>>();
        plot.add_values(&values);
        plot.add_bar_colors_by_category("Red");

        // Category B.
        let values = apples.into_iter().map(|i| i as f64).collect::<Vec<f64>>();
        plot.add_values(&values);
        plot.add_bar_colors_by_category("LawnGreen");

        // Category C.
        let values = eggplants.into_iter().map(|i| i as f64).collect::<Vec<f64>>();
        plot.add_values(&values);
        plot.add_bar_colors_by_category("Blue");

        // Override some colors.
        plot.add_bar_color_override(0, 3, "Tomato"); // Second bar from first added values 'values_a'.
        plot.add_bar_color_override(1, 5, "PaleGreen"); // Sixth bar from second added values 'values_b'.
        plot.add_bar_color_override(2, 1, "LightSkyBlue"); // Second bar from last added values 'values_c'.

        let bin_markers: Vec<String> = weekdays.iter().map(|s| s.to_string()).collect();
        plot.set_bin_markers(&bin_markers);

        let categories = ["Tomatoes", "Apples", "Eggplants"];
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
        plot.set_bin_gap(15);

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
}
