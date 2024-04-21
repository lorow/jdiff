// so how I want this view to work is simple. This view will hold a list of editors, and a
// reference to a list of stores for each editor. I also want to be able to add new editors
// and delete existing editors. I also want to be able to switch between editors. I also want
// to be able to save the current editor to a file as a workspace.
//
// now, when someone pastes and there's more than one editor, I want to show a popup and let them
// select to which editor to write to
//
// now, when someone pastes and there's only one editor, I want to just write to that editor
//
// by default, there's only one editor.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style, Styled}, text::{Line, Text}, widgets::Paragraph, Frame
};

use crate::models::{
    app_model::AppModelActions,
    app_state::{AppState, AppStateActions},
    editor_model::EditorModel,
};

use super::view::View;

#[derive(Default)]
pub struct EditorView {}

impl EditorView {
    pub fn new() -> Self {
        EditorView {}
    }
}

impl View for EditorView {
    fn render(
        &self,
        frame: &mut ratatui::Frame,
        layout: ratatui::prelude::Rect,
        app_state: &AppState,
    ) {
      
        let editors = app_state.editor_store.get_editors();
        let current_percentage = if editors.len() == 2 { 50 } else { 100 };

        let editors_container_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(current_percentage),
                Constraint::Percentage(100 - current_percentage),
            ])
            .split(layout);
        
        for (index, editor) in editors.iter().enumerate() {
            self.render_editor(frame, editors_container_layout[index], app_state, editor);
        }
        
    }

    fn handle_event(
        &mut self,
        key_event: &crossterm::event::KeyEvent,
        is_ctrl_pressed: bool,
        is_shift_pressed: bool,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        Some(AppStateActions::AppModelActions(AppModelActions::Exit))
    }
}

impl EditorView {
    fn render_editor(
        &self,
        frame: &mut Frame,
        layout: Rect,
        app_state: &AppState,
        editor: &EditorModel,
    ) {
        let editor_visible_lines = editor.get_visible_lines();
        let mut constaints = vec![Constraint::Max(1); editor.get_visible_lines().len() + 1];
        constaints.push(Constraint::Max(1));
        
        let editor_lines_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constaints)
            .split(layout);

        
        for (index, line_data) in editor_visible_lines.iter().enumerate() { 
            let line_layout = Layout::default()
                .direction(Direction::Horizontal)
                // [1]gap[text]
                .constraints([Constraint::Min(3), Constraint::Min(1),Constraint::Percentage(100)])
                .split(editor_lines_layout[index]);
            let number_widget = Text::from(Line::from(line_data.0.to_string())); 
            let number_paragraph = Paragraph::new(number_widget)
                .alignment(Alignment::Center)
                .style(Style::default().bg(Color::DarkGray));
            
            let text_widget = Text::from(Line::from(line_data.1.to_string()));
            let text_paragraph = Paragraph::new(text_widget).alignment(Alignment::Left);
            
            frame.render_widget(number_paragraph,line_layout[0]);
            frame.render_widget(text_paragraph, line_layout[2]);
        }
   }
}
