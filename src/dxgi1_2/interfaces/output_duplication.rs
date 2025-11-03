use crate::{
    com_interface,
    dxgi::{IDXGIObject, IDXGIResource, DXGI_MAPPED_RECT},
    dxgi1_2::{
        DXGI_OUTDUPL_DESC, DXGI_OUTDUPL_FRAME_INFO, DXGI_OUTDUPL_MOVE_RECT,
        DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    },
    unknwn::{IUnknown},
    HRESULT, RECT, UINT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::IDXGISurface, DXGI_ERROR_ACCESS_LOST, DXGI_ERROR_INVALID_CALL, DXGI_ERROR_MORE_DATA,
    DXGI_ERROR_WAIT_TIMEOUT, E_INVALIDARG, FALSE, INFINITE, S_OK, TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// The [`IDXGIOutputDuplication`] interface accesses and manipulates the duplicated desktop
    /// image.
    ///
    /// # Remarks
    /// A collaboration application can use [`IDXGIOutputDuplication`] to access the desktop image.
    /// [`IDXGIOutputDuplication`] is supported in Desktop Window Manager (DWM) on non-8bpp DirectX
    /// full-screen modes and non-8bpp OpenGL full-screen modes. 16-bit or 32-bit GDI non-DWM
    /// desktop modes are not supported.
    ///
    /// An application can use [`IDXGIOutputDuplication`] on a separate thread to receive the
    /// desktop images and to feed them into their specific image-processing pipeline. The
    /// application uses [`IDXGIOutputDuplication`] to perform the following operations:
    ///  1. Acquire the next desktop image.
    ///  2. Retrieve the information that describes the image.
    ///  3. Perform an operation on the image. This operation can be as simple as copying the image
    ///     to a staging buffer so that the application can read the pixel data on the image. The
    ///     application reads the pixel data after the application calls [`IDXGISurface::map`].
    ///     Alternatively, this operation can be more complex. For example, the application can run
    ///     some pixel shaders on the updated regions of the image to encode those regions for
    ///     transmission to a client.
    ///  4. After the application finishes processing each desktop image, it releases the image,
    ///     loops to step 1, and repeats the steps. The application repeats these steps until it is
    ///     finished processing desktop images.
    ///
    /// The following components of the operating system can generate the desktop image:
    ///  - The DWM by composing the desktop image
    ///  - A full-screen DirectX or OpenGL application
    ///  - An application by switching to a separate desktop, for example, the secure desktop that
    ///    is used to display the login screen
    ///
    /// All current [`IDXGIOutputDuplication`] interfaces become invalid when the operating system
    /// switches to a different component that produces the desktop image or when a mode change
    /// occurs. In these situations, the application must destroy its current
    /// [`IDXGIOutputDuplication`] interface and create a new [`IDXGIOutputDuplication`] interface.
    ///
    /// Examples of situations in which [`IDXGIOutputDuplication`] becomes invalid are:
    ///  - Desktop switch
    ///  - Mode change
    ///  - Switch from DWM on, DWM off, or other full-screen application
    ///
    /// In these situations, the application must release the [`IDXGIOutputDuplication`] interface
    /// and must create a new [`IDXGIOutputDuplication`] interface for the new content. If the
    /// application does not have the appropriate privilege to the new desktop image, its call to
    /// the [`IDXGIOutput1::duplicate_output`] method fails.
    ///
    /// While the application processes each desktop image, the operating system accumulates all
    /// the desktop image updates into a single update.
    ///
    /// The desktop image is always in the [`DXGI_FORMAT::B8G8R8A8UNorm`] format.
    ///
    /// The [`IDXGIOutputDuplication`] interface does not exist for Windows Store apps.
    pub abstract IDXGIOutputDuplication(IDXGIOutputDuplicationVTable):
        IDXGIObject(object) +
        IUnknown {
        const IID = 0x191CFAC3-0xA341-0x470D-0xB26E-0xA864F428319C;

        /// Retrieves a description of a duplicated output. This description specifies the
        /// dimensions of the surface that contains the desktop image.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`DXGI_OUTDUPL_DESC`] structure that describes the
        ///             duplicated output. This parameter must not be [`null_mut`].
        ///
        /// # Remarks
        /// After an application creates an [`IDXGIOutputDuplication`] interface, it calls
        /// [`IDXGIOutputDuplication::get_desc`] to retrieve the dimensions of the surface that
        /// contains the desktop image. The format of the desktop image is always
        /// [`DXGI_FORMAT::B8G8R8A8UNorm`].
        fn get_desc(&mut self, desc: *mut DXGI_OUTDUPL_DESC);

        /// Indicates that the application is ready to process the next desktop image.
        ///
        /// # Parameters
        ///  * `timeout_in_milliseconds` - The time-out interval, in milliseconds. This interval
        ///                                specifies the amount of time that this method waits for
        ///                                a new frame before it returns to the caller. This method
        ///                                returns if the interval elapses, and a new desktop image
        ///                                is not available.
        ///  * `frame_info` - A pointer to a memory location that receives the
        ///                   [`DXGI_OUTDUPL_FRAME_INFO`] structure that describes timing and
        ///                   presentation statistics for a frame.
        ///  * `desktop_resource` - A pointer to a variable that receives the [`IDXGIResource`]
        ///                         interface of the surface that contains the desktop bitmap.
        ///
        /// # Return Value
        /// [`IDXGIOutputDuplication::acquire_next_frame`] returns:
        ///  - [`S_OK`] if it successfully received the next desktop image.
        ///  - [`DXGI_ERROR_ACCESS_LOST`] if the desktop duplication interface is invalid. The
        ///    desktop duplication interface typically becomes invalid when a different type of
        ///    image is displayed on the desktop. Examples of this situation are:
        ///    - Desktop switch
        ///    - Mode change
        ///    - Switch from DWM on, DWM off, or other full-screen application
        ///  - In this situation, the application must release the [`IDXGIOutputDuplication`]
        ///    interface and create a new [`IDXGIOutputDuplication`] for the new content.
        ///  - [`DXGI_ERROR_WAIT_TIMEOUT`] if the time-out interval elapsed before the next desktop
        ///    frame was available.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the application called
        ///    [`IDXGIOutputDuplication::acquire_next_frame`] without releasing the previous frame.
        ///  - [`E_INVALIDARG`] if one of the parameters to
        ///    [`IDXGIOutputDuplication::acquire_next_frame`] is incorrect; for example, if
        ///    `frame_info` is [`null_mut`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// When [`IDXGIOutputDuplication::acquire_next_frame`] returns successfully, the calling
        /// application can access the desktop image that
        /// [`IDXGIOutputDuplication::acquire_next_frame`] returns in the variable at
        /// `desktop_resource`. If the caller specifies a zero time-out interval in the
        /// `timeout_in_milliseconds` parameter, [`IDXGIOutputDuplication::acquire_next_frame`]
        /// verifies whether there is a new desktop image available, returns immediately, and
        /// indicates its outcome with the return value. If the caller specifies an [`INFINITE`]
        /// time-out interval in the `timeout_in_milliseconds` parameter, the time-out interval
        /// never elapses.
        ///
        ///  [`IDXGIOutputDuplication::acquire_next_frame`] acquires a new desktop frame when the
        /// operating system either updates the desktop bitmap image or changes the shape or
        /// position of a hardware pointer. The new frame that
        /// [`IDXGIOutputDuplication::acquire_next_frame`] acquires might have only the desktop
        /// image updated, only the pointer shape or position updated, or both.
        fn acquire_next_frame(
            &mut self,
            timeout_in_milliseconds: UINT,
            frame_info: *mut DXGI_OUTDUPL_FRAME_INFO,
            desktop_resource: *mut *mut IDXGIResource
        ) -> HRESULT;

        /// Gets information about dirty rectangles for the current desktop frame.
        ///
        /// # Parameters
        ///  * `dirty_rects_buffer_size` - The size in bytes of the buffer that the caller passed
        ///                                to the `dirty_rects_buffer` parameter.
        ///  * `dirty_rects_buffer` - A pointer to an array of [`RECT`] structures that identifies
        ///                           the dirty rectangle regions for the desktop frame.
        ///  * `dirty_rects_buffer_size_required` - Pointer to a variable that receives the number
        ///                                         of bytes that
        ///                                       [`IDXGIOutputDuplication::get_frame_dirty_rects`]
        ///                                         needs to store information about dirty regions
        ///                                         in the buffer at `dirty_rects_buffer`.
        ///
        /// # Return Value
        /// [`IDXGIOutputDuplication::get_frame_dirty_rects`] returns:
        ///  - [`S_OK`] if it successfully retrieved information about dirty rectangles.
        ///  - [`DXGI_ERROR_ACCESS_LOST`] if the desktop duplication interface is invalid. The
        ///    desktop duplication interface typically becomes invalid when a different type of
        ///    image is displayed on the desktop. Examples of this situation are:
        ///    - Desktop switch
        ///    - Mode change
        ///    - Switch from DWM on, DWM off, or other full-screen application
        ///  - In this situation, the application must release the [`IDXGIOutputDuplication`]
        ///    interface and create a new [`IDXGIOutputDuplication`] for the new content.
        ///  - [`DXGI_ERROR_MORE_DATA`] if the buffer that the calling application provided was not
        ///    big enough.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the application called
        ///    [`IDXGIOutputDuplication::get_frame_dirty_rects`] without owning the desktop image.
        ///  - [`E_INVALIDARG`] if one of the parameters to
        ///    [`IDXGIOutputDuplication::get_frame_dirty_rects`] is incorrect; for example, if
        ///    `dirty_rects_buffer` is [`null_mut`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// [`IDXGIOutputDuplication::get_frame_dirty_rects`] stores a size value in the variable
        /// at `dirty_rects_buffer_size_required`. This value specifies the number of bytes that
        /// [`IDXGIOutputDuplication::get_frame_dirty_rects`] needs to store information about
        /// dirty regions. You can use this value in the following situations to determine the
        /// amount of memory to allocate for future buffers that you pass to `dirty_rects_buffer`:
        ///  - [`IDXGIOutputDuplication::get_frame_dirty_rects`] fails with
        ///    [`DXGI_ERROR_MORE_DATA`] because the buffer is not big enough.
        ///  - [`IDXGIOutputDuplication::get_frame_dirty_rects`] supplies a buffer that is bigger
        ///    than necessary. The size value returned at `dirty_rects_buffer_size_required`
        ///    informs the caller how much buffer space was actually used compared to how much
        ///    buffer space the caller allocated and specified in the `dirty_rects_buffer_size`
        ///    parameter.
        ///
        /// The caller can also use the value returned at `dirty_rects_buffer_size_required` to
        /// determine the number of [`RECT`]s returned in the `dirty_rects_buffer` array.
        ///
        /// The buffer contains the list of dirty [`RECT`]s for the current frame.
        fn get_frame_dirty_rects(
            &mut self,
            dirty_rects_buffer_size: UINT,
            dirty_rects_buffer: *mut RECT,
            dirty_rects_buffer_size_required: *mut UINT
        ) -> HRESULT;

        /// Gets information about the moved rectangles for the current desktop frame.
        ///
        /// # Parameters
        ///  * `move_rects_buffer_size` - The size in bytes of the buffer that the caller passed
        ///                               to the `move_rect_buffer` parameter.
        ///  * `move_rect_buffer` - A pointer to an array of [`DXGI_OUTDUPL_MOVE_RECT`] structures
        ///                         that identifies the moved rectangle regions for the desktop
        ///                         frame.
        ///  * `move_rects_buffer_size_required` - Pointer to a variable that receives the number
        ///                                        of bytes that
        ///                                        [`IDXGIOutputDuplication::get_frame_move_rects`]
        ///                                        needs to store information about moved regions
        ///                                        in the buffer at `move_rect_buffer`.
        ///
        /// # Return Value
        /// [`IDXGIOutputDuplication::get_frame_move_rects`] returns:
        ///  - [`S_OK`] if it successfully retrieved information about moved rectangles.
        ///  - [`DXGI_ERROR_ACCESS_LOST`] if the desktop duplication interface is invalid. The
        ///    desktop duplication interface typically becomes invalid when a different type of
        ///    image is displayed on the desktop. Examples of this situation are:
        ///    - Desktop switch
        ///    - Mode change
        ///    - Switch from DWM on, DWM off, or other full-screen application
        ///  - In this situation, the application must release the [`IDXGIOutputDuplication`]
        ///    interface and create a new [`IDXGIOutputDuplication`] for the new content.
        ///  - [`DXGI_ERROR_MORE_DATA`] if the buffer that the calling application provided is not
        ///    big enough.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the application called
        ///    [`IDXGIOutputDuplication::get_frame_move_rects`] without owning the desktop image.
        ///  - [`E_INVALIDARG`] if one of the parameters to
        ///    [`IDXGIOutputDuplication::get_frame_move_rects`] is incorrect; for example, if
        ///    `move_rect_buffer` is [`null_mut`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// [`IDXGIOutputDuplication::get_frame_move_rects`] stores a size value in the variable at
        /// `move_rects_buffer_size_required`. This value specifies the number of bytes that
        /// [`IDXGIOutputDuplication::get_frame_move_rects`] needs to store information about moved
        /// regions. You can use this value in the following situations to determine the amount of
        /// memory to allocate for future buffers that you pass to `move_rect_buffer`:
        ///  - [`IDXGIOutputDuplication::get_frame_move_rects`] fails with [`DXGI_ERROR_MORE_DATA`]
        ///    because the buffer is not big enough.
        ///  - [`IDXGIOutputDuplication::get_frame_move_rects`] supplies a buffer that is bigger
        ///    than necessary. The size value returned at `move_rects_buffer_size_required` informs
        ///    the caller how much buffer space was actually used compared to how much buffer space
        ///    the caller allocated and specified in the `move_rects_buffer_size` parameter.
        ///
        /// The caller can also use the value returned at `move_rects_buffer_size_required` to
        /// determine the number of [`DXGI_OUTDUPL_MOVE_RECT`] structures returned.
        ///
        /// The buffer contains the list of move [`RECT`]s for the current frame.
        fn get_frame_move_rects(
            &mut self,
            move_rects_buffer_size: UINT,
            move_rect_buffer: *mut DXGI_OUTDUPL_MOVE_RECT,
            move_rects_buffer_size_required: *mut UINT
        ) -> HRESULT;

        /// Gets information about the new pointer shape for the current desktop frame.
        ///
        /// # Parameters
        ///  * `pointer_shape_buffer_size` - The size in bytes of the buffer that the caller passed
        ///                                  to the `pointer_shape_buffer` parameter.
        ///  * `pointer_shape_buffer` - A pointer to a buffer to which
        ///                             [`IDXGIOutputDuplication::get_frame_pointer_shape`] copies
        ///                             and returns pixel data for the new pointer shape.
        ///  * `pointer_shape_buffer_size_required` - Pointer to a variable that receives the
        ///                                           number of bytes that
        ///                                     [`IDXGIOutputDuplication::get_frame_pointer_shape`]
        ///                                           needs to store the new pointer shape pixel
        ///                                           data in the buffer at `pointer_shape_buffer`.
        ///  * `pointer_shape_info` - Pointer to a [`DXGI_OUTDUPL_POINTER_SHAPE_INFO`] structure
        ///                           that receives the pointer shape information.
        ///
        /// # Return Value
        /// [`IDXGIOutputDuplication::get_frame_pointer_shape`] returns:
        ///  - [`S_OK`] if it successfully retrieved information about the new pointer shape.
        ///  - [`DXGI_ERROR_ACCESS_LOST`] if the desktop duplication interface is invalid. The
        ///    desktop duplication interface typically becomes invalid when a different type of
        ///    image is displayed on the desktop. Examples of this situation are:
        ///    - Desktop switch
        ///    - Mode change
        ///    - Switch from DWM on, DWM off, or other full-screen application
        ///  - In this situation, the application must release the [`IDXGIOutputDuplication`]
        ///    interface and create a new [`IDXGIOutputDuplication`] for the new content.
        ///  - [`DXGI_ERROR_MORE_DATA`] if the buffer that the calling application provided was not
        ///    big enough.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the application called
        ///    [`IDXGIOutputDuplication::get_frame_pointer_shape`] without owning the desktop
        ///    image.
        ///  - [`E_INVALIDARG`] if one of the parameters to
        ///    [`IDXGIOutputDuplication::get_frame_pointer_shape`] is incorrect; for example, if
        ///    `pointer_shape_info` is [`null_mut`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// [`IDXGIOutputDuplication::get_frame_pointer_shape`] stores a size value in the variable
        /// at `pointer_shape_buffer_size_required`. This value specifies the number of bytes that
        /// `pointer_shape_buffer_size_required` needs to store the new pointer shape pixel data.
        /// You can use the value in the following situations to determine the amount of memory to
        /// allocate for future buffers that you pass to `pointer_shape_buffer`:
        ///  - [`IDXGIOutputDuplication::get_frame_pointer_shape`] fails with
        ///    [`DXGI_ERROR_MORE_DATA`] because the buffer is not big enough.
        ///  - [`IDXGIOutputDuplication::get_frame_pointer_shape`] supplies a bigger than necessary
        ///    buffer. The size value returned at `pointer_shape_buffer_size_required` informs the
        ///    caller how much buffer space was actually used compared to how much buffer space the
        ///    caller allocated and specified in the `pointer_shape_buffer_size` parameter.
        ///
        /// The `pointer_shape_info` parameter describes the new pointer shape.
        fn get_frame_pointer_shape(
            &mut self,
            pointer_shape_buffer_size: UINT,
            pointer_shape_buffer: *mut c_void,
            pointer_shape_buffer_size_required: *mut UINT,
            pointer_shape_info: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO
        ) -> HRESULT;

        /// Provides the CPU with efficient access to a desktop image if that desktop image is
        /// already in system memory.
        ///
        /// # Parameters
        ///  * `locked_rect` - A pointer to a [`DXGI_MAPPED_RECT`] structure that receives the
        ///                    surface data that the CPU needs to directly access the surface data.
        ///
        /// # Return Value
        /// [`IDXGIOutputDuplication::map_desktop_surface`] returns:
        ///  - [`S_OK`] if it successfully retrieved the surface data.
        ///  - [`DXGI_ERROR_ACCESS_LOST`] if the desktop duplication interface is invalid. The
        ///    desktop duplication interface typically becomes invalid when a different type of
        ///    image is displayed on the desktop. Examples of this situation are:
        ///    - Desktop switch
        ///    - Mode change
        ///    - Switch from DWM on, DWM off, or other full-screen application
        ///  - In this situation, the application must release the [`IDXGIOutputDuplication`]
        ///    interface and create a new [`IDXGIOutputDuplication`] for the new content.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the application already has an outstanding map on the
        ///    desktop image. The application must call
        ///    [`IDXGIOutputDuplication::unmap_desktop_surface`] before it calls
        ///    [`IDXGIOutputDuplication::map_desktop_surface`] again. [`DXGI_ERROR_INVALID_CALL`]
        ///    is also returned if the application did not own the desktop image when it called
        ///    [`IDXGIOutputDuplication::map_desktop_surface`].
        ///  - [`DXGI_ERROR_UNSUPPORTED`] if the desktop image is not in system memory. In this
        ///    situation, the application must first transfer the image to a staging surface and
        ///    then lock the image by calling the [`IDXGISurface::map`] method.
        ///  - [`E_INVALIDARG`] if the `locked_rect` parameter is incorrect; for example, if
        ///    `locked_rect` is [`null_mut`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// You can successfully call [`IDXGIOutputDuplication::map_desktop_surface`] if the
        /// `desktop_image_in_system_memory` member of the [`DXGI_OUTDUPL_DESC`] structure is set
        /// to [`TRUE`]. If `desktop_image_in_system_memory` is [`FALSE`],
        /// [`IDXGIOutputDuplication::map_desktop_surface`] returns [`DXGI_ERROR_UNSUPPORTED`].
        /// Call [`IDXGIOutputDuplication::get_desc`] to retrieve the [`DXGI_OUTDUPL_DESC`]
        /// structure.
        fn map_desktop_surface(&mut self, locked_rect: *mut DXGI_MAPPED_RECT) -> HRESULT;

        /// Invalidates the pointer to the desktop image that was retrieved by using
        /// [`IDXGIOutputDuplication::map_desktop_surface`].
        ///
        /// # Return Value
        /// [`IDXGIOutputDuplication::unmap_desktop_surface`] returns:
        ///  - [`S_OK`] if it successfully completed.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the application did not map the desktop surface by
        ///    calling [`IDXGIOutputDuplication::map_desktop_surface`].
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        fn unmap_desktop_surface(&mut self) -> HRESULT;

        /// Indicates that the application finished processing the frame.
        ///
        /// # Return Value
        /// [`IDXGIOutputDuplication::release_frame`] returns:
        ///  - [`S_OK`] if it successfully completed.
        ///  - [`DXGI_ERROR_INVALID_CALL`] if the application already released the frame.
        ///  - [`DXGI_ERROR_ACCESS_LOST`] if the desktop duplication interface is invalid. The
        ///    desktop duplication interface typically becomes invalid when a different type of
        ///    image is displayed on the desktop. Examples of this situation are:
        ///    - Desktop switch
        ///    - Mode change
        ///    - Switch from DWM on, DWM off, or other full-screen application
        ///  - In this situation, the application must release the [`IDXGIOutputDuplication`]
        ///    interface and create a new [`IDXGIOutputDuplication`] for the new content.
        ///  - Possibly other error codes that are described in the `DXGI_ERROR` topic.
        ///
        /// # Remarks
        /// The application must release the frame before it acquires the next frame. After the
        /// frame is released, the surface that contains the desktop bitmap becomes invalid; you
        /// will not be able to use the surface in a DirectX graphics operation.
        ///
        /// For performance reasons, we recommend that you release the frame just before you call
        /// the [`IDXGIOutputDuplication::acquire_next_frame`] method to acquire the next frame.
        /// When the client does not own the frame, the operating system copies all desktop updates
        /// to the surface. This can result in wasted GPU cycles if the operating system updates
        /// the same region for each frame that occurs. When the client acquires the frame, the
        /// client is aware of only the final update to this region; therefore, any overlapping
        /// updates during previous frames are wasted. When the client acquires a frame, the client
        /// owns the surface; therefore, the operating system can track only the updated regions
        /// and cannot copy desktop updates to the surface. Because of this behavior, we recommend
        /// that you minimize the time between the call to release the current frame and the call
        /// to acquire the next frame.
        fn release_frame(&mut self) -> HRESULT;
    }
);
