use crate::{
    com_interface,
    d3d11sdklayers::{
        D3D11_INFO_QUEUE_FILTER, D3D11_MESSAGE, D3D11_MESSAGE_CATEGORY, D3D11_MESSAGE_ID,
        D3D11_MESSAGE_SEVERITY,
    },
    unknwn::IUnknown,
    BOOL, HRESULT, LPCSTR, SIZE_T, UINT, UINT64,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::ID3D11Device, FALSE, TRUE};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// An information-queue interface stores, retrieves, and filters debug messages. The queue
    /// consists of a message queue, an optional storage filter stack, and a optional retrieval
    /// filter stack.
    ///
    /// # Remarks
    /// To get this interface, turn on debug layer and use [`IUnknown::query_interface`] from the
    /// [`ID3D11Device`].
    pub abstract ID3D11InfoQueue(ID3D11InfoQueueVTable): IUnknown(unknown) {
        const IID = 0x6543DBB6-0x1B48-0x42F5-0xAB82-0xE97EC74326F6;

        /// Set the maximum number of messages that can be added to the message queue.
        ///
        /// # Parameters
        ///  * `message_count_limit` - Maximum number of messages that can be added to the message
        ///                            queue. -1 means no limit.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// When the number of messages in the message queue has reached the maximum limit, new
        /// messages coming in will push old messages out.
        fn set_message_count_limit(&mut self, message_count_limit: UINT64) -> HRESULT;

        /// Clear all messages from the message queue.
        fn clear_stored_messages(&mut self);

        /// Get a message from the message queue.
        ///
        /// # Parameters
        ///  * `message_index` - Index into message queue after an optional retrieval filter has
        ///                      been applied. This can be between 0 and the number of messages in
        ///                      the message queue that pass through the retrieval filter (which
        ///                      can be obtained with
        ///              [`ID3D11InfoQueue::get_num_stored_messages_allowed_by_retrieval_filter`]).
        ///                      0 is the message at the front of the message queue.
        ///  * `message` - Returned message (see [`D3D11_MESSAGE`]).
        ///  * `message_byte_length` - Size of `message` in bytes, including the size of the
        ///                            message string that the `message` points to.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// This method does not remove any messages from the message queue.
        ///
        /// This method gets messages from the message queue after an optional retrieval filter has
        /// been applied.
        ///
        /// Applications should call this method twice to retrieve a message - first to obtain the
        /// size of the message and second to get the message.
        fn get_message(
            &mut self,
            message_index: UINT64,
            message: *mut D3D11_MESSAGE,
            message_byte_length: *mut SIZE_T
        ) -> HRESULT;

        /// Get the number of messages that were allowed to pass through a storage filter.
        ///
        /// # Return Value
        /// Number of messages allowed by a storage filter.
        fn get_num_messages_allowed_by_storage_filter(&mut self) -> UINT64;

        /// Number of messages allowed by a storage filter.
        ///
        /// # Return Value
        /// Number of messages denied by a storage filter.
        fn get_num_messages_denied_by_storage_filter(&mut self) -> UINT64;

        /// Get the number of messages currently stored in the message queue.
        ///
        /// # Return Value
        /// Number of messages currently stored in the message queue.
        fn get_num_stored_messages(&mut self) -> UINT64;

        /// Get the number of messages that are able to pass through a retrieval filter.
        ///
        /// # Return Value
        /// Number of messages allowed by a retrieval filter.
        fn get_num_stored_messages_allowed_by_retrieval_filter(&mut self) -> UINT64;

        /// Get the number of messages that were discarded due to the message count limit.
        ///
        /// # Return Value
        /// Number of messages discarded.
        ///
        /// # Remarks
        /// Get and set the message count limit with [`ID3D11InfoQueue::get_message_count_limit`]
        /// and [`ID3D11InfoQueue::set_message_count_limit`], respectively.
        fn get_num_messages_discarded_by_message_count_limit(&mut self) -> UINT64;

        /// Get the maximum number of messages that can be added to the message queue.
        ///
        /// # Return Value
        /// Maximum number of messages that can be added to the queue. -1 means no limit.
        ///
        /// # Remarks
        /// When the number of messages in the message queue has reached the maximum limit, new
        /// messages coming in will push old messages out.
        fn get_message_count_limit(&mut self) -> UINT64;

        /// Add storage filters to the top of the storage-filter stack.
        ///
        /// # Parameters
        ///  * `filter` - Array of storage filters (see [`D3D11_INFO_QUEUE_FILTER`]).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn add_storage_filter_entries(&mut self, filter: *mut D3D11_INFO_QUEUE_FILTER) -> HRESULT;

        /// Get the storage filter at the top of the storage-filter stack.
        ///
        /// # Parameters
        ///  * `filter` - Storage filter at the top of the storage-filter stack.
        ///  * `filter_byte_length` - Size of the storage filter in bytes. If `filter` is
        ///                           [`null_mut`], the size of the storage filter will be output
        ///                           to this parameter.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn get_storage_filter(
            &mut self,
            filter: *mut D3D11_INFO_QUEUE_FILTER,
            filter_byte_length: *mut SIZE_T
        ) -> HRESULT;

        /// Remove a storage filter from the top of the storage-filter stack.
        fn clear_storage_filter(&mut self);

        /// Push an empty storage filter onto the storage-filter stack.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// An empty storage filter allows all messages to pass through.
        fn push_empty_storage_filter(&mut self) -> HRESULT;

        /// Push a copy of storage filter currently on the top of the storage-filter stack onto the
        /// storage-filter stack.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn push_copy_of_storage_filter(&mut self) -> HRESULT;

        /// Push a storage filter onto the storage-filter stack.
        ///
        /// # Parameters
        ///  * `filter` - Pointer to a storage filter (see [`D3D11_INFO_QUEUE_FILTER`]).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn push_storage_filter(&mut self, filter: *mut D3D11_INFO_QUEUE_FILTER) -> HRESULT;

        /// Pop a storage filter from the top of the storage-filter stack.
        fn pop_storage_filter(&mut self);

        /// Get the size of the storage-filter stack in bytes.
        ///
        /// # Return Value
        /// Size of the storage-filter stack in bytes.
        fn get_storage_filter_stack_size(&mut self) -> UINT;

        /// Add storage filters to the top of the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `filter` - Array of retrieval filters (see [`D3D11_INFO_QUEUE_FILTER`]).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn add_retrieval_filter_entries(
            &mut self,
            filter: *mut D3D11_INFO_QUEUE_FILTER
        ) -> HRESULT;

        /// Get the retrieval filter at the top of the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `filter` - Retrieval filter at the top of the retrieval-filter stack.
        ///  * `filter_byte_length` - Size of the retrieval filter in bytes. If `filter` is
        ///                           [`null_mut`], the size of the retrieval filter will be output
        ///                           to this parameter.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn get_retrieval_filter(
            &mut self,
            filter: *mut D3D11_INFO_QUEUE_FILTER,
            filter_byte_length: *mut SIZE_T
        ) -> HRESULT;

        /// Remove a retrieval filter from the top of the retrieval-filter stack.
        fn clear_retrieval_filter(&mut self);

        /// Push an empty retrieval filter onto the retrieval-filter stack.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// An empty retrieval filter allows all messages to pass through.
        fn push_empty_retrieval_filter(&mut self) -> HRESULT;

        /// Push a copy of retrieval filter currently on the top of the retrieval-filter stack onto
        /// the retrieval-filter stack.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn push_copy_of_retrieval_filter(&mut self) -> HRESULT;

        /// Push a retrieval filter onto the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `filter` - Pointer to a retrieval filter (see [`D3D11_INFO_QUEUE_FILTER`]).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn push_retrieval_filter(&mut self, filter: *mut D3D11_INFO_QUEUE_FILTER) -> HRESULT;

        /// Pop a retrieval filter from the top of the retrieval-filter stack.
        fn pop_retrieval_filter(&mut self);

        /// Get the size of the retrieval-filter stack in bytes.
        ///
        /// # Return Value
        /// Size of the retrieval-filter stack in bytes.
        fn get_retrieval_filter_stack_size(&mut self) -> UINT;

        /// Add a debug message to the message queue and send that message to debug output.
        ///
        /// # Parameters
        ///  * `category` - Category of a message (see [`D3D11_MESSAGE_CATEGORY`]).
        ///  * `severity` - Severity of a message (see [`D3D11_MESSAGE_SEVERITY`]).
        ///  * `id` - Unique identifier of a message (see [`D3D11_MESSAGE_ID`]).
        ///  * `description` - User-defined message.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// This method is used by the runtime's internal mechanisms to add debug messages to the
        /// message queue and send them to debug output. For applications to add their own custom
        /// messages to the message queue and send them to debug output, call
        /// [`ID3D11InfoQueue::add_application_message`].
        fn add_message(
            &mut self,
            category: D3D11_MESSAGE_CATEGORY,
            severity: D3D11_MESSAGE_SEVERITY,
            id: D3D11_MESSAGE_ID,
            description: LPCSTR
        ) -> HRESULT;

        /// Add a user-defined message to the message queue and send that message to debug output.
        ///
        /// # Parameters
        ///  * `severity` - Severity of a message (see [`D3D11_MESSAGE_SEVERITY`]).
        ///  * `description` - Message string.
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn add_application_message(
            &mut self,
            severity: D3D11_MESSAGE_SEVERITY,
            description: LPCSTR
        ) -> HRESULT;

        /// Set a message category to break on when a message with that category passes through the
        /// storage filter.
        ///
        /// # Parameters
        ///  * `category` - Message category to break on (see [`D3D11_MESSAGE_CATEGORY`]).
        ///  * `enable` - Turns this breaking condition on or off (true for on, false for off).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn set_break_on_category(
            &mut self,
            category: D3D11_MESSAGE_CATEGORY,
            enable: BOOL
        ) -> HRESULT;

        /// Set a message severity level to break on when a message with that severity level passes
        /// through the storage filter.
        ///
        /// # Parameters
        ///  * `severity` - A [`D3D11_MESSAGE_SEVERITY`], which represents a message severity level
        ///                 to break on.
        ///  * `enable` - Turns this breaking condition on or off (true for on, false for off).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn set_break_on_severity(
            &mut self,
            severity: D3D11_MESSAGE_SEVERITY,
            enable: BOOL
        ) -> HRESULT;

        /// Set a message identifier to break on when a message with that identifier passes through
        /// the storage filter.
        ///
        /// # Parameters
        ///  * `id` - Message identifier to break on (see [`D3D11_MESSAGE_ID`]).
        ///  * `enable` - Turns this breaking condition on or off (true for on, false for off).
        ///
        /// # Return Value
        /// This method returns one of the Direct3D 11 Return Codes.
        fn set_break_on_id(&mut self, id: D3D11_MESSAGE_ID, enable: BOOL) -> HRESULT;

        /// Get a message category to break on when a message with that category passes through the
        /// storage filter.
        ///
        /// # Parameters
        ///  * `category` - Message category to break on (see [`D3D11_MESSAGE_CATEGORY`]).
        ///
        /// # Return Value
        /// Whether this breaking condition is turned on or off (true for on, false for off).
        fn get_break_on_category(&mut self, category: D3D11_MESSAGE_CATEGORY) -> BOOL;

        /// Get a message severity level to break on when a message with that severity level passes
        /// through the storage filter.
        ///
        /// # Parameters
        ///  * `severity` - Message severity level to break on (see [`D3D11_MESSAGE_SEVERITY`]).
        ///
        /// # Return Value
        /// Whether this breaking condition is turned on or off (true for on, false for off).
        fn get_break_on_severity(&mut self, severity: D3D11_MESSAGE_SEVERITY) -> BOOL;

        /// Get a message identifier to break on when a message with that identifier passes through
        /// the storage filter.
        ///
        /// # Parameters
        ///  * `id` - Message identifier to break on (see [`D3D11_MESSAGE_ID`]).
        ///
        /// # Return Value
        /// Whether this breaking condition is turned on or off (true for on, false for off).
        fn get_break_on_id(&mut self, id: D3D11_MESSAGE_ID) -> BOOL;

        /// Set a boolean that turns the debug output on or off.
        ///
        /// # Parameters
        ///  * `mute` - Disable/Enable the debug output ([`TRUE`] to disable or mute the output,
        ///             [`FALSE`] to enable the output).
        ///
        /// # Remarks
        /// This will stop messages that pass the storage filter from being printed out in the
        /// debug output, however those messages will still be added to the message queue.
        fn set_mut_debug_output(&mut self, mute: BOOL);

        /// Get a boolean that turns the debug output on or off.
        ///
        /// # Return Value
        /// Whether the debug output is on or off (true for on, false for off).
        fn get_mute_debug_output(&mut self) -> BOOL;
    }
);
