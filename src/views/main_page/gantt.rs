use std::collections::BTreeMap;

use crate::views::view::View;
use crate::{
    models::data_structure::{application_context::ApplicationContext, job::Job},
    views::components::{
        gantt_grid_spacing::GridSpacing,
        gantt_group_by::{GroupBy, GroupByEnum},
        gantt_job_color::JobColor,
        gantt_sorting::Sorting,
        job_details::JobDetailsWindow,
    },
};
use chrono::DateTime;
use eframe::egui;
use egui::{
    lerp, pos2, remap_clamp, Align2, Color32, FontId, Frame, PointerButton, Pos2, Rect, Response,
    Rgba, RichText, ScrollArea, Sense, Shape, Stroke, TextStyle,
};

/**
 * GanttChart struct
 */
pub struct GanttChart {
    options: Options,                           // options for the GanttChart
    job_details_windows: Vec<JobDetailsWindow>, // job details windows
}

/**
 * Default implementation for the GanttChart
 */
impl Default for GanttChart {
    fn default() -> Self {
        GanttChart {
            options: Default::default(),
            job_details_windows: Vec::new(),
        }
    }
}

/**
 * Implementation of the View trait for the GanttChart
 */
impl View for GanttChart {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.heading(RichText::new(t!("app.gantt.title")).strong());

        // Calculate the min start of the filtered jobs
        let min_start = app
            .filtered_jobs
            .iter()
            .map(|job| job.scheduled_start)
            .min()
            .unwrap_or(0);

        // Calculate the max end of the filtered jobs
        let max_end = app
            .filtered_jobs
            .iter()
            .map(|job| job.scheduled_start + job.walltime)
            .max()
            .unwrap_or(0);

        let reset_view = false;

        // Settings menu
        ui.horizontal(|ui| {
            ui.menu_button("🔧 Settings", |ui| {
                ui.set_max_height(500.0);

                // Group by component (owner, cluster, host)
                self.options.group_by.ui(ui);

                // Grid spacing component (10 min, 30 min, 60 min)
                self.options.grid_spacing_minutes.ui(ui);

                // Job color component (random, state)
                self.options.job_color.ui(ui);

                // Sorting options component (sort by, reversed)
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

        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.visuals_mut().clip_rect_margin = 0.0;

            let available_height = ui.max_rect().bottom() - ui.min_rect().bottom();
            ScrollArea::vertical().show(ui, |ui| {
                let mut canvas = ui.available_rect_before_wrap();
                canvas.max.y = f32::INFINITY;
                let response = ui.interact(canvas, ui.id().with("canvas"), Sense::click_and_drag());

                let min_s = min_start;
                let max_s = max_end;

                // Initialize canvas info
                let info = Info {
                    ctx: ui.ctx().clone(),
                    canvas,
                    response,
                    painter: ui.painter_at(canvas),
                    text_height: 15.0,
                    start_s: min_s,
                    stop_s: max_s,
                    font_id: TextStyle::Body.resolve(ui.style()),
                };

                // When we reset the view, we want to zoom to the full range
                if reset_view {
                    self.options.zoom_to_relative_s_range = Some((
                        info.ctx.input(|i| i.time),
                        (0., (info.stop_s - info.start_s) as f64),
                    ));
                }

                // Interact with the canvas
                interact_with_canvas(&mut self.options, &info.response, &info);

                // Put the timeline
                let where_to_put_timeline = info.painter.add(Shape::Noop);

                // Paint the canvas
                let max_y = ui_canvas(
                    &mut self.options,
                    app,
                    &info,
                    (min_s, max_s),
                    &mut self.job_details_windows,
                );

                let mut used_rect = canvas;
                used_rect.max.y = max_y;

                // Fill out space that we don't use so that the `ScrollArea` doesn't collapse in height:
                used_rect.max.y = used_rect.max.y.max(used_rect.min.y + available_height);

                let timeline = paint_timeline(&info, used_rect, &self.options, min_s);
                info.painter
                    .set(where_to_put_timeline, Shape::Vec(timeline));

                // Adding a line to show the current time AFTER all other elements
                let current_time_line = paint_current_time_line(&info, &self.options, used_rect);
                info.painter.add(current_time_line);

                ui.allocate_rect(used_rect, Sense::hover());
            });
        });

        // Part to display the details of a job when clicked
        self.job_details_windows.retain(|w| w.is_open());

        // Display job detail windows
        for window in self.job_details_windows.iter_mut() {
            window.ui(ui);
        }
    }
}

/****************************************************************************************************************************/
// CANVAS INFO
/****************************************************************************************************************************/

struct Info {
    ctx: egui::Context,     // context
    canvas: Rect,           // canvas to paint
    response: Response,     // response from the canvas
    painter: egui::Painter, // painter for the canvas
    text_height: f32,       // height of a line of text
    start_s: i64,           // start time in seconds
    stop_s: i64,            // stop time in seconds
    font_id: FontId,        // font id
}

impl Info {
    /**
     * Returns the x-coordinate (in points from the canvas) to the given timestamp
     */
    fn point_from_s(&self, options: &Options, ns: i64) -> f32 {
        self.canvas.min.x
            + options.sideways_pan_in_points
            + self.canvas.width() * ((ns - self.start_s) as f32) / options.canvas_width_s
    }
}

/****************************************************************************************************************************/
// OPTIONS
/****************************************************************************************************************************/

/**
 * Options struct
 */
pub struct Options {
    pub canvas_width_s: f32,               // Canvas width
    pub sideways_pan_in_points: f32,       // Sideways pan in points
    pub cull_width: f32,                   // Culling width
    pub min_width: f32,                    // Minimum width of a job
    pub rect_height: f32,                  // Height of a job
    pub spacing: f32,                      // Vertical spacing between jobs
    pub rounding: f32,                     // Rounded corners
    pub sorting: Sorting,                  // Sorting
    pub group_by: GroupBy,                 // Group by
    pub grid_spacing_minutes: GridSpacing, // Grid spacing in minutes
    pub job_color: JobColor,               // Job color
    current_hovered_job: Option<Job>,      // Current hovered job
    #[cfg_attr(feature = "serde", serde(skip))]
    zoom_to_relative_s_range: Option<(f64, (f64, f64))>, // Zoom to relative s range
}

/**
 * Default implementation for the Options struct
 */
impl Default for Options {
    fn default() -> Self {
        Self {
            canvas_width_s: 0.0,                      // no zoom
            sideways_pan_in_points: 0.0,              // no pan
            cull_width: 0.0,                          // no culling
            min_width: 1.0,                           // minimum width of a job
            rect_height: 16.0,                        // height of a job
            spacing: 5.0,                             // vertical spacing between jobs
            rounding: 4.0,                            // rounded corners
            group_by: Default::default(),             // group by component
            grid_spacing_minutes: Default::default(), // grid spacing component
            sorting: Default::default(),              // sorting component
            job_color: Default::default(),            // job color component
            zoom_to_relative_s_range: None,           // no zooming by default
            current_hovered_job: None,                // no hovered job by default
        }
    }
}

/****************************************************************************************************************************/
// CANVAS PAINTING
/****************************************************************************************************************************/

/**
 * Paints the UI canvas
 */
fn ui_canvas(
    options: &mut Options,
    app: &ApplicationContext,
    info: &Info,
    (min_ns, max_ns): (i64, i64),
    details_window: &mut Vec<JobDetailsWindow>,
) -> f32 {
    if options.canvas_width_s <= 0.0 {
        options.canvas_width_s = (max_ns - min_ns) as f32;
        options.zoom_to_relative_s_range = None;
    }

    let mut cursor_y = info.canvas.top();
    cursor_y += info.text_height;

    // Apply sorting
    let jobs = options.sorting.sort(app.filtered_jobs.clone());

    match options.group_by.value {
        // Group by owner
        GroupByEnum::Owner => {
            let mut jobs_by_id: BTreeMap<u32, Vec<Job>> = BTreeMap::new();
            for job in jobs {
                let id = job.id.clone();
                jobs_by_id.entry(id).or_insert_with(Vec::new).push(job);
            }

            let mut jobs_by_id_by_owner: BTreeMap<String, BTreeMap<u32, Vec<Job>>> =
                BTreeMap::new();
            for (id, job_list) in jobs_by_id {
                let owner = job_list[0].owner.clone();
                jobs_by_id_by_owner
                    .entry(owner)
                    .or_insert_with(BTreeMap::new)
                    .insert(id, job_list);
            }

            cursor_y = paint_aggregated_jobs_u32(
                info,
                options,
                jobs_by_id_by_owner,
                cursor_y,
                details_window,
            );
        }
        // Group by host
        GroupByEnum::Host => {}
        // Group by cluster
        GroupByEnum::Cluster => {
            // Group by cluster and then by host
            let mut jobs_by_cluster_by_host: BTreeMap<String, BTreeMap<String, Vec<Job>>> =
                BTreeMap::new();
            for job in jobs {
                for cluster in job.clusters.iter() {
                    for host in job.hosts.iter() {
                        jobs_by_cluster_by_host
                            .entry(cluster.clone())
                            .or_insert_with(BTreeMap::new)
                            .entry(host.clone())
                            .or_insert_with(Vec::new)
                            .push(job.clone());
                    }
                }
            }

            cursor_y = paint_aggregated_jobs_string(
                info,
                options,
                jobs_by_cluster_by_host,
                cursor_y,
                details_window,
            );
        }
    }

    // Paint tooltip for hovered job if there is one
    paint_job_tooltip(info, options);

    cursor_y
}

/**
 * Interacts with the canvas
 */
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

/****************************************************************************************************************************/
// JOB PAINTING
/****************************************************************************************************************************/

/**
 * Paints a tooltip for a job
 */
fn paint_job_tooltip(info: &Info, options: &mut Options) {
    if let Some(job) = &options.current_hovered_job {
        if let Some(_pointer_pos) = info.response.hover_pos() {
            let text = format!(
                "Job ID: {}\nOwner: {:?}\nState: {}\nStart: {}\nWalltime: {} seconds",
                job.id,
                job.owner,
                job.state.get_label(),
                grid_text(job.scheduled_start),
                job.walltime
            );

            egui::show_tooltip(
                &info.ctx,
                info.response.layer_id,
                egui::Id::new("job_tooltip"),
                |ui| {
                    ui.label(text);
                },
            );
        }
        options.current_hovered_job = None; // Reset for next frame
    }
}

/**
 * Paints aggregated jobs
 */
fn paint_aggregated_jobs_string(
    info: &Info,
    options: &mut Options,
    jobs: BTreeMap<String, BTreeMap<String, Vec<Job>>>,
    mut cursor_y: f32,
    details_window: &mut Vec<JobDetailsWindow>,
) -> f32 {
    let spacing_between_owners = 20.0;
    let spacing_between_ids = 15.0; // Space between each group of IDs
    let spacing_between_jobs = 5.0;
    let offset_owner = 10.0; // Offset before displaying the owner

    cursor_y += spacing_between_owners;

    for (owner, id_map) in jobs {
        // Draw a horizontal line to separate owners
        info.painter.line_segment(
            [
                pos2(info.canvas.min.x, cursor_y),
                pos2(info.canvas.max.x, cursor_y),
            ],
            Stroke::new(1.5, Color32::WHITE), // More marked line
        );

        cursor_y += offset_owner; // Offset before displaying the owner

        // Display the owner's name
        let text_pos = pos2(info.canvas.min.x, cursor_y);
        paint_job_info_owner(info, owner, text_pos, &mut false);

        cursor_y += spacing_between_owners; // Spacing after the owner

        // Sort the IDs to ensure an ordered display (optional)
        let mut sorted_ids: Vec<_> = id_map.keys().collect();
        sorted_ids.sort();

        for name in sorted_ids {
            if let Some(job_list) = id_map.get(name) {
                // Draw a line to separate each ID
                info.painter.line_segment(
                    [
                        pos2(info.canvas.min.x, cursor_y),
                        pos2(info.canvas.max.x, cursor_y),
                    ],
                    Stroke::new(1.0, Rgba::from_white_alpha(0.8)), // Line more discreet than owners
                );

                cursor_y += spacing_between_ids; // Spacing after the ID line

                let mut once = false;
                // Display jobs under the current ID
                for job in job_list {
                    let job_start_y = cursor_y; // Ensure vertical alignment

                    // Draw the job (background)
                    paint_job(info, options, &job, job_start_y, details_window);

                    // Then, draw job_info_id (above)
                    if !once {
                        let job_text_pos = pos2(info.canvas.min.x, job_start_y);
                        paint_job_info_host(info, name, job_text_pos, &mut false);
                        once = true;
                    }

                    cursor_y += info.text_height + spacing_between_jobs + options.spacing;
                }
            }
        }
        cursor_y += spacing_between_owners;
    }

    cursor_y
}

/**
 * Paints aggregated jobs
 */
fn paint_aggregated_jobs_u32(
    info: &Info,
    options: &mut Options,
    jobs: BTreeMap<String, BTreeMap<u32, Vec<Job>>>,
    mut cursor_y: f32,
    details_window: &mut Vec<JobDetailsWindow>,
) -> f32 {
    let spacing_between_owners = 20.0;
    let spacing_between_ids = 15.0; // Space between each group of IDs
    let spacing_between_jobs = 5.0;
    let offset_owner = 10.0; // Offset before displaying the owner

    cursor_y += spacing_between_owners;

    for (owner, id_map) in jobs {
        // Draw a horizontal line to separate owners
        info.painter.line_segment(
            [
                pos2(info.canvas.min.x, cursor_y),
                pos2(info.canvas.max.x, cursor_y),
            ],
            Stroke::new(1.5, Color32::WHITE), // More marked line
        );

        cursor_y += offset_owner; // Offset before displaying the owner

        // Display the owner's name
        let text_pos = pos2(info.canvas.min.x, cursor_y);
        paint_job_info_owner(info, owner, text_pos, &mut false);

        cursor_y += spacing_between_owners; // Spacing after the owner

        // Sort the IDs to ensure an ordered display (optional)
        let mut sorted_ids: Vec<_> = id_map.keys().copied().collect();
        sorted_ids.sort();

        for id in sorted_ids {
            if let Some(job_list) = id_map.get(&id) {
                // Draw a line to separate each ID
                info.painter.line_segment(
                    [
                        pos2(info.canvas.min.x, cursor_y),
                        pos2(info.canvas.max.x, cursor_y),
                    ],
                    Stroke::new(1.0, Rgba::from_white_alpha(0.8)), // Line more discreet than owners
                );

                cursor_y += spacing_between_ids; // Spacing after the ID line

                // Display jobs under the current ID
                for job in job_list {
                    let job_start_y = cursor_y; // Ensure vertical alignment

                    // Draw the job (background)
                    paint_job(info, options, &job, job_start_y, details_window);

                    // Then, draw job_info_id (above)
                    let job_text_pos = pos2(info.canvas.min.x, job_start_y);
                    paint_job_info_id(info, job.clone(), job_text_pos, &mut false);

                    cursor_y += info.text_height + spacing_between_jobs + options.spacing;
                }
            }
        }

        cursor_y += spacing_between_owners;
    }

    cursor_y
}

#[derive(PartialEq)]
enum PaintResult {
    Culled,
    Painted,
    Hovered,
}

/**
 * Paints a job
 */
fn paint_job(
    info: &Info,
    options: &mut Options,
    job: &Job,
    top_y: f32,
    details_window: &mut Vec<JobDetailsWindow>,
) -> PaintResult {
    let start_x = info.point_from_s(options, job.scheduled_start);
    let stop_time = if (job.scheduled_start + job.walltime) > info.stop_s {
        info.stop_s
    } else {
        job.scheduled_start + job.walltime
    };
    let end_x = info.point_from_s(options, stop_time);
    let width = end_x - start_x;

    if width < options.cull_width {
        return PaintResult::Culled;
    }

    let height = options.rect_height;
    let rect = Rect::from_min_size(
        pos2(start_x, top_y),
        egui::vec2(width.max(options.min_width), height),
    );

    let is_hovered = if let Some(mouse_pos) = info.response.hover_pos() {
        rect.contains(mouse_pos)
    } else {
        false
    };

    // Draw tooltip
    if is_hovered {
        options.current_hovered_job = Some(job.clone());
    }

    // Add click detection for the job
    if is_hovered && info.response.secondary_clicked() {
        let window = JobDetailsWindow::new(job.clone(), vec![]);
        details_window.push(window);
    }

    // Zoom to job if clicked
    if is_hovered && info.response.clicked() {
        // Zoom to job
        let job_duration_s = job.walltime as f64;
        let job_start_s = job.scheduled_start as f64;
        let job_end_s = job_start_s + job_duration_s;
        options.zoom_to_relative_s_range = Some((
            info.ctx.input(|i| i.time),
            (
                job_start_s - info.start_s as f64,
                job_end_s - info.start_s as f64,
            ),
        ));
    }

    let (hovered_color, normal_color) = if options.job_color.is_random() {
        job.get_gantt_color()
    } else {
        job.state.get_color()
    };

    let fill_color = if is_hovered {
        hovered_color
    } else {
        normal_color
    };

    info.painter.rect_filled(rect, options.rounding, fill_color);

    if width > 20.0 {
        let text = format!("{} ({})", job.owner, job.id);
        info.painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            text,
            info.font_id.clone(),
            if is_hovered {
                Color32::WHITE
            } else {
                Color32::from_white_alpha(240)
            },
        );
    }

    if is_hovered {
        PaintResult::Hovered
    } else {
        PaintResult::Painted
    }
}

/**
 * Paints a job info appearing on the left side of the canvas
 */
#[allow(dead_code)]
fn paint_job_info(info: &Info, info_label: String, pos: Pos2, collapsed: &mut bool) {
    let collapsed_symbol = if *collapsed { "⏵" } else { "⏷" };

    let galley = info.ctx.fonts(|f| {
        f.layout_no_wrap(
            format!("{} {}", collapsed_symbol, info_label),
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

    // Text color
    let text_color = if is_hovered {
        Color32::WHITE
    } else {
        Color32::from_white_alpha(229)
    };

    // Background color
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

fn paint_job_info_owner(info: &Info, owner: String, pos: Pos2, collapsed: &mut bool) {
    let collapsed_symbol = if *collapsed { "⏵" } else { "⏷" };

    let galley = info.ctx.fonts(|f| {
        f.layout_no_wrap(
            format!("{} {}", collapsed_symbol, owner),
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

    // Text color
    let text_color = if is_hovered {
        Color32::WHITE
    } else {
        Color32::from_white_alpha(229)
    };

    // Background color
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

fn paint_job_info_host(info: &Info, host: &String, pos: Pos2, collapsed: &mut bool) {
    let galley = info.ctx.fonts(|f| {
        f.layout_no_wrap(
            format!("{}", host),
            info.font_id.clone(),
            egui::Color32::PLACEHOLDER,
        )
    });

    let rect = Rect::from_min_size(pos2(pos.x + 50.0, pos.y), galley.size());

    let is_hovered = if let Some(mouse_pos) = info.response.hover_pos() {
        rect.contains(mouse_pos)
    } else {
        false
    };

    // Text color
    let text_color = if is_hovered {
        Color32::WHITE
    } else {
        Color32::from_white_alpha(229)
    };

    // Background color
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

fn paint_job_info_id(info: &Info, job: Job, pos: Pos2, collapsed: &mut bool) {
    let galley = info.ctx.fonts(|f| {
        f.layout_no_wrap(
            format!("- {}", job.id),
            info.font_id.clone(),
            egui::Color32::PLACEHOLDER,
        )
    });

    let rect = Rect::from_min_size(pos2(pos.x + 50.0, pos.y), galley.size());

    let is_hovered = if let Some(mouse_pos) = info.response.hover_pos() {
        rect.contains(mouse_pos)
    } else {
        false
    };

    // Text color
    let text_color = if is_hovered {
        Color32::WHITE
    } else {
        Color32::from_white_alpha(229)
    };

    // Background color
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

/****************************************************************************************************************************/
// TIMELINE
/****************************************************************************************************************************/

/**
 * Paints the timeline
 */
fn paint_timeline(info: &Info, canvas: Rect, options: &Options, _start_s: i64) -> Vec<egui::Shape> {
    let mut shapes = vec![];

    if options.canvas_width_s <= 0.0 {
        return shapes;
    }

    let alpha_multiplier = 0.3; // make it subtle

    // We show all measurements relative to start_s

    let max_lines = canvas.width() / 4.0;
    let mut grid_spacing_minutes = (options.grid_spacing_minutes.value / 10) * 60; // convert grid spacing to seconds
    while options.canvas_width_s / (grid_spacing_minutes as f32) > max_lines {
        grid_spacing_minutes *= 10;
    }

    // We fade in lines as we zoom in:
    let num_tiny_lines = options.canvas_width_s / (grid_spacing_minutes as f32);
    let zoom_factor = remap_clamp(num_tiny_lines, (0.1 * max_lines)..=max_lines, 1.0..=0.0);
    let zoom_factor = zoom_factor * zoom_factor;
    let big_alpha = remap_clamp(zoom_factor, 0.0..=1.0, 0.5..=1.0);
    let medium_alpha = remap_clamp(zoom_factor, 0.0..=1.0, 0.1..=0.5);
    let tiny_alpha = remap_clamp(zoom_factor, 0.0..=1.0, 0.0..=0.1);

    let mut grid_s = 0;

    loop {
        let line_x = info.point_from_s(options, grid_s);
        if line_x > canvas.max.x {
            break;
        }

        if canvas.min.x <= line_x {
            let big_line = grid_s % (grid_spacing_minutes * 20) == 0; // big line every 20 grid_spacing_minutes
            let medium_line = grid_s % (grid_spacing_minutes * 10) == 0; // medium line every 10 grid_spacing_minutes

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

        grid_s += grid_spacing_minutes;
    }

    shapes
}

/**
 * Paints the current red time line on the canvas
 */
fn paint_current_time_line(info: &Info, options: &Options, canvas: Rect) -> egui::Shape {
    let current_time = chrono::Utc::now().timestamp();
    let line_x = info.point_from_s(options, current_time);
    egui::Shape::line_segment(
        [pos2(line_x, canvas.min.y), pos2(line_x, canvas.max.y)],
        Stroke::new(2.0, Color32::RED),
    )
}

/**
 * Converts a timestamp to a string
 */
fn grid_text(ts: i64) -> String {
    if ts == 0 {
        "N/A".to_string()
    } else {
        if let Some(dt) = DateTime::from_timestamp(ts, 0) {

            dt.with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            "Invalid timestamp".to_string()
        }
    }
}
