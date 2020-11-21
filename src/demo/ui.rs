use crate::demo::App;
use tui::{
    backend::Backend,
    layout::{Alignment,Constraint, Direction, Layout, Rect},
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
use std::fmt::Display;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let top_bottom = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                    .split(f.size());
    let text = vec![Spans::from(vec![
        Span::raw(app.world_datetime.to_string())
        ])];
    let top_paragraph = Paragraph::new(text.clone())
        .alignment(Alignment::Center);
    f.render_widget(top_paragraph, top_bottom[0]);

    let panels = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(top_bottom[1]);

    

    let job_block = Block::default()
        .borders(Borders::ALL)
        .title("Jobs")
        .border_type(BorderType::Rounded);

    
    let job_items: Vec<ListItem> = app.jobs.iter().map(|(_, j)|
        ListItem::new(format!("{} {}h tot - {}h left - {:.0}%", j.title.to_string(), j.duration.num_hours(), j.duration_remaining.num_hours(), j.duration_remaining.num_milliseconds() as f64 / j.duration.num_milliseconds() as f64 * 100.))
    ).collect();

    let job_list = List::new(job_items)
        .block(job_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");  
        
    f.render_widget(job_list, panels[0]);



    let dev_block = Block::default()
        .borders(Borders::ALL)
        .title("Dev")
        .border_type(BorderType::Rounded);

    let dev_items: Vec<ListItem> = app.devs.iter().map(|(_, d)| ListItem::new(d.name.to_string())).collect();
    let dev_list = List::new(dev_items)
        .block(dev_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");  
        
    f.render_widget(dev_list, panels[1]);
}
