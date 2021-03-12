use crate::Direction;
use crate::ImStr;
use crate::Ui;
use std::os::raw::c_char;

pub struct DockSpaceNode {
    id: u32,
}

impl DockSpaceNode {
    fn new(id: u32) -> Self {
        Self{id}
    }
    /// Dock window into this dockspace
    #[doc(alias = "igDockBuilderDockWindow")]
    pub fn dock_window(&self, window: &ImStr) {
        unsafe { sys::igDockBuilderDockWindow(window.as_ptr(), self.id) }
    }

    pub fn split<D, O>(&self, split_dir: Direction, size_ratio: f32, dir: D, opposite_dir: O)
    where
        D: FnOnce(DockSpaceNode),
        O: FnOnce(DockSpaceNode),
    {
        let mut out_id_at_dir: sys::ImGuiID = 0;
        let mut out_id_at_opposite_dir: sys::ImGuiID = 0;
        unsafe {
            sys::igDockBuilderSplitNode(
                self.id,
                split_dir as i32,
                size_ratio,
                &mut out_id_at_dir,
                &mut out_id_at_opposite_dir,
            );
        }

        dir(DockSpaceNode::new(out_id_at_dir));
        opposite_dir(DockSpaceNode::new(out_id_at_opposite_dir));
    }
}

/// # Docking
impl<'ui> Ui<'ui> {
    /// Create dockspace with given label. Returns a handle to the
    /// dockspace which can be used to, say, programatically split or
    /// dock windows into it
    pub fn dockspace(&'ui self, label: &ImStr) -> DockSpaceNode {
        unsafe {
            let id = sys::igGetIDStr(label.as_ptr() as *const c_char);
            sys::igDockSpace(
                id,
                [0.0, 0.0].into(),
                0,
                ::std::ptr::null::<sys::ImGuiWindowClass>(),
            );
            DockSpaceNode { id }
        }
    }

    pub fn dockspace_over_viewport(&'ui self) {
        unsafe {
            sys::igDockSpaceOverViewport(
                sys::igGetMainViewport(),
                0,
                ::std::ptr::null::<sys::ImGuiWindowClass>(),
            );
        }
    }
}
