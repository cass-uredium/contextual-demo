use std::sync::mpsc;

use objc2_app_kit::NSRunningApplication;
use objc2_application_services::{AXError, AXIsProcessTrusted, AXUIElement, AXValue};
use objc2_core_foundation::{CFRetained, CFString, CGRect};

use crate::accessibility::{AXUIElementExt, AXValueExt, attribute};
use crate::event::Event;

pub struct Controller {
    pid: libc::pid_t,
    system_wide: CFRetained<AXUIElement>,
}

impl Controller {
    pub fn new() -> Self {
        assert!(unsafe { AXIsProcessTrusted() });

        let application = unsafe { NSRunningApplication::currentApplication() };
        let pid = unsafe { application.processIdentifier() };

        let system_wide = unsafe { AXUIElement::new_system_wide() };

        Self { pid, system_wide }
    }

    pub fn focused_app(&self) -> Result<CFRetained<AXUIElement>, AXError> {
        self.system_wide
            .attribute_value(&CFString::from_static_str(
                attribute::kAXFocusedApplicationAttribute,
            ))
            .map(|value| value.downcast().unwrap())
    }

    pub fn focused_element(
        &self,
        element: Option<&AXUIElement>,
    ) -> Result<CFRetained<AXUIElement>, AXError> {
        element
            .unwrap_or(&self.system_wide)
            .attribute_value(&CFString::from_static_str(
                attribute::kAXFocusedUIElementAttribute,
            ))
            .map(|value| value.downcast().unwrap())
    }

    pub fn selected_text(&self, element: &AXUIElement) -> Result<CFRetained<CFString>, AXError> {
        let value = element.attribute_value(&CFString::from_static_str(
            attribute::kAXSelectedTextAttribute,
        ))?;
        Ok(value.downcast().unwrap())
    }

    pub fn selected_text_bounds(&self, element: &AXUIElement) -> Result<Option<CGRect>, AXError> {
        // TODO propagate size
        let range = element.attribute_value(&CFString::from_static_str(
            attribute::kAXSelectedTextRangeAttribute,
        ))?;
        let bounds = element
            .parameterized_attribute_value(&CFString::from_static_str("AXBoundsForRange"), &range)?
            .downcast::<AXValue>()
            .unwrap()
            .get_value::<CGRect>();
        Ok(bounds)
    }

    pub fn run(&self, event_tx: mpsc::Sender<Event>) {
        todo!()
    }
}
