pub mod board;

use std::io;

use board::tile::*;
use board::*;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tile1 = Tile::new(None);
    let _tile2 = Tile::new(Some(vec![Oasis {
        position: [true, false, false, true],
        resources: vec![],
        bonuses: vec![],
    }]));

    let mut board1 = Board::fill(None);

    for row in board1.0.iter_mut() {
        for tile in row.iter_mut() {
            *tile = Some(tile1.clone());
        }
    }

    terminal.clear()?;

    terminal.draw(|f| {
        let size = f.size();

        let _window = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(11 * 11), Constraint::Length(5 * 11)].as_ref());

        let block = Block::default().borders(Borders::ALL);

        let table = Table::new(
            board1
                .0
                .map(|row| {
                    Row::new(row.map(|cell| {
                        if let Some(cell) = cell {
                            return Cell::from(cell.to_string());
                        }
                        Cell::from("None")
                    }))
                    .height(5)
                })
                .to_vec(),
        )
        .block(block)
        .style(Style::default())
        .widths(&[Constraint::Length(11); 11])
        .column_spacing(1);

        f.render_widget(table, size);
    })?;

    Ok(())
}
