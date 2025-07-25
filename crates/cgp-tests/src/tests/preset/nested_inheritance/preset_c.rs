#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::tests::preset::basic::components::{BarGetterComponent, BarTypeProviderComponent};

    cgp_preset! {
        NestedPresetC {
            BarTypeProviderComponent: UseType<()>,
            BarGetterComponent: UseField<symbol!("bar")>,
        }
    }

    pub trait CheckDelegatesForNestedPresetC:
        DelegateComponent<BarTypeProviderComponent, Delegate = UseType<()>>
        + DelegateComponent<BarGetterComponent, Delegate = UseField<symbol!("bar")>>
    {
    }

    impl CheckDelegatesForNestedPresetC for NestedPresetC::Provider {}
}
