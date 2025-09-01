use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event as CEvent},
    style::Style,
    widgets::Paragraph,
    Terminal,
};

use crate::{
    color::ColorTheme,
    keys::{UserEvent, UserEventMapper},
    widget::{InputDialog, InputDialogState},
};

const PROFILE_EMPTY_ERR: &str = "Profile cannot be empty";
const DIALOG_MAX_WIDTH: u16 = 50;

/// Show a minimal input dialog and capture an AWS profile name.
///
/// This function owns a small draw + key loop: it renders an input dialog,
/// reads crossterm events synchronously, updates the input state, and
/// returns the input value when submitted.
///
/// Controls (honors configured keybindings):
/// - Submit: `UserEvent::InputDialogApply` (default: Enter)
/// - Cancel: `UserEvent::InputDialogClose` (default: Esc) or `UserEvent::Quit` (default: Ctrl-C)
pub fn get_profile(
    terminal: &mut Terminal<impl Backend>,
    mapper: &UserEventMapper,
    theme: &ColorTheme,
) -> anyhow::Result<Option<String>> {
    let mut state = InputDialogState::default();
    let mut error_msg: Option<String> = None;

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let dialog = InputDialog::default()
                .title("AWS Profile")
                .max_width(DIALOG_MAX_WIDTH)
                .theme(theme);

            // Render input dialog
            f.render_stateful_widget(dialog, area, &mut state);

            // Render validation error if any
            if let Some(msg) = &error_msg {
                // Compute same dialog area as InputDialog for consistent positioning
                let dialog_area = InputDialog::dialog_area_for(area, Some(DIALOG_MAX_WIDTH));

                // Prefer rendering one line below the dialog; otherwise place one line above
                let mut y = dialog_area
                    .y
                    .saturating_add(dialog_area.height)
                    .saturating_add(1);
                if y.saturating_add(1) > area.bottom() {
                    y = dialog_area.y.saturating_sub(2).max(area.top());
                }
                let msg_area = ratatui::layout::Rect::new(dialog_area.x, y, dialog_area.width, 1);
                let para =
                    Paragraph::new(msg.as_str()).style(Style::default().fg(theme.status_error));
                f.render_widget(para, msg_area);
            }

            let (x, y) = state.cursor();
            f.set_cursor_position((x, y));
        })?;

        match event::read()? {
            CEvent::Key(key) => {
                let user_events = mapper.find_events(key);

                let apply = user_events
                    .iter()
                    .any(|e| matches!(e, UserEvent::InputDialogApply));
                let cancel = user_events
                    .iter()
                    .any(|e| matches!(e, UserEvent::InputDialogClose | UserEvent::Quit));

                if cancel {
                    return Ok(None);
                }

                if apply {
                    let trimmed_input = state.input().trim();
                    if trimmed_input.is_empty() {
                        error_msg = Some(PROFILE_EMPTY_ERR.to_string());
                        continue;
                    }
                    return Ok(Some(trimmed_input.to_string()));
                }

                // Clear error on any other key and pass through to input widget
                if error_msg.is_some() {
                    error_msg = None;
                }
                state.handle_key_event(key);
            }
            CEvent::Resize(_, _) => {
                // trigger redraw on next loop iteration
            }
            _ => {}
        }
    }
}
