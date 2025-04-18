use objc2_application_services::{AXError, AXValue, AXValueType};
use objc2_core_foundation::{CFRange, CGPoint, CGRect, CGSize};

use super::cf_call_get;

pub trait AXValueExt {
    #[doc(alias = "AXValueGetValue")]
    fn get_value<T>(&self) -> Option<T>
    where
        T: AXValueTypeMarker + Clone;
}

impl AXValueExt for AXValue {
    fn get_value<T>(&self) -> Option<T>
    where
        T: AXValueTypeMarker + Clone,
    {
        unsafe {
            cf_call_get(|value_ptr| {
                self.value(T::VALUE_TYPE, value_ptr.cast())
                    .then_some(())
                    .ok_or(())
            })
        }
        .ok()
    }
}

pub trait AXValueTypeMarker {
    const VALUE_TYPE: AXValueType;
}

macro_rules! impl_value_types {
    ($($ident:ident),*) => {
        $(
            impl AXValueTypeMarker for $ident {
                const VALUE_TYPE: AXValueType = AXValueType::$ident;
            }
        )*
    };
}

impl_value_types!(CGPoint, CGSize, CGRect, CFRange, AXError);
