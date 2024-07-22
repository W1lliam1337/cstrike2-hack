pub mod engine_client;
use paste::paste;

/// This macro is used to define a static reference to a specific interface provided by the game engine.
/// It initializes the interface lazily, meaning it will only be created when the function is first called.
///
/// # Parameters
///
/// - `$name`: The identifier for the interface. This will be used to name the static reference and the function.
/// - `$module_fn`: The name of the function in the `crate::cs2::modules` module that returns the game engine module.
/// - `$interface_name`: The name of the interface to be retrieved from the game engine module.
///
/// # Return
///
/// This macro does not return a value. Instead, it defines a static reference and a function as per the provided parameters.
///
/// The static reference is named `INTERFACE_$name:upper` and is of type `once_cell::sync::Lazy<super::interfaces::$name::Interface>`.
/// It is initialized using the `once_cell::sync::Lazy::new` function, which creates a new lazy-initialized value.
/// Inside the closure, the interface pointer is obtained by calling the `$module_fn` function, retrieving the interface using the `$interface_name`,
/// and then creating a new instance of `super::interfaces::$name::Interface` using the obtained interface pointer.
///
/// The function named `$name` is also defined, which returns a reference to the static reference `INTERFACE_$name:upper`.
macro_rules! define_interface {
    ($name:ident, $module_fn:ident, $interface_name:expr) => {
        paste! {
            static [<INTERFACE_ $name:upper>]: once_cell::sync::Lazy<super::interfaces::$name::Interface> = once_cell::sync::Lazy::new(|| {
                let interface_ptr = crate::cs2::modules::$module_fn().get_interface($interface_name)
                    .expect(concat!("Failed to find ", $interface_name));
                super::interfaces::$name::Interface::new(interface_ptr)
            });

            pub fn $name() -> &'static super::interfaces::$name::Interface {
                &[<INTERFACE_ $name:upper>]
            }
        }
    };
}

define_interface!(engine_client, engine2, "Source2EngineToClient001");
