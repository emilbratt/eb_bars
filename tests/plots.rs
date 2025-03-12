
use std::path::Path;
use std::fs;

use rand::Rng;

use eb_bars::BarPlot;

fn _rand_range_f64(start: i32, end_incl: i32) -> f64 {
    rand::rng().random_range(start..=end_incl) as f64
}

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

    let labels = ["RED and Pink", "Yellow and Orange", "Blue and Purple", "Green and Cyan"];
    let markers = labels.into_iter().map(|s| s.to_owned()).collect::<Vec<String>>();

    let mut plot = BarPlot::new();
    plot.add_values(&values_a);
    plot.add_values(&values_b);
    plot.set_bin_markers(&markers);
    plot.add_bar_colors_from_vec(colors_a.to_vec());
    plot.add_bar_colors_from_vec(colors_b.to_vec());
    plot.set_background_color("Black");
    plot.set_bar_gap(5);
    plot.set_plot_window_size(90, 50, 90, 35);
    plot.set_scale_range(0, 10, 1);
    plot.set_x_axis_tick_length(10);
    plot.set_y_axis_tick_length(10);
    plot.set_show_horizontal_lines();
    plot.set_show_plot_border();
    plot.set_text_color("LightGoldenRodYellow");
    plot.set_text_right("Look at these beautiful colored bars.");
    plot.set_font_size(150); // Increase font size by 150%.
    plot.set_text_right_offset(50); // Set offset from side to plot border to 50%

    let contents = plot.to_svg(1600, 1000);
    if let Err(e) = fs::write(&output, contents) {
        eprintln!("Error saving plot '{}' {}", output.display(), e);
    }
}

#[test]
fn temperature() {
    let output = Path::new("temperature.test.svg");
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let recordings = [-11.5, -3.5, 1.3, 5.6, 12.3, 21.0, 23.7, 22.5, 12.5, 9.3, 5.6, -2.3];

    let mut plot = BarPlot::new();
    plot.set_negative_bars_go_down();
    plot.set_bin_markers_middle();
    plot.set_background_color("Black");
    plot.set_show_horizontal_lines();
    plot.add_values(&recordings);
    plot.set_bin_gap(0);

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

    let contents = plot.to_svg(1600, 1000);
    if let Err(e) = fs::write(output, contents) {
        eprintln!("Error saving plot '{}' {}", output.display(), e);
    }
}

#[test]
fn random_values() {
    let output = Path::new("random_values.test.svg");

    let mut rng = rand::rng();
    let mut plot = BarPlot::new();

    let values: [f64; 40] = core::array::from_fn(|_| rng.random_range(-45_f64..45_f64));
    plot.add_values(&values);

    // We provide half the markers, but they will align with each bin.
    let half = values.len()/2;
    let markers: Vec<String> = (0..half).map(|i| (i*2).to_string()).collect();
    plot.set_bin_markers(&markers);
    plot.set_bin_markers_left();
    plot.set_show_vertical_lines();

    plot.set_font_size(130);
    plot.set_background_color("Black");
    plot.set_plot_window_size(90, 80, 83, 50);
    plot.set_scale_range(-50, 50, 10);
    plot.set_x_axis_tick_length(30);
    plot.set_y_axis_tick_length(30);
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
        eprintln!("Error saving plot '{}' {}", output.display(), e);
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

    let bin_markers: Vec<String> = weekdays.iter().map(|s| s.to_string()).collect();
    plot.set_bin_markers(&bin_markers);

    let categories = ["Tomatoes", "Apples", "Eggplants"];
    plot.set_legend(&categories);
    plot.set_legend_position(91, 22);

    plot.set_text_top("The highest value from each category have its color 'overriden' with a brighter color");
    plot.set_text_top_offset(40);
    plot.set_text_left("Total harvested.");
    plot.set_text_left_offset(25);

    plot.set_background_color("Black");
    plot.set_plot_window_size(80, 35, 85, 50);
    plot.set_scale_range(0, 100, 10);
    plot.set_line_color("White");
    plot.set_text_color("LightGoldenRodYellow");
    plot.set_tick_color("LightGoldenRodYellow");
    plot.set_x_axis_tick_length(12);
    plot.set_y_axis_tick_length(12);
    plot.set_bin_markers_middle();
    plot.set_show_window_border();
    plot.set_show_plot_border();
    plot.set_show_horizontal_lines();
    plot.set_bin_gap(15);

    let contents = plot.to_svg(1600, 1000);
    if let Err(e) = fs::write(&output, contents) {
        eprintln!("Error saving plot '{}' {}", output.display(), e);
    }
}
