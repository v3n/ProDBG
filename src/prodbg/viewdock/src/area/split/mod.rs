mod serialize;

use super::{Area, DragTarget, DropTarget};
use dock::DockHandle;
use rect::{Rect, Direction};

/// Handle to a split
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SplitHandle(pub u64);

/// Given rectangle area is split in two parts.
#[derive(Debug, Clone)]
pub struct Split {
    /// Children
    pub children: Vec<Area>,
    /// Right (or bottom) border of each child. Last should always be 1.
    pub ratios: Vec<f32>,
    /// Direction of the split
    pub direction: Direction,
    /// Handle of the split
    pub handle: SplitHandle,
    /// Area occupied by this split
    pub rect: Rect,
}

impl Split {
    pub fn from_two(direction: Direction, ratio: f32, handle: SplitHandle, rect: Rect, first: Area, second: Area) -> Split {
        let mut res = Split {
            children: vec!(first, second),
            ratios: vec!(ratio, 1.0),
            direction: direction,
            handle: handle,
            rect: rect,
        };
        res.update_children_sizes();
        return res;
    }

    fn update_children_sizes(&mut self) {
        let rects = self.rect.split_by_direction(self.direction, &self.ratios);
        for (child, rect) in self.children.iter_mut().zip(rects.iter()) {
            child.update_rect(*rect);
        }
    }

    pub fn update_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.update_children_sizes();
    }

    fn get_child_at_pos(&self, pos: (f32, f32)) -> Option<&Area> {
        self.children.iter()
            .find(|child| child.get_rect().point_is_inside(pos))
    }

    pub fn get_drag_target_at_pos(&self, pos: (f32, f32)) -> Option<DragTarget> {
        let sizer_rects = self.rect.area_around_splits(self.direction, &self.ratios[0..self.ratios.len() - 1], 8.0);
        return sizer_rects.iter().enumerate()
            .find(|&(_, rect)| rect.point_is_inside(pos))
            .map(|(i, _)| DragTarget::SplitSizer(self.handle, i, self.direction))
            .or_else(|| {
                self.get_child_at_pos(pos)
                    .and_then(|child| child.get_drag_target_at_pos(pos))
            });
    }

    pub fn get_drop_target_at_pos(&self, pos: (f32, f32)) -> Option<DropTarget> {
        self.get_child_at_pos(pos)
            .and_then(|child| child.get_drop_target_at_pos(pos))
    }

    pub fn map_rect_to_delta(&self, delta: (f32, f32)) -> f32 {
        match self.direction {
            Direction::Vertical => -delta.0 / self.rect.width,
            Direction::Horizontal => -delta.1 / self.rect.height,
        }
    }

    pub fn change_ratio(&mut self, index: usize, delta: (f32, f32)) {
        let scale = Self::map_rect_to_delta(self, delta);
        let mut res = self.ratios[index] + scale;

        if res < 0.01 {
            res = 0.01;
        }

        if res > 0.99 {
            res = 0.99;
        }

        self.ratios[index] = res;
        self.update_children_sizes();
    }

    pub fn get_dock_handle_at_pos(&self, pos: (f32, f32)) -> Option<DockHandle> {
        self.children.iter()
            .find(|child| child.get_rect().point_is_inside(pos))
            .and_then(|child| child.get_dock_handle_at_pos(pos))
    }

    pub fn replace_child(&mut self, index: usize, new_child: Area) -> Area {
        self.children.push(new_child);
        let res = self.children.swap_remove(index);
        self.update_children_sizes();
        return res;
    }

    pub fn append_child(&mut self, index: usize, child: Area) {
        let existing_ratio = self.ratios[index];
        let previous_ratio = match index {
            0 => 0.0,
            _ => self.ratios[index - 1]
        };
        let diff = existing_ratio - previous_ratio;
        self.children.insert(index, child);
        self.ratios.insert(index, existing_ratio - diff / 2.0);
        self.update_children_sizes();
    }

    pub fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
        self.ratios.remove(index);
        if index == self.ratios.len() {
            self.ratios[index - 1] = 1.0;
        }
        self.update_children_sizes();
    }

    pub fn replace_child_with_children(&mut self, index: usize, children: &[Area]) {
        self.children.remove(index);
        let mut dimensions: Vec<f32> = children.iter()
            .map(|child| match self.direction {
                Direction::Horizontal => child.get_rect().height,
                Direction::Vertical => child.get_rect().width,
            }).collect();
        let dimension_sum = dimensions.iter().fold(0.0, |sum, dimension| sum + dimension);
        let mut prev = 0.0;
        for dimension in dimensions.iter_mut() {
            prev += *dimension / dimension_sum;
            *dimension = prev;
        }
        for child in children.iter().rev() {
            self.children.insert(index, child.clone());
        }

        let old_ratio = self.ratios.remove(index);
        let previous_ratio = match index {
            0 => 0.0,
            _ => self.ratios[index - 1]
        };
        let diff = old_ratio - previous_ratio;
        for pos in dimensions.iter().rev() {
            self.ratios.insert(index, previous_ratio + pos * diff);
        }
        self.update_children_sizes();
    }
}


#[cfg(test)]
mod test {
    extern crate serde_json;

    use {Split, SplitHandle, Rect, Direction, Area};
    use super::super::container::Container;
    use dock::{Dock, DockHandle};

    #[test]
    fn test_splithandle_serialize() {
        let handle_in = SplitHandle(0x4422);
        let serialized = serde_json::to_string(&handle_in).unwrap();
        let handle_out: SplitHandle = serde_json::from_str(&serialized).unwrap();

        assert_eq!(handle_in, handle_out);
    }

    #[test]
    fn test_split_serialize() {
        let split_in = Split::from_two(
            Direction::Horizontal,
            0.7,
            SplitHandle(513),
            Rect::new(17.0, 15.0, 100.0, 159.0),
            Area::Container(Container::new(Dock::new(DockHandle(14), "test"), Rect::default())),
            Area::Container(Container::new(Dock::new(DockHandle(15), "test2"), Rect::default()))
        );

        let serialized = serde_json::to_string(&split_in).unwrap();
        let split_out: Split = serde_json::from_str(&serialized).unwrap();

        assert_eq!(split_in.children.len(), split_out.children.len());
        assert_eq!(split_in.ratios.len(), split_out.ratios.len());
        assert_eq!(split_in.direction, split_out.direction);
        assert_eq!(split_in.handle, split_out.handle);

        // expect that rect is not serialized and set to zero
        assert_eq!(split_out.rect.x as i32, 0);
        assert_eq!(split_out.rect.y as i32, 0);
        assert_eq!(split_out.rect.width as i32, 0);
        assert_eq!(split_out.rect.height as i32, 0);
    }
}