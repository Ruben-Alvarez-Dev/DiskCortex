//! UI Rendering

use super::app::{App, ViewMode, format_size};
use crate::tools::RiskLevel;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs},
    Frame,
};

/// Main draw function
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_main(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);

    if app.show_help {
        draw_help(f, f.area());
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let colors = [Color::Cyan, Color::Green, Color::Yellow, Color::Magenta];
    let color = colors[app.frame as usize % colors.len()];
    
    let title = Line::from(vec![
        Span::styled("🧠 ", Style::default()),
        Span::styled("DiskCortex", Style::default().fg(color).add_modifier(Modifier::BOLD)),
        Span::raw(" - Dev Disk Cleaner v0.1.0"),
    ]);

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    
    f.render_widget(header, area);
}

fn draw_main(f: &mut Frame, app: &App, area: Rect) {
    match app.view {
        ViewMode::Overview => draw_overview(f, app, area),
        ViewMode::Tools => draw_tools(f, app, area),
        ViewMode::Cleanup => draw_cleanup(f, app, area),
        ViewMode::Settings => draw_settings(f, app, area),
    }
}

fn draw_overview(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(5)])
        .split(area);

    // Stats
    let stats = vec![
        Line::from(vec![
            Span::styled("📊 Total Space: ", Style::default().fg(Color::Gray)),
            Span::styled(format_size(app.total_size), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("📦 Tools Found: ", Style::default().fg(Color::Gray)),
            Span::styled(app.detected_tools.len().to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("✅ Selected: ", Style::default().fg(Color::Gray)),
            Span::styled(app.selected_for_cleanup.len().to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::default(),
        Line::from("Controls:"),
        Line::from(vec![
            Span::styled("  s", Style::default().fg(Color::Yellow)),
            Span::raw(" Scan  "),
            Span::styled("c", Style::default().fg(Color::Yellow)),
            Span::raw(" Clean  "),
            Span::styled("?", Style::default().fg(Color::Yellow)),
            Span::raw(" Help  "),
            Span::styled("q", Style::default().fg(Color::Red)),
            Span::raw(" Quit"),
        ]),
    ];

    let stats_panel = Paragraph::new(stats)
        .block(Block::default().title(" Overview ").borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    
    f.render_widget(stats_panel, chunks[0]);

    // Tool list
    let items: Vec<ListItem> = app.detected_tools
        .iter()
        .enumerate()
        .map(|(i, tool)| {
            let style = if i == app.selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            
            let checkbox = if app.selected_for_cleanup.contains(&tool.id) { "☑" } else { "☐" };
            let risk = risk_color(tool.risk);
            
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", checkbox), Style::default().fg(Color::Cyan)),
                Span::styled(&tool.name, Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(format_size(tool.size), Style::default().fg(Color::Yellow)),
                Span::raw("  "),
                Span::styled(tool.risk.to_string(), Style::default().fg(risk)),
            ])).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" Tools ").borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)));
    
    f.render_widget(list, chunks[1]);
}

fn draw_tools(f: &mut Frame, app: &App, area: Rect) {
    let text = vec![
        Line::from("Tools by Category (Coming Soon)"),
        Line::default(),
        Line::from(vec![
            Span::styled("Use ← → to navigate categories", Style::default().fg(Color::Gray)),
        ]),
    ];
    
    let panel = Paragraph::new(text)
        .block(Block::default().title(" Tools ").borders(Borders::ALL));
    
    f.render_widget(panel, area);
}

fn draw_cleanup(f: &mut Frame, app: &App, area: Rect) {
    let selected: Vec<_> = app.detected_tools
        .iter()
        .filter(|t| app.selected_for_cleanup.contains(&t.id))
        .collect();
    
    let total: u64 = selected.iter().map(|t| t.size).sum();
    
    let mut text = vec![
        Line::from(Span::styled("🗑️ Cleanup Plan", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
        Line::default(),
        Line::from(format!("Selected: {} tools", selected.len())),
        Line::from(vec![
            Span::raw("Total to reclaim: "),
            Span::styled(format_size(total), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::default(),
    ];
    
    for tool in selected {
        text.push(Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::Cyan)),
            Span::raw(&tool.name),
            Span::styled(format!(" ({})", format_size(tool.size)), Style::default().fg(Color::Yellow)),
        ]));
    }
    
    let panel = Paragraph::new(text)
        .block(Block::default().title(" Cleanup ").borders(Borders::ALL));
    
    f.render_widget(panel, area);
}

fn draw_settings(f: &mut Frame, app: &App, area: Rect) {
    let text = vec![
        Line::from(Span::styled("⚙️ Settings", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
        Line::default(),
        Line::from("Configuration coming soon..."),
    ];
    
    let panel = Paragraph::new(text)
        .block(Block::default().title(" Settings ").borders(Borders::ALL));
    
    f.render_widget(panel, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status = Paragraph::new(Line::from(vec![
        Span::styled(" ❯ ", Style::default().fg(Color::DarkGray)),
        Span::styled(&app.status, Style::default().fg(Color::White)),
    ])).style(Style::default().bg(Color::Black));
    
    f.render_widget(status, area);
}

fn draw_help(f: &mut Frame, area: Rect) {
    let help_area = Rect { x: area.width/4, y: area.height/4, width: area.width/2, height: area.height/2 };
    f.render_widget(Clear, help_area);
    
    let help = vec![
        Line::from(Span::styled("⌨️ Keyboard Shortcuts", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::default(),
        Line::from(vec![Span::styled("  s  ", Style::default().fg(Color::Yellow)), Span::raw("Scan for tools")]),
        Line::from(vec![Span::styled("  c  ", Style::default().fg(Color::Yellow)), Span::raw("Clean selected")]),
        Line::from(vec![Span::styled("  Space  ", Style::default().fg(Color::Yellow)), Span::raw("Toggle selection")]),
        Line::from(vec![Span::styled("  a  ", Style::default().fg(Color::Yellow)), Span::raw("Select all")]),
        Line::from(vec![Span::styled("  d  ", Style::default().fg(Color::Yellow)), Span::raw("Deselect all")]),
        Line::from(vec![Span::styled("  ↑↓  ", Style::default().fg(Color::Yellow)), Span::raw("Navigate items")]),
        Line::from(vec![Span::styled("  1-4  ", Style::default().fg(Color::Yellow)), Span::raw("Switch views")]),
        Line::from(vec![Span::styled("  ?  ", Style::default().fg(Color::Yellow)), Span::raw("Toggle help")]),
        Line::from(vec![Span::styled("  q  ", Style::default().fg(Color::Red)), Span::raw("Quit")]),
    ];
    
    let panel = Paragraph::new(help)
        .block(Block::default().title(" Help ").borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    
    f.render_widget(panel, help_area);
}

fn risk_color(risk: RiskLevel) -> Color {
    match risk {
        RiskLevel::Safe => Color::Green,
        RiskLevel::Low => Color::Yellow,
        RiskLevel::Medium => Color::LightRed,
        RiskLevel::High => Color::Red,
    }
}
