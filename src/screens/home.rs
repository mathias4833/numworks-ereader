use crate::app::AppAction;
use crate::eadk::event::Event;
use crate::screens::{Screen, ScreenContext};
use crate::ui::components::book_card::BookCard;
use crate::ui::components::menu::Menu;
use crate::ui::components::status_bar::StatusBar;
use crate::ui::icons::{Icon, IconView};
use crate::ui::layout::{Frame, Insets, SCREEN, Stack};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};

static HOME_ITEMS: [&str; 3] = ["Open reader", "Library", "Settings"];
static HOME_ICONS: [Icon; 3] = [Icon::Book, Icon::Folder, Icon::Settings];

pub struct HomeScreen {
    menu: Menu<&'static [&'static str]>,
}

impl HomeScreen {
    pub const fn new() -> Self {
        Self {
            menu: Menu::new(&HOME_ITEMS),
        }
    }
}

impl Screen for HomeScreen {
    fn draw<D>(&self, display: &mut D, ctx: ScreenContext<'_>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let frame = Frame::new(SCREEN)
            .with_status_bar(24)
            .with_padding(Insets::all(8));

        SCREEN
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(display)?;

        StatusBar::new(frame.status_bar(), "NumWorks Reader")
            .with_battery(ctx.battery)
            .draw(display)?;

        let content = frame.content();
        let book_panel = Rectangle::new(
            content.top_left,
            Size::new(content.size.width, BookCard::HEIGHT),
        );

        if let Ok(Some(book)) = ctx.library.book(ctx.reading.current_book()) {
            BookCard::new(
                book_panel,
                &book,
                ctx.reading.progress_percent(book.page_count()),
            )
            .with_label("Current book")
            .draw(display)?;
        }

        let menu_top = content.top_left.y + book_panel.size.height as i32 + 8;
        let menu_area = Rectangle::new(
            Point::new(content.top_left.x, menu_top),
            Size::new(
                content.size.width,
                content
                    .size
                    .height
                    .saturating_sub(book_panel.size.height + 8),
            ),
        );

        let menu_layout = Stack::vertical(menu_area, 22, 5);

        self.menu
            .view(menu_layout)
            .with_text_left_padding(28)
            .draw(display)?;

        draw_menu_icons(display, menu_layout, self.menu.selected())?;

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> AppAction {
        match event {
            Event::Up => AppAction::redraw_if(self.menu.move_up()),

            Event::Down => AppAction::redraw_if(self.menu.move_down()),

            Event::Ok => match self.menu.selected() {
                0 => AppAction::OpenReader,
                1 => AppAction::OpenLibrary,
                2 => AppAction::None, // TODO: Settings
                _ => AppAction::None,
            },

            _ => AppAction::None,
        }
    }
}

fn draw_menu_icons<D>(display: &mut D, layout: Stack, selected_index: usize) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    for index in 0..HOME_ITEMS.len() {
        let Some(area) = layout.item_rect(index) else {
            continue;
        };

        let color = if index == selected_index {
            Rgb565::WHITE
        } else {
            Rgb565::BLACK
        };
        let icon_area = Rectangle::new(area.top_left + Point::new(6, 5), Size::new(13, 12));

        IconView::new(HOME_ICONS[index], icon_area, color).draw(display)?;
    }

    Ok(())
}
