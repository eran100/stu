use anyhow::anyhow;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event as CEvent, KeyEvent},
    Terminal,
};

use crate::{
    color::ColorTheme,
    keys::{UserEvent, UserEventMapper},
    widget::{InputDialog, InputDialogState},
};

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

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let dialog = InputDialog::default()
                .title("AWS Profile")
                .max_width(50)
                .theme(&theme);
            f.render_stateful_widget(dialog, area, &mut state);

            let (x, y) = state.cursor();
            f.set_cursor_position((x, y));
        })?;

        match event::read()? {
            CEvent::Key(key) => {
                if let Some(result) = handle_input_keys(&mapper, key, &mut state) {
                    return result;
                }
            }
            CEvent::Resize(_, _) => {
                // trigger redraw on next loop iteration
            }
            _ => {}
        }
    }
}

fn handle_input_keys(
    mapper: &UserEventMapper,
    key: KeyEvent,
    state: &mut InputDialogState,
) -> Option<anyhow::Result<String>> {
    let user_events = mapper.find_events(key);

    for e in &user_events {
        match e {
            UserEvent::InputDialogClose | UserEvent::Quit => {
                return Some(Err(anyhow!("canceled")));
            }
            UserEvent::InputDialogApply => {
                let input = state.input().trim().to_string();
                return Some(Ok(input));
            }
            _ => {}
        }
    }

    // Not handled as a mapped action; treat as text editing input.
    state.handle_key_event(key);
    None
}

