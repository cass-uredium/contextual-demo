macro_rules! define_notifications {
    (
        $(
            $(#[$attr:meta])*
            $name:ident
        ),* $(,)?
    ) => {
        $(
            $(#[$attr])*
            #[expect(non_upper_case_globals)]
            pub const ${concat(k, $name, Notification)}: &'static str = stringify!($name);
        )*
    };
}

define_notifications! {
    // focus notifications
    AXMainWindowChanged,
    AXFocusedWindowChanged,
    AXFocusedUIElementChanged,

    // application notifications
    AXApplicationActivated,
    AXApplicationDeactivated,
    AXApplicationHidden,
    AXApplicationShown,

    // window notifications
    AXWindowCreated,
    AXWindowMoved,
    AXWindowResized,
    AXWindowMiniaturized,
    AXWindowDeminiaturized,

    // new drawer, sheet, and help notifications
    AXDrawerCreated,
    AXSheetCreated,
    AXHelpTagCreated,

    // element notifications
    AXValueChanged,
    AXUIElementDestroyed,
    AXElementBusyChanged,

    // menu notifications
    AXMenuOpened,
    AXMenuClosed,
    AXMenuItemSelected,

    // table/outline notifications
    AXRowCountChanged,

    // outline notifications
    AXRowExpanded,
    AXRowCollapsed,

    // cell-based table notifications
    AXSelectedCellsChanged,

    // layout area notifications
    AXUnitsChanged,
    AXSelectedChildrenMoved,

    // other notifications
    AXSelectedChildrenChanged,
    AXResized,
    AXMoved,
    AXCreated,
    AXSelectedRowsChanged,
    AXSelectedColumnsChanged,
    AXTitleChanged,
    AXLayoutChanged,
    AXAnnouncementRequested,
    AXUIElementsKey,
    AXPriorityKey,
    AXAnnouncementKey,
    AXUIElementTitleKey,
}
