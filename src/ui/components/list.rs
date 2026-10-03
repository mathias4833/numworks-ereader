use crate::ui::invalidation::Invalidation;
use embedded_graphics::primitives::Rectangle;

pub struct List<const N: usize> {
    area: Rectangle,
    slots: [Rectangle; N],
    item_count: usize,
    selected: usize,
}

pub struct ListSlot {
    pub index: usize,
    pub area: Rectangle,
    pub selected: bool,
}

impl<const N: usize> List<N> {
    pub fn new(area: Rectangle, slots: [Rectangle; N], item_count: usize, selected: usize) -> Self {
        Self {
            area,
            slots,
            item_count,
            selected: selected.min(item_count.saturating_sub(1)),
        }
    }

    pub const fn selected(&self) -> usize {
        self.selected
    }

    pub fn move_up(&mut self, invalidation: &mut Invalidation) {
        if self.selected > 0 {
            self.select(self.selected - 1, invalidation);
        }
    }

    pub fn move_down(&mut self, invalidation: &mut Invalidation) {
        if self.selected + 1 < self.item_count {
            self.select(self.selected + 1, invalidation);
        }
    }

    pub fn slots(&self) -> impl Iterator<Item = ListSlot> + '_ {
        let first = self.first_visible();
        let visible = self.item_count.saturating_sub(first).min(N);

        self.slots
            .iter()
            .copied()
            .take(visible)
            .enumerate()
            .map(move |(offset, area)| {
                let index = first + offset;

                ListSlot {
                    index,
                    area,
                    selected: index == self.selected,
                }
            })
    }

    fn select(&mut self, selected: usize, invalidation: &mut Invalidation) {
        let previous = self.selected;
        let previous_first = self.first_visible();

        self.selected = selected;
        let first = self.first_visible();

        if first != previous_first {
            invalidation.invalidate(self.area);
            return;
        }

        if let Some(area) = self.slot_area(previous, first) {
            invalidation.invalidate(area);
        }
        if let Some(area) = self.slot_area(self.selected, first) {
            invalidation.invalidate(area);
        }
    }

    fn first_visible(&self) -> usize {
        self.selected.saturating_sub(N.saturating_sub(1))
    }

    fn slot_area(&self, index: usize, first: usize) -> Option<Rectangle> {
        self.slots.get(index.checked_sub(first)?).copied()
    }
}
