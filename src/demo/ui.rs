use crate::demo::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap, BorderType
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(4)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

    

    let job_block = Block::default()
        .borders(Borders::ALL)
        .title("Jobs")
        .border_type(BorderType::Rounded);

    
    let job_items: Vec<ListItem> = app.job_list.iter().map(|j| ListItem::new(j.title.to_string())).collect();

    let job_list = List::new(job_items)
        .block(job_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");  
        
    f.render_widget(job_list, chunks[0]);



    let dev_block = Block::default()
        .borders(Borders::ALL)
        .title("Dev")
        .border_type(BorderType::Rounded);

    let dev_items: Vec<ListItem> = app.dev_list.iter().map(|d| ListItem::new(d.name.to_string())).collect();
    let dev_list = List::new(dev_items)
        .block(dev_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");  
        
    f.render_widget(dev_list, chunks[1]);
}
