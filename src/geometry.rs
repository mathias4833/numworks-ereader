#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn from_edges(left: u16, top: u16, right: u16, bottom: u16) -> Self {
        Self {
            x: left,
            y: top,
            width: right.saturating_sub(left),
            height: bottom.saturating_sub(top),
        }
    }

    pub const fn left(self) -> u16 {
        self.x
    }

    pub const fn top(self) -> u16 {
        self.y
    }

    // Exclusive bounds, last covered x coordinate is right() - 1
    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    // Exclusive bounds, last covered y coordinate is bottom() - 1
    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub const fn size(self) -> Size {
        Size::new(self.width, self.height)
    }

    pub const fn centered_rect(self, size: Size) -> Rect {
        Rect::new(
            self.x + self.width.saturating_sub(size.width) / 2,
            self.y + self.height.saturating_sub(size.height) / 2,
            size.width,
            size.height,
        )
    }
}
