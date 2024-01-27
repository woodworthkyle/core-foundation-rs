use crate::base::{id, BOOL, SEL};
pub use core_graphics::base::CGFloat;
pub use core_graphics::geometry::{CGPoint, CGRect, CGSize};


pub trait UIView: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(UIView), alloc]
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithFrame_(self, frameRect: CGRect) -> id;
    unsafe fn bounds(self) -> CGRect;
    unsafe fn frame(self) -> CGRect;
    
    unsafe fn addSubview_(self, view: id);
    unsafe fn superview(self) -> id;
    unsafe fn removeFromSuperview(self);
    unsafe fn setAutoresizingMask_(self, autoresizingMask: id);

    unsafe fn widthAnchor(self) -> id;
    unsafe fn heightAnchor(self) -> id;
}

impl UIView for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn initWithFrame_(self, frameRect: CGRect) -> id {
        msg_send![self, initWithFrame: frameRect]
    }

    unsafe fn bounds(self) -> CGRect {
        msg_send![self, bounds]
    }

    unsafe fn frame(self) -> CGRect {
        msg_send![self, frame]
    }

    unsafe fn addSubview_(self, view: id) {
        msg_send![self, addSubview: view]
    }

    unsafe fn superview(self) -> id {
        msg_send![self, superview]
    }

    unsafe fn removeFromSuperview(self) {
        msg_send![self, removeFromSuperview]
    }

    unsafe fn setAutoresizingMask_(self, autoresizingMask: id) {
        msg_send![self, setAutoresizingMask: autoresizingMask]
    }


    unsafe fn widthAnchor(self) -> id {
        msg_send![self, widthAnchor]
    }

    unsafe fn heightAnchor(self) -> id {
        msg_send![self, heightAnchor]
    }
}