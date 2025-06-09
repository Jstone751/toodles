use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct App {
    todos: Vec<TodoItem>,
    list_state: ListState,
}

#[derive(Debug, Default)]
struct TodoItem {
    done: bool,
    description: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::default();

    app.todos.push(TodoItem {
        done: false,
        description: "This is a test".to_owned(),
    });
    app.todos.push(TodoItem {
        done: false,
        description: "Even more test".to_owned(),
    });
    app.todos.push(TodoItem {
        done: false,
        description: "Lots more test".to_owned(),
    });
    let term = ratatui::init();
    let result = game_loop(term, &mut app);
    ratatui::restore();
    result
}

fn game_loop(mut terminal: DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app))?;
        // Inputs
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'j' => {
                        app.list_state.select_next();
                    }
                    'k' => {
                        app.list_state.select_previous();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &mut App) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    let [list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);
    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Red)
        .render(border_area, frame.buffer_mut());
    //Paragraph::new("Hello from app!").render(frame.area(), frame.buffer_mut());
    let list = List::new(
        app.todos
            .iter()
            .map(|i| ListItem::from(i.description.clone())),
    )
    .highlight_style(Style::default().fg(Color::Green))
    .highlight_symbol("->");
    frame.render_stateful_widget(list, list_area, &mut app.list_state);
}
