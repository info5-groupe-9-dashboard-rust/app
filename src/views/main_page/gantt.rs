use crate::app;
use crate::models::data_structure::cluster::{self, Cluster};
use crate::models::data_structure::resource::ResourceState;
use crate::models::utils::date_converter::format_timestamp;
use crate::views::view::View;
use crate::{
    models::data_structure::{application_context::ApplicationContext, job::Job},
    views::components::{
        gantt_aggregate_by::{AggregateBy, AggregateByLevel1Enum, AggregateByLevel2Enum},
        gantt_grid_spacing::GridSpacing,
        gantt_job_color::JobColor,
        gantt_sorting::Sorting,
        job_details::JobDetailsWindow,
    },
};
use chrono::{DateTime, Local};
use eframe::egui;
use egui::{
    lerp, pos2, remap_clamp, Align2, Color32, FontId, Frame, PointerButton, Pos2, Rect, Response,
    Rgba, RichText, ScrollArea, Sense, Shape, Stroke, TextStyle,
};
use std::collections::BTreeMap;

/**
 * GanttChart struct
 */
pub struct GanttChart {
    options: Options,                           // options for the GanttChart
    job_details_windows: Vec<JobDetailsWindow>, // job details windows
    collapsed_jobs: BTreeMap<String, bool>,     // collapsed jobs
}

/**
 * Default implementation for the GanttChart
 */
impl Default for GanttChart {
    fn default() -> Self {
        GanttChart {
            options: Default::default(),
            job_details_windows: Vec::new(),
            collapsed_jobs: BTreeMap::new(),
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

                // Aggregate by component (levels)
                self.options.aggregate_by.ui(ui);
                ui.separator();

                // Grid spacing component (10 min, 30 min, 60 min)
                self.options.grid_spacing_minutes.ui(ui);
                ui.separator();

                // Job color component (random, state)
                self.options.job_color.ui(ui);
                ui.separator();

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
                    &mut self.collapsed_jobs,
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
    pub aggregate_by: AggregateBy,         // Aggregate by
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
            aggregate_by: Default::default(),         // aggregate by component
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
    collapsed_jobs: &mut BTreeMap<String, bool>,
) -> f32 {
    if options.canvas_width_s <= 0.0 {
        options.canvas_width_s = (max_ns - min_ns) as f32;
        options.zoom_to_relative_s_range = None;
    }

    let mut cursor_y = info.canvas.top();
    cursor_y += info.text_height;

    // Apply sorting
    let jobs = options.sorting.sort(app.filtered_jobs.clone());

    match options.aggregate_by.level_1 {
        // Aggregate by owner only
        AggregateByLevel1Enum::Owner => {
            let mut jobs_by_owner: BTreeMap<String, Vec<Job>> = BTreeMap::new();
            for job in jobs {
                jobs_by_owner
                    .entry(job.owner.clone())
                    .or_insert_with(Vec::new)
                    .push(job.clone());
            }

            cursor_y = paint_aggregated_jobs_level_1(
                info,
                options,
                jobs_by_owner,
                cursor_y,
                details_window,
                collapsed_jobs,
                &app.all_clusters,
            );
        }

        // Aggregate by host
        AggregateByLevel1Enum::Host => {
            match options.aggregate_by.level_2 {
                AggregateByLevel2Enum::Owner => {
                    let mut jobs_by_host_by_owner: BTreeMap<String, BTreeMap<String, Vec<Job>>> =
                        BTreeMap::new();
                    for job in jobs {
                        for host in job.hosts.iter() {
                            jobs_by_host_by_owner
                                .entry(host.clone())
                                .or_insert_with(BTreeMap::new)
                                .entry(job.owner.clone())
                                .or_insert_with(Vec::new)
                                .push(job.clone());
                        }
                    }

                    cursor_y = paint_aggregated_jobs_level_2(
                        info,
                        options,
                        jobs_by_host_by_owner,
                        cursor_y,
                        details_window,
                        collapsed_jobs,
                        &app.all_clusters,
                    );
                }
                AggregateByLevel2Enum::None => {
                    let mut jobs_by_host: BTreeMap<String, Vec<Job>> = BTreeMap::new();
                    for job in jobs {
                        for host in job.hosts.iter() {
                            jobs_by_host
                                .entry(host.clone())
                                .or_insert_with(Vec::new)
                                .push(job.clone());
                        }
                    }

                    cursor_y = paint_aggregated_jobs_level_1(
                        info,
                        options,
                        jobs_by_host,
                        cursor_y,
                        details_window,
                        collapsed_jobs,
                        &app.all_clusters,
                    );
                }
                AggregateByLevel2Enum::Host => {
                    // nothing to do here
                }
            }
        }

        // Aggregate by cluster
        AggregateByLevel1Enum::Cluster => match options.aggregate_by.level_2 {
            AggregateByLevel2Enum::Owner => {
                let mut jobs_by_cluster_by_owner: BTreeMap<String, BTreeMap<String, Vec<Job>>> =
                    BTreeMap::new();
                for job in jobs {
                    for cluster in job.clusters.iter() {
                        jobs_by_cluster_by_owner
                            .entry(cluster.clone())
                            .or_insert_with(BTreeMap::new)
                            .entry(job.owner.clone())
                            .or_insert_with(Vec::new)
                            .push(job.clone());
                    }
                }

                cursor_y = paint_aggregated_jobs_level_2(
                    info,
                    options,
                    jobs_by_cluster_by_owner,
                    cursor_y,
                    details_window,
                    collapsed_jobs,
                    &app.all_clusters,
                );
            }
            AggregateByLevel2Enum::None => {
                let mut jobs_by_cluster: BTreeMap<String, Vec<Job>> = BTreeMap::new();
                for job in jobs {
                    for cluster in job.clusters.iter() {
                        jobs_by_cluster
                            .entry(cluster.clone())
                            .or_insert_with(Vec::new)
                            .push(job.clone());
                    }
                }

                cursor_y = paint_aggregated_jobs_level_1(
                    info,
                    options,
                    jobs_by_cluster,
                    cursor_y,
                    details_window,
                    collapsed_jobs,
                    &app.all_clusters,
                );
            }
            AggregateByLevel2Enum::Host => {
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

                cursor_y = paint_aggregated_jobs_level_2(
                    info,
                    options,
                    jobs_by_cluster_by_host,
                    cursor_y,
                    details_window,
                    collapsed_jobs,
                    &app.all_clusters,
                );
            }
        },
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
                format_timestamp(job.scheduled_start),
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
 * Paints jobs with 1 level of aggregation
 */
fn paint_aggregated_jobs_level_1(
    info: &Info,
    options: &mut Options,
    jobs: BTreeMap<String, Vec<Job>>,
    mut cursor_y: f32,
    details_window: &mut Vec<JobDetailsWindow>,
    collapsed_jobs: &mut BTreeMap<String, bool>,
    clusters: &Vec<Cluster>,
) -> f32 {
    let spacing_between_level_1 = 20.0;
    let spacing_between_jobs = 5.0;
    let offset_level_1 = 10.0;

    cursor_y += spacing_between_level_1;

    for (level_1, job_list) in jobs {
        info.painter.line_segment(
            [
                pos2(info.canvas.min.x, cursor_y),
                pos2(info.canvas.max.x, cursor_y),
            ],
            Stroke::new(1.5, Color32::WHITE), // More marked line
        );

        cursor_y += offset_level_1;

        let text_pos = pos2(info.canvas.min.x, cursor_y);
        let is_collapsed = collapsed_jobs.entry(level_1.clone()).or_insert(false);
        paint_job_info(info, level_1, text_pos, is_collapsed, 1);

        cursor_y += spacing_between_level_1; // Spacing after the owner

        // Only show jobs if section is not collapsed
        if !*is_collapsed {
            for job in job_list {
                let job_start_y = cursor_y;
                paint_job(info, options, &job, job_start_y, details_window, clusters);
                cursor_y += info.text_height + spacing_between_jobs + options.spacing;
            }
            cursor_y += spacing_between_level_1;
        }

        cursor_y += spacing_between_level_1;
    }

    cursor_y
}

/**
 * Paints jobs with 2 levels of aggregation
 */
fn paint_aggregated_jobs_level_2(
    info: &Info,
    options: &mut Options,
    jobs: BTreeMap<String, BTreeMap<String, Vec<Job>>>,
    mut cursor_y: f32,
    details_window: &mut Vec<JobDetailsWindow>,
    collapsed_jobs: &mut BTreeMap<String, bool>,
    clusters: &Vec<Cluster>,
) -> f32 {
    let spacing_between_level_1 = 20.0;
    let spacing_between_level_2 = 15.0;
    let spacing_between_jobs = 5.0;
    let offset_level_1 = 10.0;

    cursor_y += spacing_between_level_1;

    for (level_1, level_2_map) in jobs {
        info.painter.line_segment(
            [
                pos2(info.canvas.min.x, cursor_y),
                pos2(info.canvas.max.x, cursor_y),
            ],
            Stroke::new(1.5, Color32::WHITE), // More marked line
        );

        cursor_y += offset_level_1;

        let text_pos = pos2(info.canvas.min.x, cursor_y);
        let is_collapsed = collapsed_jobs.entry(level_1.clone()).or_insert(false);
        paint_job_info(info, level_1, text_pos, is_collapsed, 1);

        cursor_y += spacing_between_level_1;

        if !*is_collapsed {
            let mut sorted_level_2: Vec<_> = level_2_map.keys().collect();
            sorted_level_2.sort();

            for level_2 in sorted_level_2 {
                if let Some(job_list) = level_2_map.get(level_2) {
                    // Draw a line to separate
                    info.painter.line_segment(
                        [
                            pos2(info.canvas.min.x, cursor_y),
                            pos2(info.canvas.max.x, cursor_y),
                        ],
                        Stroke::new(0.5, Rgba::from_white_alpha(0.5)), // Line more discreet
                    );

                    cursor_y += spacing_between_level_2;

                    let mut once = false;
                    // Display jobs
                    for job in job_list {
                        let job_start_y = cursor_y; // Ensure vertical alignment

                        // Draw the job (background)
                        paint_job(info, options, &job, job_start_y, details_window, clusters);

                        // Then, draw job_info (above)
                        if !once {
                            let job_text_pos = pos2(info.canvas.min.x, job_start_y);
                            paint_job_info(info, level_2.to_string(), job_text_pos, &mut false, 2);
                            once = true;
                        }

                        cursor_y += info.text_height + spacing_between_jobs + options.spacing;
                    }
                }
            }
        }
        cursor_y += spacing_between_level_1;
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
    clusters: &Vec<Cluster>,
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

    let majority_state = job.get_majority_resource_state(clusters);

    //even or odd just to test
    // let majority_state = if job.id % 2 == 0 {
    //     ResourceState::Dead
    // } else {
    //     ResourceState::Absent
    // };

    // paint hatch
    if majority_state == ResourceState::Dead || majority_state == ResourceState::Absent {
        let hachure_color = match majority_state {
            ResourceState::Dead => Color32::from_rgba_premultiplied(255, 0, 0, 100),
            ResourceState::Absent => Color32::from_rgba_premultiplied(255, 255, 0, 100),
            _ => Color32::TRANSPARENT,
        };

        let hachure_spacing = 10.0;
        let mut shapes = Vec::new();
        let mut x = info.canvas.min.x;
        let current_time_x = info.point_from_s(options, chrono::Utc::now().timestamp());

        while x < info.canvas.max.x {
            if majority_state == ResourceState::Absent && x >= current_time_x {
                break;
            }
            shapes.push(Shape::line_segment(
                [pos2(x, top_y), pos2(x + hachure_spacing, top_y + height)],
                Stroke::new(1.0, hachure_color),
            ));
            x += hachure_spacing;
        }
        info.painter.extend(shapes);
    }

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
fn paint_job_info(info: &Info, info_label: String, pos: Pos2, collapsed: &mut bool, level: u8) {
    let collapsed_symbol = if *collapsed { "⏵" } else { "⏷" };
    let label = if level == 1 {
        format!("{} {}", collapsed_symbol, info_label)
    } else {
        info_label
    };

    let galley = info
        .ctx
        .fonts(|f| f.layout_no_wrap(label, info.font_id.clone(), egui::Color32::PLACEHOLDER));

    let offset_x = if level == 1 { 0.0 } else { 50.0 };
    let rect = Rect::from_min_size(pos2(pos.x + offset_x, pos.y), galley.size());

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
    let back_color = if level == 2 {
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
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        } else {
            "Invalid timestamp".to_string()
        }
    }
}
