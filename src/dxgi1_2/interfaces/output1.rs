use crate::{
    com_interface,
    dxgi::{
        IDXGIObject, IDXGIOutput, IDXGIResource, DXGI_FORMAT,
    },
    dxgi1_2::{IDXGIOutputDuplication, DXGI_MODE_DESC1},
    unknwn::{IUnknown},
    HRESULT, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::{ID3D11Texture2D, D3D11_TEXTURE2D_DESC};
#[allow(unused_imports)]
#[cfg(feature = "dxgi1_5")]
use crate::dxgi1_5::IDXGIOutput5;
#[allow(unused_imports)]
use crate::{
    dxgi::{
        IDXGIAdapter, IDXGIFactory1, IDXGISurface, IDXGISwapChain, DXGI_ENUM_MODES_SCALING,
        DXGI_ENUM_MODES_STEREO, DXGI_MODE_DESC,
    },
    DXGI_ERROR_MORE_DATA, DXGI_ERROR_NOT_CURRENTLY_AVAILABLE, DXGI_ERROR_SESSION_DISCONNECTED,
    DXGI_ERROR_UNSUPPORTED, E_ACCESSDENIED, E_INVALIDARG, S_OK,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// An [`IDXGIOutput1`] interface represents an adapter output (such as a monitor).
    ///
    /// # Remarks
    /// To determine the outputs that are available from the adapter, use
    /// [`IDXGIAdapter::enum_outputs`]. To determine the specific output that the swap chain will
    /// update, use [`IDXGISwapChain::get_containing_output`]. You can then call
    /// [`IUnknown::query_interface`] from any [`IDXGIOutput`] object to obtain an [`IDXGIOutput1`]
    /// object.
    pub abstract IDXGIOutput1(IDXGIOutput1VTable):
        IDXGIOutput(output) +
        IDXGIObject +
        IUnknown {
        const IID = 0x00CDDEA8-0x939B-0x4B83-0xA340-0xA685226666CC;

        /// Gets the display modes that match the requested format and other input options.
        ///
        /// # Parameters
        ///  * `enum_format` - A [`DXGI_FORMAT`]-typed value for the color format.
        ///  * `flags` - A combination of `DXGI_ENUM_MODES`-typed values that are combined by using
        ///              a bitwise OR operation. The resulting value specifies options for display
        ///              modes to include. You must specify [`DXGI_ENUM_MODES_SCALING`] to expose
        ///              the display modes that require scaling. Centered modes that require no
        ///              scaling and correspond directly to the display output are enumerated by
        ///              default.
        ///  * `num_modes` - A pointer to a variable that receives the number of display modes that
        ///                  [`IDXGIOutput1::get_display_mode_list1`] returns in the memory block
        ///                  to which `desc` points. Set `desc` to [`null_mut`] so that `num_modes`
        ///                  returns the number of display modes that match the format and the
        ///                  options. Otherwise, `num_modes` returns the number of display modes
        ///                  returned in `desc`.
        ///  * `desc` - A pointer to a list of display modes; set to [`null_mut`] to get the number
        ///             of display modes.
        ///
        /// # Return Value
        /// Returns one of the error codes described in the `DXGI_ERROR` topic. It is rare, but
        /// possible, that the display modes available can change immediately after calling this
        /// method, in which case [`DXGI_ERROR_MORE_DATA`] is returned (if there is not enough room
        /// for all the display modes).
        ///
        /// # Remarks
        /// [`IDXGIOutput1::get_display_mode_list1`] is updated from
        /// [`IDXGIOutput::get_display_mode_list`] to return a list of [`DXGI_MODE_DESC1`]
        /// structures, which are updated mode descriptions. [`IDXGIOutput::get_display_mode_list`]
        /// behaves as though it calls [`IDXGIOutput1::get_display_mode_list1`] because
        /// [`IDXGIOutput::get_display_mode_list`] can return all of the modes that are specified
        /// by `DXGI_ENUM_MODES`, including stereo mode. However,
        /// [`IDXGIOutput::get_display_mode_list`] returns a list of [`DXGI_MODE_DESC`] structures,
        /// which are the former mode descriptions and do not indicate stereo mode.
        ///
        /// The [`IDXGIOutput1::get_display_mode_list1`] method does not enumerate stereo modes
        /// unless you specify the [`DXGI_ENUM_MODES_STEREO`] flag in the `flags` parameter. If you
        /// specify [`DXGI_ENUM_MODES_STEREO`], stereo modes are included in the list of returned
        /// modes that the pDesc parameter points to. In other words, the method returns both
        /// stereo and mono modes.
        ///
        /// In general, when you switch from windowed to full-screen mode, a swap chain
        /// automatically chooses a display mode that meets (or exceeds) the resolution, color
        /// depth, and refresh rate of the swap chain. To exercise more control over the display
        /// mode, use [`IDXGIOutput1::get_display_mode_list1`] to poll the set of display modes
        /// that are validated against monitor capabilities, or all modes that match the desktop
        /// (if the desktop settings are not validated against the monitor).
        fn get_display_mode_list1(
            &mut self,
            enum_format: DXGI_FORMAT,
            flags: UINT,
            num_modes: *mut UINT,
            desc: *mut DXGI_MODE_DESC1
        ) -> HRESULT;

        /// Finds the display mode that most closely matches the requested display mode.
        ///
        /// # Parameters
        ///  * `mode_to_match` - A pointer to the [`DXGI_MODE_DESC1`] structure that describes the
        ///                      display mode to match. Members of [`DXGI_MODE_DESC1`] can be
        ///                      unspecified, which indicates no preference for that member. A
        ///                      value of 0 for `width` or `height` indicates that the value is
        ///                      unspecified. If either `width` or `height` is 0, both must be 0. A
        ///                      numerator and denominator of 0 in `refresh_rate` indicate it is
        ///                      unspecified. Other members of [`DXGI_MODE_DESC1`] have enumeration
        ///                      values that indicate that the member is unspecified. If
        ///                      `concerned_device` is [`null_mut`], the Format member of
        ///                      [`DXGI_MODE_DESC1`] cannot be [`DXGI_FORMAT::Unknown`].
        ///  * `closest_match` - A pointer to the [`DXGI_MODE_DESC1`] structure that receives a
        ///                      description of the display mode that most closely matches the
        ///                      display mode described at `mode_to_match`.
        ///  * `concerned_device` - A pointer to the Direct3D device interface. If this parameter
        ///                         is [`null_mut`], [`IDXGIOutput1::find_closest_matching_mode1`]
        ///                         returns only modes whose format matches that of
        ///                         `mode_to_match`; otherwise,
        ///                         [`IDXGIOutput1::find_closest_matching_mode1`] returns only
        ///                         those formats that are supported for scan-out by the device.
        ///
        /// # Return Value
        /// Returns one of the error codes described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// Direct3D devices require `UNORM` formats.
        ///
        /// [`IDXGIOutput1::find_closest_matching_mode1`] finds the closest matching available
        /// display mode to the mode that you specify in `mode_to_match`.
        ///
        /// If you set the `stereo` member in the [`DXGI_MODE_DESC1`] structure to which
        /// `mode_to_match` points to specify a stereo mode as input,
        /// [`IDXGIOutput1::find_closest_matching_mode1`] considers only stereo modes.
        /// [`IDXGIOutput1::find_closest_matching_mode1`] considers only mono modes if `stereo` is
        /// not set.
        ///
        /// [`IDXGIOutput1::find_closest_matching_mode1`] resolves similarly ranked members of
        /// display modes (that is, all specified, or all unspecified, and so on) in the following
        /// order:
        ///  1. `scanline_ordering`
        ///  2. `scaling`
        ///  3. `format`
        ///  4. `resolution`
        ///  5. `refresh_rate`
        ///
        /// When [`IDXGIOutput1::find_closest_matching_mode1`] determines the closest value for a
        /// particular member, it uses previously matched members to filter the display mode list
        /// choices, and ignores other members. For example, when
        /// [`IDXGIOutput1::find_closest_matching_mode1`] matches `resolution`, it already filtered
        /// the display mode list by a certain `scanline_ordering`, `scaling`, and `format`, while
        /// it ignores `refresh_rate`. This ordering doesn't define the absolute ordering for every
        /// usage scenario of [`IDXGIOutput1::find_closest_matching_mode1`], because the
        /// application can choose some values initially, which effectively changes the order of
        /// resolving members.
        ///
        /// [`IDXGIOutput1::find_closest_matching_mode1`] matches members of the display mode one
        /// at a time, generally in a specified order.
        ///
        /// If a member is unspecified, [`IDXGIOutput1::find_closest_matching_mode1`] gravitates
        /// toward the values for the desktop related to this output. If this output is not part of
        /// the desktop, [`IDXGIOutput1::find_closest_matching_mode1`] uses the default desktop
        /// output to find values. If an application uses a fully unspecified display mode,
        /// [`IDXGIOutput1::find_closest_matching_mode1`] typically returns a display mode that
        /// matches the desktop settings for this output. Because unspecified members are lower
        /// priority than specified members, [`IDXGIOutput1::find_closest_matching_mode1`] resolves
        /// unspecified members later than specified members.
        fn find_closest_matching_mode1(
            &mut self,
            mode_to_match: *const DXGI_MODE_DESC1,
            closest_match: *mut DXGI_MODE_DESC1,
            concerned_device: *mut IUnknown
        ) -> HRESULT;

        /// Copies the display surface (front buffer) to a user-provided resource.
        ///
        /// # Parameters
        ///  * `destination` - A pointer to a resource interface that represents the resource to
        ///                    which [`IDXGIOutput1::get_display_surface_data1`] copies the display
        ///                    surface.
        ///
        /// # Return Value
        /// Returns one of the error codes described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// [`IDXGIOutput1::get_display_surface_data1`] is similar to
        /// [`IDXGIOutput::get_display_surface_data`] except
        /// [`IDXGIOutput1::get_display_surface_data1`] takes an [`IDXGIResource`] and
        /// [`IDXGIOutput::get_display_surface_data`] takes an [`IDXGISurface`].
        ///
        /// [`IDXGIOutput1::get_display_surface_data1`] returns an error if the input resource is
        /// not a 2D texture (represented by the [`ID3D11Texture2D`] interface) with an array size
        /// (`array_size` member of the [`D3D11_TEXTURE2D_DESC`] structure) that is equal to the
        /// swap chain buffers.
        ///
        /// The original [`IDXGIOutput::get_display_surface_data`] and the updated
        /// [`IDXGIOutput1::get_display_surface_data1`] behave exactly the same.
        /// [`IDXGIOutput1::get_display_surface_data1`] was required because textures with an array
        /// size equal to 2 (`array_size = 2`) do not implement [`IDXGISurface`].
        ///
        /// You can call [`IDXGIOutput1::get_display_surface_data1`] only when an output is in
        /// full-screen mode. If [`IDXGIOutput1::get_display_surface_data1`] succeeds, it fills the
        /// destination resource.
        ///
        /// Use [`IDXGIOutput::get_desc`] to determine the size (width and height) of the output
        /// when you want to allocate space for the destination resource. This is true regardless
        /// of target monitor rotation. A destination resource created by a graphics component
        /// (such as Direct3D 11) must be created with CPU write permission (see
        /// [`D3D11_CPU_ACCESS_WRITE`]). Other surfaces can be created with CPU read-write
        /// permission (`D3D11_CPU_ACCESS_READ | D3D11_CPU_ACCESS_WRITE`).
        /// [`IDXGIOutput1::get_display_surface_data1`] modifies the surface data to fit the
        /// destination resource (stretch, shrink, convert format, rotate).
        /// [`IDXGIOutput1::get_display_surface_data1`] performs the stretch and shrink with point
        /// sampling.
        fn get_display_surface_data1(&mut self, destination: *mut IDXGIResource) -> HRESULT;

        /// Creates a desktop duplication interface from the [`IDXGIOutput1`] interface that
        /// represents an adapter output.
        ///
        /// # Parameters
        ///  * `device` - A pointer to the Direct3D device interface that you can use to process
        ///               the desktop image. This device must be created from the adapter to which
        ///               the output is connected.
        ///  * `output_duplication` - A pointer to a variable that receives the new
        ///                           [`IDXGIOutputDuplication`] interface.
        ///
        /// # Return Value
        /// [`IDXGIOutput1::duplicate_output`] returns:
        ///  - [`S_OK`] if [`IDXGIOutput1::duplicate_output`] successfully created the desktop
        ///    duplication interface.
        ///  - [`E_INVALIDARG`] for one of the following reasons:
        ///    - The specified device (`device`) is invalid, was not created on the correct
        ///      adapter, or was not created from [`IDXGIFactory1`] (or a later version of a DXGI
        ///      factory interface that inherits from [`IDXGIFactory1`]).
        ///    - The calling application is already duplicating this desktop output.
        ///  - [`E_ACCESSDENIED`] if the application does not have access privilege to the current
        ///    desktop image. For example, only an application that runs at `LOCAL_SYSTEM` can
        ///    access the secure desktop.
        ///  - [`DXGI_ERROR_UNSUPPORTED`] if the created [`IDXGIOutputDuplication`] interface does
        ///    not support the current desktop mode or scenario. For example, 8bpp and non-DWM
        ///    desktop modes are not supported. If [`IDXGIOutput1::duplicate_output`] fails with
        ///    [`DXGI_ERROR_UNSUPPORTED`], the application can wait for system notification of
        ///    desktop switches and mode changes and then call [`IDXGIOutput1::duplicate_output`]
        ///    again after such a notification occurs.
        ///  - [`DXGI_ERROR_NOT_CURRENTLY_AVAILABLE`] if DXGI reached the limit on the maximum
        ///    number of concurrent duplication applications (default of four). Therefore, the
        ///    calling application cannot create any desktop duplication interfaces until the other
        ///    applications close.
        ///  - [`DXGI_ERROR_SESSION_DISCONNECTED`] if [`IDXGIOutput1::duplicate_output`] failed
        ///    because the session is currently disconnected.
        ///  - Other error codes are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// If an application wants to duplicate the entire desktop, it must create a desktop
        /// duplication interface on each active output on the desktop. This interface does not
        /// provide an explicit way to synchronize the timing of each output image. Instead, the
        /// application must use the time stamp of each output, and then determine how to combine
        /// the images.
        ///
        /// For [`IDXGIOutput1::duplicate_output`] to succeed, you must create pDevice from
        /// [`IDXGIFactory1`] or a later version of a DXGI factory interface that inherits from
        /// [`IDXGIFactory1`].
        ///
        /// If the current mode is a stereo mode, the desktop duplication interface provides the
        /// image for the left stereo image only.
        ///
        /// By default, only four processes can use a [`IDXGIOutputDuplication`] interface at the
        /// same time within a single session. A process can have only one desktop duplication
        /// interface on a single desktop output; however, that process can have a desktop
        /// duplication interface for each output that is part of the desktop.
        ///
        /// For improved performance, consider using [`IDXGIOutput5::duplicate_output1`].
        fn duplicate_output(
            &mut self,
            device: *mut IUnknown,
            output_duplication: *mut *mut IDXGIOutputDuplication
        ) -> HRESULT;
    }
);
