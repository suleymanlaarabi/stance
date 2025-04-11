#[macro_export]
macro_rules! activation_state {
    ($var:ident) => {
        #[derive(bevy::prelude::States, Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $var {
            Enable,
            Disabled,
        }
    };
}

#[macro_export]
macro_rules! activations_states {
    ($($name:ident),*) => {
        $(
            activation_state!($name);
        )*
    };
}
