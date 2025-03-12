# EB Bars - It's a Super Simple Barplot Library 🦀

Simple barchart library for Rust with a focus on customizing the chart to look nice.

### So Simple that
- you can _only_ create barcharts and or histograms.
- there are no external dependencies.
- the only supported output is svg.
- all bars are drawn with the _rect_ svg element instead of for example the _path_ element.
- even your mom can use the API.

### But despite being simple you can customize your barchart by
- adding text in any of the four sides if you want.
- setting font size for text.
- applying custom colors on bars, lines, ticks/markers and text.
- showing or hiding grid lines (both horizontal and vertical).
- having bars with negative values be drawn downwards.
- setting a custom resolution.
- resizing the plot/figure and move it in all directions for best fit.

### Showcase

Click on any of the images below to view the code.

<a href="https://github.com/emilbratt/eb_bars/blob/main/tests/plots.rs#L133">
    <img src="https://raw.githubusercontent.com/emilbratt/eb_bars/refs/heads/main/image/fruit_picking.svg" width=400px></img>
</a>

<a href="https://github.com/emilbratt/eb_bars/blob/main/tests/plots.rs#L14">
    <img src="https://raw.githubusercontent.com/emilbratt/eb_bars/refs/heads/main/image/bar_colors.svg" width=400px></img>
</a>

<a href="https://github.com/emilbratt/eb_bars/blob/main/tests/plots.rs#L58">
    <img src="https://raw.githubusercontent.com/emilbratt/eb_bars/refs/heads/main/image/temperature.svg" width=400px></img>
</a>

<a href="https://github.com/emilbratt/eb_bars/blob/main/tests/plots.rs#L94">
    <img src="https://raw.githubusercontent.com/emilbratt/eb_bars/refs/heads/main/image/random_values.svg" width=400px></img>
</a>

### Getting Started

For now, refer to [crate docs] for how to produce nice looking plots or simply take a look at all the [tests]. :relaxed:

[tests]: https://github.com/emilbratt/eb_bars/blob/main/tests/plots.rs

[crate docs]: https://docs.rs/eb_bars/latest/eb_bars/
