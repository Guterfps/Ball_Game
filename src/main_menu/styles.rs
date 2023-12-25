
use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const MAIN_MENU_STYLE: Style = main_menu_style();
pub const BUTTON_STYLE: Style = button_style();
pub const IMAGE_STYLE: Style = image_style();
pub const TITLE_STYLE: Style = title_style();

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server
                .load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server
                .load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE
    }
}

const fn main_menu_style() -> Style {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);

    style

}

const fn button_style() -> Style {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    
    style
}

const fn image_style() -> Style {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::all(Val::Px(8.0));
    
    style

}

const fn title_style() -> Style {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(900.0);
    style.height = Val::Px(120.0);

    style
}
