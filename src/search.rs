use eframe::egui;

#[derive(Default)]
pub struct SearchState {
    pub show_bar: bool,
    pub query: String,
    pub results: Vec<usize>,
    pub current_match_index: Option<usize>,
}

impl SearchState {
    /// Update search results based on current query
    pub fn update_results(&mut self, text: &str) {
        self.results.clear();
        self.current_match_index = None;
        
        if self.query.is_empty() {
            return;
        }
        
        self.results = text.match_indices(&self.query).map(|(i, _)| i).collect();
        
        if !self.results.is_empty() {
            self.current_match_index = Some(0);
        }
    }
    
    /// Move to the next search result
    pub fn find_next(&mut self) {
        if self.results.is_empty() {
            return;
        }
        
        if let Some(current) = self.current_match_index {
            self.current_match_index = Some((current + 1) % self.results.len());
        } else {
            self.current_match_index = Some(0);
        }
    }
    
    /// Move to the previous search result
    pub fn find_previous(&mut self) {
        if self.results.is_empty() {
            return;
        }
        
        if let Some(current) = self.current_match_index {
            if current == 0 {
                self.current_match_index = Some(self.results.len() - 1);
            } else {
                self.current_match_index = Some(current - 1);
            }
        } else {
            self.current_match_index = Some(self.results.len() - 1);
        }
    }

    /// Render the Find bar UI
    pub fn render_bar(&mut self, ui: &mut egui::Ui, text: &str) {
        ui.horizontal(|ui| {
            ui.label("Find:");
            let response = ui.text_edit_singleline(&mut self.query);
            if response.changed() {
                self.update_results(text);
            }
            
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.find_next();
            }
            
            if ui.button("Next").clicked() {
                self.find_next();
            }
            
            if ui.button("Previous").clicked() {
                self.find_previous();
            }
            
            if let Some(index) = self.current_match_index {
                ui.label(format!("Match {} of {}", index + 1, self.results.len()));
                
                // Show context of current match
                if let Some(&match_pos) = self.results.get(index) {
                    let line_num = text[..match_pos].chars().filter(|&c| c == '\n').count() + 1;
                    
                    // Get the line containing the match
                    let line_start = text[..match_pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    let line_end = text[match_pos..].find('\n')
                        .map(|i| match_pos + i)
                        .unwrap_or(text.len());
                    let line_text = &text[line_start..line_end];
                    
                    // Show line number and preview
                    ui.label(format!("Line {}: {}", line_num, 
                        if line_text.len() > 50 {
                            format!("{}...", &line_text[..50])
                        } else {
                            line_text.to_string()
                        }
                    ));
                }
            } else if !self.query.is_empty() && self.results.is_empty() {
                ui.label("No matches found");
            }
            
            if ui.button("Close").clicked() {
                self.show_bar = false;
                self.query.clear();
                self.results.clear();
                self.current_match_index = None;
            }
        });
    }

    /// Get the layouter for highlighting
    pub fn get_layouter(&self) -> impl Fn(&egui::Ui, &dyn egui::TextBuffer, f32) -> std::sync::Arc<egui::Galley> + '_ {
        let find_results = self.results.clone();
        let match_len = self.query.len();
        let current_match_index = self.current_match_index;

        move |ui: &egui::Ui, string: &dyn egui::TextBuffer, wrap_width: f32| {
            let string = string.as_str();
            let mut layout_job = egui::text::LayoutJob::default();
            
            if find_results.is_empty() || match_len == 0 {
                layout_job.append(
                    string,
                    0.0,
                    egui::TextFormat {
                        font_id: egui::FontId::monospace(14.0),
                        ..Default::default()
                    },
                );
            } else {
                let mut last_index = 0;
                for (i, &index) in find_results.iter().enumerate() {
                    if index > last_index {
                        layout_job.append(
                            &string[last_index..index],
                            0.0,
                            egui::TextFormat {
                                font_id: egui::FontId::monospace(14.0),
                                ..Default::default()
                            },
                        );
                    }
                    
                    let bg_color = if Some(i) == current_match_index {
                        egui::Color32::from_rgb(255, 165, 0) // Orange for current
                    } else {
                        egui::Color32::YELLOW // Yellow for others
                    };
                    
                    layout_job.append(
                        &string[index..index + match_len],
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::monospace(14.0),
                            background: bg_color,
                            color: egui::Color32::BLACK,
                            ..Default::default()
                        },
                    );
                    
                    last_index = index + match_len;
                }
                
                if last_index < string.len() {
                    layout_job.append(
                        &string[last_index..],
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::monospace(14.0),
                            ..Default::default()
                        },
                    );
                }
            }
            
            layout_job.wrap.max_width = wrap_width;
            ui.painter().layout_job(layout_job)
        }
    }
}
