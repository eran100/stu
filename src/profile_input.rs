use anyhow::anyhow;
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
    widget::{calc_centered_dialog_rect, InputDialog, InputDialogState},
};

const PROFILE_EMPTY_ERR: &str = "Profile cannot be empty";

/// Show a minimal input dialog and capture an AWS profile name.
///
/// This function owns a small draw + key loop: it renders an input dialog,
/// reads crossterm events synchronously, updates the input state, and
/// returns the input value when submitted.
///
/// Controls (honors configured keybindings):
/// - Submit: `UserEvent::InputDialogApply` (default: Enter)
/// - Cancel: `UserEvent::InputDialogClose` (default: Esc) or `UserEvent::Quit` (default: Ctrl-C)
pub fn get_profile(terminal: &mut Terminal<impl Backend>) -> anyhow::Result<String> {
    let mapper = UserEventMapper::load()?;
    let theme = ColorTheme::default();

    let mut state = InputDialogState::default();
    let mut error_msg: Option<String> = None;

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let max_width = 50u16;
            let dialog = InputDialog::default()
                .title("AWS Profile")
                .max_width(max_width)
                .theme(&theme);

            // Render input dialog
            f.render_stateful_widget(dialog, area, &mut state);

            // Render validation error if any
            if let Some(msg) = &error_msg {
                // Compute same dialog area as InputDialog for consistent positioning
                let mut dialog_width = area.width - 4;
                dialog_width = dialog_width.min(max_width);
                let dialog_height = 3u16;
                let dialog_area = calc_centered_dialog_rect(area, dialog_width, dialog_height);

                // Prefer rendering one line below the dialog; otherwise place one line above
                let mut y = dialog_area
                    .y
                    .saturating_add(dialog_height)
                    .saturating_add(1);
                if y >= area.y.saturating_add(area.height) {
                    y = dialog_area.y.saturating_sub(2);
                }
                let msg_area = ratatui::layout::Rect::new(dialog_area.x, y, dialog_width, 1);
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

                // Handle cancel/quit
                if user_events
                    .iter()
                    .any(|e| matches!(e, UserEvent::InputDialogClose | UserEvent::Quit))
                {
                    return Err(anyhow!("canceled"));
                }

                // Handle apply with validation
                if user_events
                    .iter()
                    .any(|e| matches!(e, UserEvent::InputDialogApply))
                {
                    let input = state.input().trim().to_string();
                    if input.is_empty() {
                        error_msg = Some(PROFILE_EMPTY_ERR.to_string());
                        continue;
                    } else {
                        return Ok(input);
                    }
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
