use std::collections::BTreeMap;

use chrono::DateTime;
use eframe::egui;
use egui::{lerp, pos2, remap_clamp, Align2, Color32, DragValue, FontId, Frame, LayerId, PointerButton, Pos2, Rect, Response, Rgba, ScrollArea, Sense, Shape, Stroke, TextStyle, Widget};
use crate::models::{application_context::ApplicationContext, job::Job};

use super::view::View;

pub struct GanttChart {
    options: Options,
}

impl Default for GanttChart {
    fn default() -> Self {
        GanttChart {
            options: Default::default(),
        }
    }
}

// Implement the View trait for GanttChart
impl View for GanttChart {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.heading(t!("app.gantt.title"));

        let min_start = app.all_jobs.iter().map(|job| job.scheduled_start).min().unwrap_or(0);
        let max_end = app.all_jobs.iter().map(|job| job.scheduled_start + job.walltime).max().unwrap_or(0);
        let reset_view = false;

        ui.horizontal(|ui| {    
            ui.menu_button("🔧 Settings", |ui| {
                ui.set_max_height(500.0);
    
                {
                    let changed = ui
                        .checkbox(&mut self.options.merge_scopes, "Merge children with same ID")
                        .changed();
                    // If we have multiple frames selected this will toggle
                    // if we view all the frames, or an average of them,
                    // and that difference is pretty massive, so help the user:
                    // if changed && num_frames > 1 {
                    //     reset_view = true;
                    // }
                }
    
                ui.horizontal(|ui| {
                    ui.label("Grid spacing:");
                    let grid_spacing_drag = DragValue::new(&mut self.options.grid_spacing_micros)
                        .speed(0.1)
                        .range(1.0..=100.0)
                        .suffix(" µs");
                    grid_spacing_drag.ui(ui);
                });        
    
                // The number of jobs can change between frames, so always show this even if there currently is only one job:
                self.options.sorting.ui(ui);
            });
    
            ui.menu_button("❓", |ui| {
                ui.label(
                    "Drag to pan.\n\
                            Zoom: Ctrl/cmd + scroll, or drag with secondary mouse button.\n\
                            Click on a scope to zoom to it.\n\
                            Double-click to reset view.\n\
                            Press spacebar to pause/resume.",
                );
            });
        });

        Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui.visuals_mut().clip_rect_margin = 0.0;
    
            let available_height = ui.max_rect().bottom() - ui.min_rect().bottom();
            ScrollArea::vertical().show(ui, |ui| {
                let mut canvas = ui.available_rect_before_wrap();
                canvas.max.y = f32::INFINITY;
                let response = ui.interact(canvas, ui.id().with("canvas"), Sense::click_and_drag());
    
                let min_s = min_start;
                let max_s = max_end;
    
                let info = Info {
                    ctx: ui.ctx().clone(),
                    canvas,
                    response,
                    painter: ui.painter_at(canvas),
                    text_height: 15.0, // TODO
                    start_s: min_s,
                    stop_s: max_s,
                    layer_id: ui.layer_id(),
                    font_id: TextStyle::Body.resolve(ui.style()),
                };
    
                if reset_view {
                    self.options.zoom_to_relative_s_range = Some((
                        info.ctx.input(|i| i.time),
                        (0., (info.stop_s - info.start_s) as f64),
                    ));
                }
    
                interact_with_canvas(&mut self.options, &info.response, &info);
    
                let where_to_put_timeline = info.painter.add(Shape::Noop);
    
                let max_y = ui_canvas(&mut self.options,app, &info, (min_s, max_s));
    
                let mut used_rect = canvas;
                used_rect.max.y = max_y;
    
                // // Fill out space that we don't use so that the `ScrollArea` doesn't collapse in height:
                used_rect.max.y = used_rect.max.y.max(used_rect.min.y + available_height);
    
                let timeline = paint_timeline(&info, used_rect, &self.options, min_s);
                info.painter
                    .set(where_to_put_timeline, Shape::Vec(timeline));
    
                ui.allocate_rect(used_rect, Sense::hover());
            });
        });
    
    }
}

fn interact_with_canvas(options: &mut Options, response: &Response, info: &Info) {
    if response.drag_delta().x != 0.0 {
        options.sideways_pan_in_points += response.drag_delta().x;
        options.zoom_to_relative_s_range = None;
    }

    if response.hovered() {
        // Sideways pan with e.g. a touch pad:
        if info.ctx.input(|i| i.smooth_scroll_delta.x != 0.0) {
            options.sideways_pan_in_points += info.ctx.input(|i| i.smooth_scroll_delta.x);
            options.zoom_to_relative_s_range = None;
        }

        let mut zoom_factor = info.ctx.input(|i| i.zoom_delta_2d().x);

        if response.dragged_by(PointerButton::Secondary) {
            zoom_factor *= (response.drag_delta().y * 0.01).exp();
        }

        if zoom_factor != 1.0 {
            options.canvas_width_s /= zoom_factor;

            if let Some(mouse_pos) = response.hover_pos() {
                let zoom_center = mouse_pos.x - info.canvas.min.x;
                options.sideways_pan_in_points =
                    (options.sideways_pan_in_points - zoom_center) * zoom_factor + zoom_center;
            }
            options.zoom_to_relative_s_range = None;
        }
    }

    if response.double_clicked() {
        // Reset view
        options.zoom_to_relative_s_range = Some((
            info.ctx.input(|i| i.time),
            (0., (info.stop_s - info.start_s) as f64),
        ));
    }

    if let Some((start_time, (start_s, end_s))) = options.zoom_to_relative_s_range {
        const ZOOM_DURATION: f32 = 0.75;
        let t = (info.ctx.input(|i| i.time - start_time) as f32 / ZOOM_DURATION).min(1.0);

        let canvas_width = response.rect.width();

        let target_canvas_width_s = (end_s - start_s) as f32;
        let target_pan_in_points = -canvas_width * start_s as f32 / target_canvas_width_s;

        options.canvas_width_s = lerp(
            options.canvas_width_s.recip()..=target_canvas_width_s.recip(),
            t,
        )
        .recip();
        options.sideways_pan_in_points =
            lerp(options.sideways_pan_in_points..=target_pan_in_points, t);

        if t >= 1.0 {
            options.zoom_to_relative_s_range = None;
        }

        info.ctx.request_repaint();
    }
}



/// Context for painting a frame.
struct Info {
    ctx: egui::Context,
    /// Bounding box of canvas in points:
    canvas: Rect,
    /// Interaction with the profiler canvas
    response: Response,
    painter: egui::Painter,
    text_height: f32,
    /// Time of first event
    start_s: i64,
    /// Time of last event
    stop_s: i64,
    /// LayerId to use as parent for tooltips
    layer_id: LayerId,

    font_id: FontId,
}

impl Info {
    fn point_from_s(&self, options: &Options, ns: i64) -> f32 {
        self.canvas.min.x
            + options.sideways_pan_in_points
            + self.canvas.width() * ((ns - self.start_s) as f32) / options.canvas_width_s
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SortBy {
    Time,
    Owner,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Sorting {
    pub sort_by: SortBy,
    pub reversed: bool,
}

impl Default for Sorting {
    fn default() -> Self {
        Self {
            sort_by: SortBy::Time,
            reversed: false,
        }
    }
}

impl Sorting {
    fn sort(self, mut jobs: Vec<Job>) -> Vec<Job> {
        match self.sort_by {
            SortBy::Time => {
                jobs.sort_by_key(|info| info.start_time);
            }
            SortBy::Owner => {
                jobs.sort_by(|a, b| a.owner.cmp(&b.owner));
            }
        }
        if self.reversed {
            jobs.reverse();
        }
        jobs
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Sort jobs by:");

            let dir = if self.reversed { '⬆' } else { '⬇' };

            for &sort_by in &[SortBy::Time, SortBy::Owner] {
                let selected = self.sort_by == sort_by;

                let label = if selected {
                    format!("{sort_by:?} {dir}")
                } else {
                    format!("{sort_by:?}")
                };

                if ui.add(egui::RadioButton::new(selected, label)).clicked() {
                    if selected {
                        self.reversed = !self.reversed;
                    } else {
                        self.sort_by = sort_by;
                        self.reversed = false;
                    }
                }
            }
        });
    }
}

pub struct Options {
    // --------------------
    // View:
    /// Controls zoom
    pub canvas_width_s: f32,

    /// How much we have panned sideways:
    pub sideways_pan_in_points: f32,

    // --------------------
    // Visuals:
    /// Events shorter than this many points aren't painted
    pub cull_width: f32,
    /// Draw each item with at least this width (only makes sense if [`Self::cull_width`] is 0)
    pub min_width: f32,

    pub rect_height: f32,
    pub spacing: f32,
    pub rounding: f32,

    /// Aggregate child scopes with the same id?
    pub merge_scopes: bool,

    pub sorting: Sorting,

    /// Interval of vertical timeline indicators.
    grid_spacing_micros: f64,

    /// Set when user clicks a scope.
    /// First part is `now()`, second is range.
    #[cfg_attr(feature = "serde", serde(skip))]
    zoom_to_relative_s_range: Option<(f64, (f64, f64))>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            canvas_width_s: 0.0,
            sideways_pan_in_points: 0.0,

            // cull_width: 0.5, // save some CPU?
            cull_width: 0.0, // no culling
            min_width: 1.0,

            rect_height: 16.0,
            spacing: 4.0,
            rounding: 4.0,

            merge_scopes: false, // off, because it really only works well for single-jobed profiling

            grid_spacing_micros: 1.,

            sorting: Default::default(),

            zoom_to_relative_s_range: None,
        }
    }
}



fn ui_canvas(
    options: &mut Options,
    app : &ApplicationContext,
    info: &Info,
    (min_s, max_s): (i64, i64),
) -> f32 {

    if options.canvas_width_s <= 0.0 {
        options.canvas_width_s = (max_s - min_s) as f32;
        options.zoom_to_relative_s_range = None;
    }

    // We paint the jobs top-down
    let mut cursor_y = info.canvas.top();
    cursor_y += info.text_height; // Leave room for time labels

    let jobs = options.sorting.sort(app.all_jobs.clone());

    for job_info in jobs {
        // TODO Check this part
        // let job_visualization = options
        //     .flamegraph_jobs
        //     .entry(job_info.name.clone())
        //     .or_default();

        // if !job_visualization.flamegraph_show {
        //     continue;
        // }

        // Visual separator between jobs:
        cursor_y += 2.0;
        let line_y = cursor_y;
        cursor_y += 2.0;

        let text_pos = pos2(info.canvas.min.x, cursor_y);

        paint_job_info(
            info,
            &job_info,
            text_pos,
            &mut false,
        );

        // draw on top of job info background:
        info.painter.line_segment(
            [
                pos2(info.canvas.min.x, line_y),
                pos2(info.canvas.max.x, line_y),
            ],
            Stroke::new(1.0, Rgba::from_white_alpha(0.5)),
        );

        cursor_y += info.text_height; // Extra spacing between jobs
    }

    cursor_y
}


fn paint_timeline(
    info: &Info,
    canvas: Rect,
    options: &Options,
    start_s: i64,
) -> Vec<egui::Shape> {
    let mut shapes = vec![];

    if options.canvas_width_s <= 0.0 {
        return shapes;
    }

    let alpha_multiplier = 0.3; // make it subtle

    // We show all measurements relative to start_s

    let max_lines = canvas.width() / 4.0;
    let mut grid_spacing_s = (options.grid_spacing_micros * 1_000.) as i64;
    while options.canvas_width_s / (grid_spacing_s as f32) > max_lines {
        grid_spacing_s *= 10;
    }

    // We fade in lines as we zoom in:
    let num_tiny_lines = options.canvas_width_s / (grid_spacing_s as f32);
    let zoom_factor = remap_clamp(num_tiny_lines, (0.1 * max_lines)..=max_lines, 1.0..=0.0);
    let zoom_factor = zoom_factor * zoom_factor;
    let big_alpha = remap_clamp(zoom_factor, 0.0..=1.0, 0.5..=1.0);
    let medium_alpha = remap_clamp(zoom_factor, 0.0..=1.0, 0.1..=0.5);
    let tiny_alpha = remap_clamp(zoom_factor, 0.0..=1.0, 0.0..=0.1);

    let mut grid_s = 0;

    loop {
        let line_x = info.point_from_s(options, start_s + grid_s);
        if line_x > canvas.max.x {
            break;
        }

        if canvas.min.x <= line_x {
            let big_line = grid_s % (grid_spacing_s * 100) == 0;
            let medium_line = grid_s % (grid_spacing_s * 10) == 0;

            let line_alpha = if big_line {
                big_alpha
            } else if medium_line {
                medium_alpha
            } else {
                tiny_alpha
            };

            shapes.push(egui::Shape::line_segment(
                [pos2(line_x, canvas.min.y), pos2(line_x, canvas.max.y)],
                Stroke::new(1.0, Rgba::from_white_alpha(line_alpha * alpha_multiplier)),
            ));

            let text_alpha = if big_line {
                medium_alpha
            } else if medium_line {
                tiny_alpha
            } else {
                0.0
            };

            if text_alpha > 0.0 {
                let text = grid_text(grid_s);
                let text_x = line_x + 4.0;
                let text_color = Rgba::from_white_alpha((text_alpha * 2.0).min(1.0)).into();

                info.painter.fonts(|f| {
                    // Text at top:
                    shapes.push(egui::Shape::text(
                        f,
                        pos2(text_x, canvas.min.y),
                        Align2::LEFT_TOP,
                        &text,
                        info.font_id.clone(),
                        text_color,
                    ));
                });

                info.painter.fonts(|f| {
                    // Text at bottom:
                    shapes.push(egui::Shape::text(
                        f,
                        pos2(text_x, canvas.max.y - info.text_height),
                        Align2::LEFT_TOP,
                        &text,
                        info.font_id.clone(),
                        text_color,
                    ));
                });
            }
        }

        grid_s += grid_spacing_s;
    }

    shapes
}

fn grid_text(ts: i64) -> String {
    if ts == 0 {
        "N/A".to_string()
    } else {
        if let Some(dt) = DateTime::from_timestamp(ts, 0) {
            dt.format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            "Invalid timestamp".to_string()
        }
    }
}

fn paint_job_info(info: &Info, job: &Job, pos: Pos2, collapsed: &mut bool) {
    let collapsed_symbol = if *collapsed { "⏵" } else { "⏷" };

    let galley = info.ctx.fonts(|f| {
        f.layout_no_wrap(
            format!("{} {}", collapsed_symbol, job.owner.clone()),
            info.font_id.clone(),
            egui::Color32::PLACEHOLDER,
        )
    });

    let rect = Rect::from_min_size(pos, galley.size());

    let is_hovered = if let Some(mouse_pos) = info.response.hover_pos() {
        rect.contains(mouse_pos)
    } else {
        false
    };

    let text_color = if is_hovered {
        Color32::WHITE
    } else {
        Color32::from_white_alpha(229)
    };
    let back_color = if is_hovered {
        Color32::from_black_alpha(100)
    } else {
        Color32::BLACK
    };

    info.painter.rect_filled(rect.expand(2.0), 0.0, back_color);
    info.painter.galley(rect.min, galley, text_color);

    if is_hovered && info.response.clicked() {
        *collapsed = !(*collapsed);
    }
}