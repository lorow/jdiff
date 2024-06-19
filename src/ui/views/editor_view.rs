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
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::Paragraph,
    Frame,
};

use crate::models::{
    app_model::{AppMode, AppModelActions},
    app_state::{AppState, AppStateActions},
    editor::{
        editor_container_models::{EditorContainerModelActions, EditorFocus},
        editor_model::{EditorCursorDirection, EditorModel},
    },
};

use super::view::{View, ViewContext};

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

        let side_rect_used =
            editors_container_layout[app_state.editor_store.get_active_editor_index()];
        let cursor_position = app_state.editor_store.get_active_cursor_position();

        frame.set_cursor(
            // 4 is the line length, move that to a const
            side_rect_used.x + 4 + cursor_position.0,
            side_rect_used.y + cursor_position.1,
        )
    }

    fn handle_event(
        &mut self,
        key_event: &crossterm::event::KeyEvent,
        context: ViewContext,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        let current_app_mode = app_state.app_state_store.get_app_mode();
        match (key_event.code, current_app_mode) {
            (crossterm::event::KeyCode::Char(c), _) => {
                self.handle_keyboard_input(&context, app_state, c)
            }
            (crossterm::event::KeyCode::Esc, _) => Some(AppStateActions::AppModelActions(
                AppModelActions::ChangeMode(AppMode::Normal),
            )),
            (crossterm::event::KeyCode::Backspace, AppMode::Editing) => Some(
                AppStateActions::EditorActions(EditorContainerModelActions::Backspace),
            ),
            (crossterm::event::KeyCode::Up, _) => Some(AppStateActions::EditorActions(
                EditorContainerModelActions::MoveCursor(EditorCursorDirection::Up),
            )),
            (crossterm::event::KeyCode::Down, _) => Some(AppStateActions::EditorActions(
                EditorContainerModelActions::MoveCursor(EditorCursorDirection::Down),
            )),
            (crossterm::event::KeyCode::Left, _) => Some(AppStateActions::EditorActions(
                EditorContainerModelActions::MoveCursor(EditorCursorDirection::Left),
            )),
            (crossterm::event::KeyCode::Right, _) => Some(AppStateActions::EditorActions(
                EditorContainerModelActions::MoveCursor(EditorCursorDirection::Right),
            )),
            (crossterm::event::KeyCode::Enter, _) => {
                if matches!(app_state.app_state_store.get_app_mode(), AppMode::Editing) {
                    return Some(AppStateActions::EditorActions(
                        EditorContainerModelActions::Enter,
                    ));
                }
                None
            }
            _ => None,
        }
    }

    fn get_has_been_initialized(&self, app_state: &AppState) -> bool {
        app_state.editor_store.get_is_initialized()
    }

    fn get_has_been_resized(&self, app_state: &AppState) -> bool {
        app_state.editor_store.get_is_resized_set()
    }

    fn init(
        &mut self,
        frame: &mut Frame,
        rect: Rect,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        Some(AppStateActions::EditorActions(
            EditorContainerModelActions::InitEditor(rect),
        ))
    }

    fn handle_resize(
        &mut self,
        frame: &mut Frame,
        rect: Rect,
        app_state: &AppState,
    ) -> Option<AppStateActions> {
        Some(AppStateActions::EditorActions(
            EditorContainerModelActions::ResizeEditor(rect),
        ))
    }
}

impl EditorView {
    fn render_editor(
        &self,
        frame: &mut Frame,
        layout: Rect,
        _app_state: &AppState,
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
                .constraints([
                    Constraint::Min(3),
                    Constraint::Min(1),
                    Constraint::Percentage(100),
                ])
                .split(editor_lines_layout[index]);
            let number_widget = Text::from(Line::from(line_data.0.to_string()));
            let number_paragraph = Paragraph::new(number_widget)
                .alignment(Alignment::Center)
                .style(Style::default().bg(Color::DarkGray));
            let text_widget = Text::from(Line::from(line_data.1.to_string()));
            let text_paragraph = Paragraph::new(text_widget).alignment(Alignment::Left);

            frame.render_widget(number_paragraph, line_layout[0]);
            frame.render_widget(text_paragraph, line_layout[2]);
        }
    }

    fn handle_keyboard_input(
        &mut self,
        context: &ViewContext,
        app_state: &AppState,
        c: char,
    ) -> Option<AppStateActions> {
        match (c, context.is_ctrl_pressed) {
            ('h', true) => {
                return Some(AppStateActions::EditorActions(
                    EditorContainerModelActions::ChangeFocus(EditorFocus::Prev),
                ))
            }
            ('l', true) => {
                return Some(AppStateActions::EditorActions(
                    EditorContainerModelActions::ChangeFocus(EditorFocus::Next),
                ))
            }
            (_, _) => {}
        }

        if c == 'i' && app_state.app_state_store.get_app_mode() == AppMode::Normal {}

        if c == 'u' && app_state.app_state_store.get_app_mode() == AppMode::Normal {}

        if app_state.app_state_store.get_app_mode() == AppMode::Normal {
            if c == 'i' {
                return Some(AppStateActions::AppModelActions(
                    AppModelActions::ChangeMode(AppMode::Editing),
                ));
            }

            if c == 'u' {
                if context.is_shift_pressed {
                    return Some(AppStateActions::EditorActions(
                        EditorContainerModelActions::Redo,
                    ));
                }
                return Some(AppStateActions::EditorActions(
                    EditorContainerModelActions::Undo,
                ));
            }
        }

        if matches!(app_state.app_state_store.get_app_mode(), AppMode::Editing) {
            return Some(AppStateActions::EditorActions(
                EditorContainerModelActions::Input(c),
            ));
        }
        None
    }
}
