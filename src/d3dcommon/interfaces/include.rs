use crate::{com_interface, d3dcommon::D3D_INCLUDE_TYPE, HRESULT, LPCSTR, LPCVOID, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::S_OK;
#[allow(unused_imports)]
use std::ptr::null;

com_interface!(
    /// [`ID3DInclude`] is an include interface that the user implements to allow an application to
    /// call user-overridable methods for opening and closing shader `#include` files.
    ///
    /// # Remarks
    /// To use this interface, create an interface that inherits from [`ID3DInclude`] and implement
    /// custom behavior for the methods.
    pub abstract ID3DInclude(ID3DIncludeVTable/ID3DIncludeTrait) {
        const IID = 0x00000000-0x0000-0x0000-0x0000-0x000000000000;

        /// A user-implemented method for opening and reading the contents of a shader `#include`
        /// file.
        ///
        /// # Parameters
        ///  * `include_type` - A [`D3D_INCLUDE_TYPE`]-typed value that indicates the location of
        ///                     the `#include` file.
        ///  * `file_name` - Name of the `#include` file.
        ///  * `parent_data` - Pointer to the container that includes the `#include` file. The
        ///                    compiler might pass [`null`] in `parent_data`.
        ///  * `data` - Pointer to the buffer that contains the include directives. This pointer
        ///             remains valid until you call [`ID3DInclude::close`].
        ///  * `bytes` - Pointer to the number of bytes that [`ID3DInclude::open`] returns in
        ///              `Data`.
        ///
        /// # Return Value
        /// The user-implemented method must return [`S_OK`]. If [`ID3DInclude::open`] fails when
        /// it reads the `#include` file, the application programming interface (API) that caused
        /// [`ID3DInclude::open`] to be called fails. This failure can occur in one of the
        /// following situations:
        ///  - The high-level shader language (HLSL) shader fails one of the
        ///    `D3D10CompileShader***` functions.
        ///  - The effect fails one of the `D3D10CreateEffect***` functions.
        fn open(
            &mut self,
            include_type: D3D_INCLUDE_TYPE,
            file_name: LPCSTR,
            parent_data: LPCVOID,
            data: *mut LPCVOID,
            bytes: *mut UINT
        ) -> HRESULT;

        /// A user-implemented method for closing a shader `#include` file.
        ///
        /// # Parameters
        ///  * `data` - Pointer to the buffer that contains the include directives. This is the
        ///             pointer that was returned by the corresponding [`ID3DInclude::open`] call.
        ///
        /// # Return Value
        /// The user-implemented [`ID3DInclude::close`] method should return [`S_OK`]. If
        /// [`ID3DInclude::close`] fails when it closes the `#include` file, the application
        /// programming interface (API) that caused [`ID3DInclude::close`] to be called fails. This
        /// failure can occur in one of the following situations:
        ///  - The high-level shader language (HLSL) shader fails one of the
        ///    `D3D10CompileShader***` functions.
        ///  - The effect fails one of the `D3D10CreateEffect***` functions.
        ///
        /// # Remarks
        /// If [`ID3DInclude::open`] was successful, [`ID3DInclude::close`] is guaranteed to be
        /// called before the API using the [`ID3DInclude`] interface returns.
        fn close(&mut self, data: LPCVOID) -> HRESULT;
    }
);
