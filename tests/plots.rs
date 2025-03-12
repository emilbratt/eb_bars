
use std::path::Path;
use std::fs;

use rand::Rng;

use eb_bars::BarPlot;

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
    if let Err(e) = fs::write(&path, contents) {
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
    if let Err(e) = fs::write(path, contents) {
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
    if let Err(e) = fs::write(&path, contents) {
        eprintln!("Error saving plot '{}' {}", path.display(), e);
    }
}
