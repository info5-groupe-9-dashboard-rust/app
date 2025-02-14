use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;


// Convert a job ID to a color (using hash)
pub fn convert_id_to_color(id: u32) -> egui::Color32 {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let hash = hasher.finish();

    let r = ((hash >> 16) & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = (hash & 0xFF) as u8;

    egui::Color32::from_rgb(r, g, b)
}