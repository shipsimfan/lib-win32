use crate::{
    com_interface,
    d3d11sdklayers::{
        D3D11_INFO_QUEUE_FILTER, D3D11_MESSAGE, D3D11_MESSAGE_CATEGORY, D3D11_MESSAGE_ID,
        D3D11_MESSAGE_SEVERITY,
    },
    unknwn::{IUnknown, IUnknownTrait},
    BOOL, HRESULT, LPCSTR, SIZE_T, UINT, UINT64,
};

com_interface!(
    /// An information-queue interface stores, retrieves, and filters debug messages. The queue
    /// consists of a message queue, an optional storage filter stack, and a optional retrieval
    /// filter stack.
    ///
    /// # Remarks
    /// To get this interface, turn on debug layer and use [`IUnknown::query_interface`] from the
    /// [`ID3D11Device`].
    pub abstract ID3D11InfoQueue(ID3D11InfoQueueVTable/ID3D11InfoQueueTrait):
        IUnknown/IUnknownTrait(unknown) {
        const IID = 0x6543DBB6-0x1B48-0x42F5-0xAB82-0xE97EC74326F6;

        fn set_message_count_limit(&mut self, message_count_limit: UINT64) -> HRESULT;

        fn clear_stored_messages(&mut self);

        fn get_message(
            &mut self,
            message_index: UINT64,
            message: *mut D3D11_MESSAGE,
            message_byte_length: *mut SIZE_T
        ) -> HRESULT;

        fn get_num_messages_allowed_by_storage_filter(&mut self) -> UINT64;

        fn get_num_messages_denied_by_storage_filter(&mut self) -> UINT64;

        fn get_num_stored_messages(&mut self) -> UINT64;

        fn get_num_stored_messages_allowed_by_retrieval_filter(&mut self) -> UINT64;

        fn get_num_messages_discarded_by_message_count_limit(&mut self) -> UINT64;

        fn get_message_count_limit(&mut self) -> UINT64;

        fn add_storage_filter_entries(&mut self, filter: *mut D3D11_INFO_QUEUE_FILTER) -> HRESULT;

        fn get_storage_filter(
            &mut self,
            filter: *mut D3D11_INFO_QUEUE_FILTER,
            filter_byte_length: *mut SIZE_T
        ) -> HRESULT;

        fn clear_storage_filter(&mut self);

        fn push_empty_storage_filter(&mut self) -> HRESULT;

        fn push_copy_of_storage_filter(&mut self) -> HRESULT;

        fn push_storage_filter(&mut self, filter: *mut D3D11_INFO_QUEUE_FILTER) -> HRESULT;

        fn pop_storage_filter(&mut self);

        fn get_storage_filter_stack_size(&mut self) -> UINT;

        fn add_retrieval_filter_entries(
            &mut self,
            filter: *mut D3D11_INFO_QUEUE_FILTER
        ) -> HRESULT;

        fn get_retrieval_filter(
            &mut self,
            filter: *mut D3D11_INFO_QUEUE_FILTER,
            filter_byte_length: *mut SIZE_T
        ) -> HRESULT;

        fn clear_retrieval_filter(&mut self);

        fn push_empty_retrieval_filter(&mut self) -> HRESULT;

        fn push_copy_of_retrieval_filter(&mut self) -> HRESULT;

        fn push_retrieval_filter(&mut self, filter: *mut D3D11_INFO_QUEUE_FILTER) -> HRESULT;

        fn pop_retrieval_filter(&mut self);

        fn get_retrieval_filter_stack_size(&mut self) -> UINT;

        fn add_message(
            &mut self,
            category: D3D11_MESSAGE_CATEGORY,
            severity: D3D11_MESSAGE_SEVERITY,
            id: D3D11_MESSAGE_ID,
            description: LPCSTR
        ) -> HRESULT;

        fn add_application_message(
            &mut self,
            severity: D3D11_MESSAGE_SEVERITY,
            description: LPCSTR
        ) -> HRESULT;

        fn set_break_on_category(
            &mut self,
            category: D3D11_MESSAGE_CATEGORY,
            enable: BOOL
        ) -> HRESULT;

        fn set_break_on_severity(
            &mut self,
            severity: D3D11_MESSAGE_SEVERITY,
            enable: BOOL
        ) -> HRESULT;

        fn set_break_on_id(&mut self, id: D3D11_MESSAGE_ID, enable: BOOL) -> HRESULT;

        fn get_break_on_category(&mut self, category: D3D11_MESSAGE_CATEGORY) -> BOOL;

        fn get_break_on_severity(&mut self, severity: D3D11_MESSAGE_SEVERITY) -> BOOL;

        fn set_mut_debug_output(&mut self, mute: BOOL);

        fn get_mute_debug_output(&mut self) -> BOOL;
    }
);
