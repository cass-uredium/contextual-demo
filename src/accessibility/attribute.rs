macro_rules! define_attributes {
    (
        $(
            $(#[$attr:meta])*
            $name:ident
        ),* $(,)?
    ) => {
        $(
            $(#[$attr])*
            #[expect(non_upper_case_globals)]
            pub const ${concat(k, $name, Attribute)}: &'static str = stringify!($name);
        )*
    };
}

define_attributes! {
    // informational attributes
    AXRole,
    AXSubrole,
    AXRoleDescription,
    AXTitle,
    AXDescription,
    AXHelp,

    // hierarchy or relationship attributes
    AXParent,
    AXChildren,
    AXSelectedChildren,
    AXVisibleChildren,
    AXWindow,
    AXTopLevelUIElement,
    AXTitleUIElement,
    AXServesAsTitleForUIElements,
    AXLinkedUIElements,
    AXSharedFocusElements,

    // visual state attributes
    AXEnabled,
    AXFocused,
    AXPosition,
    AXSize,

    // value attributes
    AXValue,
    AXValueDescription,
    AXMinValue,
    AXMaxValue,
    AXValueIncrement,
    AXValueWraps,
    AXAllowedValues,

    // text-specific attributes
    AXSelectedText,
    AXSelectedTextRange,
    AXSelectedTextRanges,
    AXVisibleCharacterRange,
    AXNumberOfCharacters,
    AXSharedTextUIElements,
    AXSharedCharacterRange,

    // window, sheet, or drawer-specific attributes
    AXMain,
    AXMinimized,
    AXCloseButton,
    AXZoomButton,
    AXMinimizeButton,
    AXToolbarButton,
    AXFullScreenButton,
    AXProxy,
    AXGrowArea,
    AXModal,
    AXDefaultButton,
    AXCancelButton,

    // menu or menu item-specific attributes
    AXMenuItemCmdChar,
    AXMenuItemCmdVirtualKey,
    AXMenuItemCmdGlyph,
    AXMenuItemCmdModifiers,
    AXMenuItemMarkChar,
    AXMenuItemPrimaryUIElement,

    // application element-specific attributes
    AXMenuBar,
    AXWindows,
    AXFrontmost,
    AXHidden,
    AXMainWindow,
    AXFocusedWindow,
    AXFocusedUIElement,
    AXExtrasMenuBar,

    // date/time-specific attributes
    AXHourField,
    AXMinuteField,
    AXSecondField,
    AXAMPMField,
    AXDayField,
    AXMonthField,
    AXYearField,

    // table, outline, or browser-specific attributes
    AXRows,
    AXVisibleRows,
    AXSelectedRows,
    AXColumns,
    AXVisibleColumns,
    AXSelectedColumns,
    AXSortDirection,
    AXColumnHeaderUIElements,
    AXIndex,
    AXDisclosing,
    AXDisclosedRows,
    AXDisclosedByRow,

    // matte-specific attributes
    AXMatteHole,
    AXMatteContentUIElement,

    // ruler-specific attributes
    AXMarkerUIElements,
    AXUnits,
    AXUnitDescription,
    AXMarkerType,
    AXMarkerTypeDescription,

    // miscellaneous or role-specific attributes
    AXHorizontalScrollBar,
    AXVerticalScrollBar,
    AXOrientation,
    AXHeader,
    AXEdited,
    AXTabs,
    AXOverflowButton,
    AXFilename,
    AXExpanded,
    AXSelected,
    AXSplitters,
    AXContents,
    AXNextContents,
    AXPreviousContents,
    AXDocument,
    AXIncrementor,
    AXDecrementButton,
    AXIncrementButton,
    AXColumnTitle,
    AXURL,
    AXLabelUIElements,
    AXLabelValue,
    AXShownMenuUIElement,
    AXIsApplicationRunning,
    AXFocusedApplication,
    AXElementBusy,
    AXAlternateUIVisible,
}
