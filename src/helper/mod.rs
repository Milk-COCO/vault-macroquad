pub mod roll;
pub mod chain;
pub mod watch;
pub mod viewport;

macro_rules! define_with_methods {
    ($(,)*) => {};
    
    ($(,)* $field:ident : $box:ident < $dyn: ident $trait:ident > $($other: tt)*) => {
        ::paste::paste! {
            pub fn [<with_ $field>](self, val: impl $trait + 'static) -> Self {
                use std::marker::PhantomData;
                let _: PhantomData<$box<$dyn $trait>> = PhantomData;
                Self {
                    $field: $box::new(val),
                    ..self
                }
            }
        }
    
        define_with_methods!($($other)*);
    };
    
    // 为什么把这个Option搞个元变量是因为这样idea才会有高亮。看着爽一点ww
    ($(,)* $field:ident : $option:ident <$type:ty> $($other: tt)*) => {
        ::paste::paste! {
            pub fn [<with_ $field>](self, val: $type) -> Self {
                Self {
                    $field: Some(val),
                    ..self
                }
            }

            pub fn [<without_ $field>](self) -> Self {
                Self {
                    $field: None,
                    ..self
                }
            }

            pub fn [<with_opt_ $field>](self, val: $option<$type>)  -> Self {
                Self {
                    $field: val,
                    ..self
                }
            }
        }
    
        define_with_methods!($($other)*);
    };
    
    ($(,)* $field:ident : enum $type:ident {
        $($v_field:ident : $var:ident $(,)? )+
    } $($other: tt)*) => {
        ::paste::paste! {
            $(
            pub fn [<with_ $v_field>](self) -> Self {
                Self {
                    $field: $type::$var,
                    ..self
                }
            }
            )+
            pub fn [<with_ $field>](self, val: $type) -> Self {
                Self {
                    $field: val,
                    ..self
                }
            }
        }
        
        define_with_methods!($($other)*);
    };
    
    ($(,)* $field:ident : $type:ident $($other: tt)*) => {
        ::paste::paste! {
            pub fn [<with_ $field>](self, val: $type) -> Self {
                Self {
                    $field: val,
                    ..self
                }
            }
        }
    
        define_with_methods!($($other)*);
    };
    
    ($(,)* $field:ident : ($($type:tt),+ $(,)?) $($other: tt)*) => {
        ::paste::paste! {
            pub fn [<with_ $field>](self, val: ($($type),+)) -> Self {
                Self {
                    $field: val,
                    ..self
                }
            }
        }
    
        define_with_methods!($($other)*);
    };
}

pub(crate) use define_with_methods;

macro_rules! define_fix_methods {
    ($(,)*) => {};
    
    
    ($(,)* $field:ident : $box:ident < $dyn: ident $trait:ident > $($other: tt)*) => {
        ::paste::paste! {
            pub fn [<$field>](&mut self, val: impl $trait + 'static) -> &mut Self {
                use std::marker::PhantomData;
                let _: PhantomData<$box<$dyn $trait>> = PhantomData;
                self.$field = $box::new(val);
                self
            }
        }
    
        define_fix_methods!($($other)*);
    };
    
    // 为什么把这个Option搞个元变量是因为这样idea才会有高亮。看着爽一点ww
    ($(,)* $field:ident : $option:ident <$type:ty> $($other: tt)*) => {
        ::paste::paste! {
            pub fn [<$field>](&mut self, val: $type) -> &mut Self {
                self.$field = Some(val);
                self
            }

            pub fn [<non_ $field>](&mut self) -> &mut Self {
                self.$field = None;
                self
            }

            pub fn [<set_ $field>](&mut self, val: $option<$type>) -> &mut Self {
                self.$field = val;
                self
            }
        }
    
        define_fix_methods!($($other)*);
    };
    
    ($(,)* $field:ident : enum $type:ident {
        $($v_field:ident : $var:ident $(,)? )+
    } $($other: tt)*) => {
        ::paste::paste! {
            $(
            pub fn [<$v_field>](&mut self) -> &mut Self {
                self.$field = $type :: $var;
                self
            }
            )+
            pub fn [<$field>](&mut self, val: $type) -> &mut Self {
                self.$field = val;
                self
            }
        }
        
        define_fix_methods!($($other)*);
    };
    
    ($(,)* $field:ident : $type:ident $($other: tt)*) => {
        ::paste::paste! {
            pub fn [<$field>](&mut self, val: $type) -> &mut Self {
                self.$field = val;
                self
            }
        }
    
        define_fix_methods!($($other)*);
    };
    
    
    ($(,)* $field:ident : ($($type:tt),+ $(,)?) $($other: tt)*) => {
        ::paste::paste! {
            pub fn [<$field>](&mut self, val: ($($type),+)) -> &mut Self {
                self.$field = val;
                self
            }
        }
    
        define_fix_methods!($($other)*);
    };
}

pub(crate) use define_fix_methods;


macro_rules! define_with_and_fix_methods {
    ($($some:tt)*) => {
        $crate::helper::define_with_methods!($($some)*);
        $crate::helper::define_fix_methods!($($some)*);
    };
}

pub(crate) use define_with_and_fix_methods;
