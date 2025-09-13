macro_rules! impl_unit_conversions {
    (
        $(
            $unit:ident {
                $( $set_name:ident, $get_name:ident => $factor:expr $(=> offset= $offset:expr)? ),+ $(,)?
                $(=> constants {
                    $( $const_name:ident , $const_value:expr ),* $(,)?
                })?
            }
        )+ $(,)?
    ) => {
        $(
            impl $unit {
                $($(
                    pub const $const_name: Self = SiValue::new($const_value);
                )*)?
                
                $(
                    pub fn $set_name(value: f64) -> Self {
                        SiValue::new((value $(+ $offset)? ) * $factor)
                    }

                    pub fn $get_name(&self) -> f64 {
                        self.value / $factor $( - $offset )?
                    }
                )+
            }
        )+
    };
}

macro_rules! new_types {
    (
        $(
            $name:ident, $symbol:expr => kg^$mass:ty, m^$length:ty, s^$time:ty, A^$current:ty, K^$kelvin:ty, mol^$mol:ty, cd^$candela:ty
        ),* $(,)?
    ) => {
        $(
            #[doc = concat!("SI unit type for ", stringify!($name), ".")]
            pub type $name = SiValue<$length, $mass, $time, $current, $kelvin, $mol, $candela>;
        )*
    
        impl<L, M, T, A, K, Mol, Cd> SiValue<L, M, T, A, K, Mol, Cd>
        where
            L: Integer,
            M: Integer,
            T: Integer,
            A: Integer,
            K: Integer,
            Mol: Integer,
            Cd: Integer,
        {
            fn unit_symbol(&self) -> Option<String> {
                $(
                    if new_types!($length, $mass, $time, $current, $kelvin, $mol, $candela) {
                        if let Some::<&str>(sym) = $symbol {
                            return Some(sym.to_string());
                        } 
                    }
                )*
                None
            }
        }
    };
    ($l:ty, $m:ty, $t:ty, $a:ty, $k:ty, $mol:ty, $cd:ty) => {
        L::to_i32() == <$l>::to_i32() &&
        M::to_i32() == <$m>::to_i32() &&
        T::to_i32() == <$t>::to_i32() &&
        A::to_i32() == <$a>::to_i32() &&
        K::to_i32() == <$k>::to_i32() &&
        Mol::to_i32() == <$mol>::to_i32() &&
        Cd::to_i32() == <$cd>::to_i32()
    };
}

pub(crate) use impl_unit_conversions;
pub(crate) use new_types;