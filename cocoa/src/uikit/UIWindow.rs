use crate::base::{id, BOOL, SEL};
pub use core_graphics::base::CGFloat;
pub use core_graphics::geometry::{CGPoint, CGRect, CGSize};
pub trait NSWindow: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSWindow), alloc]
    }
}

impl NSWindow for id {
}

/*
pub trait NSWindow: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSWindow), alloc]
    }

    // Creating Windows
    unsafe fn initWithContentRect_styleMask_backing_defer_(
        self,
        rect: CGRect,
        style: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: BOOL,
    ) -> id;
    unsafe fn initWithContentRect_styleMask_backing_defer_screen_(
        self,
        rect: CGRect,
        style: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: BOOL,
        screen: id,
    ) -> id;

    // Configuring Windows
    unsafe fn styleMask(self) -> NSWindowStyleMask;
    unsafe fn setStyleMask_(self, styleMask: NSWindowStyleMask);
    unsafe fn toggleFullScreen_(self, sender: id);
    unsafe fn worksWhenModal(self) -> BOOL;
    unsafe fn alphaValue(self) -> CGFloat;
    unsafe fn setAlphaValue_(self, windowAlpha: CGFloat);
    unsafe fn backgroundColor(self) -> id;
    unsafe fn setBackgroundColor_(self, color: id);
    unsafe fn colorSpace(self) -> id;
    unsafe fn setColorSpace_(self, colorSpace: id);
    unsafe fn contentView(self) -> id;
    unsafe fn setContentView_(self, view: id);
    unsafe fn canHide(self) -> BOOL;
    unsafe fn setCanHide_(self, canHide: BOOL);
    unsafe fn hidesOnDeactivate(self) -> BOOL;
    unsafe fn setHidesOnDeactivate_(self, hideOnDeactivate: BOOL);
    unsafe fn collectionBehavior(self) -> NSWindowCollectionBehavior;
    unsafe fn setCollectionBehavior_(self, collectionBehavior: NSWindowCollectionBehavior);
    unsafe fn setOpaque_(self, opaque: BOOL);
    unsafe fn hasShadow(self) -> BOOL;
    unsafe fn setHasShadow_(self, hasShadow: BOOL);
    unsafe fn invalidateShadow(self);
    unsafe fn autorecalculatesContentBorderThicknessForEdge_(self, edge: CGRectEdge) -> BOOL;
    unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(
        self,
        autorecalculateContentBorderThickness: BOOL,
        edge: CGRectEdge,
    ) -> BOOL;
    unsafe fn contentBorderThicknessForEdge_(self, edge: CGRectEdge) -> CGFloat;
    unsafe fn setContentBorderThickness_forEdge_(self, borderThickness: CGFloat, edge: CGRectEdge);
    unsafe fn delegate(self) -> id;
    unsafe fn setDelegate_(self, delegate: id);
    unsafe fn preventsApplicationTerminationWhenModal(self) -> BOOL;
    unsafe fn setPreventsApplicationTerminationWhenModal_(self, flag: BOOL);

    // TODO: Accessing Window Information

    // Getting Layout Information
    unsafe fn contentRectForFrameRect_styleMask_(
        self,
        windowFrame: CGRect,
        windowStyle: NSWindowStyleMask,
    ) -> CGRect;
    unsafe fn frameRectForContentRect_styleMask_(
        self,
        windowContentRect: CGRect,
        windowStyle: NSWindowStyleMask,
    ) -> CGRect;
    unsafe fn minFrameWidthWithTitle_styleMask_(
        self,
        windowTitle: id,
        windowStyle: NSWindowStyleMask,
    ) -> CGFloat;
    unsafe fn contentRectForFrameRect_(self, windowFrame: CGRect) -> CGRect;
    unsafe fn frameRectForContentRect_(self, windowContent: CGRect) -> CGRect;

    // Managing Windows
    unsafe fn drawers(self) -> id;
    unsafe fn windowController(self) -> id;
    unsafe fn setWindowController_(self, windowController: id);

    // TODO: Managing Sheets

    // Sizing Windows
    unsafe fn frame(self) -> CGRect;
    unsafe fn setFrameOrigin_(self, point: CGPoint);
    unsafe fn setFrameTopLeftPoint_(self, point: CGPoint);
    unsafe fn constrainFrameRect_toScreen_(self, frameRect: CGRect, screen: id);
    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: CGPoint) -> CGPoint;
    unsafe fn setFrame_display_(self, windowFrame: CGRect, display: BOOL);
    unsafe fn setFrame_display_animate_(self, windowFrame: CGRect, display: BOOL, animate: BOOL);
    unsafe fn setFrame_displayViews_(self, windowFrame: CGRect, display: BOOL);
    unsafe fn aspectRatio(self) -> CGSize;
    unsafe fn setAspectRatio_(self, aspectRatio: CGSize);
    unsafe fn minSize(self) -> CGSize;
    unsafe fn setMinSize_(self, minSize: CGSize);
    unsafe fn maxSize(self) -> CGSize;
    unsafe fn setMaxSize_(self, maxSize: CGSize);
    unsafe fn performZoom_(self, sender: id);
    unsafe fn zoom_(self, sender: id);
    unsafe fn resizeFlags(self) -> NSInteger;
    unsafe fn showsResizeIndicator(self) -> BOOL;
    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: BOOL);
    unsafe fn resizeIncrements(self) -> CGSize;
    unsafe fn setResizeIncrements_(self, resizeIncrements: CGSize);
    unsafe fn preservesContentDuringLiveResize(self) -> BOOL;
    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: BOOL);
    unsafe fn inLiveResize(self) -> BOOL;

    // Sizing Content
    unsafe fn contentAspectRatio(self) -> CGSize;
    unsafe fn setContentAspectRatio_(self, contentAspectRatio: CGSize);
    unsafe fn contentMinSize(self) -> CGSize;
    unsafe fn setContentMinSize_(self, contentMinSize: CGSize);
    unsafe fn contentSize(self) -> CGSize;
    unsafe fn setContentSize_(self, contentSize: CGSize);
    unsafe fn contentMaxSize(self) -> CGSize;
    unsafe fn setContentMaxSize_(self, contentMaxSize: CGSize);
    unsafe fn contentResizeIncrements(self) -> CGSize;
    unsafe fn setContentResizeIncrements_(self, contentResizeIncrements: CGSize);

    // Managing Window Visibility and Occlusion State
    unsafe fn isVisible(self) -> BOOL; // NOTE: Deprecated in 10.9
    unsafe fn occlusionState(self) -> NSWindowOcclusionState;

    // Managing Window Layers
    unsafe fn orderOut_(self, sender: id);
    unsafe fn orderBack_(self, sender: id);
    unsafe fn orderFront_(self, sender: id);
    unsafe fn orderFrontRegardless(self);
    unsafe fn orderFrontWindow_relativeTo_(
        self,
        orderingMode: NSWindowOrderingMode,
        otherWindowNumber: NSInteger,
    );
    unsafe fn level(self) -> NSInteger;
    unsafe fn setLevel_(self, level: NSInteger);

    // Managing Key Status
    unsafe fn isKeyWindow(self) -> BOOL;
    unsafe fn canBecomeKeyWindow(self) -> BOOL;
    unsafe fn makeKeyWindow(self);
    unsafe fn makeKeyAndOrderFront_(self, sender: id);
    // skipped: becomeKeyWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignKeyWindow (should not be invoked directly, according to Apple's documentation)

    // Managing Main Status
    unsafe fn canBecomeMainWindow(self) -> BOOL;
    unsafe fn makeMainWindow(self);
    // skipped: becomeMainWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignMainWindow (should not be invoked directly, according to Apple's documentation)

    // Managing Toolbars
    unsafe fn toolbar(self) -> id /* NSToolbar */;
    unsafe fn toolbarStyle(self) -> NSWindowToolbarStyle;
    unsafe fn setToolbar_(self, toolbar: id /* NSToolbar */);
    unsafe fn setToolbarStyle_(self, toolbarStyle: NSWindowToolbarStyle);
    unsafe fn runToolbarCustomizationPalette(self, sender: id);

    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles

    // Managing Title Bars
    unsafe fn standardWindowButton_(self, windowButtonKind: NSWindowButton) -> id;

    // Managing Window Tabs
    unsafe fn allowsAutomaticWindowTabbing(_: Self) -> BOOL;
    unsafe fn setAllowsAutomaticWindowTabbing_(_: Self, allowsAutomaticWindowTabbing: BOOL);
    unsafe fn tabbingIdentifier(self) -> id;
    unsafe fn tabbingMode(self) -> NSWindowTabbingMode;
    unsafe fn setTabbingMode_(self, tabbingMode: NSWindowTabbingMode);
    unsafe fn addTabbedWindow_ordered_(self, window: id, ordering_mode: NSWindowOrderingMode);
    unsafe fn toggleTabBar_(self, sender: id);

    // TODO: Managing Tooltips
    // TODO: Handling Events

    // Managing Responders
    unsafe fn initialFirstResponder(self) -> id;
    unsafe fn firstResponder(self) -> id;
    unsafe fn setInitialFirstResponder_(self, responder: id);
    unsafe fn makeFirstResponder_(self, responder: id) -> BOOL;

    // TODO: Managing the Key View Loop

    // Handling Keyboard Events
    unsafe fn keyDown_(self, event: id);

    // Handling Mouse Events
    unsafe fn acceptsMouseMovedEvents(self) -> BOOL;
    unsafe fn ignoresMouseEvents(self) -> BOOL;
    unsafe fn setIgnoresMouseEvents_(self, ignoreMouseEvents: BOOL);
    unsafe fn mouseLocationOutsideOfEventStream(self) -> CGPoint;
    unsafe fn setAcceptsMouseMovedEvents_(self, acceptMouseMovedEvents: BOOL);
    unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(
        _: Self,
        point: CGPoint,
        windowNumber: NSInteger,
    ) -> NSInteger;

    // TODO: Handling Window Restoration
    // TODO: Bracketing Drawing Operations
    // TODO: Drawing Windows
    // TODO: Window Animation
    // TODO: Updating Windows
    // TODO: Dragging Items

    // Converting Coordinates
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn backingAlignedRect_options_(
        self,
        rect: CGRect,
        options: NSAlignmentOptions,
    ) -> CGRect;
    unsafe fn convertRectFromBacking_(self, rect: CGRect) -> CGRect;
    unsafe fn convertRectToBacking_(self, rect: CGRect) -> CGRect;
    unsafe fn convertRectToScreen_(self, rect: CGRect) -> CGRect;
    unsafe fn convertRectFromScreen_(self, rect: CGRect) -> CGRect;

    // Accessing Edited Status
    unsafe fn isDocumentEdited(self) -> BOOL;
    unsafe fn setDocumentEdited_(self, documentEdited: BOOL);

    // Managing Titles
    unsafe fn title(self) -> id;
    unsafe fn setTitle_(self, title: id);
    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id);
    unsafe fn setTitleVisibility_(self, visibility: NSWindowTitleVisibility);
    unsafe fn setTitlebarAppearsTransparent_(self, transparent: BOOL);
    unsafe fn representedFilename(self) -> id;
    unsafe fn setRepresentedFilename_(self, filePath: id);
    unsafe fn representedURL(self) -> id;
    unsafe fn setRepresentedURL_(self, representedURL: id);

    // Accessing Screen Information
    unsafe fn screen(self) -> id;
    unsafe fn deepestScreen(self) -> id;
    unsafe fn displaysWhenScreenProfileChanges(self) -> BOOL;
    unsafe fn setDisplaysWhenScreenProfileChanges_(self, displaysWhenScreenProfileChanges: BOOL);

    // Moving Windows
    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: BOOL);
    unsafe fn setMovable_(self, movable: BOOL);
    unsafe fn center(self);

    // Closing Windows
    unsafe fn performClose_(self, sender: id);
    unsafe fn close(self);
    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: BOOL);

    // Minimizing Windows
    unsafe fn performMiniaturize_(self, sender: id);
    unsafe fn miniaturize_(self, sender: id);
    unsafe fn deminiaturize_(self, sender: id);
    unsafe fn miniwindowImage(self) -> id;
    unsafe fn setMiniwindowImage_(self, miniwindowImage: id);
    unsafe fn miniwindowTitle(self) -> id;
    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id);
    unsafe fn setAppearance(self, appearance: id);

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}

impl NSWindow for id {
    // Creating Windows

    unsafe fn initWithContentRect_styleMask_backing_defer_(
        self,
        rect: CGRect,
        style: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: BOOL,
    ) -> id {
        msg_send![self, initWithContentRect:rect
                                  styleMask:style.bits
                                    backing:backing as NSUInteger
                                      defer:defer]
    }

    unsafe fn initWithContentRect_styleMask_backing_defer_screen_(
        self,
        rect: CGRect,
        style: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: BOOL,
        screen: id,
    ) -> id {
        msg_send![self, initWithContentRect:rect
                                  styleMask:style.bits
                                    backing:backing as NSUInteger
                                      defer:defer
                                     screen:screen]
    }

    // Configuring Windows

    unsafe fn styleMask(self) -> NSWindowStyleMask {
        NSWindowStyleMask::from_bits_truncate(msg_send![self, styleMask])
    }

    unsafe fn setStyleMask_(self, styleMask: NSWindowStyleMask) {
        msg_send![self, setStyleMask:styleMask.bits]
    }

    unsafe fn toggleFullScreen_(self, sender: id) {
        msg_send![self, toggleFullScreen: sender]
    }

    unsafe fn worksWhenModal(self) -> BOOL {
        msg_send![self, worksWhenModal]
    }

    unsafe fn alphaValue(self) -> CGFloat {
        msg_send![self, alphaValue]
    }

    unsafe fn setAlphaValue_(self, windowAlpha: CGFloat) {
        msg_send![self, setAlphaValue: windowAlpha]
    }

    unsafe fn backgroundColor(self) -> id {
        msg_send![self, backgroundColor]
    }

    unsafe fn setBackgroundColor_(self, color: id) {
        msg_send![self, setBackgroundColor: color]
    }

    unsafe fn colorSpace(self) -> id {
        msg_send![self, colorSpace]
    }

    unsafe fn setColorSpace_(self, colorSpace: id) {
        msg_send![self, setColorSpace: colorSpace]
    }

    unsafe fn contentView(self) -> id {
        msg_send![self, contentView]
    }

    unsafe fn setContentView_(self, view: id) {
        msg_send![self, setContentView: view]
    }

    unsafe fn canHide(self) -> BOOL {
        msg_send![self, canHide]
    }

    unsafe fn setCanHide_(self, canHide: BOOL) {
        msg_send![self, setCanHide: canHide]
    }

    unsafe fn hidesOnDeactivate(self) -> BOOL {
        msg_send![self, hidesOnDeactivate]
    }

    unsafe fn setHidesOnDeactivate_(self, hideOnDeactivate: BOOL) {
        msg_send![self, setHidesOnDeactivate: hideOnDeactivate]
    }

    unsafe fn collectionBehavior(self) -> NSWindowCollectionBehavior {
        msg_send![self, collectionBehavior]
    }

    unsafe fn setCollectionBehavior_(self, collectionBehavior: NSWindowCollectionBehavior) {
        msg_send![self, setCollectionBehavior: collectionBehavior]
    }

    unsafe fn setOpaque_(self, opaque: BOOL) {
        msg_send![self, setOpaque: opaque]
    }

    unsafe fn hasShadow(self) -> BOOL {
        msg_send![self, hasShadow]
    }

    unsafe fn setHasShadow_(self, hasShadow: BOOL) {
        msg_send![self, setHasShadow: hasShadow]
    }

    unsafe fn invalidateShadow(self) {
        msg_send![self, invalidateShadow]
    }

    unsafe fn autorecalculatesContentBorderThicknessForEdge_(self, edge: CGRectEdge) -> BOOL {
        msg_send![self, autorecalculatesContentBorderThicknessForEdge: edge]
    }

    unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(
        self,
        autorecalculateContentBorderThickness: BOOL,
        edge: CGRectEdge,
    ) -> BOOL {
        msg_send![self, setAutorecalculatesContentBorderThickness:
                        autorecalculateContentBorderThickness forEdge:edge]
    }

    unsafe fn contentBorderThicknessForEdge_(self, edge: CGRectEdge) -> CGFloat {
        msg_send![self, contentBorderThicknessForEdge: edge]
    }

    unsafe fn setContentBorderThickness_forEdge_(self, borderThickness: CGFloat, edge: CGRectEdge) {
        msg_send![self, setContentBorderThickness:borderThickness forEdge:edge]
    }

    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn setDelegate_(self, delegate: id) {
        msg_send![self, setDelegate: delegate]
    }

    unsafe fn preventsApplicationTerminationWhenModal(self) -> BOOL {
        msg_send![self, preventsApplicationTerminationWhenModal]
    }

    unsafe fn setPreventsApplicationTerminationWhenModal_(self, flag: BOOL) {
        msg_send![self, setPreventsApplicationTerminationWhenModal: flag]
    }

    // TODO: Accessing Window Information

    // Getting Layout Information

    unsafe fn contentRectForFrameRect_styleMask_(
        self,
        windowFrame: CGRect,
        windowStyle: NSWindowStyleMask,
    ) -> CGRect {
        msg_send![self, contentRectForFrameRect:windowFrame styleMask:windowStyle.bits]
    }

    unsafe fn frameRectForContentRect_styleMask_(
        self,
        windowContentRect: CGRect,
        windowStyle: NSWindowStyleMask,
    ) -> CGRect {
        msg_send![self, frameRectForContentRect:windowContentRect styleMask:windowStyle.bits]
    }

    unsafe fn minFrameWidthWithTitle_styleMask_(
        self,
        windowTitle: id,
        windowStyle: NSWindowStyleMask,
    ) -> CGFloat {
        msg_send![self, minFrameWidthWithTitle:windowTitle styleMask:windowStyle.bits]
    }

    unsafe fn contentRectForFrameRect_(self, windowFrame: CGRect) -> CGRect {
        msg_send![self, contentRectForFrameRect: windowFrame]
    }

    unsafe fn frameRectForContentRect_(self, windowContent: CGRect) -> CGRect {
        msg_send![self, frameRectForContentRect: windowContent]
    }

    // Managing Windows

    unsafe fn drawers(self) -> id {
        msg_send![self, drawers]
    }

    unsafe fn windowController(self) -> id {
        msg_send![self, windowController]
    }

    unsafe fn setWindowController_(self, windowController: id) {
        msg_send![self, setWindowController: windowController]
    }

    // TODO: Managing Sheets

    // Sizing Windows

    unsafe fn frame(self) -> CGRect {
        msg_send![self, frame]
    }

    unsafe fn setFrameOrigin_(self, point: CGPoint) {
        msg_send![self, setFrameOrigin: point]
    }

    unsafe fn setFrameTopLeftPoint_(self, point: CGPoint) {
        msg_send![self, setFrameTopLeftPoint: point]
    }

    unsafe fn constrainFrameRect_toScreen_(self, frameRect: CGRect, screen: id) {
        msg_send![self, constrainFrameRect:frameRect toScreen:screen]
    }

    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: CGPoint) -> CGPoint {
        msg_send![self, cascadeTopLeftFromPoint: topLeft]
    }

    unsafe fn setFrame_display_(self, windowFrame: CGRect, display: BOOL) {
        msg_send![self, setFrame:windowFrame display:display]
    }

    unsafe fn setFrame_display_animate_(self, windowFrame: CGRect, display: BOOL, animate: BOOL) {
        msg_send![self, setFrame:windowFrame display:display animate: animate]
    }

    unsafe fn setFrame_displayViews_(self, windowFrame: CGRect, display: BOOL) {
        msg_send![self, setFrame:windowFrame displayViews:display]
    }

    unsafe fn aspectRatio(self) -> CGSize {
        msg_send![self, aspectRatio]
    }

    unsafe fn setAspectRatio_(self, aspectRatio: CGSize) {
        msg_send![self, setAspectRatio: aspectRatio]
    }

    unsafe fn minSize(self) -> CGSize {
        msg_send![self, minSize]
    }

    unsafe fn setMinSize_(self, minSize: CGSize) {
        msg_send![self, setMinSize: minSize]
    }

    unsafe fn maxSize(self) -> CGSize {
        msg_send![self, maxSize]
    }

    unsafe fn setMaxSize_(self, maxSize: CGSize) {
        msg_send![self, setMaxSize: maxSize]
    }

    unsafe fn performZoom_(self, sender: id) {
        msg_send![self, performZoom: sender]
    }

    unsafe fn zoom_(self, sender: id) {
        msg_send![self, zoom: sender]
    }

    unsafe fn resizeFlags(self) -> NSInteger {
        msg_send![self, resizeFlags]
    }

    unsafe fn showsResizeIndicator(self) -> BOOL {
        msg_send![self, showsResizeIndicator]
    }

    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: BOOL) {
        msg_send![self, setShowsResizeIndicator: showsResizeIndicator]
    }

    unsafe fn resizeIncrements(self) -> CGSize {
        msg_send![self, resizeIncrements]
    }

    unsafe fn setResizeIncrements_(self, resizeIncrements: CGSize) {
        msg_send![self, setResizeIncrements: resizeIncrements]
    }

    unsafe fn preservesContentDuringLiveResize(self) -> BOOL {
        msg_send![self, preservesContentDuringLiveResize]
    }

    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: BOOL) {
        msg_send![
            self,
            setPreservesContentDuringLiveResize: preservesContentDuringLiveResize
        ]
    }

    unsafe fn inLiveResize(self) -> BOOL {
        msg_send![self, inLiveResize]
    }

    // Sizing Content

    unsafe fn contentAspectRatio(self) -> CGSize {
        msg_send![self, contentAspectRatio]
    }

    unsafe fn setContentAspectRatio_(self, contentAspectRatio: CGSize) {
        msg_send![self, setContentAspectRatio: contentAspectRatio]
    }

    unsafe fn contentMinSize(self) -> CGSize {
        msg_send![self, contentMinSize]
    }

    unsafe fn setContentMinSize_(self, contentMinSize: CGSize) {
        msg_send![self, setContentMinSize: contentMinSize]
    }

    unsafe fn contentSize(self) -> CGSize {
        msg_send![self, contentSize]
    }

    unsafe fn setContentSize_(self, contentSize: CGSize) {
        msg_send![self, setContentSize: contentSize]
    }

    unsafe fn contentMaxSize(self) -> CGSize {
        msg_send![self, contentMaxSize]
    }

    unsafe fn setContentMaxSize_(self, contentMaxSize: CGSize) {
        msg_send![self, setContentMaxSize: contentMaxSize]
    }

    unsafe fn contentResizeIncrements(self) -> CGSize {
        msg_send![self, contentResizeIncrements]
    }

    unsafe fn setContentResizeIncrements_(self, contentResizeIncrements: CGSize) {
        msg_send![self, setContentResizeIncrements: contentResizeIncrements]
    }

    // Managing Window Visibility and Occlusion State

    unsafe fn isVisible(self) -> BOOL {
        msg_send![self, isVisible]
    }

    unsafe fn occlusionState(self) -> NSWindowOcclusionState {
        msg_send![self, occlusionState]
    }

    // Managing Window Layers

    unsafe fn orderOut_(self, sender: id) {
        msg_send![self, orderOut: sender]
    }

    unsafe fn orderBack_(self, sender: id) {
        msg_send![self, orderBack: sender]
    }

    unsafe fn orderFront_(self, sender: id) {
        msg_send![self, orderFront: sender]
    }

    unsafe fn orderFrontRegardless(self) {
        msg_send![self, orderFrontRegardless]
    }

    unsafe fn orderFrontWindow_relativeTo_(
        self,
        ordering_mode: NSWindowOrderingMode,
        other_window_number: NSInteger,
    ) {
        msg_send![self, orderWindow:ordering_mode relativeTo:other_window_number]
    }

    unsafe fn level(self) -> NSInteger {
        msg_send![self, level]
    }

    unsafe fn setLevel_(self, level: NSInteger) {
        msg_send![self, setLevel: level]
    }

    // Managing Key Status

    unsafe fn isKeyWindow(self) -> BOOL {
        msg_send![self, isKeyWindow]
    }

    unsafe fn canBecomeKeyWindow(self) -> BOOL {
        msg_send![self, canBecomeKeyWindow]
    }

    unsafe fn makeKeyWindow(self) {
        msg_send![self, makeKeyWindow]
    }

    unsafe fn makeKeyAndOrderFront_(self, sender: id) {
        msg_send![self, makeKeyAndOrderFront: sender]
    }

    // Managing Main Status

    unsafe fn canBecomeMainWindow(self) -> BOOL {
        msg_send![self, canBecomeMainWindow]
    }

    unsafe fn makeMainWindow(self) {
        msg_send![self, makeMainWindow]
    }

    // Managing Toolbars

    unsafe fn toolbar(self) -> id /* NSToolbar */ {
        msg_send![self, toolbar]
    }

    unsafe fn toolbarStyle(self) -> NSWindowToolbarStyle {
        msg_send![self, toolbarStyle]
    }

    unsafe fn setToolbar_(self, toolbar: id /* NSToolbar */) {
        msg_send![self, setToolbar: toolbar]
    }

    unsafe fn setToolbarStyle_(self, toolbarStyle: NSWindowToolbarStyle) {
        msg_send![self, setToolbarStyle: toolbarStyle]
    }

    unsafe fn runToolbarCustomizationPalette(self, sender: id) {
        msg_send![self, runToolbarCustomizationPalette: sender]
    }

    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles

    // Managing Title Bars

    unsafe fn standardWindowButton_(self, windowButtonKind: NSWindowButton) -> id {
        msg_send![self, standardWindowButton: windowButtonKind]
    }

    // Managing Window Tabs
    unsafe fn allowsAutomaticWindowTabbing(_: Self) -> BOOL {
        msg_send![class!(NSWindow), allowsAutomaticWindowTabbing]
    }

    unsafe fn setAllowsAutomaticWindowTabbing_(_: Self, allowsAutomaticWindowTabbing: BOOL) {
        msg_send![
            class!(NSWindow),
            setAllowsAutomaticWindowTabbing: allowsAutomaticWindowTabbing
        ]
    }

    unsafe fn tabbingIdentifier(self) -> id {
        msg_send![self, tabbingIdentifier]
    }

    unsafe fn tabbingMode(self) -> NSWindowTabbingMode {
        msg_send!(self, tabbingMode)
    }

    unsafe fn setTabbingMode_(self, tabbingMode: NSWindowTabbingMode) {
        msg_send![self, setTabbingMode: tabbingMode]
    }

    unsafe fn addTabbedWindow_ordered_(self, window: id, ordering_mode: NSWindowOrderingMode) {
        msg_send![self, addTabbedWindow:window ordered: ordering_mode]
    }

    unsafe fn toggleTabBar_(self, sender: id) {
        msg_send![self, toggleTabBar: sender]
    }
    // TODO: Managing Tooltips
    // TODO: Handling Events

    // Managing Responders

    unsafe fn initialFirstResponder(self) -> id {
        msg_send![self, initialFirstResponder]
    }

    unsafe fn firstResponder(self) -> id {
        msg_send![self, firstResponder]
    }

    unsafe fn setInitialFirstResponder_(self, responder: id) {
        msg_send![self, setInitialFirstResponder: responder]
    }

    unsafe fn makeFirstResponder_(self, responder: id) -> BOOL {
        msg_send![self, makeFirstResponder: responder]
    }

    // TODO: Managing the Key View Loop

    // Handling Keyboard Events

    unsafe fn keyDown_(self, event: id) {
        msg_send![self, keyDown: event]
    }

    // Handling Mouse Events

    unsafe fn acceptsMouseMovedEvents(self) -> BOOL {
        msg_send![self, acceptsMouseMovedEvents]
    }

    unsafe fn ignoresMouseEvents(self) -> BOOL {
        msg_send![self, ignoresMouseEvents]
    }

    unsafe fn setIgnoresMouseEvents_(self, ignoreMouseEvents: BOOL) {
        msg_send![self, setIgnoresMouseEvents: ignoreMouseEvents]
    }

    unsafe fn mouseLocationOutsideOfEventStream(self) -> CGPoint {
        msg_send![self, mouseLocationOutsideOfEventStream]
    }

    unsafe fn setAcceptsMouseMovedEvents_(self, acceptMouseMovedEvents: BOOL) {
        msg_send![self, setAcceptsMouseMovedEvents: acceptMouseMovedEvents]
    }

    unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(
        _: Self,
        point: CGPoint,
        windowNumber: NSInteger,
    ) -> NSInteger {
        msg_send![class!(NSWindow), windowNumberAtPoint:point belowWindowWithWindowNumber:windowNumber]
    }

    // Converting Coordinates

    unsafe fn backingScaleFactor(self) -> CGFloat {
        msg_send![self, backingScaleFactor]
    }

    unsafe fn backingAlignedRect_options_(
        self,
        rect: CGRect,
        options: NSAlignmentOptions,
    ) -> CGRect {
        msg_send![self, backingAlignedRect:rect options:options]
    }

    unsafe fn convertRectFromBacking_(self, rect: CGRect) -> CGRect {
        msg_send![self, convertRectFromBacking: rect]
    }

    unsafe fn convertRectToBacking_(self, rect: CGRect) -> CGRect {
        msg_send![self, convertRectToBacking: rect]
    }

    unsafe fn convertRectToScreen_(self, rect: CGRect) -> CGRect {
        msg_send![self, convertRectToScreen: rect]
    }

    unsafe fn convertRectFromScreen_(self, rect: CGRect) -> CGRect {
        msg_send![self, convertRectFromScreen: rect]
    }

    // Accessing Edited Status

    unsafe fn isDocumentEdited(self) -> BOOL {
        msg_send![self, isDocumentEdited]
    }

    unsafe fn setDocumentEdited_(self, documentEdited: BOOL) {
        msg_send![self, setDocumentEdited: documentEdited]
    }

    // Managing Titles

    unsafe fn title(self) -> id {
        msg_send![self, title]
    }

    unsafe fn setTitle_(self, title: id) {
        msg_send![self, setTitle: title]
    }

    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id) {
        msg_send![self, setTitleWithRepresentedFilename: filePath]
    }

    unsafe fn setTitleVisibility_(self, visibility: NSWindowTitleVisibility) {
        msg_send![self, setTitleVisibility: visibility]
    }

    unsafe fn setTitlebarAppearsTransparent_(self, transparent: BOOL) {
        msg_send![self, setTitlebarAppearsTransparent: transparent]
    }

    unsafe fn representedFilename(self) -> id {
        msg_send![self, representedFilename]
    }

    unsafe fn setRepresentedFilename_(self, filePath: id) {
        msg_send![self, setRepresentedFilename: filePath]
    }

    unsafe fn representedURL(self) -> id {
        msg_send![self, representedURL]
    }

    unsafe fn setRepresentedURL_(self, representedURL: id) {
        msg_send![self, setRepresentedURL: representedURL]
    }

    // Accessing Screen Information

    unsafe fn screen(self) -> id {
        msg_send![self, screen]
    }

    unsafe fn deepestScreen(self) -> id {
        msg_send![self, deepestScreen]
    }

    unsafe fn displaysWhenScreenProfileChanges(self) -> BOOL {
        msg_send![self, displaysWhenScreenProfileChanges]
    }

    unsafe fn setDisplaysWhenScreenProfileChanges_(self, displaysWhenScreenProfileChanges: BOOL) {
        msg_send![
            self,
            setDisplaysWhenScreenProfileChanges: displaysWhenScreenProfileChanges
        ]
    }

    // Moving Windows

    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: BOOL) {
        msg_send![
            self,
            setMovableByWindowBackground: movableByWindowBackground
        ]
    }

    unsafe fn setMovable_(self, movable: BOOL) {
        msg_send![self, setMovable: movable]
    }

    unsafe fn center(self) {
        msg_send![self, center]
    }

    // Closing Windows

    unsafe fn performClose_(self, sender: id) {
        msg_send![self, performClose: sender]
    }

    unsafe fn close(self) {
        msg_send![self, close]
    }

    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: BOOL) {
        msg_send![self, setReleasedWhenClosed: releasedWhenClosed]
    }

    // Minimizing Windows

    unsafe fn performMiniaturize_(self, sender: id) {
        msg_send![self, performMiniaturize: sender]
    }

    unsafe fn miniaturize_(self, sender: id) {
        msg_send![self, miniaturize: sender]
    }

    unsafe fn deminiaturize_(self, sender: id) {
        msg_send![self, deminiaturize: sender]
    }

    unsafe fn miniwindowImage(self) -> id {
        msg_send![self, miniwindowImage]
    }

    unsafe fn setMiniwindowImage_(self, miniwindowImage: id) {
        msg_send![self, setMiniwindowImage: miniwindowImage]
    }

    unsafe fn miniwindowTitle(self) -> id {
        msg_send![self, miniwindowTitle]
    }

    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id) {
        msg_send![self, setMiniwindowTitle: miniwindowTitle]
    }

    unsafe fn setAppearance(self, appearance: id) {
        msg_send![self, setAppearance: appearance]
    }

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}
*/