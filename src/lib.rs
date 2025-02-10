#![allow(unused)]

use std::path::Path;

mod svg;

type Percentage = usize;

#[derive(Debug)]
pub struct BarPlot<'a> {
    values: &'a [f64],
    bar_markers: Option<&'a [String]>,
    scale_range: Option<(isize, isize, usize)>,
    res: Option<(usize, usize)>,
    plot_window_scale: Option<(Percentage, Percentage, Percentage, Percentage)>,
    x_axis_tick_length: Option<Percentage>,
    x_marks_middle: bool,
    y_axis_tick_length: Option<Percentage>,
    negative_bars_go_down: bool,
    window_border: bool,
    plot_border: bool,
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
            x_marks_middle: false,
            y_axis_tick_length: None,
            negative_bars_go_down: false,
            window_border: false,
            plot_border: false,
        }
    }

    pub fn plot_window_scale(
        &mut self, x_length: Percentage, x_offset: Percentage, y_length: Percentage, y_offset: Percentage
    ) {
        assert!(x_length <= 100 && x_offset <= 100, "values cannot exceed 100%");
        assert!(y_length <= 100 && y_offset <= 100, "values cannot exceed 100%");
        self.plot_window_scale = Some((x_length, x_offset, y_length, y_offset));
    }

    pub fn scale_range(&mut self, min: isize, max: isize, step: usize) {
        // assert!(self.plot_window_scale.is_some(), "plot_window_scale must be set before scale_range");
        self.scale_range = Some((min, max, step));
    }

    pub fn set_bar_markers(&mut self, bar_markers: &'a [String]) {
        self.bar_markers = Some(bar_markers);
    }

    pub fn x_marks_middle(&mut self) {
        self.x_marks_middle = true;
    }

    pub fn y_axis_tick_length(&mut self, offset: Percentage) {
        self.y_axis_tick_length = Some(offset);
    }

    pub fn x_axis_tick_length(&mut self, offset: Percentage) {
        self.x_axis_tick_length = Some(offset);
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
    use super::*;

    #[test]
    fn positive_values() {
        let path = Path::new("positive_values.svg");
        let values: Vec<f64> = vec![
            29.67, 41.99, 64.25, 73.07, 59.71, 42.71, 65.15, 58.86,
            9.52, 91.53, 77.34, 55.66, 11.30, 91.90, 43.09, 65.47,
            66.84, 18.63, 13.42, 30.13, 1.94, 10.38, 58.25, 44.29,
        ];
        let bar_markers: Vec<String> = (0_u8..24).map(|i| i.to_string()).collect();

        let mut plot = BarPlot::new(&values);
        plot.plot_window_scale(95, 80, 90, 40);
        plot.scale_range(0, 100, 10);
        plot.x_axis_tick_length(10);
        plot.x_marks_middle();
        plot.y_axis_tick_length(10);
        plot.window_border();
        plot.plot_border();
        plot.set_bar_markers(&bar_markers);

        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }

    #[test]
    fn includes_negative_values() {
        let path = Path::new("includes_negative_values.svg");
        let values: Vec<f64> = vec![
            29.67, 41.99, 64.25, 73.07, -59.71, 42.71, 65.15, 58.86,
            9.52, 91.53, -77.34, 55.66, -11.30, 91.90, 43.09, 65.47,
            66.84, 18.63, 13.42, 30.13, 1.94, 10.38, -58.25, 44.29,
        ];

        let mut plot = BarPlot::new(&values);
        plot.plot_window_scale(90, 80, 85, 30);
        plot.scale_range(-80, 100, 10);
        plot.y_axis_tick_length(10);
        plot.negative_bars_go_down();
        plot.window_border();
        plot.plot_border();

        let contents = plot.to_svg(1600, 1000);
        if let Err(e) = std::fs::write(&path, contents) {
            eprintln!("Error saving plot '{}' {}", path.display(), e);
        }
    }
}
