// Adapted from OpenCADStudio/src/ui/ribbon/{mod.rs,widgets.rs,draw_panel.rs}
// at commit 1fec34d84a129a2e4c6466e184893f78ae596777.
// Copyright OpenCADStudio contributors. Licensed under GPL-3.0.
// Changes: adapted the ribbon primitives to iced 0.13 and point-cloud commands.

use iced::widget::{button, column, container, row, text};
use iced::{Background, Element, Fill, Length, Theme};

use crate::{ui_style, ui_theme, Message};

mod adaptive;
pub use adaptive::{AdaptivePanel, AdaptiveRibbon};

pub const TOOL_BAR_H: f32 = openaec_ui::model::metrics::RIBBON as f32;
pub const QUICK_ACCESS_W: f32 = 30.0;

/// Compact quick access actions on the dark title strip.
pub fn quick_access_btn<'a>(
    icon: Element<'a, Message>,
    label: &'static str,
    message: Option<Message>,
) -> Element<'a, Message> {
    let control = button(container(icon).width(Fill).height(Fill).center(Fill))
        .on_press_maybe(message)
        .style(title_action_style)
        .width(Length::Fixed(QUICK_ACCESS_W))
        .height(26)
        .padding([1, 0]);
    // Below the tools of the ribbon, which it would cover.
    ui_style::ribbon_tooltip(control, crate::i18n::tr(label))
        .gap(90)
        .into()
}

/// The wider, single-row groups of the approved desktop reference. The
/// adaptive ribbon still measures and collapses each complete group.
pub fn render_reference_group<'a>(
    title: &'static str,
    width: f32,
    tools: Element<'a, Message>,
) -> Element<'a, Message> {
    let caption = text(crate::i18n::tr(title))
        .size(12)
        .font(crate::fonts::MEDIUM)
        .style(|theme| text::Style {
            color: Some(ui_theme::colors(theme).ribbon_group_label),
        });
    let content = column![
        container(tools)
            .width(Fill)
            .height(64)
            .center_x(Fill)
            .align_y(iced::Alignment::Center),
        container(caption).width(Fill).center_x(Fill),
    ]
    .spacing(4)
    .padding([4, 0])
    .width(Length::Fixed(width - 1.0))
    .height(TOOL_BAR_H);
    row![
        content,
        container(text(""))
            .width(1)
            .height(80)
            .style(|theme| container::Style::default()
                .background(ui_theme::colors(theme).ribbon_group_separator)),
    ]
    .align_y(iced::Alignment::Center)
    .width(Length::Fixed(width))
    .into()
}

fn title_action_style(theme: &Theme, status: button::Status) -> button::Style {
    let colors = ui_theme::colors(theme);
    let hovered = matches!(status, button::Status::Hovered);
    button::Style {
        background: hovered.then_some(Background::Color(colors.ribbon_btn_hover)),
        text_color: colors.text,
        ..button::Style::default()
    }
}

/// Shared Tauri-derived tab state for the 28 px tab strip.
pub fn tab_style(theme: &Theme, active: bool, status: button::Status) -> button::Style {
    openaec_ui::widgets::ribbon_tab_style(ui_theme::shared_theme(theme), active, status)
}
