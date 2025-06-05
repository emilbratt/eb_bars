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
//! As the above example stands, the largest number will have its bar take up the full height of the window.
//! Also, the lowest value will be a bar of zero height e.g. no bar at all.
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
//! plot.set_plot_window_size(95.0, 85.0, 93.0, 50.0);
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

type Percentage = f64;

const VERSION: &str = "0.7.0";
const REPOSITORY: &str = "https://github.com/emilbratt/eb_bars";

const DEFAULT_SIZE: (u32, u32) = (1600, 1000);

const DEFAULT_BAR_COLOR: &str = "rgb(112, 153, 182)";
const DEFAULT_BASE_COLOR: &str = "rgb(197, 197, 197)";
const DEFAULT_BAR_GAP: Percentage = 0.0;
const DEFAULT_BIN_GAP: Percentage = 10.0;

const DEFAULT_FONT_SIZE: Percentage = 100.0;
const DEFAULT_LEGEND_POSITION: (Percentage, Percentage) = (90.0, 20.0);
const DEFAULT_TEXT_SIDE_OFFSET: Percentage = 35.0;
const DEFAULT_TICK_LENGTH: Percentage = 10.0;


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
enum BarColorLayout<'a> {
    Category(Vec<&'a str>), // Each category has its own color.
    Indexed(Vec<Vec<&'a str>>), // Every bar has its own selected color.
    Threshold((&'a str, &'a str, &'a str, &'a str)), // Every bar is given its color based on its value.
    Uniform(&'a str), // All bars are the same color.
}

impl Default for BarColorLayout<'_> {
    fn default() -> Self {
        Self::Uniform(DEFAULT_BAR_COLOR)
    }
}

#[derive(Debug, Default)]
struct BarColors<'a> {
    layout: BarColorLayout<'a>,
    overrides: Vec<(usize, usize, &'a str)>,
}

#[derive(Debug)]
struct Colors<'a> {
    background: Option<&'a str>,
    bars: BarColors<'a>,
    line: &'a str,
    text: &'a str,
    tick: &'a str,
}

impl Default for Colors<'_> {
    fn default() -> Self {
        Self {
            background: None,
            bars: BarColors::default(),
            line: DEFAULT_BASE_COLOR,
            text: DEFAULT_BASE_COLOR,
            tick: DEFAULT_BASE_COLOR,
        }
    }
}

#[derive(Debug, Default)]
struct Show {
    window_border: bool,
    plot_border: bool,
    horizontal_lines: bool,
    vertical_lines: bool,
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
struct PlotLayout {
    bar_gap: Percentage,
    bin_gap: Percentage,
    bin_marker_position: BinMarkerPosition,
    font_size: Percentage,
    plot_window_scale: Option<(Percentage, Percentage, Percentage, Percentage)>,
    scale_range: Option<(i64, i64, usize)>,
    x_axis_tick_length: Percentage,
    y_axis_tick_length: Percentage,
    negative_bars_go_down: bool,
}

impl Default for PlotLayout {
    fn default() -> Self {
        Self {
            bin_gap: DEFAULT_BIN_GAP,
            bar_gap: DEFAULT_BAR_GAP,
            bin_marker_position: BinMarkerPosition::default(),
            font_size: DEFAULT_FONT_SIZE,
            plot_window_scale: None,
            scale_range: None,
            x_axis_tick_length: DEFAULT_TICK_LENGTH,
            y_axis_tick_length: DEFAULT_TICK_LENGTH,
            negative_bars_go_down: false,
        }
    }
}

#[derive(Debug)]
enum LinesAt<'a> {
    Horizontal(f64, &'a str),
    Vertical(f64, &'a str),
}

#[derive(Debug)]
pub struct BarPlot<'a> {
    values: Vec<&'a [f64]>,
    markers: Option<&'a [&'a str]>,
    lines_at: Vec<LinesAt<'a>>,
    size: (u32, u32),
    colors: Colors<'a>,
    legend: PlotLegend<'a>,
    layout: PlotLayout,
    show: Show,
    plot_text: PlotText<'a>,
}

// FIXME: add new with default, allow for now with attribute below..
#[allow(clippy::new_without_default)]
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
            markers: None,
            lines_at: Vec::new(),
            size: DEFAULT_SIZE,
            colors: Colors::default(),
            legend: PlotLegend::default(),
            layout: PlotLayout::default(),
            show: Show::default(),
            plot_text: PlotText::default(),
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
    /// # This method is required.
    ///
    /// There must be at least one set of values to produce a plot. :)
    ///
    /// # Grouped bars
    ///
    /// Calling this method more than once will create `groups` for values of same index.
    /// This means that the first datapoint of each added dataset will be the first group,
    /// the second datapoint of each added dataset will be the second group and so on..
    /// E.g. calling this method 5 times will add groups of 5 bars in each bin.
    ///
    /// # Short summary
    ///
    /// * Must be called at least once. A plot without values does not make any sense.. :)
    /// * If called multiple times, each bin will contain a group with values of the same index.
    /// * All arrays passed after first call must be of the `exact` same length as the first array.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
    /// let mut plot = BarPlot::new();
    ///
    /// let apples: Vec<f64> = vec![5., 16., 17., 8., 3.];
    /// let oranges: Vec<f64> = vec![7., 6., 7., 16., 9.];
    /// // The first group contains 5 apples and 7 oranges.
    /// // The second one 16 and 6 respectively.
    /// // The last group contains 3 apples and 9 oranges.
    ///
    /// // Add the first set of values.
    /// plot.add_values(&apples);
    ///
    /// // Add the second set of values.
    /// plot.add_values(&oranges);
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
    ///
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_background_color("Black");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_background_color(&mut self, color: &'a str) {
        self.colors.background = Some(color);
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
    ///
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_line_color("Yellow");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_line_color(&mut self, color: &'a str) {
        self.colors.line = color;
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
    ///
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_text_color("LightBlue");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_text_color(&mut self, color: &'a str) {
        self.colors.text = color;
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
    ///
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_text_color("LightBlue");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_tick_color(&mut self, color: &'a str) {
        self.colors.tick = color;
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
    ///
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// plot.set_bar_colors_by_uniform("Green");
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bar_colors_by_uniform(&mut self, color: &'a str) {
        self.colors.bars.layout = BarColorLayout::Uniform(color);
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
    ///
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
        self.colors.bars.layout = BarColorLayout::Threshold((min, low, high, max));
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
    ///
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
        if let BarColorLayout::Category(v) = &mut self.colors.bars.layout {
            v.push(color);
        } else {
            self.colors.bars.layout = BarColorLayout::Category(vec![color]);
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
    ///
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
        if let BarColorLayout::Indexed(v) = &mut self.colors.bars.layout {
            v.push(colors);
        } else {
            self.colors.bars.layout = BarColorLayout::Indexed(vec![colors]);
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
    ///
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
        self.colors.bars.overrides.push((category, bar, color));
    }

    /// Show horizontal grid lines.
    ///
    /// # Important
    ///
    /// Call [`BarPlot::set_scale_range`] first, otherwise there are no values to base the grid on.
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
    /// // Needed for horizontal (y-grid) lines.
    /// plot.set_scale_range(0, 20, 2);
    ///
    /// plot.set_show_horizontal_lines();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_show_horizontal_lines(&mut self) {
        self.show.horizontal_lines = true;
    }

    /// Add custom horizontal grid lines.
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
    /// // Add a horizontal lines.
    /// let line_color = "rgb(126, 255, 165)";
    /// plot.add_horizontal_line_at(25.0, line_color);
    /// plot.add_horizontal_line_at(50.0, line_color);
    /// plot.add_horizontal_line_at(75.0, line_color);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn add_horizontal_line_at(&mut self, p: Percentage, color: &'a str) {
        self.lines_at.push(LinesAt::Horizontal(p, color));
    }

    /// Show vertical grid lines.
    ///
    /// # Important
    ///
    ///  Call [`BarPlot::set_bin_markers`] first, otherwise there are no values to base the grid on.
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
    /// let markers = markers.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    /// plot.set_bin_markers(&markers);
    ///
    /// plot.set_show_vertical_lines();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_show_vertical_lines(&mut self) {
        self.show.vertical_lines = true;
    }

    /// Add custom vertical grid lines.
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
    /// // Add vertical lines.
    /// let line_color = "rgb(126, 255, 165)";
    /// plot.add_vertical_line_at(25.0, line_color);
    /// plot.add_vertical_line_at(50.0, line_color);
    /// plot.add_vertical_line_at(75.0, line_color);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn add_vertical_line_at(&mut self, p: Percentage, color: &'a str) {

        self.lines_at.push(LinesAt::Vertical(p, color));
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
    ///
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// let width = 90.0; // Set width to 90%,
    /// let horizontal_offset = 65.0; // Move 65% right
    /// let height = 85.0; // Set height to 85%
    /// let vertical_offset = 40.0; // Move 40% down.
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
        assert!(x_length <= 100.0 && x_offset <= 100.0, "plot window width cannot exceed 100%");
        assert!(y_length <= 100.0 && y_offset <= 100.0, "plot window height cannot exceed 100%");

        self.layout.plot_window_scale = Some((x_length, x_offset, y_length, y_offset));
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
        self.layout.scale_range = Some((min, max, step as usize));
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
    /// let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday",];
    ///
    /// plot.add_values(&absence_boys);
    /// plot.add_values(&absence_girls);
    /// plot.set_bin_markers(&weekdays);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_markers(&mut self, markers: &'a [&'a str]) {
        self.markers = Some(markers);
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
    /// let markers = markers.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    /// plot.set_bin_markers(&markers);
    ///
    /// plot.set_bin_markers_middle();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_markers_middle(&mut self) {
        self.layout.bin_marker_position = BinMarkerPosition::Middle;
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
    /// let markers = markers.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
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
        self.layout.bin_marker_position = BinMarkerPosition::Left;
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
    /// let markers = markers.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    /// plot.set_bin_markers(&markers);
    ///
    /// plot.set_bin_markers_right();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_markers_right(&mut self) {
        self.layout.bin_marker_position = BinMarkerPosition::Right;
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
    /// let gap = 30.0; // The gap will take up 30% of the space, leaving 70% for bar.
    /// plot.set_bar_gap(gap);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bar_gap(&mut self, gap: Percentage) {
        self.layout.bar_gap = gap;
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
    /// let gap = 30.0; // The gap will take up 30% of the space, leaving 70% for bin.
    /// plot.set_bin_gap(gap);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_bin_gap(&mut self, gap: Percentage) {
        self.layout.bin_gap = gap;
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
    /// let len = 20.0; // The tick length will be of considerate length.
    /// plot.set_y_axis_tick_length(len);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_y_axis_tick_length(&mut self, p: Percentage) {
        self.layout.y_axis_tick_length = p;
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
    /// let len = 20.0; // The tick length will be of considerate length.
    /// plot.set_x_axis_tick_length(len);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_x_axis_tick_length(&mut self, p: Percentage) {
        self.layout.x_axis_tick_length = p;
    }

    /// Anchor bars at zero instead of the floor.
    /// This will make negative bars grow downwards.
    /// If your dataset contains only negative values, then this might not make sense to use.
    /// Rather, `use this when your dataset is likely to contain both positive and negative values`.
    ///
    /// An area where it makes sens is when visualizing temperature differences. :)
    ///
    /// By default, bars are anchored at the floor of the barchart.
    /// However, you might want negative values to stand out by having them point downwards.
    /// This method will apply anchoring bars at the zero line instead of the floor.
    ///
    /// # Important
    /// Call [`BarPlot::set_scale_range`] first, and make sure to set `min < 0` and `max >= 0`.
    /// Otherwise, you ~might~ will get a barchart that looks goofy. Consider yourself warned.
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
    /// let min = -20;
    /// let max = 20;
    /// let step = 2;
    /// plot.set_scale_range(min, max, step);
    /// plot.set_plot_window_size(90.0, 80.0, 83.0, 50.0);
    ///
    /// // Negative bars will now grow downwards instead of upwards. :)
    /// plot.set_negative_bars_go_down();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_negative_bars_go_down(&mut self) {
        self.layout.negative_bars_go_down = true;
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
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
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
    /// Note: you need to explicitly apply text first with [`BarPlot::set_text_lef.
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
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
    ///
    /// plot.set_text_left("This is some text.");
    ///
    /// let percent = 30.0;
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
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
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
    /// Note: you need to explicitly apply text first with [`BarPlot::set_text_right`].
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
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
    ///
    /// plot.set_text_right("This is some text.");
    ///
    /// let percent = 30.0;
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
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
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
    /// Note: you need to explicitly apply text first with [`BarPlot::set_text_bottom`].
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
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
    ///
    /// plot.set_text_bottom("This is some text.");
    ///
    /// let percent = 30.0;
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
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
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
    /// Note: you need to explicitly apply text first with [`BarPlot::set_text_top`].
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
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
    ///
    /// plot.set_text_top("This is some text.");
    ///
    /// let percent = 30.0;
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
    ///
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
    /// Note: you might want to resize the plot window to accomodate for the legend if you want it
    /// to be drawn outside the plot, otherwise it will be drawn on top of the plot figure.
    /// Check out [`BarPlot::set_plot_window_size`] for that.
    ///
    /// # Example
    ///
    /// ```
    /// use eb_bars::BarPlot;
    ///
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
    /// plot.set_plot_window_size(80.0, 30.0, 85.0, 40.0);
    ///
    /// let offset_x = 90.0;
    /// let offset_y = 22.0;
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
    ///
    /// plot.add_values(&[1., 2., 3.,]);
    ///
    /// // Scale down the plot figure size so text can become be visible.
    /// plot.set_plot_window_size(85.0, 65.0, 80.0, 40.0);
    ///
    /// // Apply some text.
    /// plot.set_text_left("This is some text.");
    ///
    /// // Increase font-size using a percentage greater than 100%.
    /// let size = 130.0;
    /// plot.set_font_size(size);
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_font_size(&mut self, p: Percentage) {
        self.layout.font_size = p;
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
    /// plot.set_plot_window_size(80.0, 30.0, 85.0, 40.0);
    ///
    /// plot.set_show_window_border();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_show_window_border(&mut self) {
        self.show.window_border = true;
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
    /// plot.set_plot_window_size(80.0, 30.0, 85.0, 40.0);
    ///
    /// plot.set_show_plot_border();
    ///
    /// let svg: String = plot.to_svg(1600, 1000);
    /// ```
    pub fn set_show_plot_border(&mut self) {
        self.show.plot_border = true;
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
    /// plot.set_plot_window_size(80.0, 30.0, 85.0, 40.0);
    ///
    /// // Lets overwrite the old plot with this new configuration.
    /// svg = plot.to_svg(1600, 1000);
    /// ```
    pub fn to_svg(&mut self, width: u32, height: u32) -> String {
        assert!(!self.values.is_empty(), "Can not generate plot without any values..");

        self.size = (width, height);

        let n_categories = self.values.len();
        match &mut self.colors.bars.layout {
            BarColorLayout::Category(colors) => {
                let n_colors = colors.len();
                assert_eq!(
                    n_categories,
                    n_colors,
                    "Got {n_categories} categories and {n_colors} colors.",
                );
            }
            BarColorLayout::Indexed(matrix) => {
                let n_color_vectors = matrix.len();
                assert_eq!(
                    n_categories,
                    n_color_vectors,
                    "Got {n_categories} categories and {n_color_vectors} color vectors.",
                );

                for (i, colors) in matrix.iter().enumerate() {
                    let values = self.values[i].len();
                    let n_colors = colors.len();
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

        svg::render(self)
    }
}

#[cfg(test)]
mod tests {
    // All other tests are in directory ./tests

    use std::fs;

    use toml;

    use super::{VERSION, REPOSITORY};

    #[test]
    fn version_and_repo() {
        // When updating Cargo.toml, make sure to update corresponding values in src files as well.
        let contents = fs::read_to_string("Cargo.toml").unwrap();
        let value = contents.parse::<toml::Table>().unwrap();

        let version = value["package"]["version"].as_str().unwrap();
        assert_eq!(version, VERSION);

        let repository = value["package"]["repository"].as_str().unwrap();
        assert_eq!(repository, REPOSITORY);
    }
}
