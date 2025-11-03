/// Defines a COM interface
#[macro_export]
macro_rules! com_interface {
    (
        $(#[$meta: meta])*
        $vis: vis abstract $struct_name: ident($vtable_name: ident)$(: $first_super_type: ident($first_super_name: ident))* {
            const IID = $iid1: literal - $iid2: literal - $iid3: literal - $iid4: literal - $iid5: literal;

            $(
                $(#[$fn_meta: meta])*
                fn $fn_name: ident(&mut self$(, $parameter_name: ident: $parameter_type: ty)*)$( -> $return_type: ty)*;
            )*
        }
    ) => {
        $(#[$meta])*
        #[repr(C)]
        $vis struct $struct_name {
            /// The table with function calls for this interface
            pub vtable: Option<::std::ptr::NonNull<$vtable_name>>,
        }

        #[doc = ::std::concat!("Virtual function call table for [`", ::std::stringify!($struct_name), "`]")]
        #[repr(C)]
        $vis struct $vtable_name {
            $(
                #[allow(missing_docs)]
                pub $first_super_name: <$first_super_type as $crate::ComInterface>::VTable,
            )*
            $(
                $(#[$fn_meta])*
                pub $fn_name: Option<extern "system" fn(this: *mut $struct_name, $($parameter_name: $parameter_type),*)$( -> $return_type)*>,
            )*
        }

        impl $struct_name {
            #[doc = ::std::concat!("Get the v-table for [`", ::std::stringify!($struct_name), "`]")]
            pub fn vtable(&self) -> &$vtable_name {
                unsafe { self.vtable.unwrap().as_ref() }
            }

            $(
                $(#[$fn_meta])*
                pub fn $fn_name(&mut self, $($parameter_name: $parameter_type),*) $(-> $return_type)* {
                    self.vtable().$fn_name.unwrap()(self as *mut Self as *mut _, $($parameter_name),*)
                }
            )*
        }

        impl $crate::utility::ComInterface for $struct_name {
            type VTable = $vtable_name;

            const IID: $crate::IID = $crate::IID {
                data1: $iid1,
                data2: $iid2,
                data3: $iid3,
                data4: {
                    let iid4 = ($iid4 as u16).to_be_bytes();
                    let iid5 = ($iid5 as u64).to_be_bytes();

                    [iid4[0], iid4[1], iid5[2], iid5[3], iid5[4], iid5[5], iid5[6], iid5[7]]
                },
            };
        }

        $(
            impl std::ops::Deref for $struct_name {
                type Target = $first_super_type;

                fn deref(&self) -> &Self::Target {
                    unsafe { std::mem::transmute(self) }
                }
            }

            impl std::ops::DerefMut for $struct_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { std::mem::transmute(self) }
                }
            }
        )*
    };
}

/// Defines an immutable COM interface
#[macro_export]
macro_rules! immut_com_interface {
    (
        $(#[$meta: meta])*
        $vis: vis abstract $struct_name: ident($vtable_name: ident)$(: $first_super_type: ident($first_super_name: ident))* {
            const IID = $iid1: literal - $iid2: literal - $iid3: literal - $iid4: literal - $iid5: literal;

            $(
                $(#[$fn_meta: meta])*
                fn $fn_name: ident(&self$(, $parameter_name: ident: $parameter_type: ty)*)$( -> $return_type: ty)*;
            )*
        }
    ) => {
        $(#[$meta])*
        #[repr(C)]
        $vis struct $struct_name {
            /// The table with function calls for this interface
            pub vtable: Option<::std::ptr::NonNull<$vtable_name>>,
        }

        #[doc = ::std::concat!("Virtual function call table for [`", ::std::stringify!($struct_name), "`]")]
        #[repr(C)]
        $vis struct $vtable_name {
            $(
                #[allow(missing_docs)]
                pub $first_super_name: <$first_super_type as $crate::ComInterface>::VTable,
            )*
            $(
                $(#[$fn_meta])*
                pub $fn_name: Option<extern "system" fn(this: *mut $struct_name, $($parameter_name: $parameter_type),*)$( -> $return_type)*>,
            )*
        }

        impl $struct_name {
            #[doc = ::std::concat!("Get the v-table for [`", ::std::stringify!($struct_name), "`]")]
            pub fn vtable(&self) -> &$vtable_name {
                unsafe { self.vtable.unwrap().as_ref() }
            }

            $(
                $(#[$fn_meta])*
                pub fn $fn_name(&self, $($parameter_name: $parameter_type),*) $(-> $return_type)* {
                    self.vtable().$fn_name.unwrap()(self as *const Self as *mut _, $($parameter_name),*)
                }
            )*
        }

        impl $crate::utility::ComInterface for $struct_name {
            type VTable = $vtable_name;

            const IID: $crate::IID = $crate::IID {
                data1: $iid1,
                data2: $iid2,
                data3: $iid3,
                data4: {
                    let iid4 = ($iid4 as u16).to_be_bytes();
                    let iid5 = ($iid5 as u64).to_be_bytes();

                    [iid4[0], iid4[1], iid5[2], iid5[3], iid5[4], iid5[5], iid5[6], iid5[7]]
                },
            };
        }

        $(
            impl std::ops::Deref for $struct_name {
                type Target = $first_super_type;

                fn deref(&self) -> &Self::Target {
                    unsafe { std::mem::transmute(self) }
                }
            }

            impl std::ops::DerefMut for $struct_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { std::mem::transmute(self) }
                }
            }
        )*
    };
}
