use crate::{
    com_interface,
    dxgi::{IDXGIAdapter, IDXGIAdapter1, IDXGIObject},
    dxgi1_2::IDXGIAdapter2,
    dxgi1_4::{DXGI_MEMORY_SEGMENT_GROUP, DXGI_QUERY_VIDEO_MEMORY_INFO},
    unknwn::IUnknown,
    DWORD, HANDLE, HRESULT, UINT, UINT64,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateEvent, S_OK};

com_interface!(
    /// This interface adds some memory residency methods, for budgeting and reserving physical
    /// memory.
    pub abstract IDXGIAdapter3(IDXGIAdapter3VTable):
        IDXGIAdapter2(adapter2) +
        IDXGIAdapter1 +
        IDXGIAdapter +
        IDXGIObject +
        IUnknown {
        const IID = 0x645967A4-0x1392-0x4310-0xA798-0x8053CE3E93FD;

        /// Registers to receive notification of hardware content protection teardown events.
        ///
        /// # Parameters
        ///  * `event` - A handle to the event object that the operating system sets when hardware
        ///              content protection teardown occurs. The [`CreateEvent`] or [`OpenEvent`]
        ///              function returns this handle.
        ///  * `cookie` - A pointer to a key value that an application can pass to the
        ///               [`IDXGIAdapter3::unregister_hardware_content_protection_teardown_status`]
        ///               method to unregister the notification event that `event` specifies.
        ///
        /// # Return Value
        /// If this method succeeds, it returns [`S_OK`]. Otherwise, it returns an [`HRESULT`]
        /// error code.
        ///
        /// # Remarks
        /// Call [`ID3D11VideoDevice::get_content_protection_caps`] to check for the presence of
        /// the [`D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_TEARDOWN`] capability to know whether the
        /// hardware contains an automatic teardown mechanism.
        ///
        /// After the event is signaled, the application can call
        /// [`ID3D11VideoContext1::check_crypto_session_status`] to determine the impact of the
        /// hardware teardown for a specific [`ID3D11CryptoSession`] interface.
        fn register_hardware_content_protection_teardown_status_event(
            &mut self,
            event: HANDLE,
            cookie: *mut DWORD
        ) -> HRESULT;

        /// Unregisters an event to stop it from receiving notification of hardware content
        /// protection teardown events.
        ///
        /// # Parameters
        ///  * `cookie` - A key value for the window or event to unregister. The
        ///           [`IDXGIAdapter3::register_hardware_content_protection_teardown_status_event`]
        ///               method returns this value.
        fn unregister_hardware_content_protection_teardown_status(&mut self, cookie: DWORD);

        /// This method informs the process of the current budget and process usage.
        ///
        /// # Parameters
        ///  * `node_index` - Specifies the device's physical adapter for which the video memory
        ///                   information is queried. For single-GPU operation, set this to zero.
        ///                   If there are multiple GPU nodes, set this to the index of the node
        ///                   (the device's physical adapter) for which the video memory
        ///                   information is queried.
        ///  * `memory_segment_group` - Specifies a [`DXGI_MEMORY_SEGMENT_GROUP`] that identifies
        ///                             the group as local or non-local.
        ///  * `video_memory_info` - Fills in a [`DXGI_QUERY_VIDEO_MEMORY_INFO`] structure with the
        ///                          current values.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// Applications must explicitly manage their usage of physical memory explicitly and keep
        /// usage within the budget assigned to the application process. Processes that cannot kept
        /// their usage within their assigned budgets will likely experience stuttering, as they
        /// are intermittently frozen and paged-out to allow other processes to run.
        fn query_video_memory_info(
            &mut self,
            node_index: UINT,
            memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
            video_memory_info: *mut DXGI_QUERY_VIDEO_MEMORY_INFO
        ) -> HRESULT;

        /// This method sends the minimum required physical memory for an application, to the OS.
        ///
        /// # Parameters
        ///  * `node_index` - Specifies the device's physical adapter for which the video memory
        ///                   information is being set. For single-GPU operation, set this to zero.
        ///                   If there are multiple GPU nodes, set this to the index of the node
        ///                   (the device's physical adapter) for which the video memory
        ///                   information is being set.
        ///  * `memory_segment_group` - Specifies a [`DXGI_MEMORY_SEGMENT_GROUP`] that identifies
        ///                             the group as local or non-local.
        ///  * `reservation` - Specifies a [`UINT64`] that sets the minimum required physical
        ///                    memory, in bytes.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// Applications are encouraged to set a video reservation to denote the amount of physical
        /// memory they cannot go without. This value helps the OS quickly minimize the impact of
        /// large memory pressure situations.
        fn set_video_memory_reservation(
            &mut self,
            node_index: UINT,
            memory_segment_group: DXGI_MEMORY_SEGMENT_GROUP,
            reservation: UINT64
        ) -> HRESULT;

        /// This method establishes a correlation between a CPU synchronization object and the
        /// budget change event.
        ///
        /// # Parameters
        ///  * `event` - A handle to the event object that the operating system sets when memory
        ///              budgets change. The [`CreateEvent`] and [`OpenEvent`] functions return
        ///              this handle.
        ///  * `cookie` - A pointer to a key value that you can pass to the
        ///               [`IDXGIAdapter3::unregister_video_memory_budget_change_notification`]
        ///               method to unregister the notification event that `event` specifies.
        ///
        /// # Return Value
        /// This method returns an [`HRESULT`] success or error code.
        ///
        /// # Remarks
        /// Instead of calling [`IDXGIAdapter3::query_video_memory_info`] regularly, applications
        /// can use CPU synchronization objects to efficiently wake threads when budget changes
        /// occur.
        fn register_video_memory_budget_change_notification_event(
            &mut self,
            event: HANDLE,
            cookie: *mut DWORD
        ) -> HRESULT;

        /// This method stops notifying a CPU synchronization object whenever a budget change
        /// occurs. An application may switch back to polling the information regularly.
        ///
        /// # Parameters
        ///  * `cookie` - A key value for the window or event to unregister. The
        ///               [`IDXGIAdapter3::register_video_memory_budget_change_notification_event`]
        ///               method returns this value.
        ///
        /// # Remarks
        /// An application may switch back to polling for the information regularly.
        fn unregister_video_memory_budget_change_notification(&mut self, cookie: DWORD);
    }
);
