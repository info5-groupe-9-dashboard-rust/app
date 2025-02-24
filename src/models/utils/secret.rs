pub struct Secret {
    konami_sequence: Vec<egui::Key>,
    is_konami_active: bool,
    konami_start_time: Option<std::time::Instant>,
}

impl Default for Secret {
    fn default() -> Self {
        Secret {
            konami_sequence: Vec::new(),
            is_konami_active: false,
            konami_start_time: None,
        }
    }
}

const KONAMI_CODE: [egui::Key; 10] = [
    egui::Key::ArrowUp,
    egui::Key::ArrowUp,
    egui::Key::ArrowDown,
    egui::Key::ArrowDown,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
    egui::Key::A,
    egui::Key::B,
];

impl Secret {
    pub fn random_secret(&mut self, ctx: &egui::Context) -> () {
        let input_state = ctx.input( | i | i.raw.clone() );
    
            // Process key events.
            for event in &input_state.events {
                if let egui::Event::Key { key, pressed, .. } = event {
                    print!("Key: {:?}, pressed: {:?} \n", key, pressed);
                    if *pressed {
                        self.konami_sequence.push(*key);
    
                        // Keep the most recent keys (same length as the Konami code).
                        if self.konami_sequence.len() > KONAMI_CODE.len() {
                            self.konami_sequence.remove(0);
                        }
    
                        // Check if the sequence matches.
                        if self.konami_sequence.len() == KONAMI_CODE.len()
                            && self.konami_sequence == KONAMI_CODE
                        {
                            self.is_konami_active = true;
                            print!("Konami code activated!");
                            self.konami_start_time = Some(std::time::Instant::now());
                        }
                    }
                }
            }
    }
}

