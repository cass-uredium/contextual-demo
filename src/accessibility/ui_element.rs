use objc2_application_services::{AXError, AXUIElement};
use objc2_core_foundation::{CFArray, CFRetained, CFString, CFType};

use super::{AXErrorExt, cf_call_get, cf_call_owned};

pub trait AXUIElementExt {
    fn pid(&self) -> Result<i32, AXError>;

    fn attribute_names(&self) -> Result<CFRetained<CFArray<CFString>>, AXError>;

    fn attribute_value(&self, attribute: &CFString) -> Result<CFRetained<CFType>, AXError>;

    fn parameterized_attribute_names(&self) -> Result<CFRetained<CFArray<CFString>>, AXError>;

    fn parameterized_attribute_value(
        &self,
        parameterized_attribute: &CFString,
        parameter: &CFType,
    ) -> Result<CFRetained<CFType>, AXError>;
}

impl AXUIElementExt for AXUIElement {
    fn pid(&self) -> Result<i32, AXError> {
        unsafe { cf_call_get(|pid| self.pid(pid).into_result()) }
    }

    fn attribute_names(&self) -> Result<CFRetained<CFArray<CFString>>, AXError> {
        let array =
            unsafe { cf_call_owned(|names| self.copy_attribute_names(names).into_result()) }?;
        Ok(unsafe { CFRetained::cast_unchecked(array) })
    }

    fn attribute_value(&self, attribute: &CFString) -> Result<CFRetained<CFType>, AXError> {
        unsafe { cf_call_owned(|value| self.copy_attribute_value(attribute, value).into_result()) }
    }

    fn parameterized_attribute_names(&self) -> Result<CFRetained<CFArray<CFString>>, AXError> {
        let array = unsafe {
            cf_call_owned(|names| self.copy_parameterized_attribute_names(names).into_result())
        }?;
        Ok(unsafe { CFRetained::cast_unchecked(array) })
    }

    fn parameterized_attribute_value(
        &self,
        parameterized_attribute: &CFString,
        parameter: &CFType,
    ) -> Result<CFRetained<CFType>, AXError> {
        unsafe {
            cf_call_owned(|value| {
                self.copy_parameterized_attribute_value(parameterized_attribute, parameter, value)
                    .into_result()
            })
        }
    }
}
