use std::path::Path;
use std::fs;

use chrono::{DateTime, Duration, Local, Timelike};
use rand::Rng;

use eb_bars::BarPlot;

#[test]
fn bar_colors() {
    let output = Path::new("bar_colors.test.svg");

    let values_a = [3.6, 6.9, 6.5, 3.7];
    // Different ways to express colors :)
    let red = "Red"; // color-name
    let yellow = "rgb(244, 244, 32)"; // rgb value
    let blue = r"#1111FA"; // hex-value
    let green = "hsl(115, 90.50%, 50.30%)"; // hsl value
    // Putting them in an array with same length as our values.
    let colors_a = [red, yellow, blue , green];

    let values_b = [5.7, 3.9, 8.2, 7.0];
    let colors_b = ["Pink", "Orange", "Purple" , "Cyan"];

    let labels = ["Red and Pink", "Yellow and Orange", "Blue and Purple", "Green and Cyan"];

    let mut plot = BarPlot::new();
    plot.add_values(&values_a);
    plot.add_values(&values_b);
    plot.set_bin_markers(&labels);
    plot.add_bar_colors_from_vec(colors_a.to_vec());
    plot.add_bar_colors_from_vec(colors_b.to_vec());
    plot.set_background_color("Black");
    plot.set_bar_gap(5.0);
    plot.set_plot_window_size(90.0, 50.0, 90.0, 35.0);
    plot.set_scale_range(0, 10, 1);
    plot.set_x_axis_tick_length(10.0);
    plot.set_y_axis_tick_length(10.0);
    plot.set_show_horizontal_lines();
    plot.set_show_plot_border();
    plot.set_text_color("LightGoldenRodYellow");
    plot.set_text_right("Look at these beautiful colored bars.");
    plot.set_font_size(150.0); // Increase font size by 150%.
    plot.set_text_right_offset(50.0); // Set offset from side to plot border to 50%

    let contents = plot.to_svg(1600, 1000);
    if let Err(e) = fs::write(&output, contents) {
        eprintln!("Error saving '{}', {}", output.display(), e);
    }
}

#[test]
fn temperature_year() {
    let output = Path::new("temperature_year.test.svg");
    let months = [
        "Jan", "Feb", "Mar", "Apr",
        "May", "Jun", "Jul", "Aug",
        "Sep", "Oct", "Nov", "Dec"
    ];
    let temperature = [
        -11.5, -3.5, 1.3, 5.6,
        12.3, 21.0, 23.7, 22.5,
        12.5, 9.3, 5.6, -2.3
    ];

    let mut plot = BarPlot::new();
    plot.set_negative_bars_go_down();
    plot.set_bin_markers_middle();
    plot.set_background_color("Black");
    plot.set_show_horizontal_lines();
    plot.add_values(&temperature);
    plot.set_bin_gap(0.0);

    let min_color = "rgb(107, 235, 255)";
    let low_color = "rgb(126, 255, 165)";
    let high_color = "rgb(255, 233, 133)";
    let max_color = "rgb(250, 107, 91)";
    plot.set_bar_colors_by_threshold(min_color, low_color, high_color, max_color);

    plot.set_bin_markers(&months);

    plot.set_text_top("Monthly mean temperature °C some particular place for some particular year :)");
    plot.set_text_top_offset(40.0);

    plot.set_plot_window_size(95.0, 80.0, 87.0, 55.0);
    plot.set_scale_range(-20, 30, 10);
    plot.set_y_axis_tick_length(0.0);

    let contents = plot.to_svg(1600, 1000);
    if let Err(e) = fs::write(output, contents) {
        eprintln!("Error saving '{}', {}", output.display(), e);
    }
}

#[test]
fn wind_forecast() {
    let output = Path::new("wind_forecast.test.svg");

    fn __prepare_data(dt_current: DateTime<Local>) -> (Vec<String>, Vec<f64>, f64) {
        let rand_max = 18.0;
        let rand_min = 2.0;

        // Due to sometimes having more or less than 24 hours in a day
        // ..we need this convoluted code.
        let dt_start = dt_current
            .with_hour(0).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap();

        let seconds_passed = (dt_current - dt_start).num_seconds();

        let mut rng = rand::rng();
        let mut dt_end = dt_start.clone();
        let mut hour_marks: Vec<String> = Vec::with_capacity(25);
        let mut wind_values: Vec<f64> = Vec::with_capacity(25);

        while dt_end.date_naive() == dt_start.date_naive() {
            let max = rng.random_range(12.0..=rand_max);
            let min = rng.random_range(rand_min..=8.0);
            wind_values.push(rng.random_range(min..=max));

            hour_marks.push(dt_end.hour().to_string());

            dt_end += Duration::hours(1);
        }

        // Adds an extra marker at the right end of the chart (midnight time).
        hour_marks.push(dt_end.hour().to_string());

        let total_seconds_in_day = (dt_end - dt_start).num_seconds();
        let percent_of_day = (seconds_passed as f64 / total_seconds_in_day as f64) * 100.0;

        (hour_marks, wind_values, percent_of_day)
    }

    // Pretend this is the current time of day.
    // let dt = chrono_tz::Europe::Oslo.with_ymd_and_hms(2025, 3, 30, 15, 32, 17).unwrap();
    let dt = Local::now();
    let (hour_marks, wind_values, percent_of_day) = __prepare_data(dt);

    let mut plot = BarPlot::new();

    plot.set_background_color("Black");
    plot.set_show_horizontal_lines();
    plot.add_values(&wind_values);
    plot.set_scale_range(0, 30, 2);
    plot.set_bin_gap(8.0);
    plot.set_bar_colors_by_uniform("rgb(137, 174, 255)");

    plot.add_vertical_line_at(percent_of_day, "White");
    // NOTE: A value of 50.0 means a vertical line is placed right in the middle.
    // ..increasing or decreasing the value moves it right or left respectively.

    let hour_marks = hour_marks.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    plot.set_bin_markers(&hour_marks);
    plot.set_bin_markers_left();

    let text_top = format!(
        "Wind forecast for {} with vertical line indicating value for current time @ {}",
        dt.format("%e %B %Y"),
        dt.format("%H:%M"),
    );
    plot.set_text_top(&text_top);
    plot.set_text_top_offset(40.0);
    plot.set_text_left("Wind speed (m/s)");
    plot.set_text_left_offset(20.0);
    plot.set_text_bottom("Time of day in hours");
    plot.set_text_bottom_offset(25.0);

    plot.set_plot_window_size(93.0, 70.0, 85.0, 40.0);
    plot.set_show_plot_border();

    let contents = plot.to_svg(1600, 1000);
    if let Err(e) = fs::write(output, contents) {
        eprintln!("Error saving '{}', {}", output.display(), e);
    }
}

#[test]
fn random_values() {
    let output = Path::new("random_values.test.svg");

    let mut rng = rand::rng();
    let mut plot = BarPlot::new();

    let values: [f64; 40] = core::array::from_fn(|_| rng.random_range(-45_f64..45_f64));
    plot.add_values(&values);

    let markers: Vec<usize> = (0..=values.len()).collect();
    let markers: Vec<String> = markers.iter().map(|i| {
        if i % 2 == 0 {
            // Only show every other marker number.
            i.to_string()
        } else {
            "".to_string()
        }
    }).collect();
    let markers = markers.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    plot.set_bin_markers(&markers);
    plot.set_bin_markers_left();

    plot.set_show_vertical_lines();

    plot.set_font_size(130.0);
    plot.set_background_color("Black");
    plot.set_plot_window_size(90.0, 75.0, 83.0, 50.0);
    plot.set_scale_range(-50, 50, 10);
    plot.set_x_axis_tick_length(30.0);
    plot.set_y_axis_tick_length(30.0);
    plot.set_show_plot_border();
    plot.set_show_horizontal_lines();
    plot.set_text_top("A set of random values where greatest is light blue and lowest is red.");

    let max_color = "rgb(107, 235, 255)";
    let high_color = "rgb(126, 255, 165)";
    let low_color = "rgb(255, 233, 133)";
    let min_color = "rgb(250, 107, 91)";
    plot.set_bar_colors_by_threshold(min_color, low_color, high_color, max_color);

    let contents = plot.to_svg(1600, 1000);
    if let Err(e) = std::fs::write(&output, contents) {
        eprintln!("Error saving '{}', {}", output.display(), e);
    }
}

#[test]
fn fruit_picking() {
    let output = Path::new("fruit_picking.test.svg");

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

    plot.set_bin_markers(&weekdays);

    let categories = ["Tomatoes", "Apples", "Eggplants"];
    plot.set_legend(&categories);
    plot.set_legend_position(91.2, 22.2);

    plot.set_text_top("The highest value from each category have its color 'overriden' with a brighter color");
    plot.set_text_top_offset(40.0);
    plot.set_text_left("Total harvested.");
    plot.set_text_left_offset(25.0);

    plot.set_background_color("Black");
    plot.set_plot_window_size(80.0, 35.0, 85.0, 50.0);
    plot.set_scale_range(0, 100, 10);
    plot.set_line_color("White");
    plot.set_text_color("LightGoldenRodYellow");
    plot.set_tick_color("LightGoldenRodYellow");
    plot.set_x_axis_tick_length(12.0);
    plot.set_y_axis_tick_length(12.0);
    plot.set_bin_markers_middle();
    plot.set_show_plot_border();
    plot.set_show_horizontal_lines();
    plot.set_bin_gap(15.0);

    let contents = plot.to_svg(1600, 1000);
    if let Err(e) = fs::write(&output, contents) {
        eprintln!("Error saving '{}', {}", output.display(), e);
    }
}
