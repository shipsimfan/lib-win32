use crate::HRESULT;

/// The operation completed successfully.
pub const ERROR_SUCCESS: HRESULT = 0;

/// There was no error
pub const NO_ERROR: HRESULT = 0;

/// There was no error
pub const S_OK: HRESULT = 0;

/// Incorrect function.
pub const ERROR_INVALID_FUNCTION: HRESULT = 1;

/// The system cannot find the file specified.
pub const ERROR_FILE_NOT_FOUND: HRESULT = 2;

/// The system cannot find the path specified.
pub const ERROR_PATH_NOT_FOUND: HRESULT = 3;

/// The system cannot open the file.
pub const ERROR_TOO_MANY_OPEN_FILES: HRESULT = 4;

/// Access is denied.
pub const ERROR_ACCESS_DENIED: HRESULT = 5;

/// The handle is invalid.
pub const ERROR_INVALID_HANDLE: HRESULT = 6;

/// The storage control blocks were destroyed.
pub const ERROR_ARENA_TRASHED: HRESULT = 7;

/// Not enough memory resources are available to process this command.
pub const ERROR_NOT_ENOUGH_MEMORY: HRESULT = 8;

/// The storage control block address is invalid.
pub const ERROR_INVALID_BLOCK: HRESULT = 9;

/// The environment is incorrect.
pub const ERROR_BAD_ENVIRONMENT: HRESULT = 10;

/// An attempt was made to load a program with an incorrect format.
pub const ERROR_BAD_FORMAT: HRESULT = 11;

/// The access code is invalid.
pub const ERROR_INVALID_ACCESS: HRESULT = 12;

/// The data is invalid.
pub const ERROR_INVALID_DATA: HRESULT = 13;

/// Not enough memory resources are available to complete this operation.
pub const ERROR_OUTOFMEMORY: HRESULT = 14;

/// The system cannot find the drive specified.
pub const ERROR_INVALID_DRIVE: HRESULT = 15;

/// The directory cannot be removed.
pub const ERROR_CURRENT_DIRECTORY: HRESULT = 16;

/// The system cannot move the file to a different disk drive.
pub const ERROR_NOT_SAME_DEVICE: HRESULT = 17;

/// There are no more files.
pub const ERROR_NO_MORE_FILES: HRESULT = 18;

/// The media is write protected.
pub const ERROR_WRITE_PROTECT: HRESULT = 19;

/// The system cannot find the device specified.
pub const ERROR_BAD_UNIT: HRESULT = 20;

/// The device is not ready.
pub const ERROR_NOT_READY: HRESULT = 21;

/// The device does not recognize the command.
pub const ERROR_BAD_COMMAND: HRESULT = 22;

/// Data error (cyclic redundancy check).
pub const ERROR_CRC: HRESULT = 23;

/// The program issued a command but the command length is incorrect.
pub const ERROR_BAD_LENGTH: HRESULT = 24;

/// The drive cannot locate a specific area or track on the disk.
pub const ERROR_SEEK: HRESULT = 25;

/// The specified disk or diskette cannot be accessed.
pub const ERROR_NOT_DOS_DISK: HRESULT = 26;

/// The drive cannot find the sector requested.
pub const ERROR_SECTOR_NOT_FOUND: HRESULT = 27;

/// The printer is out of paper.
pub const ERROR_OUT_OF_PAPER: HRESULT = 28;

/// The system cannot write to the specified device.
pub const ERROR_WRITE_FAULT: HRESULT = 29;

/// The system cannot read from the specified device.
pub const ERROR_READ_FAULT: HRESULT = 30;

/// A device attached to the system is not functioning.
pub const ERROR_GEN_FAILURE: HRESULT = 31;

/// The process cannot access the file because it is being used by another process.
pub const ERROR_SHARING_VIOLATION: HRESULT = 32;

/// The process cannot access the file because another process has locked a portion of the file.
pub const ERROR_LOCK_VIOLATION: HRESULT = 33;

///  The wrong diskette is in the drive.
///
/// Insert %2 (Volume Serial Number: %3) into drive %1.
pub const ERROR_WRONG_DISK: HRESULT = 34;

/// Too many files opened for sharing.
pub const ERROR_SHARING_BUFFER_EXCEEDED: HRESULT = 36;

/// Reached the end of the file.
pub const ERROR_HANDLE_EOF: HRESULT = 38;

/// The disk is full.
pub const ERROR_HANDLE_DISK_FULL: HRESULT = 39;

/// The request is not supported.
pub const ERROR_NOT_SUPPORTED: HRESULT = 50;

/// Windows cannot find the network path. Verify that the network path is correct and the destination computer is not busy or turned off. If Windows still cannot find the network path, contact your network administrator.
pub const ERROR_REM_NOT_LIST: HRESULT = 51;

/// You were not connected because a duplicate name exists on the network. If joining a domain, go to System in Control Panel to change the computer name and try again. If joining a workgroup, choose another workgroup name.
pub const ERROR_DUP_NAME: HRESULT = 52;

/// The network path was not found.
pub const ERROR_BAD_NETPATH: HRESULT = 53;

/// The network is busy.
pub const ERROR_NETWORK_BUSY: HRESULT = 54;

/// The specified network resource or device is no longer available.
pub const ERROR_DEV_NOT_EXIST: HRESULT = 55;

/// The network BIOS command limit has been reached.
pub const ERROR_TOO_MANY_CMDS: HRESULT = 56;

/// A network adapter hardware error occurred.
pub const ERROR_ADAP_HDW_ERR: HRESULT = 57;

/// The specified server cannot perform the requested operation.
pub const ERROR_BAD_NET_RESP: HRESULT = 58;

/// An unexpected network error occurred.
pub const ERROR_UNEXP_NET_ERR: HRESULT = 59;

/// The remote adapter is not compatible.
pub const ERROR_BAD_REM_ADAP: HRESULT = 60;

/// The printer queue is full.
pub const ERROR_PRINTQ_FULL: HRESULT = 61;

/// Space to store the file waiting to be printed is not available on the server.
pub const ERROR_NO_SPOOL_SPACE: HRESULT = 62;

/// Your file waiting to be printed was deleted.
pub const ERROR_PRINT_CANCELLED: HRESULT = 63;

/// The specified network name is no longer available.
pub const ERROR_NETNAME_DELETED: HRESULT = 64;

/// Network access is denied.
pub const ERROR_NETWORK_ACCESS_DENIED: HRESULT = 65;

/// The network resource type is not correct.
pub const ERROR_BAD_DEV_TYPE: HRESULT = 66;

/// The network name cannot be found.
pub const ERROR_BAD_NET_NAME: HRESULT = 67;

/// The name limit for the local computer network adapter card was exceeded.
pub const ERROR_TOO_MANY_NAMES: HRESULT = 68;

/// The network BIOS session limit was exceeded.
pub const ERROR_TOO_MANY_SESS: HRESULT = 69;

/// The remote server has been paused or is in the process of being started.
pub const ERROR_SHARING_PAUSED: HRESULT = 70;

/// No more connections can be made to this remote computer at this time because there are already as many connections as the computer can accept.
pub const ERROR_REQ_NOT_ACCEP: HRESULT = 71;

/// The specified printer or disk device has been paused.
pub const ERROR_REDIR_PAUSED: HRESULT = 72;

/// The file exists.
pub const ERROR_FILE_EXISTS: HRESULT = 80;

/// The directory or file cannot be created.
pub const ERROR_CANNOT_MAKE: HRESULT = 82;

/// Fail on INT 24.
pub const ERROR_FAIL_I24: HRESULT = 83;

/// Storage to process this request is not available.
pub const ERROR_OUT_OF_STRUCTURES: HRESULT = 84;

/// The local device name is already in use.
pub const ERROR_ALREADY_ASSIGNED: HRESULT = 85;

/// The specified network password is not correct.
pub const ERROR_INVALID_PASSWORD: HRESULT = 86;

/// The parameter is incorrect.
pub const ERROR_INVALID_PARAMETER: HRESULT = 87;

/// A write fault occurred on the network.
pub const ERROR_NET_WRITE_FAULT: HRESULT = 88;

/// The system cannot start another process at this time.
pub const ERROR_NO_PROC_SLOTS: HRESULT = 89;

/// Cannot create another system semaphore.
pub const ERROR_TOO_MANY_SEMAPHORES: HRESULT = 100;

/// The exclusive semaphore is owned by another process.
pub const ERROR_EXCL_SEM_ALREADY_OWNED: HRESULT = 101;

/// The semaphore is set and cannot be closed.
pub const ERROR_SEM_IS_SET: HRESULT = 102;

/// The semaphore cannot be set again.
pub const ERROR_TOO_MANY_SEM_REQUESTS: HRESULT = 103;

/// Cannot request exclusive semaphores at interrupt time.
pub const ERROR_INVALID_AT_INTERRUPT_TIME: HRESULT = 104;

/// The previous ownership of this semaphore has ended.
pub const ERROR_SEM_OWNER_DIED: HRESULT = 105;

/// Insert the diskette for drive %1.
pub const ERROR_SEM_USER_LIMIT: HRESULT = 106;

/// The program stopped because an alternate diskette was not inserted.
pub const ERROR_DISK_CHANGE: HRESULT = 107;

/// The disk is in use or locked by another process.
pub const ERROR_DRIVE_LOCKED: HRESULT = 108;

/// The pipe has been ended.
pub const ERROR_BROKEN_PIPE: HRESULT = 109;

/// The system cannot open the device or file specified.
pub const ERROR_OPEN_FAILED: HRESULT = 110;

/// The file name is too long.
pub const ERROR_BUFFER_OVERFLOW: HRESULT = 111;

/// There is not enough space on the disk.
pub const ERROR_DISK_FULL: HRESULT = 112;

/// No more internal file identifiers available.
pub const ERROR_NO_MORE_SEARCH_HANDLES: HRESULT = 113;

/// The target internal file identifier is incorrect.
pub const ERROR_INVALID_TARGET_HANDLE: HRESULT = 114;

/// The IOCTL call made by the application program is not correct.
pub const ERROR_INVALID_CATEGORY: HRESULT = 117;

/// The verify-on-write switch parameter value is not correct.
pub const ERROR_INVALID_VERIFY_SWITCH: HRESULT = 118;

/// The system does not support the command requested.
pub const ERROR_BAD_DRIVER_LEVEL: HRESULT = 119;

/// This function is not supported on this system.
pub const ERROR_CALL_NOT_IMPLEMENTED: HRESULT = 120;

/// The semaphore timeout period has expired.
pub const ERROR_SEM_TIMEOUT: HRESULT = 121;

/// The data area passed to a system call is too small.
pub const ERROR_INSUFFICIENT_BUFFER: HRESULT = 122;

/// The filename, directory name, or volume label syntax is incorrect.
pub const ERROR_INVALID_NAME: HRESULT = 123;

/// The system call level is not correct.
pub const ERROR_INVALID_LEVEL: HRESULT = 124;

/// The disk has no volume label.
pub const ERROR_NO_VOLUME_LABEL: HRESULT = 125;

/// The specified module could not be found.
pub const ERROR_MOD_NOT_FOUND: HRESULT = 126;

/// The specified procedure could not be found.
pub const ERROR_PROC_NOT_FOUND: HRESULT = 127;

/// There are no child processes to wait for.
pub const ERROR_WAIT_NO_CHILDREN: HRESULT = 128;

/// The %1 application cannot be run in Win32 mode.
pub const ERROR_CHILD_NOT_COMPLETE: HRESULT = 129;

/// Attempt to use a file handle to an open disk partition for an operation other than raw disk I/O.
pub const ERROR_DIRECT_ACCESS_HANDLE: HRESULT = 130;

/// An attempt was made to move the file pointer before the beginning of the file.
pub const ERROR_NEGATIVE_SEEK: HRESULT = 131;

/// The file pointer cannot be set on the specified device or file.
pub const ERROR_SEEK_ON_DEVICE: HRESULT = 132;

/// A JOIN or SUBST command cannot be used for a drive that contains previously joined drives.
pub const ERROR_IS_JOIN_TARGET: HRESULT = 133;

/// An attempt was made to use a JOIN or SUBST command on a drive that has already been joined.
pub const ERROR_IS_JOINED: HRESULT = 134;

/// An attempt was made to use a JOIN or SUBST command on a drive that has already been substituted.
pub const ERROR_IS_SUBSTED: HRESULT = 135;

/// The system tried to delete the JOIN of a drive that is not joined.
pub const ERROR_NOT_JOINED: HRESULT = 136;

/// The system tried to delete the substitution of a drive that is not substituted.
pub const ERROR_NOT_SUBSTED: HRESULT = 137;

/// The system tried to join a drive to a directory on a joined drive.
pub const ERROR_JOIN_TO_JOIN: HRESULT = 138;

/// The system tried to substitute a drive to a directory on a substituted drive.
pub const ERROR_SUBST_TO_SUBST: HRESULT = 139;

/// The system tried to join a drive to a directory on a substituted drive.
pub const ERROR_JOIN_TO_SUBST: HRESULT = 140;

/// The system tried to SUBST a drive to a directory on a joined drive.
pub const ERROR_SUBST_TO_JOIN: HRESULT = 141;

/// The system cannot perform a JOIN or SUBST at this time.
pub const ERROR_BUSY_DRIVE: HRESULT = 142;

/// The system cannot join or substitute a drive to or for a directory on the same drive.
pub const ERROR_SAME_DRIVE: HRESULT = 143;

/// The directory is not a subdirectory of the root directory.
pub const ERROR_DIR_NOT_ROOT: HRESULT = 144;

/// The directory is not empty.
pub const ERROR_DIR_NOT_EMPTY: HRESULT = 145;

/// The path specified is being used in a substitute.
pub const ERROR_IS_SUBST_PATH: HRESULT = 146;

/// Not enough resources are available to process this command.
pub const ERROR_IS_JOIN_PATH: HRESULT = 147;

/// The path specified cannot be used at this time.
pub const ERROR_PATH_BUSY: HRESULT = 148;

/// An attempt was made to join or substitute a drive for which a directory on the drive is the target of a previous substitute.
pub const ERROR_IS_SUBST_TARGET: HRESULT = 149;

/// System trace information was not specified in your CONFIG.SYS file, or tracing is disallowed.
pub const ERROR_SYSTEM_TRACE: HRESULT = 150;

/// The number of specified semaphore events for DosMuxSemWait is not correct.
pub const ERROR_INVALID_EVENT_COUNT: HRESULT = 151;

/// DosMuxSemWait did not execute; too many semaphores are already set.
pub const ERROR_TOO_MANY_MUXWAITERS: HRESULT = 152;

/// The DosMuxSemWait list is not correct.
pub const ERROR_INVALID_LIST_FORMAT: HRESULT = 153;

/// The volume label you entered exceeds the label character limit of the target file system.
pub const ERROR_LABEL_TOO_LONG: HRESULT = 154;

/// Cannot create another thread.
pub const ERROR_TOO_MANY_TCBS: HRESULT = 155;

/// The recipient process has refused the signal.
pub const ERROR_SIGNAL_REFUSED: HRESULT = 156;

/// The segment is already discarded and cannot be locked.
pub const ERROR_DISCARDED: HRESULT = 157;

/// The segment is already unlocked.
pub const ERROR_NOT_LOCKED: HRESULT = 158;

/// The address for the thread ID is not correct.
pub const ERROR_BAD_THREADID_ADDR: HRESULT = 159;

/// One or more arguments are not correct.
pub const ERROR_BAD_ARGUMENTS: HRESULT = 160;

/// The specified path is invalid.
pub const ERROR_BAD_PATHNAME: HRESULT = 161;

/// A signal is already pending.
pub const ERROR_SIGNAL_PENDING: HRESULT = 162;

/// No more threads can be created in the system.
pub const ERROR_MAX_THRDS_REACHED: HRESULT = 164;

/// Unable to lock a region of a file.
pub const ERROR_LOCK_FAILED: HRESULT = 167;

/// The requested resource is in use.
pub const ERROR_BUSY: HRESULT = 170;

/// Device's command support detection is in progress.
pub const ERROR_DEVICE_SUPPORT_IN_PROGRESS: HRESULT = 171;

/// A lock request was not outstanding for the supplied cancel region.
pub const ERROR_CANCEL_VIOLATION: HRESULT = 173;

/// The file system does not support atomic changes to the lock type.
pub const ERROR_ATOMIC_LOCKS_NOT_SUPPORTED: HRESULT = 174;

/// The system detected a segment number that was not correct.
pub const ERROR_INVALID_SEGMENT_NUMBER: HRESULT = 180;

/// The operating system cannot run %1.
pub const ERROR_INVALID_ORDINAL: HRESULT = 182;

/// Cannot create a file when that file already exists.
pub const ERROR_ALREADY_EXISTS: HRESULT = 183;

/// The flag passed is not correct.
pub const ERROR_INVALID_FLAG_NUMBER: HRESULT = 186;

/// The specified system semaphore name was not found.
pub const ERROR_SEM_NOT_FOUND: HRESULT = 187;

/// The operating system cannot run %1.
pub const ERROR_INVALID_STARTING_CODESEG: HRESULT = 188;

/// The operating system cannot run %1.
pub const ERROR_INVALID_STACKSEG: HRESULT = 189;

/// The operating system cannot run %1.
pub const ERROR_INVALID_MODULETYPE: HRESULT = 190;

/// Cannot run %1 in Win32 mode.
pub const ERROR_INVALID_EXE_SIGNATURE: HRESULT = 191;

/// The operating system cannot run %1.
pub const ERROR_EXE_MARKED_INVALID: HRESULT = 192;

/// %1 is not a valid Win32 application.
pub const ERROR_BAD_EXE_FORMAT: HRESULT = 193;

/// The operating system cannot run %1.
pub const ERROR_ITERATED_DATA_EXCEEDS_64K: HRESULT = 194;

/// The operating system cannot run %1.
pub const ERROR_INVALID_MINALLOCSIZE: HRESULT = 195;

/// The operating system cannot run this application program.
pub const ERROR_DYNLINK_FROM_INVALID_RING: HRESULT = 196;

/// The operating system is not presently configured to run this application.
pub const ERROR_IOPL_NOT_ENABLED: HRESULT = 197;

/// The operating system cannot run %1.
pub const ERROR_INVALID_SEGDPL: HRESULT = 198;

/// The operating system cannot run this application program.
pub const ERROR_AUTODATASEG_EXCEEDS_64K: HRESULT = 199;

/// The code segment cannot be greater than or equal to 64K.
pub const ERROR_RING2SEG_MUST_BE_MOVABLE: HRESULT = 200;

/// The operating system cannot run %1.
pub const ERROR_RELOC_CHAIN_XEEDS_SEGLIM: HRESULT = 201;

/// The operating system cannot run %1.
pub const ERROR_INFLOOP_IN_RELOC_CHAIN: HRESULT = 202;

/// The system could not find the environment option that was entered.
pub const ERROR_ENVVAR_NOT_FOUND: HRESULT = 203;

/// No process in the command subtree has a signal handler.
pub const ERROR_NO_SIGNAL_SENT: HRESULT = 205;

/// The filename or extension is too long.
pub const ERROR_FILENAME_EXCED_RANGE: HRESULT = 206;

/// The ring 2 stack is in use.
pub const ERROR_RING2_STACK_IN_USE: HRESULT = 207;

/// The global filename characters, * or ?, are entered incorrectly or too many global filename characters are specified.
pub const ERROR_META_EXPANSION_TOO_LONG: HRESULT = 208;

/// The signal being posted is not correct.
pub const ERROR_INVALID_SIGNAL_NUMBER: HRESULT = 209;

/// The signal handler cannot be set.
pub const ERROR_THREAD_1_INACTIVE: HRESULT = 210;

/// The segment is locked and cannot be reallocated.
pub const ERROR_LOCKED: HRESULT = 212;

/// Too many dynamic-link modules are attached to this program or dynamic-link module.
pub const ERROR_TOO_MANY_MODULES: HRESULT = 214;

/// Cannot nest calls to LoadModule.
pub const ERROR_NESTING_NOT_ALLOWED: HRESULT = 215;

/// This version of %1 is not compatible with the version of Windows you're running. Check your computer's system information and then contact the software publisher.
pub const ERROR_EXE_MACHINE_TYPE_MISMATCH: HRESULT = 216;

/// The image file %1 is signed, unable to modify.
pub const ERROR_EXE_CANNOT_MODIFY_SIGNED_BINARY: HRESULT = 217;

/// The image file %1 is strong signed, unable to modify.
pub const ERROR_EXE_CANNOT_MODIFY_STRONG_SIGNED_BINARY: HRESULT = 218;

/// This file is checked out or locked for editing by another user.
pub const ERROR_FILE_CHECKED_OUT: HRESULT = 220;

/// The file must be checked out before saving changes.
pub const ERROR_CHECKOUT_REQUIRED: HRESULT = 221;

/// The file type being saved or retrieved has been blocked.
pub const ERROR_BAD_FILE_TYPE: HRESULT = 222;

/// The file size exceeds the limit allowed and cannot be saved.
pub const ERROR_FILE_TOO_LARGE: HRESULT = 223;

/// Access Denied. Before opening files in this location, you must first add the web site to your trusted sites list, browse to the web site, and select the option to login automatically.
pub const ERROR_FORMS_AUTH_REQUIRED: HRESULT = 224;

/// Operation did not complete successfully because the file contains a virus or potentially unwanted software.
pub const ERROR_VIRUS_INFECTED: HRESULT = 225;

/// This file contains a virus or potentially unwanted software and cannot be opened. Due to the nature of this virus or potentially unwanted software, the file has been removed from this location.
pub const ERROR_VIRUS_DELETED: HRESULT = 226;

/// The pipe is local.
pub const ERROR_PIPE_LOCAL: HRESULT = 229;

/// The pipe state is invalid.
pub const ERROR_BAD_PIPE: HRESULT = 230;

/// All pipe instances are busy.
pub const ERROR_PIPE_BUSY: HRESULT = 231;

/// The pipe is being closed.
pub const ERROR_NO_DATA: HRESULT = 232;

/// No process is on the other end of the pipe.
pub const ERROR_PIPE_NOT_CONNECTED: HRESULT = 233;

/// More data is available.
pub const ERROR_MORE_DATA: HRESULT = 234;

/// The action requested resulted in no work being done. Error-style clean-up has been performed.
pub const ERROR_NO_WORK_DONE: HRESULT = 235;

/// The session was canceled.
pub const ERROR_VC_DISCONNECTED: HRESULT = 240;

/// The specified extended attribute name was invalid.
pub const ERROR_INVALID_EA_NAME: HRESULT = 254;

/// The extended attributes are inconsistent.
pub const ERROR_EA_LIST_INCONSISTENT: HRESULT = 255;

/// The wait operation timed out.
pub const WAIT_TIMEOUT: HRESULT = 258;

/// No more data is available.
pub const ERROR_NO_MORE_ITEMS: HRESULT = 259;

/// The copy functions cannot be used.
pub const ERROR_CANNOT_COPY: HRESULT = 266;

/// The directory name is invalid.
pub const ERROR_DIRECTORY: HRESULT = 267;

/// The extended attributes did not fit in the buffer.
pub const ERROR_EAS_DIDNT_FIT: HRESULT = 275;

/// The extended attribute file on the mounted file system is corrupt.
pub const ERROR_EA_FILE_CORRUPT: HRESULT = 276;

/// The extended attribute table file is full.
pub const ERROR_EA_TABLE_FULL: HRESULT = 277;

/// The specified extended attribute handle is invalid.
pub const ERROR_INVALID_EA_HANDLE: HRESULT = 278;

/// The mounted file system does not support extended attributes.
pub const ERROR_EAS_NOT_SUPPORTED: HRESULT = 282;

/// Attempt to release mutex not owned by caller.
pub const ERROR_NOT_OWNER: HRESULT = 288;

/// Too many posts were made to a semaphore.
pub const ERROR_TOO_MANY_POSTS: HRESULT = 298;

/// Only part of a ReadProcessMemory or WriteProcessMemory request was completed.
pub const ERROR_PARTIAL_COPY: HRESULT = 299;

/// The oplock request is denied.
pub const ERROR_OPLOCK_NOT_GRANTED: HRESULT = 300;

/// An invalid oplock acknowledgment was received by the system.
pub const ERROR_INVALID_OPLOCK_PROTOCOL: HRESULT = 301;

/// The volume is too fragmented to complete this operation.
pub const ERROR_DISK_TOO_FRAGMENTED: HRESULT = 302;

/// The file cannot be opened because it is in the process of being deleted.
pub const ERROR_DELETE_PENDING: HRESULT = 303;

/// Short name settings may not be changed on this volume due to the global registry setting.
pub const ERROR_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING: HRESULT = 304;

/// Short names are not enabled on this volume.
pub const ERROR_SHORT_NAMES_NOT_ENABLED_ON_VOLUME: HRESULT = 305;

///  The security stream for the given volume is in an inconsistent state.
///
/// Please run CHKDSK on the volume.
pub const ERROR_SECURITY_STREAM_IS_INCONSISTENT: HRESULT = 306;

/// A requested file lock operation cannot be processed due to an invalid byte range.
pub const ERROR_INVALID_LOCK_RANGE: HRESULT = 307;

/// The subsystem needed to support the image type is not present.
pub const ERROR_IMAGE_SUBSYSTEM_NOT_PRESENT: HRESULT = 308;

/// The specified file already has a notification GUID associated with it.
pub const ERROR_NOTIFICATION_GUID_ALREADY_DEFINED: HRESULT = 309;

/// An invalid exception handler routine has been detected.
pub const ERROR_INVALID_EXCEPTION_HANDLER: HRESULT = 310;

/// Duplicate privileges were specified for the token.
pub const ERROR_DUPLICATE_PRIVILEGES: HRESULT = 311;

/// No ranges for the specified operation were able to be processed.
pub const ERROR_NO_RANGES_PROCESSED: HRESULT = 312;

/// Operation is not allowed on a file system internal file.
pub const ERROR_NOT_ALLOWED_ON_SYSTEM_FILE: HRESULT = 313;

/// The physical resources of this disk have been exhausted.
pub const ERROR_DISK_RESOURCES_EXHAUSTED: HRESULT = 314;

/// The token representing the data is invalid.
pub const ERROR_INVALID_TOKEN: HRESULT = 315;

/// The device does not support the command feature.
pub const ERROR_DEVICE_FEATURE_NOT_SUPPORTED: HRESULT = 316;

/// The system cannot find message text for message number 0x%1 in the message file for %2.
pub const ERROR_MR_MID_NOT_FOUND: HRESULT = 317;

/// The scope specified was not found.
pub const ERROR_SCOPE_NOT_FOUND: HRESULT = 318;

/// The Central Access Policy specified is not defined on the target machine.
pub const ERROR_UNDEFINED_SCOPE: HRESULT = 319;

/// The Central Access Policy obtained from Active Directory is invalid.
pub const ERROR_INVALID_CAP: HRESULT = 320;

/// The device is unreachable.
pub const ERROR_DEVICE_UNREACHABLE: HRESULT = 321;

/// The target device has insufficient resources to complete the operation.
pub const ERROR_DEVICE_NO_RESOURCES: HRESULT = 322;

/// A data integrity checksum error occurred. Data in the file stream is corrupt.
pub const ERROR_DATA_CHECKSUM_ERROR: HRESULT = 323;

/// An attempt was made to modify both a KERNEL and normal Extended Attribute (EA) in the same operation.
pub const ERROR_INTERMIXED_KERNEL_EA_OPERATION: HRESULT = 324;

/// Device does not support file-level TRIM.
pub const ERROR_FILE_LEVEL_TRIM_NOT_SUPPORTED: HRESULT = 326;

/// The command specified a data offset that does not align to the device's granularity/alignment.
pub const ERROR_OFFSET_ALIGNMENT_VIOLATION: HRESULT = 327;

/// The command specified an invalid field in its parameter list.
pub const ERROR_INVALID_FIELD_IN_PARAMETER_LIST: HRESULT = 328;

/// An operation is currently in progress with the device.
pub const ERROR_OPERATION_IN_PROGRESS: HRESULT = 329;

/// An attempt was made to send down the command via an invalid path to the target device.
pub const ERROR_BAD_DEVICE_PATH: HRESULT = 330;

/// The command specified a number of descriptors that exceeded the maximum supported by the device.
pub const ERROR_TOO_MANY_DESCRIPTORS: HRESULT = 331;

/// Scrub is disabled on the specified file.
pub const ERROR_SCRUB_DATA_DISABLED: HRESULT = 332;

/// The storage device does not provide redundancy.
pub const ERROR_NOT_REDUNDANT_STORAGE: HRESULT = 333;

/// The specified operation is not supported on a resident file.
pub const ERROR_RESIDENT_FILE_NOT_SUPPORTED: HRESULT = 334;

/// The specified operation is not supported on a compressed file.
pub const ERROR_COMPRESSED_FILE_NOT_SUPPORTED: HRESULT = 335;

/// The specified operation is not supported on a directory.
pub const ERROR_DIRECTORY_NOT_SUPPORTED: HRESULT = 336;

/// The specified copy of the requested data could not be read.
pub const ERROR_NOT_READ_FROM_COPY: HRESULT = 337;

/// The specified data could not be written to any of the copies.
pub const ERROR_FT_WRITE_FAILURE: HRESULT = 338;

/// One or more copies of data on this device may be out of sync. No writes may be performed until a data integrity scan is completed.
pub const ERROR_FT_DI_SCAN_REQUIRED: HRESULT = 339;

/// The supplied kernel information version is invalid.
pub const ERROR_INVALID_KERNEL_INFO_VERSION: HRESULT = 340;

/// The supplied PEP information version is invalid.
pub const ERROR_INVALID_PEP_INFO_VERSION: HRESULT = 341;

/// This object is not externally backed by any provider.
pub const ERROR_OBJECT_NOT_EXTERNALLY_BACKED: HRESULT = 342;

/// The external backing provider is not recognized.
pub const ERROR_EXTERNAL_BACKING_PROVIDER_UNKNOWN: HRESULT = 343;

/// Compressing this object would not save space.
pub const ERROR_COMPRESSION_NOT_BENEFICIAL: HRESULT = 344;

/// The request failed due to a storage topology ID mismatch.
pub const ERROR_STORAGE_TOPOLOGY_ID_MISMATCH: HRESULT = 345;

/// The operation was blocked by parental controls.
pub const ERROR_BLOCKED_BY_PARENTAL_CONTROLS: HRESULT = 346;

/// A file system block being referenced has already reached the maximum reference count and can't be referenced any further.
pub const ERROR_BLOCK_TOO_MANY_REFERENCES: HRESULT = 347;

/// The requested operation failed because the file stream is marked to disallow writes.
pub const ERROR_MARKED_TO_DISALLOW_WRITES: HRESULT = 348;

/// The requested operation failed with an architecture-specific failure code.
pub const ERROR_ENCLAVE_FAILURE: HRESULT = 349;

/// No action was taken as a system reboot is required.
pub const ERROR_FAIL_NOACTION_REBOOT: HRESULT = 350;

/// The shutdown operation failed.
pub const ERROR_FAIL_SHUTDOWN: HRESULT = 351;

/// The restart operation failed.
pub const ERROR_FAIL_RESTART: HRESULT = 352;

/// The maximum number of sessions has been reached.
pub const ERROR_MAX_SESSIONS_REACHED: HRESULT = 353;

/// Windows Information Protection policy does not allow access to this network resource.
pub const ERROR_NETWORK_ACCESS_DENIED_EDP: HRESULT = 354;

/// The device hint name buffer is too small to receive the remaining name.
pub const ERROR_DEVICE_HINT_NAME_BUFFER_TOO_SMALL: HRESULT = 355;

/// The requested operation was blocked by Windows Information Protection policy. For more information, contact your system administrator.
pub const ERROR_EDP_POLICY_DENIES_OPERATION: HRESULT = 356;

/// The requested operation cannot be performed because hardware or software configuration of the device does not comply with Windows Information Protection under Lock policy. Please, verify that user PIN has been created. For more information, contact your system administrator.
pub const ERROR_EDP_DPL_POLICY_CANT_BE_SATISFIED: HRESULT = 357;

/// The cloud sync root metadata is corrupted.
pub const ERROR_CLOUD_FILE_SYNC_ROOT_METADATA_CORRUPT: HRESULT = 358;

/// The device is in maintenance mode.
pub const ERROR_DEVICE_IN_MAINTENANCE: HRESULT = 359;

/// The specified operation is not supported on a DAX volume.
pub const ERROR_NOT_SUPPORTED_ON_DAX: HRESULT = 360;

/// The volume has active DAX mappings.
pub const ERROR_DAX_MAPPING_EXISTS: HRESULT = 361;

/// The cloud file provider is not running.
pub const ERROR_CLOUD_FILE_PROVIDER_NOT_RUNNING: HRESULT = 362;

/// The cloud file metadata is corrupt and unreadable.
pub const ERROR_CLOUD_FILE_METADATA_CORRUPT: HRESULT = 363;

/// The cloud file metadata is too large.
pub const ERROR_CLOUD_FILE_METADATA_TOO_LARGE: HRESULT = 364;

/// The cloud file property is too large.
pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_TOO_LARGE: HRESULT = 365;

/// The cloud file property is possibly corrupt. The on-disk checksum does not match the computed checksum.
pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_CHECKSUM_MISMATCH: HRESULT = 366;

/// The process creation has been blocked.
pub const ERROR_CHILD_PROCESS_BLOCKED: HRESULT = 367;

/// The storage device has lost data or persistence.
pub const ERROR_STORAGE_LOST_DATA_PERSISTENCE: HRESULT = 368;

/// The provider that supports file system virtualization is temporarily unavailable.
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_UNAVAILABLE: HRESULT = 369;

/// The metadata for file system virtualization is corrupt and unreadable.
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_METADATA_CORRUPT: HRESULT = 370;

/// The provider that supports file system virtualization is too busy to complete this operation.
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_BUSY: HRESULT = 371;

/// The provider that supports file system virtualization is unknown.
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_PROVIDER_UNKNOWN: HRESULT = 372;

/// GDI handles were potentially leaked by the application.
pub const ERROR_GDI_HANDLE_LEAK: HRESULT = 373;

/// The maximum number of cloud file properties has been reached.
pub const ERROR_CLOUD_FILE_TOO_MANY_PROPERTY_BLOBS: HRESULT = 374;

/// The version of the cloud file property store is not supported.
pub const ERROR_CLOUD_FILE_PROPERTY_VERSION_NOT_SUPPORTED: HRESULT = 375;

/// The file is not a cloud file.
pub const ERROR_NOT_A_CLOUD_FILE: HRESULT = 376;

/// The file is not in sync with the cloud.
pub const ERROR_CLOUD_FILE_NOT_IN_SYNC: HRESULT = 377;

/// The cloud sync root is already connected with another cloud sync provider.
pub const ERROR_CLOUD_FILE_ALREADY_CONNECTED: HRESULT = 378;

/// The operation is not supported by the cloud sync provider.
pub const ERROR_CLOUD_FILE_NOT_SUPPORTED: HRESULT = 379;

/// The cloud operation is invalid.
pub const ERROR_CLOUD_FILE_INVALID_REQUEST: HRESULT = 380;

/// The cloud operation is not supported on a read-only volume.
pub const ERROR_CLOUD_FILE_READ_ONLY_VOLUME: HRESULT = 381;

/// The operation is reserved for a connected cloud sync provider.
pub const ERROR_CLOUD_FILE_CONNECTED_PROVIDER_ONLY: HRESULT = 382;

/// The cloud sync provider failed to validate the downloaded data.
pub const ERROR_CLOUD_FILE_VALIDATION_FAILED: HRESULT = 383;

///  You can't connect to the file share because it's not secure. This share requires the obsolete SMB1 protocol, which is unsafe and could expose your system to attack.
///
/// Your system requires SMB2 or higher. For more info on resolving this issue, see: <https://go.microsoft.com/fwlink/?linkid=852747>
pub const ERROR_SMB1_NOT_AVAILABLE: HRESULT = 384;

/// The virtualization operation is not allowed on the file in its current state.
pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_INVALID_OPERATION: HRESULT = 385;

/// The cloud sync provider failed user authentication.
pub const ERROR_CLOUD_FILE_AUTHENTICATION_FAILED: HRESULT = 386;

/// The cloud sync provider failed to perform the operation due to low system resources.
pub const ERROR_CLOUD_FILE_INSUFFICIENT_RESOURCES: HRESULT = 387;

/// The cloud sync provider failed to perform the operation due to network being unavailable.
pub const ERROR_CLOUD_FILE_NETWORK_UNAVAILABLE: HRESULT = 388;

/// The cloud operation was unsuccessful.
pub const ERROR_CLOUD_FILE_UNSUCCESSFUL: HRESULT = 389;

/// The operation is only supported on files under a cloud sync root.
pub const ERROR_CLOUD_FILE_NOT_UNDER_SYNC_ROOT: HRESULT = 390;

/// The operation cannot be performed on cloud files in use.
pub const ERROR_CLOUD_FILE_IN_USE: HRESULT = 391;

/// The operation cannot be performed on pinned cloud files.
pub const ERROR_CLOUD_FILE_PINNED: HRESULT = 392;

/// The cloud operation was aborted.
pub const ERROR_CLOUD_FILE_REQUEST_ABORTED: HRESULT = 393;

/// The cloud file's property store is corrupt.
pub const ERROR_CLOUD_FILE_PROPERTY_CORRUPT: HRESULT = 394;

/// Access to the cloud file is denied.
pub const ERROR_CLOUD_FILE_ACCESS_DENIED: HRESULT = 395;

/// The cloud operation cannot be performed on a file with incompatible hardlinks.
pub const ERROR_CLOUD_FILE_INCOMPATIBLE_HARDLINKS: HRESULT = 396;

/// The operation failed due to a conflicting cloud file property lock.
pub const ERROR_CLOUD_FILE_PROPERTY_LOCK_CONFLICT: HRESULT = 397;

/// The cloud operation was canceled by user.
pub const ERROR_CLOUD_FILE_REQUEST_CANCELED: HRESULT = 398;

/// An externally encrypted syskey has been configured, but the system no longer supports this feature.  Please see <https://go.microsoft.com/fwlink/?linkid=851152> for more information.
pub const ERROR_EXTERNAL_SYSKEY_NOT_SUPPORTED: HRESULT = 399;

/// The thread is already in background processing mode.
pub const ERROR_THREAD_MODE_ALREADY_BACKGROUND: HRESULT = 400;

/// The thread is not in background processing mode.
pub const ERROR_THREAD_MODE_NOT_BACKGROUND: HRESULT = 401;

/// The process is already in background processing mode.
pub const ERROR_PROCESS_MODE_ALREADY_BACKGROUND: HRESULT = 402;

/// The process is not in background processing mode.
pub const ERROR_PROCESS_MODE_NOT_BACKGROUND: HRESULT = 403;

/// The cloud file provider exited unexpectedly.
pub const ERROR_CLOUD_FILE_PROVIDER_TERMINATED: HRESULT = 404;

/// The file is not a cloud sync root.
pub const ERROR_NOT_A_CLOUD_SYNC_ROOT: HRESULT = 405;

/// The read or write operation to an encrypted file could not be completed because the file can only be accessed when the device is unlocked.
pub const ERROR_FILE_PROTECTED_UNDER_DPL: HRESULT = 406;

/// The volume is not cluster aligned on the disk.
pub const ERROR_VOLUME_NOT_CLUSTER_ALIGNED: HRESULT = 407;

/// No physically aligned free space was found on the volume.
pub const ERROR_NO_PHYSICALLY_ALIGNED_FREE_SPACE_FOUND: HRESULT = 408;

/// The APPX file can not be accessed because it is not encrypted as expected.
pub const ERROR_APPX_FILE_NOT_ENCRYPTED: HRESULT = 409;

/// A read or write of raw encrypted data cannot be performed because the file is not encrypted.
pub const ERROR_RWRAW_ENCRYPTED_FILE_NOT_ENCRYPTED: HRESULT = 410;

/// An invalid file offset in the encrypted data info block was passed for read or write operation of file's raw encrypted data.
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILEOFFSET: HRESULT = 411;

/// An invalid offset and length combination in the encrypted data info block was passed for read or write operation of file's raw encrypted data.
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILERANGE: HRESULT = 412;

/// An invalid parameter in the encrypted data info block was passed for read or write operation of file's raw encrypted data.
pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_PARAMETER: HRESULT = 413;

/// The Windows Subsystem for Linux has not been enabled.
pub const ERROR_LINUX_SUBSYSTEM_NOT_PRESENT: HRESULT = 414;

/// The specified data could not be read from any of the copies.
pub const ERROR_FT_READ_FAILURE: HRESULT = 415;

/// The specified storage reserve ID is invalid.
pub const ERROR_STORAGE_RESERVE_ID_INVALID: HRESULT = 416;

/// The specified storage reserve does not exist.
pub const ERROR_STORAGE_RESERVE_DOES_NOT_EXIST: HRESULT = 417;

/// The specified storage reserve already exists.
pub const ERROR_STORAGE_RESERVE_ALREADY_EXISTS: HRESULT = 418;

/// The specified storage reserve is not empty.
pub const ERROR_STORAGE_RESERVE_NOT_EMPTY: HRESULT = 419;

/// This operation requires a DAX volume.
pub const ERROR_NOT_A_DAX_VOLUME: HRESULT = 420;

/// This stream is not DAX mappable.
pub const ERROR_NOT_DAX_MAPPABLE: HRESULT = 421;

/// Operation cannot be performed on a time critical thread.
pub const ERROR_TIME_SENSITIVE_THREAD: HRESULT = 422;

/// User data protection is not supported for the current or provided user.
pub const ERROR_DPL_NOT_SUPPORTED_FOR_USER: HRESULT = 423;

/// This directory contains entries whose names differ only in case.
pub const ERROR_CASE_DIFFERING_NAMES_IN_DIR: HRESULT = 424;

/// The file cannot be safely opened because it is not supported by this version of Windows.
pub const ERROR_FILE_NOT_SUPPORTED: HRESULT = 425;

/// The cloud operation was not completed before the time-out period expired.
pub const ERROR_CLOUD_FILE_REQUEST_TIMEOUT: HRESULT = 426;

/// A task queue is required for this operation but none is available.
pub const ERROR_NO_TASK_QUEUE: HRESULT = 427;

/// Failed loading a valid version of srcsrv.dll.
pub const ERROR_SRC_SRV_DLL_LOAD_FAILED: HRESULT = 428;

/// This operation is not supported with BTT enabled.
pub const ERROR_NOT_SUPPORTED_WITH_BTT: HRESULT = 429;

/// This operation cannot be performed because encryption is currently disabled.
pub const ERROR_ENCRYPTION_DISABLED: HRESULT = 430;

/// This encryption operation cannot be performed on filesystem metadata.
pub const ERROR_ENCRYPTING_METADATA_DISALLOWED: HRESULT = 431;

/// Encryption cannot be cleared on this file/directory because it still has an encrypted attribute.
pub const ERROR_CANT_CLEAR_ENCRYPTION_FLAG: HRESULT = 432;

/// A device which does not exist was specified.
pub const ERROR_NO_SUCH_DEVICE: HRESULT = 433;

/// Dehydration of the cloud file is disallowed by the cloud sync provider.
pub const ERROR_CLOUD_FILE_DEHYDRATION_DISALLOWED: HRESULT = 434;

/// A file snapshot operation was attempted when one is already in progress.
pub const ERROR_FILE_SNAP_IN_PROGRESS: HRESULT = 435;

/// A snapshot of the file cannot be taken because a user-mapped section is present.
pub const ERROR_FILE_SNAP_USER_SECTION_NOT_SUPPORTED: HRESULT = 436;

/// The file snapshot operation was terminated because one of the files was modified in a way incompatible with a snapshot operation.  Please try again.
pub const ERROR_FILE_SNAP_MODIFY_NOT_SUPPORTED: HRESULT = 437;

/// An I/O request could not be coordinated with a file snapshot operation.
pub const ERROR_FILE_SNAP_IO_NOT_COORDINATED: HRESULT = 438;

/// An unexpected error occurred while processing a file snapshot operation.
pub const ERROR_FILE_SNAP_UNEXPECTED_ERROR: HRESULT = 439;

/// A file snapshot operation received an invalid parameter.
pub const ERROR_FILE_SNAP_INVALID_PARAMETER: HRESULT = 440;

/// The operation could not be completed due to one or more unsatisfied dependencies.
pub const ERROR_UNSATISFIED_DEPENDENCIES: HRESULT = 441;

/// The file cannot be opened because the path has a case-sensitive directory.
pub const ERROR_CASE_SENSITIVE_PATH: HRESULT = 442;

/// The filesystem couldn't handle one of the CacheManager's callback error codes.
pub const ERROR_UNEXPECTED_NTCACHEMANAGER_ERROR: HRESULT = 443;

/// WSL 2 requires an update to its kernel component. For information please visit <https://aka.ms/wsl2kernel>
pub const ERROR_LINUX_SUBSYSTEM_UPDATE_REQUIRED: HRESULT = 444;

/// This action is blocked, but you can choose to allow it. Please refer to the data loss prevention notification for further information.
pub const ERROR_DLP_POLICY_WARNS_AGAINST_OPERATION: HRESULT = 445;

/// This action is blocked. Please refer to the data loss prevention notification for further information.
pub const ERROR_DLP_POLICY_DENIES_OPERATION: HRESULT = 446;

/// Access is denied because the file contains potentially unwanted software or content the security administrator decided to block.
pub const ERROR_SECURITY_DENIES_OPERATION: HRESULT = 447;

/// The path cannot be traversed because it contains an untrusted mount point.
pub const ERROR_UNTRUSTED_MOUNT_POINT: HRESULT = 448;

/// This action is blocked. Please refer to the data loss prevention notification for further information.
pub const ERROR_DLP_POLICY_SILENTLY_FAIL: HRESULT = 449;

/// Neither developer unlocked mode nor side loading mode is enabled on the device.
pub const ERROR_CAPAUTHZ_NOT_DEVUNLOCKED: HRESULT = 450;

/// Can not change application type during upgrade or re-provision.
pub const ERROR_CAPAUTHZ_CHANGE_TYPE: HRESULT = 451;

/// The application has not been provisioned.
pub const ERROR_CAPAUTHZ_NOT_PROVISIONED: HRESULT = 452;

/// The requested capability can not be authorized for this application.
pub const ERROR_CAPAUTHZ_NOT_AUTHORIZED: HRESULT = 453;

/// There is no capability authorization policy on the device.
pub const ERROR_CAPAUTHZ_NO_POLICY: HRESULT = 454;

/// The capability authorization database has been corrupted.
pub const ERROR_CAPAUTHZ_DB_CORRUPTED: HRESULT = 455;

/// The custom capability's SCCD has an invalid catalog.
pub const ERROR_CAPAUTHZ_SCCD_INVALID_CATALOG: HRESULT = 456;

/// None of the authorized entity elements in the SCCD matched the app being installed; either the PFNs don't match, or the element's signature hash doesn't validate.
pub const ERROR_CAPAUTHZ_SCCD_NO_AUTH_ENTITY: HRESULT = 457;

/// The custom capability's SCCD failed to parse.
pub const ERROR_CAPAUTHZ_SCCD_PARSE_ERROR: HRESULT = 458;

/// The custom capability's SCCD requires developer mode.
pub const ERROR_CAPAUTHZ_SCCD_DEV_MODE_REQUIRED: HRESULT = 459;

/// There not all declared custom capabilities are found in the SCCD.
pub const ERROR_CAPAUTHZ_SCCD_NO_CAPABILITY_MATCH: HRESULT = 460;

/// The CimFS image is corrupt.
pub const ERROR_CIMFS_IMAGE_CORRUPT: HRESULT = 470;

/// The system does not support this version of the CimFS image.
pub const ERROR_CIMFS_IMAGE_VERSION_NOT_SUPPORTED: HRESULT = 471;

/// The storage stack returned STATUS_ACCESS_DENEID for the current operation.
pub const ERROR_STORAGE_STACK_ACCESS_DENIED: HRESULT = 472;

/// Insufficient Virtual Address resources to complete the operation.
pub const ERROR_INSUFFICIENT_VIRTUAL_ADDR_RESOURCES: HRESULT = 473;

/// The specified index is beyond the bounds of valid range.
pub const ERROR_INDEX_OUT_OF_BOUNDS: HRESULT = 474;

/// The cloud provider failed to acknowledge a message before the time-out expired.
pub const ERROR_CLOUD_FILE_US_MESSAGE_TIMEOUT: HRESULT = 475;

/// This operation requires a developer volume.
pub const ERROR_NOT_A_DEV_VOLUME: HRESULT = 476;

/// The file system GUID in the per machine state did not match the one on disk.
pub const ERROR_FS_GUID_MISMATCH: HRESULT = 477;

/// The file system minifilter cannot attach to the developer volume.
pub const ERROR_CANT_ATTACH_TO_DEV_VOLUME: HRESULT = 478;

/// The configured value is not valid.
pub const ERROR_INVALID_CONFIG_VALUE: HRESULT = 479;

/// The operation timed out waiting for this device to complete a PnP query-remove request due to a potential hang in its device stack. The system may need to be rebooted to complete the request.
pub const ERROR_PNP_QUERY_REMOVE_DEVICE_TIMEOUT: HRESULT = 480;

/// The operation timed out waiting for this device to complete a PnP query-remove request due to a potential hang in the device stack of a related device. The system may need to be rebooted to complete the operation.
pub const ERROR_PNP_QUERY_REMOVE_RELATED_DEVICE_TIMEOUT: HRESULT = 481;

/// The operation timed out waiting for this device to complete a PnP query-remove request due to a potential hang in the device stack of an unrelated device. The system may need to be rebooted to complete the operation.
pub const ERROR_PNP_QUERY_REMOVE_UNRELATED_DEVICE_TIMEOUT: HRESULT = 482;

/// The request failed due to a fatal device hardware error.
pub const ERROR_DEVICE_HARDWARE_ERROR: HRESULT = 483;

/// Attempt to access invalid address.
pub const ERROR_INVALID_ADDRESS: HRESULT = 487;

/// The volume contains paging, crash dump or other system critical files.
pub const ERROR_HAS_SYSTEM_CRITICAL_FILES: HRESULT = 488;

/// The specified operation is not supported on an encrypted file.
pub const ERROR_ENCRYPTED_FILE_NOT_SUPPORTED: HRESULT = 489;

/// The specified operation is not supported on a sparse file.
pub const ERROR_SPARSE_FILE_NOT_SUPPORTED: HRESULT = 490;

/// The specified operation is not supported on a pagefile.
pub const ERROR_PAGEFILE_NOT_SUPPORTED: HRESULT = 491;

/// The specified operation is not supported on a volume.
pub const ERROR_VOLUME_NOT_SUPPORTED: HRESULT = 492;

/// The specified operation is not supported on a BypassIO enabled file.
pub const ERROR_NOT_SUPPORTED_WITH_BYPASSIO: HRESULT = 493;

/// The specified driver does not support BypassIO operations.
pub const ERROR_NO_BYPASSIO_DRIVER_SUPPORT: HRESULT = 494;

/// The specified operation is not supported while encryption is enabled on the target object.
pub const ERROR_NOT_SUPPORTED_WITH_ENCRYPTION: HRESULT = 495;

/// The specified operation is not supported while compression is enabled on the target object.
pub const ERROR_NOT_SUPPORTED_WITH_COMPRESSION: HRESULT = 496;

/// The specified operation is not supported while replication is enabled on the target object.
pub const ERROR_NOT_SUPPORTED_WITH_REPLICATION: HRESULT = 497;

/// The specified operation is not supported while deduplication is enabled on the target object.
pub const ERROR_NOT_SUPPORTED_WITH_DEDUPLICATION: HRESULT = 498;

/// The specified operation is not supported while auditing is enabled on the target object.
pub const ERROR_NOT_SUPPORTED_WITH_AUDITING: HRESULT = 499;

/// User profile cannot be loaded.
pub const ERROR_USER_PROFILE_LOAD: HRESULT = 500;

/// The negotiated session key does not meet the minimum length requirement.
pub const ERROR_SESSION_KEY_TOO_SHORT: HRESULT = 501;

/// Access denied when accessing the user profile.
pub const ERROR_ACCESS_DENIED_APPDATA: HRESULT = 502;

/// The specified operation is not supported while monitoring is enabled on the target object.
pub const ERROR_NOT_SUPPORTED_WITH_MONITORING: HRESULT = 503;

/// The specified operation is not supported while snapshot is enabled on the target object.
pub const ERROR_NOT_SUPPORTED_WITH_SNAPSHOT: HRESULT = 504;

/// The specified operation is not supported while virtualization is enabled on the target object.
pub const ERROR_NOT_SUPPORTED_WITH_VIRTUALIZATION: HRESULT = 505;

/// At least one minifilter does not support bypass IO.
pub const ERROR_BYPASSIO_FLT_NOT_SUPPORTED: HRESULT = 506;

/// The device needs to be reset.
pub const ERROR_DEVICE_RESET_REQUIRED: HRESULT = 507;

/// The volume is opened for exclusive write access, preventing files from being opened for write access.
pub const ERROR_VOLUME_WRITE_ACCESS_DENIED: HRESULT = 508;

/// The specified operation is not supported on a file opened for cached IO.
pub const ERROR_NOT_SUPPORTED_WITH_CACHED_HANDLE: HRESULT = 509;

/// The file system encountered a metadata file with inconsistent data.
pub const ERROR_FS_METADATA_INCONSISTENT: HRESULT = 510;

/// A file system block being referenced has been modified after containing a weak reference.
pub const ERROR_BLOCK_WEAK_REFERENCE_INVALID: HRESULT = 511;

/// The source file system block being referenced has been modified after containing a weak reference.
pub const ERROR_BLOCK_SOURCE_WEAK_REFERENCE_INVALID: HRESULT = 512;

/// The target file system block being referenced has been modified after containing a weak reference.
pub const ERROR_BLOCK_TARGET_WEAK_REFERENCE_INVALID: HRESULT = 513;

/// The target file system block is shared between multiple extents.
pub const ERROR_BLOCK_SHARED: HRESULT = 514;

/// Volume format is up to date already.
pub const ERROR_VOLUME_UPGRADE_NOT_NEEDED: HRESULT = 515;

/// Volume upgrade is pending.  A reboot or re-mount of the volume is required.
pub const ERROR_VOLUME_UPGRADE_PENDING: HRESULT = 516;

/// Volume upgrade is disabled.
pub const ERROR_VOLUME_UPGRADE_DISABLED: HRESULT = 517;

/// Volume upgrade is disabled until Windows OS downgrade period has expired.
pub const ERROR_VOLUME_UPGRADE_DISABLED_TILL_OS_DOWNGRADE_EXPIRED: HRESULT = 518;

/// Arithmetic result exceeded 32 bits.
pub const ERROR_ARITHMETIC_OVERFLOW: HRESULT = 534;

/// There is a process on other end of the pipe.
pub const ERROR_PIPE_CONNECTED: HRESULT = 535;

/// Waiting for a process to open the other end of the pipe.
pub const ERROR_PIPE_LISTENING: HRESULT = 536;

/// Application verifier has found an error in the current process.
pub const ERROR_VERIFIER_STOP: HRESULT = 537;

/// An error occurred in the ABIOS subsystem.
pub const ERROR_ABIOS_ERROR: HRESULT = 538;

/// A warning occurred in the WX86 subsystem.
pub const ERROR_WX86_WARNING: HRESULT = 539;

/// An error occurred in the WX86 subsystem.
pub const ERROR_WX86_ERROR: HRESULT = 540;

/// An attempt was made to cancel or set a timer that has an associated APC and the subject thread is not the thread that originally set the timer with an associated APC routine.
pub const ERROR_TIMER_NOT_CANCELED: HRESULT = 541;

/// Unwind exception code.
pub const ERROR_UNWIND: HRESULT = 542;

/// An invalid or unaligned stack was encountered during an unwind operation.
pub const ERROR_BAD_STACK: HRESULT = 543;

/// An invalid unwind target was encountered during an unwind operation.
pub const ERROR_INVALID_UNWIND_TARGET: HRESULT = 544;

/// Invalid Object Attributes specified to NtCreatePort or invalid Port Attributes specified to NtConnectPort
pub const ERROR_INVALID_PORT_ATTRIBUTES: HRESULT = 545;

/// Length of message passed to NtRequestPort or NtRequestWaitReplyPort was longer than the maximum message allowed by the port.
pub const ERROR_PORT_MESSAGE_TOO_LONG: HRESULT = 546;

/// An attempt was made to lower a quota limit below the current usage.
pub const ERROR_INVALID_QUOTA_LOWER: HRESULT = 547;

/// An attempt was made to attach to a device that was already attached to another device.
pub const ERROR_DEVICE_ALREADY_ATTACHED: HRESULT = 548;

/// An attempt was made to execute an instruction at an unaligned address and the host system does not support unaligned instruction references.
pub const ERROR_INSTRUCTION_MISALIGNMENT: HRESULT = 549;

/// Profiling not started.
pub const ERROR_PROFILING_NOT_STARTED: HRESULT = 550;

/// Profiling not stopped.
pub const ERROR_PROFILING_NOT_STOPPED: HRESULT = 551;

/// The passed ACL did not contain the minimum required information.
pub const ERROR_COULD_NOT_INTERPRET: HRESULT = 552;

/// The number of active profiling objects is at the maximum and no more may be started.
pub const ERROR_PROFILING_AT_LIMIT: HRESULT = 553;

/// Used to indicate that an operation cannot continue without blocking for I/O.
pub const ERROR_CANT_WAIT: HRESULT = 554;

/// Indicates that a thread attempted to terminate itself by default (called NtTerminateThread with NULL) and it was the last thread in the current process.
pub const ERROR_CANT_TERMINATE_SELF: HRESULT = 555;

///  If an MM error is returned which is not defined in the standard FsRtl filter, it is converted to one of the following errors which is guaranteed to be in the filter.
///
/// In this case information is lost, however, the filter correctly handles the exception.
pub const ERROR_UNEXPECTED_MM_CREATE_ERR: HRESULT = 556;

///  If an MM error is returned which is not defined in the standard FsRtl filter, it is converted to one of the following errors which is guaranteed to be in the filter.
///
/// In this case information is lost, however, the filter correctly handles the exception.
pub const ERROR_UNEXPECTED_MM_MAP_ERROR: HRESULT = 557;

///  If an MM error is returned which is not defined in the standard FsRtl filter, it is converted to one of the following errors which is guaranteed to be in the filter.
///
/// In this case information is lost, however, the filter correctly handles the exception.
pub const ERROR_UNEXPECTED_MM_EXTEND_ERR: HRESULT = 558;

/// A malformed function table was encountered during an unwind operation.
pub const ERROR_BAD_FUNCTION_TABLE: HRESULT = 559;

///  Indicates that an attempt was made to assign protection to a file system file or directory and one of the SIDs in the security descriptor could not be translated into a GUID that could be stored by the file system.
///
/// This causes the protection attempt to fail, which may cause a file creation attempt to fail.
pub const ERROR_NO_GUID_TRANSLATION: HRESULT = 560;

/// Indicates that an attempt was made to grow an LDT by setting its size, or that the size was not an even number of selectors.
pub const ERROR_INVALID_LDT_SIZE: HRESULT = 561;

/// Indicates that the starting value for the LDT information was not an integral multiple of the selector size.
pub const ERROR_INVALID_LDT_OFFSET: HRESULT = 563;

/// Indicates that the user supplied an invalid descriptor when trying to set up Ldt descriptors.
pub const ERROR_INVALID_LDT_DESCRIPTOR: HRESULT = 564;

/// Indicates a process has too many threads to perform the requested action. For example, assignment of a primary token may only be performed when a process has zero or one threads.
pub const ERROR_TOO_MANY_THREADS: HRESULT = 565;

/// An attempt was made to operate on a thread within a specific process, but the thread specified is not in the process specified.
pub const ERROR_THREAD_NOT_IN_PROCESS: HRESULT = 566;

/// Pagefile quota was exceeded.
pub const ERROR_PAGEFILE_QUOTA_EXCEEDED: HRESULT = 567;

/// The Netlogon service cannot start because another Netlogon service running in the domain conflicts with the specified role.
pub const ERROR_LOGON_SERVER_CONFLICT: HRESULT = 568;

/// The SAM database on a Windows Server is significantly out of synchronization with the copy on the Domain Controller. A complete synchronization is required.
pub const ERROR_SYNCHRONIZATION_REQUIRED: HRESULT = 569;

/// The NtCreateFile API failed. This error should never be returned to an application, it is a place holder for the Windows Lan Manager Redirector to use in its internal error mapping routines.
pub const ERROR_NET_OPEN_FAILED: HRESULT = 570;

/// The I/O permissions for the process could not be changed.
pub const ERROR_IO_PRIVILEGE_FAILED: HRESULT = 571;

/// The application terminated as a result of a CTRL+C.
pub const ERROR_CONTROL_C_EXIT: HRESULT = 572;

/// The required system file %hs is bad or missing.
pub const ERROR_MISSING_SYSTEMFILE: HRESULT = 573;

/// The exception %s (0x%08lx) occurred in the application at location 0x%08lx.
pub const ERROR_UNHANDLED_EXCEPTION: HRESULT = 574;

/// The application was unable to start correctly (0x%lx). Click OK to close the application.
pub const ERROR_APP_INIT_FAILURE: HRESULT = 575;

/// The creation of the paging file %hs failed (%lx). The requested size was %ld.
pub const ERROR_PAGEFILE_CREATE_FAILED: HRESULT = 576;

/// Windows cannot verify the digital signature for this file. A recent hardware or software change might have installed a file that is signed incorrectly or damaged, or that might be malicious software from an unknown source.
pub const ERROR_INVALID_IMAGE_HASH: HRESULT = 577;

/// No paging file was specified in the system configuration.
pub const ERROR_NO_PAGEFILE: HRESULT = 578;

/// A real-mode application issued a floating-point instruction and floating-point hardware is not present.
pub const ERROR_ILLEGAL_FLOAT_CONTEXT: HRESULT = 579;

/// An event pair synchronization operation was performed using the thread specific client/server event pair object, but no event pair object was associated with the thread.
pub const ERROR_NO_EVENT_PAIR: HRESULT = 580;

/// A Windows Server has an incorrect configuration.
pub const ERROR_DOMAIN_CTRLR_CONFIG_ERROR: HRESULT = 581;

/// An illegal character was encountered. For a multi-byte character set this includes a lead byte without a succeeding trail byte. For the Unicode character set this includes the characters 0xFFFF and 0xFFFE.
pub const ERROR_ILLEGAL_CHARACTER: HRESULT = 582;

/// The Unicode character is not defined in the Unicode character set installed on the system.
pub const ERROR_UNDEFINED_CHARACTER: HRESULT = 583;

/// The paging file cannot be created on a floppy diskette.
pub const ERROR_FLOPPY_VOLUME: HRESULT = 584;

/// The system BIOS failed to connect a system interrupt to the device or bus for which the device is connected.
pub const ERROR_BIOS_FAILED_TO_CONNECT_INTERRUPT: HRESULT = 585;

/// This operation is only allowed for the Primary Domain Controller of the domain.
pub const ERROR_BACKUP_CONTROLLER: HRESULT = 586;

/// An attempt was made to acquire a mutant such that its maximum count would have been exceeded.
pub const ERROR_MUTANT_LIMIT_EXCEEDED: HRESULT = 587;

/// A volume has been accessed for which a file system driver is required that has not yet been loaded.
pub const ERROR_FS_DRIVER_REQUIRED: HRESULT = 588;

///  The registry cannot load the hive (file):
///
/// %hs
///  or its log or alternate.
///
/// It is corrupt, absent, or not writable.
pub const ERROR_CANNOT_LOAD_REGISTRY_FILE: HRESULT = 589;

/// An unexpected failure occurred while processing a DebugActiveProcess API request. You may choose OK to terminate the process, or Cancel to ignore the error.
pub const ERROR_DEBUG_ATTACH_FAILED: HRESULT = 590;

///  The %hs system process terminated unexpectedly with a status of 0x%08x (0x%08x 0x%08x).
///
/// The system has been shut down.
pub const ERROR_SYSTEM_PROCESS_TERMINATED: HRESULT = 591;

/// The TDI client could not handle the data received during an indication.
pub const ERROR_DATA_NOT_ACCEPTED: HRESULT = 592;

/// NTVDM encountered a hard error.
pub const ERROR_VDM_HARD_ERROR: HRESULT = 593;

/// The driver %hs failed to complete a cancelled I/O request in the allotted time.
pub const ERROR_DRIVER_CANCEL_TIMEOUT: HRESULT = 594;

/// An attempt was made to reply to an LPC message, but the thread specified by the client ID in the message was not waiting on that message.
pub const ERROR_REPLY_MESSAGE_MISMATCH: HRESULT = 595;

///  Windows was unable to save all the data for the file %hs. The data has been lost.
///
/// This error may be caused by a failure of your computer hardware or network connection. Please try to save this file elsewhere.
pub const ERROR_LOST_WRITEBEHIND_DATA: HRESULT = 596;

/// The parameter(s) passed to the server in the client/server shared memory window were invalid. Too much data may have been put in the shared memory window.
pub const ERROR_CLIENT_SERVER_PARAMETERS_INVALID: HRESULT = 597;

/// The stream is not a tiny stream.
pub const ERROR_NOT_TINY_STREAM: HRESULT = 598;

/// The request must be handled by the stack overflow code.
pub const ERROR_STACK_OVERFLOW_READ: HRESULT = 599;

/// Internal OFS status codes indicating how an allocation operation is handled. Either it is retried after the containing onode is moved or the extent stream is converted to a large stream.
pub const ERROR_CONVERT_TO_LARGE: HRESULT = 600;

/// The attempt to find the object found an object matching by ID on the volume but it is out of the scope of the handle used for the operation.
pub const ERROR_FOUND_OUT_OF_SCOPE: HRESULT = 601;

/// The bucket array must be grown. Retry transaction after doing so.
pub const ERROR_ALLOCATE_BUCKET: HRESULT = 602;

/// The user/kernel marshalling buffer has overflowed.
pub const ERROR_MARSHALL_OVERFLOW: HRESULT = 603;

/// The supplied variant structure contains invalid data.
pub const ERROR_INVALID_VARIANT: HRESULT = 604;

/// The specified buffer contains ill-formed data.
pub const ERROR_BAD_COMPRESSION_BUFFER: HRESULT = 605;

/// An attempt to generate a security audit failed.
pub const ERROR_AUDIT_FAILED: HRESULT = 606;

/// The timer resolution was not previously set by the current process.
pub const ERROR_TIMER_RESOLUTION_NOT_SET: HRESULT = 607;

/// There is insufficient account information to log you on.
pub const ERROR_INSUFFICIENT_LOGON_INFO: HRESULT = 608;

///  The dynamic link library %hs is not written correctly. The stack pointer has been left in an inconsistent state.
///
/// The entrypoint should be declared as WINAPI or STDCALL. Select YES to fail the DLL load. Select NO to continue execution. Selecting NO may cause the application to operate incorrectly.
pub const ERROR_BAD_DLL_ENTRYPOINT: HRESULT = 609;

///  The %hs service is not written correctly. The stack pointer has been left in an inconsistent state.
///
/// The callback entrypoint should be declared as WINAPI or STDCALL. Selecting OK will cause the service to continue operation. However, the service process may operate incorrectly.
pub const ERROR_BAD_SERVICE_ENTRYPOINT: HRESULT = 610;

/// There is an IP address conflict with another system on the network
pub const ERROR_IP_ADDRESS_CONFLICT1: HRESULT = 611;

/// There is an IP address conflict with another system on the network
pub const ERROR_IP_ADDRESS_CONFLICT2: HRESULT = 612;

/// The system has reached the maximum size allowed for the system part of the registry. Additional storage requests will be ignored.
pub const ERROR_REGISTRY_QUOTA_LIMIT: HRESULT = 613;

/// A callback return system service cannot be executed when no callback is active.
pub const ERROR_NO_CALLBACK_ACTIVE: HRESULT = 614;

///  The password provided is too short to meet the policy of your user account.
///
/// Please choose a longer password.
pub const ERROR_PWD_TOO_SHORT: HRESULT = 615;

///  The policy of your user account does not allow you to change passwords too frequently.
///
/// This is done to prevent users from changing back to a familiar, but potentially discovered, password.
/// If you feel your password has been compromised then please contact your administrator immediately to have a new one assigned.
pub const ERROR_PWD_TOO_RECENT: HRESULT = 616;

///  You have attempted to change your password to one that you have used in the past.
///
/// The policy of your user account does not allow this. Please select a password that you have not previously used.
pub const ERROR_PWD_HISTORY_CONFLICT: HRESULT = 617;

/// The specified compression format is unsupported.
pub const ERROR_UNSUPPORTED_COMPRESSION: HRESULT = 618;

/// The specified hardware profile configuration is invalid.
pub const ERROR_INVALID_HW_PROFILE: HRESULT = 619;

/// The specified Plug and Play registry device path is invalid.
pub const ERROR_INVALID_PLUGPLAY_DEVICE_PATH: HRESULT = 620;

/// The specified quota list is internally inconsistent with its descriptor.
pub const ERROR_QUOTA_LIST_INCONSISTENT: HRESULT = 621;

/// The evaluation period for this installation of Windows has expired. This system will shutdown in 1 hour. To restore access to this installation of Windows, please upgrade this installation using a licensed distribution of this product.
pub const ERROR_EVALUATION_EXPIRATION: HRESULT = 622;

///  The system DLL %hs was relocated in memory. The application will not run properly.
///
/// The relocation occurred because the DLL %hs occupied an address range reserved for Windows system DLLs. The vendor supplying the DLL should be contacted for a new DLL.
pub const ERROR_ILLEGAL_DLL_RELOCATION: HRESULT = 623;

/// The application failed to initialize because the window station is shutting down.
pub const ERROR_DLL_INIT_FAILED_LOGOFF: HRESULT = 624;

/// The validation process needs to continue on to the next step.
pub const ERROR_VALIDATE_CONTINUE: HRESULT = 625;

/// There are no more matches for the current index enumeration.
pub const ERROR_NO_MORE_MATCHES: HRESULT = 626;

/// The range could not be added to the range list because of a conflict.
pub const ERROR_RANGE_LIST_CONFLICT: HRESULT = 627;

/// The server process is running under a SID different than that required by client.
pub const ERROR_SERVER_SID_MISMATCH: HRESULT = 628;

/// A group marked use for deny only cannot be enabled.
pub const ERROR_CANT_ENABLE_DENY_ONLY: HRESULT = 629;

/// Multiple floating point faults.
pub const ERROR_FLOAT_MULTIPLE_FAULTS: HRESULT = 630;

/// Multiple floating point traps.
pub const ERROR_FLOAT_MULTIPLE_TRAPS: HRESULT = 631;

/// The requested interface is not supported.
pub const ERROR_NOINTERFACE: HRESULT = 632;

/// The driver %hs does not support standby mode. Updating this driver may allow the system to go to standby mode.
pub const ERROR_DRIVER_FAILED_SLEEP: HRESULT = 633;

/// The system file %1 has become corrupt and has been replaced.
pub const ERROR_CORRUPT_SYSTEM_FILE: HRESULT = 634;

///  Your system is low on virtual memory. Windows is increasing the size of your virtual memory paging file.
///
/// During this process, memory requests for some applications may be denied. For more information, see Help.
pub const ERROR_COMMITMENT_MINIMUM: HRESULT = 635;

/// A device was removed so enumeration must be restarted.
pub const ERROR_PNP_RESTART_ENUMERATION: HRESULT = 636;

///  The system image %s is not properly signed.
///
/// The file has been replaced with the signed file.
/// The system has been shut down.
pub const ERROR_SYSTEM_IMAGE_BAD_SIGNATURE: HRESULT = 637;

/// Device will not start without a reboot.
pub const ERROR_PNP_REBOOT_REQUIRED: HRESULT = 638;

/// There is not enough power to complete the requested operation.
pub const ERROR_INSUFFICIENT_POWER: HRESULT = 639;

///  ERROR_MULTIPLE_FAULT_VIOLATION
pub const ERROR_MULTIPLE_FAULT_VIOLATION: HRESULT = 640;

/// The system is in the process of shutting down.
pub const ERROR_SYSTEM_SHUTDOWN: HRESULT = 641;

/// An attempt to remove a processes DebugPort was made, but a port was not already associated with the process.
pub const ERROR_PORT_NOT_SET: HRESULT = 642;

/// This version of Windows is not compatible with the behavior version of directory forest, domain or domain controller.
pub const ERROR_DS_VERSION_CHECK_FAILURE: HRESULT = 643;

/// The specified range could not be found in the range list.
pub const ERROR_RANGE_NOT_FOUND: HRESULT = 644;

/// The driver was not loaded because the system is booting into safe mode.
pub const ERROR_NOT_SAFE_MODE_DRIVER: HRESULT = 646;

/// The driver was not loaded because it failed its initialization call.
pub const ERROR_FAILED_DRIVER_ENTRY: HRESULT = 647;

///  The "%hs" encountered an error while applying power or reading the device configuration.
///
/// This may be caused by a failure of your hardware or by a poor connection.
pub const ERROR_DEVICE_ENUMERATION_ERROR: HRESULT = 648;

/// The create operation failed because the name contained at least one mount point which resolves to a volume to which the specified device object is not attached.
pub const ERROR_MOUNT_POINT_NOT_RESOLVED: HRESULT = 649;

/// The device object parameter is either not a valid device object or is not attached to the volume specified by the file name.
pub const ERROR_INVALID_DEVICE_OBJECT_PARAMETER: HRESULT = 650;

/// A Machine Check Error has occurred. Please check the system eventlog for additional information.
pub const ERROR_MCA_OCCURED: HRESULT = 651;

/// There was error [%2] processing the driver database.
pub const ERROR_DRIVER_DATABASE_ERROR: HRESULT = 652;

/// System hive size has exceeded its limit.
pub const ERROR_SYSTEM_HIVE_TOO_LARGE: HRESULT = 653;

/// The driver could not be loaded because a previous version of the driver is still in memory.
pub const ERROR_DRIVER_FAILED_PRIOR_UNLOAD: HRESULT = 654;

/// Please wait while the Volume Shadow Copy Service prepares volume %hs for hibernation.
pub const ERROR_VOLSNAP_PREPARE_HIBERNATE: HRESULT = 655;

/// The system has failed to hibernate (The error code is %hs). Hibernation will be disabled until the system is restarted.
pub const ERROR_HIBERNATION_FAILURE: HRESULT = 656;

///  The password provided is too long to meet the policy of your user account.
///
/// Please choose a shorter password.
pub const ERROR_PWD_TOO_LONG: HRESULT = 657;

/// The requested operation could not be completed due to a file system limitation
pub const ERROR_FILE_SYSTEM_LIMITATION: HRESULT = 665;

/// An assertion failure has occurred.
pub const ERROR_ASSERTION_FAILURE: HRESULT = 668;

/// An error occurred in the ACPI subsystem.
pub const ERROR_ACPI_ERROR: HRESULT = 669;

/// WOW Assertion Error.
pub const ERROR_WOW_ASSERTION: HRESULT = 670;

///  A device is missing in the system BIOS MPS table. This device will not be used.
///
/// Please contact your system vendor for system BIOS update.
pub const ERROR_PNP_BAD_MPS_TABLE: HRESULT = 671;

/// A translator failed to translate resources.
pub const ERROR_PNP_TRANSLATION_FAILED: HRESULT = 672;

/// A IRQ translator failed to translate resources.
pub const ERROR_PNP_IRQ_TRANSLATION_FAILED: HRESULT = 673;

/// Driver %2 returned invalid ID for a child device (%3).
pub const ERROR_PNP_INVALID_ID: HRESULT = 674;

/// the system debugger was awakened by an interrupt.
pub const ERROR_WAKE_SYSTEM_DEBUGGER: HRESULT = 675;

/// Handles to objects have been automatically closed as a result of the requested operation.
pub const ERROR_HANDLES_CLOSED: HRESULT = 676;

/// The specified access control list (ACL) contained more information than was expected.
pub const ERROR_EXTRANEOUS_INFORMATION: HRESULT = 677;

///  This warning level status indicates that the transaction state already exists for the registry sub-tree, but that a transaction commit was previously aborted.
///
/// The commit has NOT been completed, but has not been rolled back either (so it may still be committed if desired).
pub const ERROR_RXACT_COMMIT_NECESSARY: HRESULT = 678;

/// The media may have changed.
pub const ERROR_MEDIA_CHECK: HRESULT = 679;

///  During the translation of a global identifier (GUID) to a Windows security ID (SID), no administratively-defined GUID prefix was found.
///
/// A substitute prefix was used, which will not compromise system security. However, this may provide a more restrictive access than intended.
pub const ERROR_GUID_SUBSTITUTION_MADE: HRESULT = 680;

/// The create operation stopped after reaching a symbolic link
pub const ERROR_STOPPED_ON_SYMLINK: HRESULT = 681;

/// A long jump has been executed.
pub const ERROR_LONGJUMP: HRESULT = 682;

/// The Plug and Play query operation was not successful.
pub const ERROR_PLUGPLAY_QUERY_VETOED: HRESULT = 683;

/// A frame consolidation has been executed.
pub const ERROR_UNWIND_CONSOLIDATE: HRESULT = 684;

///  Registry hive (file):
///
/// %hs
/// was corrupted and it has been recovered. Some data might have been lost.
pub const ERROR_REGISTRY_HIVE_RECOVERED: HRESULT = 685;

/// The application is attempting to run executable code from the module %hs. This may be insecure. An alternative, %hs, is available. Should the application use the secure module %hs?
pub const ERROR_DLL_MIGHT_BE_INSECURE: HRESULT = 686;

/// The application is loading executable code from the module %hs. This is secure, but may be incompatible with previous releases of the operating system. An alternative, %hs, is available. Should the application use the secure module %hs?
pub const ERROR_DLL_MIGHT_BE_INCOMPATIBLE: HRESULT = 687;

/// Debugger did not handle the exception.
pub const ERROR_DBG_EXCEPTION_NOT_HANDLED: HRESULT = 688;

/// Debugger will reply later.
pub const ERROR_DBG_REPLY_LATER: HRESULT = 689;

/// Debugger cannot provide handle.
pub const ERROR_DBG_UNABLE_TO_PROVIDE_HANDLE: HRESULT = 690;

/// Debugger terminated thread.
pub const ERROR_DBG_TERMINATE_THREAD: HRESULT = 691;

/// Debugger terminated process.
pub const ERROR_DBG_TERMINATE_PROCESS: HRESULT = 692;

/// Debugger got control C.
pub const ERROR_DBG_CONTROL_C: HRESULT = 693;

/// Debugger printed exception on control C.
pub const ERROR_DBG_PRINTEXCEPTION_C: HRESULT = 694;

/// Debugger received RIP exception.
pub const ERROR_DBG_RIPEXCEPTION: HRESULT = 695;

/// Debugger received control break.
pub const ERROR_DBG_CONTROL_BREAK: HRESULT = 696;

/// Debugger command communication exception.
pub const ERROR_DBG_COMMAND_EXCEPTION: HRESULT = 697;

/// An attempt was made to create an object and the object name already existed.
pub const ERROR_OBJECT_NAME_EXISTS: HRESULT = 698;

/// A thread termination occurred while the thread was suspended. The thread was resumed, and termination proceeded.
pub const ERROR_THREAD_WAS_SUSPENDED: HRESULT = 699;

/// An image file could not be mapped at the address specified in the image file. Local fixups must be performed on this image.
pub const ERROR_IMAGE_NOT_AT_BASE: HRESULT = 700;

/// This informational level status indicates that a specified registry sub-tree transaction state did not yet exist and had to be created.
pub const ERROR_RXACT_STATE_CREATED: HRESULT = 701;

///  A virtual DOS machine (VDM) is loading, unloading, or moving an MS-DOS or Win16 program segment image.
///
/// An exception is raised so a debugger can load, unload or track symbols and breakpoints within these 16-bit segments.
pub const ERROR_SEGMENT_NOTIFICATION: HRESULT = 702;

///  The process cannot switch to the startup current directory %hs.
///
/// Select OK to set current directory to %hs, or select CANCEL to exit.
pub const ERROR_BAD_CURRENT_DIRECTORY: HRESULT = 703;

///  To satisfy a read request, the NT fault-tolerant file system successfully read the requested data from a redundant copy.
///
/// This was done because the file system encountered a failure on a member of the fault-tolerant volume, but was unable to reassign the failing area of the device.
pub const ERROR_FT_READ_RECOVERY_FROM_BACKUP: HRESULT = 704;

///  To satisfy a write request, the NT fault-tolerant file system successfully wrote a redundant copy of the information.
///
/// This was done because the file system encountered a failure on a member of the fault-tolerant volume, but was not able to reassign the failing area of the device.
pub const ERROR_FT_WRITE_RECOVERY: HRESULT = 705;

/// The image file %hs is valid, but is for a machine type other than the current machine. Select OK to continue, or CANCEL to fail the DLL load.
pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH: HRESULT = 706;

/// The network transport returned partial data to its client. The remaining data will be sent later.
pub const ERROR_RECEIVE_PARTIAL: HRESULT = 707;

/// The network transport returned data to its client that was marked as expedited by the remote system.
pub const ERROR_RECEIVE_EXPEDITED: HRESULT = 708;

/// The network transport returned partial data to its client and this data was marked as expedited by the remote system. The remaining data will be sent later.
pub const ERROR_RECEIVE_PARTIAL_EXPEDITED: HRESULT = 709;

/// The TDI indication has completed successfully.
pub const ERROR_EVENT_DONE: HRESULT = 710;

/// The TDI indication has entered the pending state.
pub const ERROR_EVENT_PENDING: HRESULT = 711;

/// Checking file system on %wZ
pub const ERROR_CHECKING_FILE_SYSTEM: HRESULT = 712;

/// %hs
pub const ERROR_FATAL_APP_EXIT: HRESULT = 713;

/// The specified registry key is referenced by a predefined handle.
pub const ERROR_PREDEFINED_HANDLE: HRESULT = 714;

/// The page protection of a locked page was changed to 'No Access' and the page was unlocked from memory and from the process.
pub const ERROR_WAS_UNLOCKED: HRESULT = 715;

/// %hs
pub const ERROR_SERVICE_NOTIFICATION: HRESULT = 716;

/// One of the pages to lock was already locked.
pub const ERROR_WAS_LOCKED: HRESULT = 717;

/// Application popup: %1 : %2
pub const ERROR_LOG_HARD_ERROR: HRESULT = 718;

///  ERROR_ALREADY_WIN32
pub const ERROR_ALREADY_WIN32: HRESULT = 719;

/// The image file %hs is valid, but is for a machine type other than the current machine.
pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH_EXE: HRESULT = 720;

/// A yield execution was performed and no thread was available to run.
pub const ERROR_NO_YIELD_PERFORMED: HRESULT = 721;

/// The resumable flag to a timer API was ignored.
pub const ERROR_TIMER_RESUME_IGNORED: HRESULT = 722;

/// The arbiter has deferred arbitration of these resources to its parent
pub const ERROR_ARBITRATION_UNHANDLED: HRESULT = 723;

/// The inserted CardBus device cannot be started because of a configuration error on "%hs".
pub const ERROR_CARDBUS_NOT_SUPPORTED: HRESULT = 724;

/// The CPUs in this multiprocessor system are not all the same revision level. To use all processors the operating system restricts itself to the features of the least capable processor in the system. Should problems occur with this system, contact the CPU manufacturer to see if this mix of processors is supported.
pub const ERROR_MP_PROCESSOR_MISMATCH: HRESULT = 725;

/// The system was put into hibernation.
pub const ERROR_HIBERNATED: HRESULT = 726;

/// The system was resumed from hibernation.
pub const ERROR_RESUME_HIBERNATION: HRESULT = 727;

/// Windows has detected that the system firmware (BIOS) was updated [previous firmware date = %2, current firmware date %3].
pub const ERROR_FIRMWARE_UPDATED: HRESULT = 728;

/// A device driver is leaking locked I/O pages causing system degradation. The system has automatically enabled tracking code in order to try and catch the culprit.
pub const ERROR_DRIVERS_LEAKING_LOCKED_PAGES: HRESULT = 729;

/// The system has awoken
pub const ERROR_WAKE_SYSTEM: HRESULT = 730;

///  ERROR_WAIT_1
pub const ERROR_WAIT_1: HRESULT = 731;

///  ERROR_WAIT_2
pub const ERROR_WAIT_2: HRESULT = 732;

///  ERROR_WAIT_3
pub const ERROR_WAIT_3: HRESULT = 733;

///  ERROR_WAIT_63
pub const ERROR_WAIT_63: HRESULT = 734;

///  ERROR_ABANDONED_WAIT_0
pub const ERROR_ABANDONED_WAIT_0: HRESULT = 735;

///  ERROR_ABANDONED_WAIT_63
pub const ERROR_ABANDONED_WAIT_63: HRESULT = 736;

///  ERROR_USER_APC
pub const ERROR_USER_APC: HRESULT = 737;

///  ERROR_KERNEL_APC
pub const ERROR_KERNEL_APC: HRESULT = 738;

///  ERROR_ALERTED
pub const ERROR_ALERTED: HRESULT = 739;

/// The requested operation requires elevation.
pub const ERROR_ELEVATION_REQUIRED: HRESULT = 740;

/// A reparse should be performed by the Object Manager since the name of the file resulted in a symbolic link.
pub const ERROR_REPARSE: HRESULT = 741;

/// An open/create operation completed while an oplock break is underway.
pub const ERROR_OPLOCK_BREAK_IN_PROGRESS: HRESULT = 742;

/// A new volume has been mounted by a file system.
pub const ERROR_VOLUME_MOUNTED: HRESULT = 743;

///  This success level status indicates that the transaction state already exists for the registry sub-tree, but that a transaction commit was previously aborted.
///
/// The commit has now been completed.
pub const ERROR_RXACT_COMMITTED: HRESULT = 744;

/// This indicates that a notify change request has been completed due to closing the handle which made the notify change request.
pub const ERROR_NOTIFY_CLEANUP: HRESULT = 745;

///  An attempt was made to connect to the remote server %hs on the primary transport, but the connection failed.
///
/// The computer WAS able to connect on a secondary transport.
pub const ERROR_PRIMARY_TRANSPORT_CONNECT_FAILED: HRESULT = 746;

/// Page fault was a transition fault.
pub const ERROR_PAGE_FAULT_TRANSITION: HRESULT = 747;

/// Page fault was a demand zero fault.
pub const ERROR_PAGE_FAULT_DEMAND_ZERO: HRESULT = 748;

/// Page fault was a demand zero fault.
pub const ERROR_PAGE_FAULT_COPY_ON_WRITE: HRESULT = 749;

/// Page fault was a demand zero fault.
pub const ERROR_PAGE_FAULT_GUARD_PAGE: HRESULT = 750;

/// Page fault was satisfied by reading from a secondary storage device.
pub const ERROR_PAGE_FAULT_PAGING_FILE: HRESULT = 751;

/// Cached page was locked during operation.
pub const ERROR_CACHE_PAGE_LOCKED: HRESULT = 752;

/// Crash dump exists in paging file.
pub const ERROR_CRASH_DUMP: HRESULT = 753;

/// Specified buffer contains all zeros.
pub const ERROR_BUFFER_ALL_ZEROS: HRESULT = 754;

/// A reparse should be performed by the Object Manager since the name of the file resulted in a symbolic link.
pub const ERROR_REPARSE_OBJECT: HRESULT = 755;

/// The device has succeeded a query-stop and its resource requirements have changed.
pub const ERROR_RESOURCE_REQUIREMENTS_CHANGED: HRESULT = 756;

/// The translator has translated these resources into the global space and no further translations should be performed.
pub const ERROR_TRANSLATION_COMPLETE: HRESULT = 757;

/// A process being terminated has no threads to terminate.
pub const ERROR_NOTHING_TO_TERMINATE: HRESULT = 758;

/// The specified process is not part of a job.
pub const ERROR_PROCESS_NOT_IN_JOB: HRESULT = 759;

/// The specified process is part of a job.
pub const ERROR_PROCESS_IN_JOB: HRESULT = 760;

/// The system is now ready for hibernation.
pub const ERROR_VOLSNAP_HIBERNATE_READY: HRESULT = 761;

/// A file system or file system filter driver has successfully completed an FsFilter operation.
pub const ERROR_FSFILTER_OP_COMPLETED_SUCCESSFULLY: HRESULT = 762;

/// The specified interrupt vector was already connected.
pub const ERROR_INTERRUPT_VECTOR_ALREADY_CONNECTED: HRESULT = 763;

/// The specified interrupt vector is still connected.
pub const ERROR_INTERRUPT_STILL_CONNECTED: HRESULT = 764;

/// An operation is blocked waiting for an oplock.
pub const ERROR_WAIT_FOR_OPLOCK: HRESULT = 765;

/// Debugger handled exception
pub const ERROR_DBG_EXCEPTION_HANDLED: HRESULT = 766;

/// Debugger continued
pub const ERROR_DBG_CONTINUE: HRESULT = 767;

/// An exception occurred in a user mode callback and the kernel callback frame should be removed.
pub const ERROR_CALLBACK_POP_STACK: HRESULT = 768;

/// Compression is disabled for this volume.
pub const ERROR_COMPRESSION_DISABLED: HRESULT = 769;

/// The data provider cannot fetch backwards through a result set.
pub const ERROR_CANTFETCHBACKWARDS: HRESULT = 770;

/// The data provider cannot scroll backwards through a result set.
pub const ERROR_CANTSCROLLBACKWARDS: HRESULT = 771;

/// The data provider requires that previously fetched data is released before asking for more data.
pub const ERROR_ROWSNOTRELEASED: HRESULT = 772;

/// The data provider was not able to interpret the flags set for a column binding in an accessor.
pub const ERROR_BAD_ACCESSOR_FLAGS: HRESULT = 773;

/// One or more errors occurred while processing the request.
pub const ERROR_ERRORS_ENCOUNTERED: HRESULT = 774;

/// The implementation is not capable of performing the request.
pub const ERROR_NOT_CAPABLE: HRESULT = 775;

/// The client of a component requested an operation which is not valid given the state of the component instance.
pub const ERROR_REQUEST_OUT_OF_SEQUENCE: HRESULT = 776;

/// A version number could not be parsed.
pub const ERROR_VERSION_PARSE_ERROR: HRESULT = 777;

/// The iterator's start position is invalid.
pub const ERROR_BADSTARTPOSITION: HRESULT = 778;

/// The hardware has reported an uncorrectable memory error.
pub const ERROR_MEMORY_HARDWARE: HRESULT = 779;

/// The attempted operation required self healing to be enabled.
pub const ERROR_DISK_REPAIR_DISABLED: HRESULT = 780;

/// The Desktop heap encountered an error while allocating session memory. There is more information in the system event log.
pub const ERROR_INSUFFICIENT_RESOURCE_FOR_SPECIFIED_SHARED_SECTION_SIZE: HRESULT = 781;

/// The system power state is transitioning from %2 to %3.
pub const ERROR_SYSTEM_POWERSTATE_TRANSITION: HRESULT = 782;

/// The system power state is transitioning from %2 to %3 but could enter %4.
pub const ERROR_SYSTEM_POWERSTATE_COMPLEX_TRANSITION: HRESULT = 783;

/// A thread is getting dispatched with MCA EXCEPTION because of MCA.
pub const ERROR_MCA_EXCEPTION: HRESULT = 784;

/// Access to %1 is monitored by policy rule %2.
pub const ERROR_ACCESS_AUDIT_BY_POLICY: HRESULT = 785;

/// Access to %1 has been restricted by your Administrator by policy rule %2.
pub const ERROR_ACCESS_DISABLED_NO_SAFER_UI_BY_POLICY: HRESULT = 786;

/// A valid hibernation file has been invalidated and should be abandoned.
pub const ERROR_ABANDON_HIBERFILE: HRESULT = 787;

///  Windows was unable to save all the data for the file %hs; the data has been lost.
///
/// This error may be caused by network connectivity issues. Please try to save this file elsewhere.
pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_DISCONNECTED: HRESULT = 788;

///  Windows was unable to save all the data for the file %hs; the data has been lost.
///
/// This error was returned by the server on which the file exists. Please try to save this file elsewhere.
pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_SERVER_ERROR: HRESULT = 789;

///  Windows was unable to save all the data for the file %hs; the data has been lost.
///
/// This error may be caused if the device has been removed or the media is write-protected.
pub const ERROR_LOST_WRITEBEHIND_DATA_LOCAL_DISK_ERROR: HRESULT = 790;

/// The resources required for this device conflict with the MCFG table.
pub const ERROR_BAD_MCFG_TABLE: HRESULT = 791;

///  The volume repair could not be performed while it is online.
///
/// Please schedule to take the volume offline so that it can be repaired.
pub const ERROR_DISK_REPAIR_REDIRECTED: HRESULT = 792;

/// The volume repair was not successful.
pub const ERROR_DISK_REPAIR_UNSUCCESSFUL: HRESULT = 793;

/// One of the volume corruption logs is full. Further corruptions that may be detected won't be logged.
pub const ERROR_CORRUPT_LOG_OVERFULL: HRESULT = 794;

/// One of the volume corruption logs is internally corrupted and needs to be recreated. The volume may contain undetected corruptions and must be scanned.
pub const ERROR_CORRUPT_LOG_CORRUPTED: HRESULT = 795;

/// One of the volume corruption logs is unavailable for being operated on.
pub const ERROR_CORRUPT_LOG_UNAVAILABLE: HRESULT = 796;

/// One of the volume corruption logs was deleted while still having corruption records in them. The volume contains detected corruptions and must be scanned.
pub const ERROR_CORRUPT_LOG_DELETED_FULL: HRESULT = 797;

/// One of the volume corruption logs was cleared by chkdsk and no longer contains real corruptions.
pub const ERROR_CORRUPT_LOG_CLEARED: HRESULT = 798;

/// Orphaned files exist on the volume but could not be recovered because no more new names could be created in the recovery directory. Files must be moved from the recovery directory.
pub const ERROR_ORPHAN_NAME_EXHAUSTED: HRESULT = 799;

/// The oplock that was associated with this handle is now associated with a different handle.
pub const ERROR_OPLOCK_SWITCHED_TO_NEW_HANDLE: HRESULT = 800;

/// An oplock of the requested level cannot be granted.  An oplock of a lower level may be available.
pub const ERROR_CANNOT_GRANT_REQUESTED_OPLOCK: HRESULT = 801;

/// The operation did not complete successfully because it would cause an oplock to be broken. The caller has requested that existing oplocks not be broken.
pub const ERROR_CANNOT_BREAK_OPLOCK: HRESULT = 802;

/// The handle with which this oplock was associated has been closed.  The oplock is now broken.
pub const ERROR_OPLOCK_HANDLE_CLOSED: HRESULT = 803;

/// The specified access control entry (ACE) does not contain a condition.
pub const ERROR_NO_ACE_CONDITION: HRESULT = 804;

/// The specified access control entry (ACE) contains an invalid condition.
pub const ERROR_INVALID_ACE_CONDITION: HRESULT = 805;

/// Access to the specified file handle has been revoked.
pub const ERROR_FILE_HANDLE_REVOKED: HRESULT = 806;

/// An image file was mapped at a different address from the one specified in the image file but fixups will still be automatically performed on the image.
pub const ERROR_IMAGE_AT_DIFFERENT_BASE: HRESULT = 807;

/// The read or write operation to an encrypted file could not be completed because the file has not been opened for data access.
pub const ERROR_ENCRYPTED_IO_NOT_POSSIBLE: HRESULT = 808;

/// File metadata optimization is already in progress.
pub const ERROR_FILE_METADATA_OPTIMIZATION_IN_PROGRESS: HRESULT = 809;

/// The requested operation failed due to quota operation is still in progress.
pub const ERROR_QUOTA_ACTIVITY: HRESULT = 810;

/// Access to the specified handle has been revoked.
pub const ERROR_HANDLE_REVOKED: HRESULT = 811;

/// The callback function must be invoked inline.
pub const ERROR_CALLBACK_INVOKE_INLINE: HRESULT = 812;

/// The specified CPU Set IDs are invalid.
pub const ERROR_CPU_SET_INVALID: HRESULT = 813;

/// The specified enclave has not yet been terminated.
pub const ERROR_ENCLAVE_NOT_TERMINATED: HRESULT = 814;

/// An attempt was made to access protected memory in violation of its secure access policy.
pub const ERROR_ENCLAVE_VIOLATION: HRESULT = 815;

/// Multiple mappings to shared resource(s) on a server, using more than one transport, are not allowed. Use a single transport for all mappings to a server and try again.
pub const ERROR_SERVER_TRANSPORT_CONFLICT: HRESULT = 816;

/// Multiple mappings to shared resource(s) on a server, using different certificate validation preferences, are not allowed. Use the same preference for all mappings to a server and try again.
pub const ERROR_CERTIFICATE_VALIDATION_PREFERENCE_CONFLICT: HRESULT = 817;

/// The specified copy of the requested data could not be read.
pub const ERROR_FT_READ_FROM_COPY_FAILURE: HRESULT = 818;

/// The section creation request was failed because it would have been satisfied with a direct map and the caller explicitly signified this was not wanted.
pub const ERROR_SECTION_DIRECT_MAP_ONLY: HRESULT = 819;

/// Access to the extended attribute was denied.
pub const ERROR_EA_ACCESS_DENIED: HRESULT = 994;

/// The I/O operation has been aborted because of either a thread exit or an application request.
pub const ERROR_OPERATION_ABORTED: HRESULT = 995;

/// Overlapped I/O event is not in a signaled state.
pub const ERROR_IO_INCOMPLETE: HRESULT = 996;

/// Overlapped I/O operation is in progress.
pub const ERROR_IO_PENDING: HRESULT = 997;

/// Invalid access to memory location.
pub const ERROR_NOACCESS: HRESULT = 998;

/// Error performing inpage operation.
pub const ERROR_SWAPERROR: HRESULT = 999;

/// Recursion too deep; the stack overflowed.
pub const ERROR_STACK_OVERFLOW: HRESULT = 1001;

/// The window cannot act on the sent message.
pub const ERROR_INVALID_MESSAGE: HRESULT = 1002;

/// Cannot complete this function.
pub const ERROR_CAN_NOT_COMPLETE: HRESULT = 1003;

/// Invalid flags.
pub const ERROR_INVALID_FLAGS: HRESULT = 1004;

///  The volume does not contain a recognized file system.
///
/// Please make sure that all required file system drivers are loaded and that the volume is not corrupted.
pub const ERROR_UNRECOGNIZED_VOLUME: HRESULT = 1005;

/// The volume for a file has been externally altered so that the opened file is no longer valid.
pub const ERROR_FILE_INVALID: HRESULT = 1006;

/// The requested operation cannot be performed in full-screen mode.
pub const ERROR_FULLSCREEN_MODE: HRESULT = 1007;

/// An attempt was made to reference a token that does not exist.
pub const ERROR_NO_TOKEN: HRESULT = 1008;

/// The configuration registry database is corrupt.
pub const ERROR_BADDB: HRESULT = 1009;

/// The configuration registry key is invalid.
pub const ERROR_BADKEY: HRESULT = 1010;

/// The configuration registry key could not be opened.
pub const ERROR_CANTOPEN: HRESULT = 1011;

/// The configuration registry key could not be read.
pub const ERROR_CANTREAD: HRESULT = 1012;

/// The configuration registry key could not be written.
pub const ERROR_CANTWRITE: HRESULT = 1013;

/// One of the files in the registry database had to be recovered by use of a log or alternate copy. The recovery was successful.
pub const ERROR_REGISTRY_RECOVERED: HRESULT = 1014;

/// The registry is corrupted. The structure of one of the files containing registry data is corrupted, or the system's memory image of the file is corrupted, or the file could not be recovered because the alternate copy or log was absent or corrupted.
pub const ERROR_REGISTRY_CORRUPT: HRESULT = 1015;

/// An I/O operation initiated by the registry failed unrecoverably. The registry could not read in, or write out, or flush, one of the files that contain the system's image of the registry.
pub const ERROR_REGISTRY_IO_FAILED: HRESULT = 1016;

/// The system has attempted to load or restore a file into the registry, but the specified file is not in a registry file format.
pub const ERROR_NOT_REGISTRY_FILE: HRESULT = 1017;

/// Illegal operation attempted on a registry key that has been marked for deletion.
pub const ERROR_KEY_DELETED: HRESULT = 1018;

/// System could not allocate the required space in a registry log.
pub const ERROR_NO_LOG_SPACE: HRESULT = 1019;

/// Cannot create a symbolic link in a registry key that already has subkeys or values.
pub const ERROR_KEY_HAS_CHILDREN: HRESULT = 1020;

/// Cannot create a stable subkey under a volatile parent key.
pub const ERROR_CHILD_MUST_BE_VOLATILE: HRESULT = 1021;

/// A notify change request is being completed and the information is not being returned in the caller's buffer. The caller now needs to enumerate the files to find the changes.
pub const ERROR_NOTIFY_ENUM_DIR: HRESULT = 1022;

/// A stop control has been sent to a service that other running services are dependent on.
pub const ERROR_DEPENDENT_SERVICES_RUNNING: HRESULT = 1051;

/// The requested control is not valid for this service.
pub const ERROR_INVALID_SERVICE_CONTROL: HRESULT = 1052;

/// The service did not respond to the start or control request in a timely fashion.
pub const ERROR_SERVICE_REQUEST_TIMEOUT: HRESULT = 1053;

/// A thread could not be created for the service.
pub const ERROR_SERVICE_NO_THREAD: HRESULT = 1054;

/// The service database is locked.
pub const ERROR_SERVICE_DATABASE_LOCKED: HRESULT = 1055;

/// An instance of the service is already running.
pub const ERROR_SERVICE_ALREADY_RUNNING: HRESULT = 1056;

/// The account name is invalid or does not exist, or the password is invalid for the account name specified.
pub const ERROR_INVALID_SERVICE_ACCOUNT: HRESULT = 1057;

/// The service cannot be started, either because it is disabled or because it has no enabled devices associated with it.
pub const ERROR_SERVICE_DISABLED: HRESULT = 1058;

/// Circular service dependency was specified.
pub const ERROR_CIRCULAR_DEPENDENCY: HRESULT = 1059;

/// The specified service does not exist as an installed service.
pub const ERROR_SERVICE_DOES_NOT_EXIST: HRESULT = 1060;

/// The service cannot accept control messages at this time.
pub const ERROR_SERVICE_CANNOT_ACCEPT_CTRL: HRESULT = 1061;

/// The service has not been started.
pub const ERROR_SERVICE_NOT_ACTIVE: HRESULT = 1062;

/// The service process could not connect to the service controller.
pub const ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: HRESULT = 1063;

/// An exception occurred in the service when handling the control request.
pub const ERROR_EXCEPTION_IN_SERVICE: HRESULT = 1064;

/// The database specified does not exist.
pub const ERROR_DATABASE_DOES_NOT_EXIST: HRESULT = 1065;

/// The service has returned a service-specific error code.
pub const ERROR_SERVICE_SPECIFIC_ERROR: HRESULT = 1066;

/// The process terminated unexpectedly.
pub const ERROR_PROCESS_ABORTED: HRESULT = 1067;

/// The dependency service or group failed to start.
pub const ERROR_SERVICE_DEPENDENCY_FAIL: HRESULT = 1068;

/// The service did not start due to a logon failure.
pub const ERROR_SERVICE_LOGON_FAILED: HRESULT = 1069;

/// After starting, the service hung in a start-pending state.
pub const ERROR_SERVICE_START_HANG: HRESULT = 1070;

/// The specified service database lock is invalid.
pub const ERROR_INVALID_SERVICE_LOCK: HRESULT = 1071;

/// The specified service has been marked for deletion.
pub const ERROR_SERVICE_MARKED_FOR_DELETE: HRESULT = 1072;

/// The specified service already exists.
pub const ERROR_SERVICE_EXISTS: HRESULT = 1073;

/// The system is currently running with the last-known-good configuration.
pub const ERROR_ALREADY_RUNNING_LKG: HRESULT = 1074;

/// The dependency service does not exist or has been marked for deletion.
pub const ERROR_SERVICE_DEPENDENCY_DELETED: HRESULT = 1075;

/// The current boot has already been accepted for use as the last-known-good control set.
pub const ERROR_BOOT_ALREADY_ACCEPTED: HRESULT = 1076;

/// No attempts to start the service have been made since the last boot.
pub const ERROR_SERVICE_NEVER_STARTED: HRESULT = 1077;

/// The name is already in use as either a service name or a service display name.
pub const ERROR_DUPLICATE_SERVICE_NAME: HRESULT = 1078;

/// The account specified for this service is different from the account specified for other services running in the same process.
pub const ERROR_DIFFERENT_SERVICE_ACCOUNT: HRESULT = 1079;

/// Failure actions can only be set for Win32 services, not for drivers.
pub const ERROR_CANNOT_DETECT_DRIVER_FAILURE: HRESULT = 1080;

///  This service runs in the same process as the service control manager.
///
/// Therefore, the service control manager cannot take action if this service's process terminates unexpectedly.
pub const ERROR_CANNOT_DETECT_PROCESS_ABORT: HRESULT = 1081;

/// No recovery program has been configured for this service.
pub const ERROR_NO_RECOVERY_PROGRAM: HRESULT = 1082;

/// The executable program that this service is configured to run in does not implement the service.
pub const ERROR_SERVICE_NOT_IN_EXE: HRESULT = 1083;

/// This service cannot be started in Safe Mode
pub const ERROR_NOT_SAFEBOOT_SERVICE: HRESULT = 1084;

/// The physical end of the tape has been reached.
pub const ERROR_END_OF_MEDIA: HRESULT = 1100;

/// A tape access reached a filemark.
pub const ERROR_FILEMARK_DETECTED: HRESULT = 1101;

/// The beginning of the tape or a partition was encountered.
pub const ERROR_BEGINNING_OF_MEDIA: HRESULT = 1102;

/// A tape access reached the end of a set of files.
pub const ERROR_SETMARK_DETECTED: HRESULT = 1103;

/// No more data is on the tape.
pub const ERROR_NO_DATA_DETECTED: HRESULT = 1104;

/// Tape could not be partitioned.
pub const ERROR_PARTITION_FAILURE: HRESULT = 1105;

/// When accessing a new tape of a multivolume partition, the current block size is incorrect.
pub const ERROR_INVALID_BLOCK_LENGTH: HRESULT = 1106;

/// Tape partition information could not be found when loading a tape.
pub const ERROR_DEVICE_NOT_PARTITIONED: HRESULT = 1107;

/// Unable to lock the media eject mechanism.
pub const ERROR_UNABLE_TO_LOCK_MEDIA: HRESULT = 1108;

/// Unable to unload the media.
pub const ERROR_UNABLE_TO_UNLOAD_MEDIA: HRESULT = 1109;

/// The media in the drive may have changed.
pub const ERROR_MEDIA_CHANGED: HRESULT = 1110;

/// The I/O bus was reset.
pub const ERROR_BUS_RESET: HRESULT = 1111;

/// No media in drive.
pub const ERROR_NO_MEDIA_IN_DRIVE: HRESULT = 1112;

/// No mapping for the Unicode character exists in the target multi-byte code page.
pub const ERROR_NO_UNICODE_TRANSLATION: HRESULT = 1113;

/// A dynamic link library (DLL) initialization routine failed.
pub const ERROR_DLL_INIT_FAILED: HRESULT = 1114;

/// A system shutdown is in progress.
pub const ERROR_SHUTDOWN_IN_PROGRESS: HRESULT = 1115;

/// Unable to abort the system shutdown because no shutdown was in progress.
pub const ERROR_NO_SHUTDOWN_IN_PROGRESS: HRESULT = 1116;

/// The request could not be performed because of an I/O device error.
pub const ERROR_IO_DEVICE: HRESULT = 1117;

/// No serial device was successfully initialized. The serial driver will unload.
pub const ERROR_SERIAL_NO_DEVICE: HRESULT = 1118;

/// Unable to open a device that was sharing an interrupt request (IRQ) with other devices. At least one other device that uses that IRQ was already opened.
pub const ERROR_IRQ_BUSY: HRESULT = 1119;

///  A serial I/O operation was completed by another write to the serial port.
///
/// (The IOCTL_SERIAL_XOFF_COUNTER reached zero.)
pub const ERROR_MORE_WRITES: HRESULT = 1120;

///  A serial I/O operation completed because the timeout period expired.
///
/// (The IOCTL_SERIAL_XOFF_COUNTER did not reach zero.)
pub const ERROR_COUNTER_TIMEOUT: HRESULT = 1121;

/// No ID address mark was found on the floppy disk.
pub const ERROR_FLOPPY_ID_MARK_NOT_FOUND: HRESULT = 1122;

/// Mismatch between the floppy disk sector ID field and the floppy disk controller track address.
pub const ERROR_FLOPPY_WRONG_CYLINDER: HRESULT = 1123;

/// The floppy disk controller reported an error that is not recognized by the floppy disk driver.
pub const ERROR_FLOPPY_UNKNOWN_ERROR: HRESULT = 1124;

/// The floppy disk controller returned inconsistent results in its registers.
pub const ERROR_FLOPPY_BAD_REGISTERS: HRESULT = 1125;

/// While accessing the hard disk, a recalibrate operation failed, even after retries.
pub const ERROR_DISK_RECALIBRATE_FAILED: HRESULT = 1126;

/// While accessing the hard disk, a disk operation failed even after retries.
pub const ERROR_DISK_OPERATION_FAILED: HRESULT = 1127;

/// While accessing the hard disk, a disk controller reset was needed, but even that failed.
pub const ERROR_DISK_RESET_FAILED: HRESULT = 1128;

/// Physical end of tape encountered.
pub const ERROR_EOM_OVERFLOW: HRESULT = 1129;

/// Not enough server memory resources are available to process this command.
pub const ERROR_NOT_ENOUGH_SERVER_MEMORY: HRESULT = 1130;

/// A potential deadlock condition has been detected.
pub const ERROR_POSSIBLE_DEADLOCK: HRESULT = 1131;

/// The base address or the file offset specified does not have the proper alignment.
pub const ERROR_MAPPED_ALIGNMENT: HRESULT = 1132;

/// An attempt to change the system power state was vetoed by another application or driver.
pub const ERROR_SET_POWER_STATE_VETOED: HRESULT = 1140;

/// The system BIOS failed an attempt to change the system power state.
pub const ERROR_SET_POWER_STATE_FAILED: HRESULT = 1141;

/// An attempt was made to create more links on a file than the file system supports.
pub const ERROR_TOO_MANY_LINKS: HRESULT = 1142;

/// The specified program requires a newer version of Windows.
pub const ERROR_OLD_WIN_VERSION: HRESULT = 1150;

/// The specified program is not a Windows or MS-DOS program.
pub const ERROR_APP_WRONG_OS: HRESULT = 1151;

/// Cannot start more than one instance of the specified program.
pub const ERROR_SINGLE_INSTANCE_APP: HRESULT = 1152;

/// The specified program was written for an earlier version of Windows.
pub const ERROR_RMODE_APP: HRESULT = 1153;

/// One of the library files needed to run this application is damaged.
pub const ERROR_INVALID_DLL: HRESULT = 1154;

/// No application is associated with the specified file for this operation.
pub const ERROR_NO_ASSOCIATION: HRESULT = 1155;

/// An error occurred in sending the command to the application.
pub const ERROR_DDE_FAIL: HRESULT = 1156;

/// One of the library files needed to run this application cannot be found.
pub const ERROR_DLL_NOT_FOUND: HRESULT = 1157;

/// The current process has used all of its system allowance of handles for Window Manager objects.
pub const ERROR_NO_MORE_USER_HANDLES: HRESULT = 1158;

/// The message can be used only with synchronous operations.
pub const ERROR_MESSAGE_SYNC_ONLY: HRESULT = 1159;

/// The indicated source element has no media.
pub const ERROR_SOURCE_ELEMENT_EMPTY: HRESULT = 1160;

/// The indicated destination element already contains media.
pub const ERROR_DESTINATION_ELEMENT_FULL: HRESULT = 1161;

/// The indicated element does not exist.
pub const ERROR_ILLEGAL_ELEMENT_ADDRESS: HRESULT = 1162;

/// The indicated element is part of a magazine that is not present.
pub const ERROR_MAGAZINE_NOT_PRESENT: HRESULT = 1163;

/// The indicated device requires reinitialization due to hardware errors.
pub const ERROR_DEVICE_REINITIALIZATION_NEEDED: HRESULT = 1164;

/// The device has indicated that cleaning is required before further operations are attempted.
pub const ERROR_DEVICE_REQUIRES_CLEANING: HRESULT = 1165;

/// The device has indicated that its door is open.
pub const ERROR_DEVICE_DOOR_OPEN: HRESULT = 1166;

/// The device is not connected.
pub const ERROR_DEVICE_NOT_CONNECTED: HRESULT = 1167;

/// Element not found.
pub const ERROR_NOT_FOUND: HRESULT = 1168;

/// There was no match for the specified key in the index.
pub const ERROR_NO_MATCH: HRESULT = 1169;

/// The property set specified does not exist on the object.
pub const ERROR_SET_NOT_FOUND: HRESULT = 1170;

/// The point passed to GetMouseMovePoints is not in the buffer.
pub const ERROR_POINT_NOT_FOUND: HRESULT = 1171;

/// The tracking (workstation) service is not running.
pub const ERROR_NO_TRACKING_SERVICE: HRESULT = 1172;

/// The Volume ID could not be found.
pub const ERROR_NO_VOLUME_ID: HRESULT = 1173;

/// Unable to remove the file to be replaced.
pub const ERROR_UNABLE_TO_REMOVE_REPLACED: HRESULT = 1175;

/// Unable to move the replacement file to the file to be replaced. The file to be replaced has retained its original name.
pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT: HRESULT = 1176;

/// Unable to move the replacement file to the file to be replaced. The file to be replaced has been renamed using the backup name.
pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT_2: HRESULT = 1177;

/// The volume change journal is being deleted.
pub const ERROR_JOURNAL_DELETE_IN_PROGRESS: HRESULT = 1178;

/// The volume change journal is not active.
pub const ERROR_JOURNAL_NOT_ACTIVE: HRESULT = 1179;

/// A file was found, but it may not be the correct file.
pub const ERROR_POTENTIAL_FILE_FOUND: HRESULT = 1180;

/// The journal entry has been deleted from the journal.
pub const ERROR_JOURNAL_ENTRY_DELETED: HRESULT = 1181;

/// An attempt was made to access a partition that has begun termination.
pub const ERROR_PARTITION_TERMINATING: HRESULT = 1184;

/// A system shutdown has already been scheduled.
pub const ERROR_SHUTDOWN_IS_SCHEDULED: HRESULT = 1190;

/// The system shutdown cannot be initiated because there are other users logged on to the computer.
pub const ERROR_SHUTDOWN_USERS_LOGGED_ON: HRESULT = 1191;

/// The system shutdown cannot safely proceed without enabling node maintenance mode for cluster node and waiting for the drain to complete.
pub const ERROR_SHUTDOWN_DISKS_NOT_IN_MAINTENANCE_MODE: HRESULT = 1192;

/// The specified device name is invalid.
pub const ERROR_BAD_DEVICE: HRESULT = 1200;

/// The device is not currently connected but it is a remembered connection.
pub const ERROR_CONNECTION_UNAVAIL: HRESULT = 1201;

/// The local device name has a remembered connection to another network resource.
pub const ERROR_DEVICE_ALREADY_REMEMBERED: HRESULT = 1202;

/// The network path was either typed incorrectly, does not exist, or the network provider is not currently available. Please try retyping the path or contact your network administrator.
pub const ERROR_NO_NET_OR_BAD_PATH: HRESULT = 1203;

/// The specified network provider name is invalid.
pub const ERROR_BAD_PROVIDER: HRESULT = 1204;

/// Unable to open the network connection profile.
pub const ERROR_CANNOT_OPEN_PROFILE: HRESULT = 1205;

/// The network connection profile is corrupted.
pub const ERROR_BAD_PROFILE: HRESULT = 1206;

/// Cannot enumerate a noncontainer.
pub const ERROR_NOT_CONTAINER: HRESULT = 1207;

/// An extended error has occurred.
pub const ERROR_EXTENDED_ERROR: HRESULT = 1208;

/// The format of the specified group name is invalid.
pub const ERROR_INVALID_GROUPNAME: HRESULT = 1209;

/// The format of the specified computer name is invalid.
pub const ERROR_INVALID_COMPUTERNAME: HRESULT = 1210;

/// The format of the specified event name is invalid.
pub const ERROR_INVALID_EVENTNAME: HRESULT = 1211;

/// The format of the specified domain name is invalid.
pub const ERROR_INVALID_DOMAINNAME: HRESULT = 1212;

/// The format of the specified service name is invalid.
pub const ERROR_INVALID_SERVICENAME: HRESULT = 1213;

/// The format of the specified network name is invalid.
pub const ERROR_INVALID_NETNAME: HRESULT = 1214;

/// The format of the specified share name is invalid.
pub const ERROR_INVALID_SHARENAME: HRESULT = 1215;

/// The format of the specified password is invalid.
pub const ERROR_INVALID_PASSWORDNAME: HRESULT = 1216;

/// The format of the specified message name is invalid.
pub const ERROR_INVALID_MESSAGENAME: HRESULT = 1217;

/// The format of the specified message destination is invalid.
pub const ERROR_INVALID_MESSAGEDEST: HRESULT = 1218;

/// Multiple connections to a server or shared resource by the same user, using more than one user name, are not allowed. Disconnect all previous connections to the server or shared resource and try again.
pub const ERROR_SESSION_CREDENTIAL_CONFLICT: HRESULT = 1219;

/// An attempt was made to establish a session to a network server, but there are already too many sessions established to that server.
pub const ERROR_REMOTE_SESSION_LIMIT_EXCEEDED: HRESULT = 1220;

/// The workgroup or domain name is already in use by another computer on the network.
pub const ERROR_DUP_DOMAINNAME: HRESULT = 1221;

/// The network is not present or not started.
pub const ERROR_NO_NETWORK: HRESULT = 1222;

/// The operation was canceled by the user.
pub const ERROR_CANCELLED: HRESULT = 1223;

/// The requested operation cannot be performed on a file with a user-mapped section open.
pub const ERROR_USER_MAPPED_FILE: HRESULT = 1224;

/// The remote computer refused the network connection.
pub const ERROR_CONNECTION_REFUSED: HRESULT = 1225;

/// The network connection was gracefully closed.
pub const ERROR_GRACEFUL_DISCONNECT: HRESULT = 1226;

/// The network transport endpoint already has an address associated with it.
pub const ERROR_ADDRESS_ALREADY_ASSOCIATED: HRESULT = 1227;

/// An address has not yet been associated with the network endpoint.
pub const ERROR_ADDRESS_NOT_ASSOCIATED: HRESULT = 1228;

/// An operation was attempted on a nonexistent network connection.
pub const ERROR_CONNECTION_INVALID: HRESULT = 1229;

/// An invalid operation was attempted on an active network connection.
pub const ERROR_CONNECTION_ACTIVE: HRESULT = 1230;

/// The network location cannot be reached. For information about network troubleshooting, see Windows Help.
pub const ERROR_NETWORK_UNREACHABLE: HRESULT = 1231;

/// The network location cannot be reached. For information about network troubleshooting, see Windows Help.
pub const ERROR_HOST_UNREACHABLE: HRESULT = 1232;

/// The network location cannot be reached. For information about network troubleshooting, see Windows Help.
pub const ERROR_PROTOCOL_UNREACHABLE: HRESULT = 1233;

/// No service is operating at the destination network endpoint on the remote system.
pub const ERROR_PORT_UNREACHABLE: HRESULT = 1234;

/// The request was aborted.
pub const ERROR_REQUEST_ABORTED: HRESULT = 1235;

/// The network connection was aborted by the local system.
pub const ERROR_CONNECTION_ABORTED: HRESULT = 1236;

/// The operation could not be completed. A retry should be performed.
pub const ERROR_RETRY: HRESULT = 1237;

/// A connection to the server could not be made because the limit on the number of concurrent connections for this account has been reached.
pub const ERROR_CONNECTION_COUNT_LIMIT: HRESULT = 1238;

/// Attempting to log in during an unauthorized time of day for this account.
pub const ERROR_LOGIN_TIME_RESTRICTION: HRESULT = 1239;

/// The account is not authorized to log in from this station.
pub const ERROR_LOGIN_WKSTA_RESTRICTION: HRESULT = 1240;

/// The network address could not be used for the operation requested.
pub const ERROR_INCORRECT_ADDRESS: HRESULT = 1241;

/// The service is already registered.
pub const ERROR_ALREADY_REGISTERED: HRESULT = 1242;

/// The specified service does not exist.
pub const ERROR_SERVICE_NOT_FOUND: HRESULT = 1243;

/// The operation being requested was not performed because the user has not been authenticated.
pub const ERROR_NOT_AUTHENTICATED: HRESULT = 1244;

/// The operation being requested was not performed because the user has not logged on to the network. The specified service does not exist.
pub const ERROR_NOT_LOGGED_ON: HRESULT = 1245;

/// Continue with work in progress.
pub const ERROR_CONTINUE: HRESULT = 1246;

/// An attempt was made to perform an initialization operation when initialization has already been completed.
pub const ERROR_ALREADY_INITIALIZED: HRESULT = 1247;

/// No more local devices.
pub const ERROR_NO_MORE_DEVICES: HRESULT = 1248;

/// The specified site does not exist.
pub const ERROR_NO_SUCH_SITE: HRESULT = 1249;

/// A domain controller with the specified name already exists.
pub const ERROR_DOMAIN_CONTROLLER_EXISTS: HRESULT = 1250;

/// This operation is supported only when you are connected to the server.
pub const ERROR_ONLY_IF_CONNECTED: HRESULT = 1251;

/// The group policy framework should call the extension even if there are no changes.
pub const ERROR_OVERRIDE_NOCHANGES: HRESULT = 1252;

/// The specified user does not have a valid profile.
pub const ERROR_BAD_USER_PROFILE: HRESULT = 1253;

/// This operation is not supported on a computer running Windows Server 2003 for Small Business Server
pub const ERROR_NOT_SUPPORTED_ON_SBS: HRESULT = 1254;

/// The server machine is shutting down.
pub const ERROR_SERVER_SHUTDOWN_IN_PROGRESS: HRESULT = 1255;

/// The remote system is not available. For information about network troubleshooting, see Windows Help.
pub const ERROR_HOST_DOWN: HRESULT = 1256;

/// The security identifier provided is not from an account domain.
pub const ERROR_NON_ACCOUNT_SID: HRESULT = 1257;

/// The security identifier provided does not have a domain component.
pub const ERROR_NON_DOMAIN_SID: HRESULT = 1258;

/// AppHelp dialog canceled thus preventing the application from starting.
pub const ERROR_APPHELP_BLOCK: HRESULT = 1259;

/// This program is blocked by group policy. For more information, contact your system administrator.
pub const ERROR_ACCESS_DISABLED_BY_POLICY: HRESULT = 1260;

/// A program attempt to use an invalid register value. Normally caused by an uninitialized register. This error is Itanium specific.
pub const ERROR_REG_NAT_CONSUMPTION: HRESULT = 1261;

/// The share is currently offline or does not exist.
pub const ERROR_CSCSHARE_OFFLINE: HRESULT = 1262;

/// The Kerberos protocol encountered an error while validating the KDC certificate during smartcard logon. There is more information in the system event log.
pub const ERROR_PKINIT_FAILURE: HRESULT = 1263;

/// The Kerberos protocol encountered an error while attempting to utilize the smartcard subsystem.
pub const ERROR_SMARTCARD_SUBSYSTEM_FAILURE: HRESULT = 1264;

/// The system cannot contact a domain controller to service the authentication request. Please try again later.
pub const ERROR_DOWNGRADE_DETECTED: HRESULT = 1265;

/// The machine is locked and cannot be shut down without the force option.
pub const ERROR_MACHINE_LOCKED: HRESULT = 1271;

/// You can't access this shared folder because your organization's security policies block unauthenticated guest access. These policies help protect your PC from unsafe or malicious devices on the network.
pub const ERROR_SMB_GUEST_LOGON_BLOCKED: HRESULT = 1272;

/// An application-defined callback gave invalid data when called.
pub const ERROR_CALLBACK_SUPPLIED_INVALID_DATA: HRESULT = 1273;

/// The group policy framework should call the extension in the synchronous foreground policy refresh.
pub const ERROR_SYNC_FOREGROUND_REFRESH_REQUIRED: HRESULT = 1274;

/// This driver has been blocked from loading
pub const ERROR_DRIVER_BLOCKED: HRESULT = 1275;

/// A dynamic link library (DLL) referenced a module that was neither a DLL nor the process's executable image.
pub const ERROR_INVALID_IMPORT_OF_NON_DLL: HRESULT = 1276;

/// Windows cannot open this program since it has been disabled.
pub const ERROR_ACCESS_DISABLED_WEBBLADE: HRESULT = 1277;

/// Windows cannot open this program because the license enforcement system has been tampered with or become corrupted.
pub const ERROR_ACCESS_DISABLED_WEBBLADE_TAMPER: HRESULT = 1278;

/// A transaction recover failed.
pub const ERROR_RECOVERY_FAILURE: HRESULT = 1279;

/// The current thread has already been converted to a fiber.
pub const ERROR_ALREADY_FIBER: HRESULT = 1280;

/// The current thread has already been converted from a fiber.
pub const ERROR_ALREADY_THREAD: HRESULT = 1281;

/// The system detected an overrun of a stack-based buffer in this application. This overrun could potentially allow a malicious user to gain control of this application.
pub const ERROR_STACK_BUFFER_OVERRUN: HRESULT = 1282;

/// Data present in one of the parameters is more than the function can operate on.
pub const ERROR_PARAMETER_QUOTA_EXCEEDED: HRESULT = 1283;

/// An attempt to do an operation on a debug object failed because the object is in the process of being deleted.
pub const ERROR_DEBUGGER_INACTIVE: HRESULT = 1284;

/// An attempt to delay-load a .dll or get a function address in a delay-loaded .dll failed.
pub const ERROR_DELAY_LOAD_FAILED: HRESULT = 1285;

/// %1 is a 16-bit application. You do not have permissions to execute 16-bit applications. Check your permissions with your system administrator.
pub const ERROR_VDM_DISALLOWED: HRESULT = 1286;

/// Insufficient information exists to identify the cause of failure.
pub const ERROR_UNIDENTIFIED_ERROR: HRESULT = 1287;

/// The parameter passed to a C runtime function is incorrect.
pub const ERROR_INVALID_CRUNTIME_PARAMETER: HRESULT = 1288;

/// The operation occurred beyond the valid data length of the file.
pub const ERROR_BEYOND_VDL: HRESULT = 1289;

/// The service start failed since one or more services in the same process have an incompatible service SID type setting. A service with restricted service SID type can only coexist in the same process with other services with a restricted SID type. If the service SID type for this service was just configured, the hosting process must be restarted in order to start this service.
pub const ERROR_INCOMPATIBLE_SERVICE_SID_TYPE: HRESULT = 1290;

/// The process hosting the driver for this device has been terminated.
pub const ERROR_DRIVER_PROCESS_TERMINATED: HRESULT = 1291;

/// An operation attempted to exceed an implementation-defined limit.
pub const ERROR_IMPLEMENTATION_LIMIT: HRESULT = 1292;

/// Either the target process, or the target thread's containing process, is a protected process.
pub const ERROR_PROCESS_IS_PROTECTED: HRESULT = 1293;

/// The service notification client is lagging too far behind the current state of services in the machine.
pub const ERROR_SERVICE_NOTIFY_CLIENT_LAGGING: HRESULT = 1294;

///  The requested file operation failed because the storage quota was exceeded.
///
/// To free up disk space, move files to a different location or delete unnecessary files. For more information, contact your system administrator.
pub const ERROR_DISK_QUOTA_EXCEEDED: HRESULT = 1295;

/// The requested file operation failed because the storage policy blocks that type of file. For more information, contact your system administrator.
pub const ERROR_CONTENT_BLOCKED: HRESULT = 1296;

///  A privilege that the service requires to function properly does not exist in the service account configuration.
///
/// You may use the Services Microsoft Management Console (MMC) snap-in (services.msc) and the Local Security Settings MMC snap-in (secpol.msc) to view the service configuration and the account configuration.
pub const ERROR_INCOMPATIBLE_SERVICE_PRIVILEGE: HRESULT = 1297;

/// A thread involved in this operation appears to be unresponsive.
pub const ERROR_APP_HANG: HRESULT = 1298;

/// Indicates a particular Security ID may not be assigned as the label of an object.
pub const ERROR_INVALID_LABEL: HRESULT = 1299;

/// Not all privileges or groups referenced are assigned to the caller.
pub const ERROR_NOT_ALL_ASSIGNED: HRESULT = 1300;

/// Some mapping between account names and security IDs was not done.
pub const ERROR_SOME_NOT_MAPPED: HRESULT = 1301;

/// No system quota limits are specifically set for this account.
pub const ERROR_NO_QUOTAS_FOR_ACCOUNT: HRESULT = 1302;

/// No encryption key is available. A well-known encryption key was returned.
pub const ERROR_LOCAL_USER_SESSION_KEY: HRESULT = 1303;

/// The password is too complex to be converted to a LAN Manager password. The LAN Manager password returned is a NULL string.
pub const ERROR_NULL_LM_PASSWORD: HRESULT = 1304;

/// The revision level is unknown.
pub const ERROR_UNKNOWN_REVISION: HRESULT = 1305;

/// Indicates two revision levels are incompatible.
pub const ERROR_REVISION_MISMATCH: HRESULT = 1306;

/// This security ID may not be assigned as the owner of this object.
pub const ERROR_INVALID_OWNER: HRESULT = 1307;

/// This security ID may not be assigned as the primary group of an object.
pub const ERROR_INVALID_PRIMARY_GROUP: HRESULT = 1308;

/// An attempt has been made to operate on an impersonation token by a thread that is not currently impersonating a client.
pub const ERROR_NO_IMPERSONATION_TOKEN: HRESULT = 1309;

/// The group may not be disabled.
pub const ERROR_CANT_DISABLE_MANDATORY: HRESULT = 1310;

/// We can't sign you in with this credential because your domain isn't available. Make sure your device is connected to your organization's network and try again. If you previously signed in on this device with another credential, you can sign in with that credential.
pub const ERROR_NO_LOGON_SERVERS: HRESULT = 1311;

/// A specified logon session does not exist. It may already have been terminated.
pub const ERROR_NO_SUCH_LOGON_SESSION: HRESULT = 1312;

/// A specified privilege does not exist.
pub const ERROR_NO_SUCH_PRIVILEGE: HRESULT = 1313;

/// A required privilege is not held by the client.
pub const ERROR_PRIVILEGE_NOT_HELD: HRESULT = 1314;

/// The name provided is not a properly formed account name.
pub const ERROR_INVALID_ACCOUNT_NAME: HRESULT = 1315;

/// The specified account already exists.
pub const ERROR_USER_EXISTS: HRESULT = 1316;

/// The specified account does not exist.
pub const ERROR_NO_SUCH_USER: HRESULT = 1317;

/// The specified group already exists.
pub const ERROR_GROUP_EXISTS: HRESULT = 1318;

/// The specified group does not exist.
pub const ERROR_NO_SUCH_GROUP: HRESULT = 1319;

/// Either the specified user account is already a member of the specified group, or the specified group cannot be deleted because it contains a member.
pub const ERROR_MEMBER_IN_GROUP: HRESULT = 1320;

/// The specified user account is not a member of the specified group account.
pub const ERROR_MEMBER_NOT_IN_GROUP: HRESULT = 1321;

/// This operation is disallowed as it could result in an administration account being disabled, deleted or unable to logon.
pub const ERROR_LAST_ADMIN: HRESULT = 1322;

/// Unable to update the password. The value provided as the current password is incorrect.
pub const ERROR_WRONG_PASSWORD: HRESULT = 1323;

/// Unable to update the password. The value provided for the new password contains values that are not allowed in passwords.
pub const ERROR_ILL_FORMED_PASSWORD: HRESULT = 1324;

/// Unable to update the password. The value provided for the new password does not meet the length, complexity, or history requirements of the domain.
pub const ERROR_PASSWORD_RESTRICTION: HRESULT = 1325;

/// The user name or password is incorrect.
pub const ERROR_LOGON_FAILURE: HRESULT = 1326;

/// Account restrictions are preventing this user from signing in. For example: blank passwords aren't allowed, sign-in times are limited, or a policy restriction has been enforced.
pub const ERROR_ACCOUNT_RESTRICTION: HRESULT = 1327;

/// Your account has time restrictions that keep you from signing in right now.
pub const ERROR_INVALID_LOGON_HOURS: HRESULT = 1328;

/// This user isn't allowed to sign in to this computer.
pub const ERROR_INVALID_WORKSTATION: HRESULT = 1329;

/// The password for this account has expired.
pub const ERROR_PASSWORD_EXPIRED: HRESULT = 1330;

/// This user can't sign in because this account is currently disabled.
pub const ERROR_ACCOUNT_DISABLED: HRESULT = 1331;

/// No mapping between account names and security IDs was done.
pub const ERROR_NONE_MAPPED: HRESULT = 1332;

/// Too many local user identifiers (LUIDs) were requested at one time.
pub const ERROR_TOO_MANY_LUIDS_REQUESTED: HRESULT = 1333;

/// No more local user identifiers (LUIDs) are available.
pub const ERROR_LUIDS_EXHAUSTED: HRESULT = 1334;

/// The subauthority part of a security ID is invalid for this particular use.
pub const ERROR_INVALID_SUB_AUTHORITY: HRESULT = 1335;

/// The access control list (ACL) structure is invalid.
pub const ERROR_INVALID_ACL: HRESULT = 1336;

/// The security ID structure is invalid.
pub const ERROR_INVALID_SID: HRESULT = 1337;

/// The security descriptor structure is invalid.
pub const ERROR_INVALID_SECURITY_DESCR: HRESULT = 1338;

/// The inherited access control list (ACL) or access control entry (ACE) could not be built.
pub const ERROR_BAD_INHERITANCE_ACL: HRESULT = 1340;

/// The server is currently disabled.
pub const ERROR_SERVER_DISABLED: HRESULT = 1341;

/// The server is currently enabled.
pub const ERROR_SERVER_NOT_DISABLED: HRESULT = 1342;

/// The value provided was an invalid value for an identifier authority.
pub const ERROR_INVALID_ID_AUTHORITY: HRESULT = 1343;

/// No more memory is available for security information updates.
pub const ERROR_ALLOTTED_SPACE_EXCEEDED: HRESULT = 1344;

/// The specified attributes are invalid, or incompatible with the attributes for the group as a whole.
pub const ERROR_INVALID_GROUP_ATTRIBUTES: HRESULT = 1345;

/// Either a required impersonation level was not provided, or the provided impersonation level is invalid.
pub const ERROR_BAD_IMPERSONATION_LEVEL: HRESULT = 1346;

/// Cannot open an anonymous level security token.
pub const ERROR_CANT_OPEN_ANONYMOUS: HRESULT = 1347;

/// The validation information class requested was invalid.
pub const ERROR_BAD_VALIDATION_CLASS: HRESULT = 1348;

/// The type of the token is inappropriate for its attempted use.
pub const ERROR_BAD_TOKEN_TYPE: HRESULT = 1349;

/// Unable to perform a security operation on an object that has no associated security.
pub const ERROR_NO_SECURITY_ON_OBJECT: HRESULT = 1350;

/// Configuration information could not be read from the domain controller, either because the machine is unavailable, or access has been denied.
pub const ERROR_CANT_ACCESS_DOMAIN_INFO: HRESULT = 1351;

/// The security account manager (SAM) or local security authority (LSA) server was in the wrong state to perform the security operation.
pub const ERROR_INVALID_SERVER_STATE: HRESULT = 1352;

/// The domain was in the wrong state to perform the security operation.
pub const ERROR_INVALID_DOMAIN_STATE: HRESULT = 1353;

/// This operation is only allowed for the Primary Domain Controller of the domain.
pub const ERROR_INVALID_DOMAIN_ROLE: HRESULT = 1354;

/// The specified domain either does not exist or could not be contacted.
pub const ERROR_NO_SUCH_DOMAIN: HRESULT = 1355;

/// The specified domain already exists.
pub const ERROR_DOMAIN_EXISTS: HRESULT = 1356;

/// An attempt was made to exceed the limit on the number of domains per server.
pub const ERROR_DOMAIN_LIMIT_EXCEEDED: HRESULT = 1357;

/// Unable to complete the requested operation because of either a catastrophic media failure or a data structure corruption on the disk.
pub const ERROR_INTERNAL_DB_CORRUPTION: HRESULT = 1358;

/// An internal error occurred.
pub const ERROR_INTERNAL_ERROR: HRESULT = 1359;

/// Generic access types were contained in an access mask which should already be mapped to nongeneric types.
pub const ERROR_GENERIC_NOT_MAPPED: HRESULT = 1360;

/// A security descriptor is not in the right format (absolute or self-relative).
pub const ERROR_BAD_DESCRIPTOR_FORMAT: HRESULT = 1361;

/// The requested action is restricted for use by logon processes only. The calling process has not registered as a logon process.
pub const ERROR_NOT_LOGON_PROCESS: HRESULT = 1362;

/// Cannot start a new logon session with an ID that is already in use.
pub const ERROR_LOGON_SESSION_EXISTS: HRESULT = 1363;

/// A specified authentication package is unknown.
pub const ERROR_NO_SUCH_PACKAGE: HRESULT = 1364;

/// The logon session is not in a state that is consistent with the requested operation.
pub const ERROR_BAD_LOGON_SESSION_STATE: HRESULT = 1365;

/// The logon session ID is already in use.
pub const ERROR_LOGON_SESSION_COLLISION: HRESULT = 1366;

/// A logon request contained an invalid logon type value.
pub const ERROR_INVALID_LOGON_TYPE: HRESULT = 1367;

/// Unable to impersonate using a named pipe until data has been read from that pipe.
pub const ERROR_CANNOT_IMPERSONATE: HRESULT = 1368;

/// The transaction state of a registry subtree is incompatible with the requested operation.
pub const ERROR_RXACT_INVALID_STATE: HRESULT = 1369;

/// An internal security database corruption has been encountered.
pub const ERROR_RXACT_COMMIT_FAILURE: HRESULT = 1370;

/// Cannot perform this operation on built-in accounts.
pub const ERROR_SPECIAL_ACCOUNT: HRESULT = 1371;

/// Cannot perform this operation on this built-in special group.
pub const ERROR_SPECIAL_GROUP: HRESULT = 1372;

/// Cannot perform this operation on this built-in special user.
pub const ERROR_SPECIAL_USER: HRESULT = 1373;

/// The user cannot be removed from a group because the group is currently the user's primary group.
pub const ERROR_MEMBERS_PRIMARY_GROUP: HRESULT = 1374;

/// The token is already in use as a primary token.
pub const ERROR_TOKEN_ALREADY_IN_USE: HRESULT = 1375;

/// The specified local group does not exist.
pub const ERROR_NO_SUCH_ALIAS: HRESULT = 1376;

/// The specified account name is not a member of the group.
pub const ERROR_MEMBER_NOT_IN_ALIAS: HRESULT = 1377;

/// The specified account name is already a member of the group.
pub const ERROR_MEMBER_IN_ALIAS: HRESULT = 1378;

/// The specified local group already exists.
pub const ERROR_ALIAS_EXISTS: HRESULT = 1379;

/// Logon failure: the user has not been granted the requested logon type at this computer.
pub const ERROR_LOGON_NOT_GRANTED: HRESULT = 1380;

/// The maximum number of secrets that may be stored in a single system has been exceeded.
pub const ERROR_TOO_MANY_SECRETS: HRESULT = 1381;

/// The length of a secret exceeds the maximum length allowed.
pub const ERROR_SECRET_TOO_LONG: HRESULT = 1382;

/// The local security authority database contains an internal inconsistency.
pub const ERROR_INTERNAL_DB_ERROR: HRESULT = 1383;

/// During a logon attempt, the user's security context accumulated too many security IDs.
pub const ERROR_TOO_MANY_CONTEXT_IDS: HRESULT = 1384;

/// Logon failure: the user has not been granted the requested logon type at this computer.
pub const ERROR_LOGON_TYPE_NOT_GRANTED: HRESULT = 1385;

/// A cross-encrypted password is necessary to change a user password.
pub const ERROR_NT_CROSS_ENCRYPTION_REQUIRED: HRESULT = 1386;

/// A member could not be added to or removed from the local group because the member does not exist.
pub const ERROR_NO_SUCH_MEMBER: HRESULT = 1387;

/// A new member could not be added to a local group because the member has the wrong account type.
pub const ERROR_INVALID_MEMBER: HRESULT = 1388;

/// Too many security IDs have been specified.
pub const ERROR_TOO_MANY_SIDS: HRESULT = 1389;

/// A cross-encrypted password is necessary to change this user password.
pub const ERROR_LM_CROSS_ENCRYPTION_REQUIRED: HRESULT = 1390;

/// Indicates an ACL contains no inheritable components.
pub const ERROR_NO_INHERITANCE: HRESULT = 1391;

/// The file or directory is corrupted and unreadable.
pub const ERROR_FILE_CORRUPT: HRESULT = 1392;

/// The disk structure is corrupted and unreadable.
pub const ERROR_DISK_CORRUPT: HRESULT = 1393;

/// There is no user session key for the specified logon session.
pub const ERROR_NO_USER_SESSION_KEY: HRESULT = 1394;

/// The service being accessed is licensed for a particular number of connections. No more connections can be made to the service at this time because there are already as many connections as the service can accept.
pub const ERROR_LICENSE_QUOTA_EXCEEDED: HRESULT = 1395;

/// The target account name is incorrect.
pub const ERROR_WRONG_TARGET_NAME: HRESULT = 1396;

/// Mutual Authentication failed. The server's password is out of date at the domain controller.
pub const ERROR_MUTUAL_AUTH_FAILED: HRESULT = 1397;

/// There is a time and/or date difference between the client and server.
pub const ERROR_TIME_SKEW: HRESULT = 1398;

/// This operation cannot be performed on the current domain.
pub const ERROR_CURRENT_DOMAIN_NOT_ALLOWED: HRESULT = 1399;

/// Invalid window handle.
pub const ERROR_INVALID_WINDOW_HANDLE: HRESULT = 1400;

/// Invalid menu handle.
pub const ERROR_INVALID_MENU_HANDLE: HRESULT = 1401;

/// Invalid cursor handle.
pub const ERROR_INVALID_CURSOR_HANDLE: HRESULT = 1402;

/// Invalid accelerator table handle.
pub const ERROR_INVALID_ACCEL_HANDLE: HRESULT = 1403;

/// Invalid hook handle.
pub const ERROR_INVALID_HOOK_HANDLE: HRESULT = 1404;

/// Invalid handle to a multiple-window position structure.
pub const ERROR_INVALID_DWP_HANDLE: HRESULT = 1405;

/// Cannot create a top-level child window.
pub const ERROR_TLW_WITH_WSCHILD: HRESULT = 1406;

/// Cannot find window class.
pub const ERROR_CANNOT_FIND_WND_CLASS: HRESULT = 1407;

/// Invalid window; it belongs to other thread.
pub const ERROR_WINDOW_OF_OTHER_THREAD: HRESULT = 1408;

/// Hot key is already registered.
pub const ERROR_HOTKEY_ALREADY_REGISTERED: HRESULT = 1409;

/// Class already exists.
pub const ERROR_CLASS_ALREADY_EXISTS: HRESULT = 1410;

/// Class does not exist.
pub const ERROR_CLASS_DOES_NOT_EXIST: HRESULT = 1411;

/// Class still has open windows.
pub const ERROR_CLASS_HAS_WINDOWS: HRESULT = 1412;

/// Invalid index.
pub const ERROR_INVALID_INDEX: HRESULT = 1413;

/// Invalid icon handle.
pub const ERROR_INVALID_ICON_HANDLE: HRESULT = 1414;

/// Using private DIALOG window words.
pub const ERROR_PRIVATE_DIALOG_INDEX: HRESULT = 1415;

/// The list box identifier was not found.
pub const ERROR_LISTBOX_ID_NOT_FOUND: HRESULT = 1416;

/// No wildcards were found.
pub const ERROR_NO_WILDCARD_CHARACTERS: HRESULT = 1417;

/// Thread does not have a clipboard open.
pub const ERROR_CLIPBOARD_NOT_OPEN: HRESULT = 1418;

/// Hot key is not registered.
pub const ERROR_HOTKEY_NOT_REGISTERED: HRESULT = 1419;

/// The window is not a valid dialog window.
pub const ERROR_WINDOW_NOT_DIALOG: HRESULT = 1420;

/// Control ID not found.
pub const ERROR_CONTROL_ID_NOT_FOUND: HRESULT = 1421;

/// Invalid message for a combo box because it does not have an edit control.
pub const ERROR_INVALID_COMBOBOX_MESSAGE: HRESULT = 1422;

/// The window is not a combo box.
pub const ERROR_WINDOW_NOT_COMBOBOX: HRESULT = 1423;

/// Height must be less than 256.
pub const ERROR_INVALID_EDIT_HEIGHT: HRESULT = 1424;

/// Invalid device context (DC) handle.
pub const ERROR_DC_NOT_FOUND: HRESULT = 1425;

/// Invalid hook procedure type.
pub const ERROR_INVALID_HOOK_FILTER: HRESULT = 1426;

/// Invalid hook procedure.
pub const ERROR_INVALID_FILTER_PROC: HRESULT = 1427;

/// Cannot set nonlocal hook without a module handle.
pub const ERROR_HOOK_NEEDS_HMOD: HRESULT = 1428;

/// This hook procedure can only be set globally.
pub const ERROR_GLOBAL_ONLY_HOOK: HRESULT = 1429;

/// The journal hook procedure is already installed.
pub const ERROR_JOURNAL_HOOK_SET: HRESULT = 1430;

/// The hook procedure is not installed.
pub const ERROR_HOOK_NOT_INSTALLED: HRESULT = 1431;

/// Invalid message for single-selection list box.
pub const ERROR_INVALID_LB_MESSAGE: HRESULT = 1432;

/// LB_SETCOUNT sent to non-lazy list box.
pub const ERROR_SETCOUNT_ON_BAD_LB: HRESULT = 1433;

/// This list box does not support tab stops.
pub const ERROR_LB_WITHOUT_TABSTOPS: HRESULT = 1434;

/// Cannot destroy object created by another thread.
pub const ERROR_DESTROY_OBJECT_OF_OTHER_THREAD: HRESULT = 1435;

/// Child windows cannot have menus.
pub const ERROR_CHILD_WINDOW_MENU: HRESULT = 1436;

/// The window does not have a system menu.
pub const ERROR_NO_SYSTEM_MENU: HRESULT = 1437;

/// Invalid message box style.
pub const ERROR_INVALID_MSGBOX_STYLE: HRESULT = 1438;

/// Invalid system-wide (SPI_*) parameter.
pub const ERROR_INVALID_SPI_VALUE: HRESULT = 1439;

/// Screen already locked.
pub const ERROR_SCREEN_ALREADY_LOCKED: HRESULT = 1440;

/// All handles to windows in a multiple-window position structure must have the same parent.
pub const ERROR_HWNDS_HAVE_DIFF_PARENT: HRESULT = 1441;

/// The window is not a child window.
pub const ERROR_NOT_CHILD_WINDOW: HRESULT = 1442;

/// Invalid GW_* command.
pub const ERROR_INVALID_GW_COMMAND: HRESULT = 1443;

/// Invalid thread identifier.
pub const ERROR_INVALID_THREAD_ID: HRESULT = 1444;

/// Cannot process a message from a window that is not a multiple document interface (MDI) window.
pub const ERROR_NON_MDICHILD_WINDOW: HRESULT = 1445;

/// Popup menu already active.
pub const ERROR_POPUP_ALREADY_ACTIVE: HRESULT = 1446;

/// The window does not have scroll bars.
pub const ERROR_NO_SCROLLBARS: HRESULT = 1447;

/// Scroll bar range cannot be greater than MAXLONG.
pub const ERROR_INVALID_SCROLLBAR_RANGE: HRESULT = 1448;

/// Cannot show or remove the window in the way specified.
pub const ERROR_INVALID_SHOWWIN_COMMAND: HRESULT = 1449;

/// Insufficient system resources exist to complete the requested service.
pub const ERROR_NO_SYSTEM_RESOURCES: HRESULT = 1450;

/// Insufficient system resources exist to complete the requested service.
pub const ERROR_NONPAGED_SYSTEM_RESOURCES: HRESULT = 1451;

/// Insufficient system resources exist to complete the requested service.
pub const ERROR_PAGED_SYSTEM_RESOURCES: HRESULT = 1452;

/// Insufficient quota to complete the requested service.
pub const ERROR_WORKING_SET_QUOTA: HRESULT = 1453;

/// Insufficient quota to complete the requested service.
pub const ERROR_PAGEFILE_QUOTA: HRESULT = 1454;

/// The paging file is too small for this operation to complete.
pub const ERROR_COMMITMENT_LIMIT: HRESULT = 1455;

/// A menu item was not found.
pub const ERROR_MENU_ITEM_NOT_FOUND: HRESULT = 1456;

/// Invalid keyboard layout handle.
pub const ERROR_INVALID_KEYBOARD_HANDLE: HRESULT = 1457;

/// Hook type not allowed.
pub const ERROR_HOOK_TYPE_NOT_ALLOWED: HRESULT = 1458;

/// This operation requires an interactive window station.
pub const ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION: HRESULT = 1459;

/// This operation returned because the timeout period expired.
pub const ERROR_TIMEOUT: HRESULT = 1460;

/// Invalid monitor handle.
pub const ERROR_INVALID_MONITOR_HANDLE: HRESULT = 1461;

/// Incorrect size argument.
pub const ERROR_INCORRECT_SIZE: HRESULT = 1462;

/// The symbolic link cannot be followed because its type is disabled.
pub const ERROR_SYMLINK_CLASS_DISABLED: HRESULT = 1463;

/// This application does not support the current operation on symbolic links.
pub const ERROR_SYMLINK_NOT_SUPPORTED: HRESULT = 1464;

/// Windows was unable to parse the requested XML data.
pub const ERROR_XML_PARSE_ERROR: HRESULT = 1465;

/// An error was encountered while processing an XML digital signature.
pub const ERROR_XMLDSIG_ERROR: HRESULT = 1466;

/// This application must be restarted.
pub const ERROR_RESTART_APPLICATION: HRESULT = 1467;

/// The caller made the connection request in the wrong routing compartment.
pub const ERROR_WRONG_COMPARTMENT: HRESULT = 1468;

/// There was an AuthIP failure when attempting to connect to the remote host.
pub const ERROR_AUTHIP_FAILURE: HRESULT = 1469;

/// Insufficient NVRAM resources exist to complete the requested service. A reboot might be required.
pub const ERROR_NO_NVRAM_RESOURCES: HRESULT = 1470;

/// Unable to finish the requested operation because the specified process is not a GUI process.
pub const ERROR_NOT_GUI_PROCESS: HRESULT = 1471;

/// The event log file is corrupted.
pub const ERROR_EVENTLOG_FILE_CORRUPT: HRESULT = 1500;

/// No event log file could be opened, so the event logging service did not start.
pub const ERROR_EVENTLOG_CANT_START: HRESULT = 1501;

/// The event log file is full.
pub const ERROR_LOG_FILE_FULL: HRESULT = 1502;

/// The event log file has changed between read operations.
pub const ERROR_EVENTLOG_FILE_CHANGED: HRESULT = 1503;

/// The specified Job already has a container assigned to it.
pub const ERROR_CONTAINER_ASSIGNED: HRESULT = 1504;

/// The specified Job does not have a container assigned to it.
pub const ERROR_JOB_NO_CONTAINER: HRESULT = 1505;

/// The specified task name is invalid.
pub const ERROR_INVALID_TASK_NAME: HRESULT = 1550;

/// The specified task index is invalid.
pub const ERROR_INVALID_TASK_INDEX: HRESULT = 1551;

/// The specified thread is already joining a task.
pub const ERROR_THREAD_ALREADY_IN_TASK: HRESULT = 1552;

/// The Windows Installer Service could not be accessed. This can occur if the Windows Installer is not correctly installed. Contact your support personnel for assistance.
pub const ERROR_INSTALL_SERVICE_FAILURE: HRESULT = 1601;

/// User cancelled installation.
pub const ERROR_INSTALL_USEREXIT: HRESULT = 1602;

/// Fatal error during installation.
pub const ERROR_INSTALL_FAILURE: HRESULT = 1603;

/// Installation suspended, incomplete.
pub const ERROR_INSTALL_SUSPEND: HRESULT = 1604;

/// This action is only valid for products that are currently installed.
pub const ERROR_UNKNOWN_PRODUCT: HRESULT = 1605;

/// Feature ID not registered.
pub const ERROR_UNKNOWN_FEATURE: HRESULT = 1606;

/// Component ID not registered.
pub const ERROR_UNKNOWN_COMPONENT: HRESULT = 1607;

/// Unknown property.
pub const ERROR_UNKNOWN_PROPERTY: HRESULT = 1608;

/// Handle is in an invalid state.
pub const ERROR_INVALID_HANDLE_STATE: HRESULT = 1609;

/// The configuration data for this product is corrupt. Contact your support personnel.
pub const ERROR_BAD_CONFIGURATION: HRESULT = 1610;

/// Component qualifier not present.
pub const ERROR_INDEX_ABSENT: HRESULT = 1611;

/// The installation source for this product is not available. Verify that the source exists and that you can access it.
pub const ERROR_INSTALL_SOURCE_ABSENT: HRESULT = 1612;

/// This installation package cannot be installed by the Windows Installer service. You must install a Windows service pack that contains a newer version of the Windows Installer service.
pub const ERROR_INSTALL_PACKAGE_VERSION: HRESULT = 1613;

/// Product is uninstalled.
pub const ERROR_PRODUCT_UNINSTALLED: HRESULT = 1614;

/// SQL query syntax invalid or unsupported.
pub const ERROR_BAD_QUERY_SYNTAX: HRESULT = 1615;

/// Record field does not exist.
pub const ERROR_INVALID_FIELD: HRESULT = 1616;

/// The device has been removed.
pub const ERROR_DEVICE_REMOVED: HRESULT = 1617;

/// Another installation is already in progress. Complete that installation before proceeding with this install.
pub const ERROR_INSTALL_ALREADY_RUNNING: HRESULT = 1618;

/// This installation package could not be opened. Verify that the package exists and that you can access it, or contact the application vendor to verify that this is a valid Windows Installer package.
pub const ERROR_INSTALL_PACKAGE_OPEN_FAILED: HRESULT = 1619;

/// This installation package could not be opened. Contact the application vendor to verify that this is a valid Windows Installer package.
pub const ERROR_INSTALL_PACKAGE_INVALID: HRESULT = 1620;

/// There was an error starting the Windows Installer service user interface. Contact your support personnel.
pub const ERROR_INSTALL_UI_FAILURE: HRESULT = 1621;

/// Error opening installation log file. Verify that the specified log file location exists and that you can write to it.
pub const ERROR_INSTALL_LOG_FAILURE: HRESULT = 1622;

/// The language of this installation package is not supported by your system.
pub const ERROR_INSTALL_LANGUAGE_UNSUPPORTED: HRESULT = 1623;

/// Error applying transforms. Verify that the specified transform paths are valid.
pub const ERROR_INSTALL_TRANSFORM_FAILURE: HRESULT = 1624;

/// This installation is forbidden by system policy. Contact your system administrator.
pub const ERROR_INSTALL_PACKAGE_REJECTED: HRESULT = 1625;

/// Function could not be executed.
pub const ERROR_FUNCTION_NOT_CALLED: HRESULT = 1626;

/// Function failed during execution.
pub const ERROR_FUNCTION_FAILED: HRESULT = 1627;

/// Invalid or unknown table specified.
pub const ERROR_INVALID_TABLE: HRESULT = 1628;

/// Data supplied is of wrong type.
pub const ERROR_DATATYPE_MISMATCH: HRESULT = 1629;

/// Data of this type is not supported.
pub const ERROR_UNSUPPORTED_TYPE: HRESULT = 1630;

/// The Windows Installer service failed to start. Contact your support personnel.
pub const ERROR_CREATE_FAILED: HRESULT = 1631;

/// The Temp folder is on a drive that is full or is inaccessible. Free up space on the drive or verify that you have write permission on the Temp folder.
pub const ERROR_INSTALL_TEMP_UNWRITABLE: HRESULT = 1632;

/// This installation package is not supported by this processor type. Contact your product vendor.
pub const ERROR_INSTALL_PLATFORM_UNSUPPORTED: HRESULT = 1633;

/// Component not used on this computer.
pub const ERROR_INSTALL_NOTUSED: HRESULT = 1634;

/// This update package could not be opened. Verify that the update package exists and that you can access it, or contact the application vendor to verify that this is a valid Windows Installer update package.
pub const ERROR_PATCH_PACKAGE_OPEN_FAILED: HRESULT = 1635;

/// This update package could not be opened. Contact the application vendor to verify that this is a valid Windows Installer update package.
pub const ERROR_PATCH_PACKAGE_INVALID: HRESULT = 1636;

/// This update package cannot be processed by the Windows Installer service. You must install a Windows service pack that contains a newer version of the Windows Installer service.
pub const ERROR_PATCH_PACKAGE_UNSUPPORTED: HRESULT = 1637;

/// Another version of this product is already installed. Installation of this version cannot continue. To configure or remove the existing version of this product, use Add/Remove Programs on the Control Panel.
pub const ERROR_PRODUCT_VERSION: HRESULT = 1638;

/// Invalid command line argument. Consult the Windows Installer SDK for detailed command line help.
pub const ERROR_INVALID_COMMAND_LINE: HRESULT = 1639;

/// Only administrators have permission to add, remove, or configure server software during a Terminal services remote session. If you want to install or configure software on the server, contact your network administrator.
pub const ERROR_INSTALL_REMOTE_DISALLOWED: HRESULT = 1640;

/// The requested operation completed successfully. The system will be restarted so the changes can take effect.
pub const ERROR_SUCCESS_REBOOT_INITIATED: HRESULT = 1641;

/// The upgrade cannot be installed by the Windows Installer service because the program to be upgraded may be missing, or the upgrade may update a different version of the program. Verify that the program to be upgraded exists on your computer and that you have the correct upgrade.
pub const ERROR_PATCH_TARGET_NOT_FOUND: HRESULT = 1642;

/// The update package is not permitted by software restriction policy.
pub const ERROR_PATCH_PACKAGE_REJECTED: HRESULT = 1643;

/// One or more customizations are not permitted by software restriction policy.
pub const ERROR_INSTALL_TRANSFORM_REJECTED: HRESULT = 1644;

/// The Windows Installer does not permit installation from a Remote Desktop Connection.
pub const ERROR_INSTALL_REMOTE_PROHIBITED: HRESULT = 1645;

/// Uninstallation of the update package is not supported.
pub const ERROR_PATCH_REMOVAL_UNSUPPORTED: HRESULT = 1646;

/// The update is not applied to this product.
pub const ERROR_UNKNOWN_PATCH: HRESULT = 1647;

/// No valid sequence could be found for the set of updates.
pub const ERROR_PATCH_NO_SEQUENCE: HRESULT = 1648;

/// Update removal was disallowed by policy.
pub const ERROR_PATCH_REMOVAL_DISALLOWED: HRESULT = 1649;

/// The XML update data is invalid.
pub const ERROR_INVALID_PATCH_XML: HRESULT = 1650;

/// Windows Installer does not permit updating of managed advertised products. At least one feature of the product must be installed before applying the update.
pub const ERROR_PATCH_MANAGED_ADVERTISED_PRODUCT: HRESULT = 1651;

/// The Windows Installer service is not accessible in Safe Mode. Please try again when your computer is not in Safe Mode or you can use System Restore to return your machine to a previous good state.
pub const ERROR_INSTALL_SERVICE_SAFEBOOT: HRESULT = 1652;

/// A fail fast exception occurred. Exception handlers will not be invoked and the process will be terminated immediately.
pub const ERROR_FAIL_FAST_EXCEPTION: HRESULT = 1653;

/// The app that you are trying to run is not supported on this version of Windows.
pub const ERROR_INSTALL_REJECTED: HRESULT = 1654;

/// The operation was blocked as the process prohibits dynamic code generation.
pub const ERROR_DYNAMIC_CODE_BLOCKED: HRESULT = 1655;

/// The objects are not identical.
pub const ERROR_NOT_SAME_OBJECT: HRESULT = 1656;

/// The specified image file was blocked from loading because it does not enable a feature required by the process: Control Flow Guard.
pub const ERROR_STRICT_CFG_VIOLATION: HRESULT = 1657;

/// The thread context could not be updated because this has been restricted for the process.
pub const ERROR_SET_CONTEXT_DENIED: HRESULT = 1660;

/// An invalid cross-partition private file/section access was attempted.
pub const ERROR_CROSS_PARTITION_VIOLATION: HRESULT = 1661;

/// A return address hijack is being attempted. This is supported by the operating system when user-mode shadow stacks are enabled.
pub const ERROR_RETURN_ADDRESS_HIJACK_ATTEMPT: HRESULT = 1662;

/// The string binding is invalid.
pub const RPC_S_INVALID_STRING_BINDING: HRESULT = 1700;

/// The binding handle is not the correct type.
pub const RPC_S_WRONG_KIND_OF_BINDING: HRESULT = 1701;

/// The binding handle is invalid.
pub const RPC_S_INVALID_BINDING: HRESULT = 1702;

/// The RPC protocol sequence is not supported.
pub const RPC_S_PROTSEQ_NOT_SUPPORTED: HRESULT = 1703;

/// The RPC protocol sequence is invalid.
pub const RPC_S_INVALID_RPC_PROTSEQ: HRESULT = 1704;

/// The string universal unique identifier (UUID) is invalid.
pub const RPC_S_INVALID_STRING_UUID: HRESULT = 1705;

/// The endpoint format is invalid.
pub const RPC_S_INVALID_ENDPOINT_FORMAT: HRESULT = 1706;

/// The network address is invalid.
pub const RPC_S_INVALID_NET_ADDR: HRESULT = 1707;

/// No endpoint was found.
pub const RPC_S_NO_ENDPOINT_FOUND: HRESULT = 1708;

/// The timeout value is invalid.
pub const RPC_S_INVALID_TIMEOUT: HRESULT = 1709;

/// The object universal unique identifier (UUID) was not found.
pub const RPC_S_OBJECT_NOT_FOUND: HRESULT = 1710;

/// The object universal unique identifier (UUID) has already been registered.
pub const RPC_S_ALREADY_REGISTERED: HRESULT = 1711;

/// The type universal unique identifier (UUID) has already been registered.
pub const RPC_S_TYPE_ALREADY_REGISTERED: HRESULT = 1712;

/// The RPC server is already listening.
pub const RPC_S_ALREADY_LISTENING: HRESULT = 1713;

/// No protocol sequences have been registered.
pub const RPC_S_NO_PROTSEQS_REGISTERED: HRESULT = 1714;

/// The RPC server is not listening.
pub const RPC_S_NOT_LISTENING: HRESULT = 1715;

/// The manager type is unknown.
pub const RPC_S_UNKNOWN_MGR_TYPE: HRESULT = 1716;

/// The interface is unknown.
pub const RPC_S_UNKNOWN_IF: HRESULT = 1717;

/// There are no bindings.
pub const RPC_S_NO_BINDINGS: HRESULT = 1718;

/// There are no protocol sequences.
pub const RPC_S_NO_PROTSEQS: HRESULT = 1719;

/// The endpoint cannot be created.
pub const RPC_S_CANT_CREATE_ENDPOINT: HRESULT = 1720;

/// Not enough resources are available to complete this operation.
pub const RPC_S_OUT_OF_RESOURCES: HRESULT = 1721;

/// The RPC server is unavailable.
pub const RPC_S_SERVER_UNAVAILABLE: HRESULT = 1722;

/// The RPC server is too busy to complete this operation.
pub const RPC_S_SERVER_TOO_BUSY: HRESULT = 1723;

/// The network options are invalid.
pub const RPC_S_INVALID_NETWORK_OPTIONS: HRESULT = 1724;

/// There are no remote procedure calls active on this thread.
pub const RPC_S_NO_CALL_ACTIVE: HRESULT = 1725;

/// The remote procedure call failed.
pub const RPC_S_CALL_FAILED: HRESULT = 1726;

/// The remote procedure call failed and did not execute.
pub const RPC_S_CALL_FAILED_DNE: HRESULT = 1727;

/// A remote procedure call (RPC) protocol error occurred.
pub const RPC_S_PROTOCOL_ERROR: HRESULT = 1728;

/// Access to the HTTP proxy is denied.
pub const RPC_S_PROXY_ACCESS_DENIED: HRESULT = 1729;

/// The transfer syntax is not supported by the RPC server.
pub const RPC_S_UNSUPPORTED_TRANS_SYN: HRESULT = 1730;

/// The universal unique identifier (UUID) type is not supported.
pub const RPC_S_UNSUPPORTED_TYPE: HRESULT = 1732;

/// The tag is invalid.
pub const RPC_S_INVALID_TAG: HRESULT = 1733;

/// The array bounds are invalid.
pub const RPC_S_INVALID_BOUND: HRESULT = 1734;

/// The binding does not contain an entry name.
pub const RPC_S_NO_ENTRY_NAME: HRESULT = 1735;

/// The name syntax is invalid.
pub const RPC_S_INVALID_NAME_SYNTAX: HRESULT = 1736;

/// The name syntax is not supported.
pub const RPC_S_UNSUPPORTED_NAME_SYNTAX: HRESULT = 1737;

/// No network address is available to use to construct a universal unique identifier (UUID).
pub const RPC_S_UUID_NO_ADDRESS: HRESULT = 1739;

/// The endpoint is a duplicate.
pub const RPC_S_DUPLICATE_ENDPOINT: HRESULT = 1740;

/// The authentication type is unknown.
pub const RPC_S_UNKNOWN_AUTHN_TYPE: HRESULT = 1741;

/// The maximum number of calls is too small.
pub const RPC_S_MAX_CALLS_TOO_SMALL: HRESULT = 1742;

/// The string is too long.
pub const RPC_S_STRING_TOO_LONG: HRESULT = 1743;

/// The RPC protocol sequence was not found.
pub const RPC_S_PROTSEQ_NOT_FOUND: HRESULT = 1744;

/// The procedure number is out of range.
pub const RPC_S_PROCNUM_OUT_OF_RANGE: HRESULT = 1745;

/// The binding does not contain any authentication information.
pub const RPC_S_BINDING_HAS_NO_AUTH: HRESULT = 1746;

/// The authentication service is unknown.
pub const RPC_S_UNKNOWN_AUTHN_SERVICE: HRESULT = 1747;

/// The authentication level is unknown.
pub const RPC_S_UNKNOWN_AUTHN_LEVEL: HRESULT = 1748;

/// The security context is invalid.
pub const RPC_S_INVALID_AUTH_IDENTITY: HRESULT = 1749;

/// The authorization service is unknown.
pub const RPC_S_UNKNOWN_AUTHZ_SERVICE: HRESULT = 1750;

/// The entry is invalid.
pub const EPT_S_INVALID_ENTRY: HRESULT = 1751;

/// The server endpoint cannot perform the operation.
pub const EPT_S_CANT_PERFORM_OP: HRESULT = 1752;

/// There are no more endpoints available from the endpoint mapper.
pub const EPT_S_NOT_REGISTERED: HRESULT = 1753;

/// No interfaces have been exported.
pub const RPC_S_NOTHING_TO_EXPORT: HRESULT = 1754;

/// The entry name is incomplete.
pub const RPC_S_INCOMPLETE_NAME: HRESULT = 1755;

/// The version option is invalid.
pub const RPC_S_INVALID_VERS_OPTION: HRESULT = 1756;

/// There are no more members.
pub const RPC_S_NO_MORE_MEMBERS: HRESULT = 1757;

/// There is nothing to unexport.
pub const RPC_S_NOT_ALL_OBJS_UNEXPORTED: HRESULT = 1758;

/// The interface was not found.
pub const RPC_S_INTERFACE_NOT_FOUND: HRESULT = 1759;

/// The entry already exists.
pub const RPC_S_ENTRY_ALREADY_EXISTS: HRESULT = 1760;

/// The entry is not found.
pub const RPC_S_ENTRY_NOT_FOUND: HRESULT = 1761;

/// The name service is unavailable.
pub const RPC_S_NAME_SERVICE_UNAVAILABLE: HRESULT = 1762;

/// The network address family is invalid.
pub const RPC_S_INVALID_NAF_ID: HRESULT = 1763;

/// The requested operation is not supported.
pub const RPC_S_CANNOT_SUPPORT: HRESULT = 1764;

/// No security context is available to allow impersonation.
pub const RPC_S_NO_CONTEXT_AVAILABLE: HRESULT = 1765;

/// An internal error occurred in a remote procedure call (RPC).
pub const RPC_S_INTERNAL_ERROR: HRESULT = 1766;

/// The RPC server attempted an integer division by zero.
pub const RPC_S_ZERO_DIVIDE: HRESULT = 1767;

/// An addressing error occurred in the RPC server.
pub const RPC_S_ADDRESS_ERROR: HRESULT = 1768;

/// A floating-point operation at the RPC server caused a division by zero.
pub const RPC_S_FP_DIV_ZERO: HRESULT = 1769;

/// A floating-point underflow occurred at the RPC server.
pub const RPC_S_FP_UNDERFLOW: HRESULT = 1770;

/// A floating-point overflow occurred at the RPC server.
pub const RPC_S_FP_OVERFLOW: HRESULT = 1771;

/// The list of RPC servers available for the binding of auto handles has been exhausted.
pub const RPC_X_NO_MORE_ENTRIES: HRESULT = 1772;

/// Unable to open the character translation table file.
pub const RPC_X_SS_CHAR_TRANS_OPEN_FAIL: HRESULT = 1773;

/// The file containing the character translation table has fewer than 512 bytes.
pub const RPC_X_SS_CHAR_TRANS_SHORT_FILE: HRESULT = 1774;

/// A null context handle was passed from the client to the host during a remote procedure call.
pub const RPC_X_SS_IN_NULL_CONTEXT: HRESULT = 1775;

/// The context handle changed during a remote procedure call.
pub const RPC_X_SS_CONTEXT_DAMAGED: HRESULT = 1777;

/// The binding handles passed to a remote procedure call do not match.
pub const RPC_X_SS_HANDLES_MISMATCH: HRESULT = 1778;

/// The stub is unable to get the remote procedure call handle.
pub const RPC_X_SS_CANNOT_GET_CALL_HANDLE: HRESULT = 1779;

/// A null reference pointer was passed to the stub.
pub const RPC_X_NULL_REF_POINTER: HRESULT = 1780;

/// The enumeration value is out of range.
pub const RPC_X_ENUM_VALUE_OUT_OF_RANGE: HRESULT = 1781;

/// The byte count is too small.
pub const RPC_X_BYTE_COUNT_TOO_SMALL: HRESULT = 1782;

/// The stub received bad data.
pub const RPC_X_BAD_STUB_DATA: HRESULT = 1783;

/// The supplied user buffer is not valid for the requested operation.
pub const ERROR_INVALID_USER_BUFFER: HRESULT = 1784;

/// The disk media is not recognized. It may not be formatted.
pub const ERROR_UNRECOGNIZED_MEDIA: HRESULT = 1785;

/// The workstation does not have a trust secret.
pub const ERROR_NO_TRUST_LSA_SECRET: HRESULT = 1786;

/// The security database on the server does not have a computer account for this workstation trust relationship.
pub const ERROR_NO_TRUST_SAM_ACCOUNT: HRESULT = 1787;

/// The trust relationship between the primary domain and the trusted domain failed.
pub const ERROR_TRUSTED_DOMAIN_FAILURE: HRESULT = 1788;

/// The trust relationship between this workstation and the primary domain failed.
pub const ERROR_TRUSTED_RELATIONSHIP_FAILURE: HRESULT = 1789;

/// The network logon failed.
pub const ERROR_TRUST_FAILURE: HRESULT = 1790;

/// A remote procedure call is already in progress for this thread.
pub const RPC_S_CALL_IN_PROGRESS: HRESULT = 1791;

/// An attempt was made to logon, but the network logon service was not started.
pub const ERROR_NETLOGON_NOT_STARTED: HRESULT = 1792;

/// The user's account has expired.
pub const ERROR_ACCOUNT_EXPIRED: HRESULT = 1793;

/// The redirector is in use and cannot be unloaded.
pub const ERROR_REDIRECTOR_HAS_OPEN_HANDLES: HRESULT = 1794;

/// The specified printer driver is already installed.
pub const ERROR_PRINTER_DRIVER_ALREADY_INSTALLED: HRESULT = 1795;

/// The specified port is unknown.
pub const ERROR_UNKNOWN_PORT: HRESULT = 1796;

/// The printer driver is unknown.
pub const ERROR_UNKNOWN_PRINTER_DRIVER: HRESULT = 1797;

/// The print processor is unknown.
pub const ERROR_UNKNOWN_PRINTPROCESSOR: HRESULT = 1798;

/// The specified separator file is invalid.
pub const ERROR_INVALID_SEPARATOR_FILE: HRESULT = 1799;

/// The specified priority is invalid.
pub const ERROR_INVALID_PRIORITY: HRESULT = 1800;

/// The printer name is invalid.
pub const ERROR_INVALID_PRINTER_NAME: HRESULT = 1801;

/// The printer already exists.
pub const ERROR_PRINTER_ALREADY_EXISTS: HRESULT = 1802;

/// The printer command is invalid.
pub const ERROR_INVALID_PRINTER_COMMAND: HRESULT = 1803;

/// The specified datatype is invalid.
pub const ERROR_INVALID_DATATYPE: HRESULT = 1804;

/// The environment specified is invalid.
pub const ERROR_INVALID_ENVIRONMENT: HRESULT = 1805;

/// There are no more bindings.
pub const RPC_S_NO_MORE_BINDINGS: HRESULT = 1806;

/// The account used is an interdomain trust account. Use your global user account or local user account to access this server.
pub const ERROR_NOLOGON_INTERDOMAIN_TRUST_ACCOUNT: HRESULT = 1807;

/// The account used is a computer account. Use your global user account or local user account to access this server.
pub const ERROR_NOLOGON_WORKSTATION_TRUST_ACCOUNT: HRESULT = 1808;

/// The account used is a server trust account. Use your global user account or local user account to access this server.
pub const ERROR_NOLOGON_SERVER_TRUST_ACCOUNT: HRESULT = 1809;

/// The name or security ID (SID) of the domain specified is inconsistent with the trust information for that domain.
pub const ERROR_DOMAIN_TRUST_INCONSISTENT: HRESULT = 1810;

/// The server is in use and cannot be unloaded.
pub const ERROR_SERVER_HAS_OPEN_HANDLES: HRESULT = 1811;

/// The specified image file did not contain a resource section.
pub const ERROR_RESOURCE_DATA_NOT_FOUND: HRESULT = 1812;

/// The specified resource type cannot be found in the image file.
pub const ERROR_RESOURCE_TYPE_NOT_FOUND: HRESULT = 1813;

/// The specified resource name cannot be found in the image file.
pub const ERROR_RESOURCE_NAME_NOT_FOUND: HRESULT = 1814;

/// The specified resource language ID cannot be found in the image file.
pub const ERROR_RESOURCE_LANG_NOT_FOUND: HRESULT = 1815;

/// Not enough quota is available to process this command.
pub const ERROR_NOT_ENOUGH_QUOTA: HRESULT = 1816;

/// No interfaces have been registered.
pub const RPC_S_NO_INTERFACES: HRESULT = 1817;

/// The remote procedure call was cancelled.
pub const RPC_S_CALL_CANCELLED: HRESULT = 1818;

/// The binding handle does not contain all required information.
pub const RPC_S_BINDING_INCOMPLETE: HRESULT = 1819;

/// A communications failure occurred during a remote procedure call.
pub const RPC_S_COMM_FAILURE: HRESULT = 1820;

/// The requested authentication level is not supported.
pub const RPC_S_UNSUPPORTED_AUTHN_LEVEL: HRESULT = 1821;

/// No principal name registered.
pub const RPC_S_NO_PRINC_NAME: HRESULT = 1822;

/// The error specified is not a valid Windows RPC error code.
pub const RPC_S_NOT_RPC_ERROR: HRESULT = 1823;

/// A UUID that is valid only on this computer has been allocated.
pub const RPC_S_UUID_LOCAL_ONLY: HRESULT = 1824;

/// A security package specific error occurred.
pub const RPC_S_SEC_PKG_ERROR: HRESULT = 1825;

/// Thread is not canceled.
pub const RPC_S_NOT_CANCELLED: HRESULT = 1826;

/// Invalid operation on the encoding/decoding handle.
pub const RPC_X_INVALID_ES_ACTION: HRESULT = 1827;

/// Incompatible version of the serializing package.
pub const RPC_X_WRONG_ES_VERSION: HRESULT = 1828;

/// Incompatible version of the RPC stub.
pub const RPC_X_WRONG_STUB_VERSION: HRESULT = 1829;

/// The RPC pipe object is invalid or corrupted.
pub const RPC_X_INVALID_PIPE_OBJECT: HRESULT = 1830;

/// An invalid operation was attempted on an RPC pipe object.
pub const RPC_X_WRONG_PIPE_ORDER: HRESULT = 1831;

/// Unsupported RPC pipe version.
pub const RPC_X_WRONG_PIPE_VERSION: HRESULT = 1832;

/// HTTP proxy server rejected the connection because the cookie authentication failed.
pub const RPC_S_COOKIE_AUTH_FAILED: HRESULT = 1833;

/// The RPC server is suspended, and could not be resumed for this request. The call did not execute.
pub const RPC_S_DO_NOT_DISTURB: HRESULT = 1834;

/// The RPC call contains too many handles to be transmitted in a single request.
pub const RPC_S_SYSTEM_HANDLE_COUNT_EXCEEDED: HRESULT = 1835;

/// The RPC call contains a handle that differs from the declared handle type.
pub const RPC_S_SYSTEM_HANDLE_TYPE_MISMATCH: HRESULT = 1836;

/// The group member was not found.
pub const RPC_S_GROUP_MEMBER_NOT_FOUND: HRESULT = 1898;

/// The endpoint mapper database entry could not be created.
pub const EPT_S_CANT_CREATE: HRESULT = 1899;

/// The object universal unique identifier (UUID) is the nil UUID.
pub const RPC_S_INVALID_OBJECT: HRESULT = 1900;

/// The specified time is invalid.
pub const ERROR_INVALID_TIME: HRESULT = 1901;

/// The specified form name is invalid.
pub const ERROR_INVALID_FORM_NAME: HRESULT = 1902;

/// The specified form size is invalid.
pub const ERROR_INVALID_FORM_SIZE: HRESULT = 1903;

/// The specified printer handle is already being waited on
pub const ERROR_ALREADY_WAITING: HRESULT = 1904;

/// The specified printer has been deleted.
pub const ERROR_PRINTER_DELETED: HRESULT = 1905;

/// The state of the printer is invalid.
pub const ERROR_INVALID_PRINTER_STATE: HRESULT = 1906;

/// The user's password must be changed before signing in.
pub const ERROR_PASSWORD_MUST_CHANGE: HRESULT = 1907;

/// Could not find the domain controller for this domain.
pub const ERROR_DOMAIN_CONTROLLER_NOT_FOUND: HRESULT = 1908;

/// The referenced account is currently locked out and may not be logged on to.
pub const ERROR_ACCOUNT_LOCKED_OUT: HRESULT = 1909;

/// The object exporter specified was not found.
pub const OR_INVALID_OXID: HRESULT = 1910;

/// The object specified was not found.
pub const OR_INVALID_OID: HRESULT = 1911;

/// The object resolver set specified was not found.
pub const OR_INVALID_SET: HRESULT = 1912;

/// Some data remains to be sent in the request buffer.
pub const RPC_S_SEND_INCOMPLETE: HRESULT = 1913;

/// Invalid asynchronous remote procedure call handle.
pub const RPC_S_INVALID_ASYNC_HANDLE: HRESULT = 1914;

/// Invalid asynchronous RPC call handle for this operation.
pub const RPC_S_INVALID_ASYNC_CALL: HRESULT = 1915;

/// The RPC pipe object has already been closed.
pub const RPC_X_PIPE_CLOSED: HRESULT = 1916;

/// The RPC call completed before all pipes were processed.
pub const RPC_X_PIPE_DISCIPLINE_ERROR: HRESULT = 1917;

/// No more data is available from the RPC pipe.
pub const RPC_X_PIPE_EMPTY: HRESULT = 1918;

/// No site name is available for this machine.
pub const ERROR_NO_SITENAME: HRESULT = 1919;

/// The file cannot be accessed by the system.
pub const ERROR_CANT_ACCESS_FILE: HRESULT = 1920;

/// The name of the file cannot be resolved by the system.
pub const ERROR_CANT_RESOLVE_FILENAME: HRESULT = 1921;

/// The entry is not of the expected type.
pub const RPC_S_ENTRY_TYPE_MISMATCH: HRESULT = 1922;

/// Not all object UUIDs could be exported to the specified entry.
pub const RPC_S_NOT_ALL_OBJS_EXPORTED: HRESULT = 1923;

/// Interface could not be exported to the specified entry.
pub const RPC_S_INTERFACE_NOT_EXPORTED: HRESULT = 1924;

/// The specified profile entry could not be added.
pub const RPC_S_PROFILE_NOT_ADDED: HRESULT = 1925;

/// The specified profile element could not be added.
pub const RPC_S_PRF_ELT_NOT_ADDED: HRESULT = 1926;

/// The specified profile element could not be removed.
pub const RPC_S_PRF_ELT_NOT_REMOVED: HRESULT = 1927;

/// The group element could not be added.
pub const RPC_S_GRP_ELT_NOT_ADDED: HRESULT = 1928;

/// The group element could not be removed.
pub const RPC_S_GRP_ELT_NOT_REMOVED: HRESULT = 1929;

/// The printer driver is not compatible with a policy enabled on your computer that blocks NT 4.0 drivers.
pub const ERROR_KM_DRIVER_BLOCKED: HRESULT = 1930;

/// The context has expired and can no longer be used.
pub const ERROR_CONTEXT_EXPIRED: HRESULT = 1931;

/// The current user's delegated trust creation quota has been exceeded.
pub const ERROR_PER_USER_TRUST_QUOTA_EXCEEDED: HRESULT = 1932;

/// The total delegated trust creation quota has been exceeded.
pub const ERROR_ALL_USER_TRUST_QUOTA_EXCEEDED: HRESULT = 1933;

/// The current user's delegated trust deletion quota has been exceeded.
pub const ERROR_USER_DELETE_TRUST_QUOTA_EXCEEDED: HRESULT = 1934;

/// The computer you are signing into is protected by an authentication firewall. The specified account is not allowed to authenticate to the computer.
pub const ERROR_AUTHENTICATION_FIREWALL_FAILED: HRESULT = 1935;

/// Remote connections to the Print Spooler are blocked by a policy set on your machine.
pub const ERROR_REMOTE_PRINT_CONNECTIONS_BLOCKED: HRESULT = 1936;

/// Authentication failed because NTLM authentication has been disabled.
pub const ERROR_NTLM_BLOCKED: HRESULT = 1937;

/// Logon Failure: EAS policy requires that the user change their password before this operation can be performed.
pub const ERROR_PASSWORD_CHANGE_REQUIRED: HRESULT = 1938;

/// An administrator has restricted sign in. To sign in, make sure your device is connected to the Internet, and have your administrator sign in first.
pub const ERROR_LOST_MODE_LOGON_RESTRICTION: HRESULT = 1939;

/// The pixel format is invalid.
pub const ERROR_INVALID_PIXEL_FORMAT: HRESULT = 2000;

/// The specified driver is invalid.
pub const ERROR_BAD_DRIVER: HRESULT = 2001;

/// The window style or class attribute is invalid for this operation.
pub const ERROR_INVALID_WINDOW_STYLE: HRESULT = 2002;

/// The requested metafile operation is not supported.
pub const ERROR_METAFILE_NOT_SUPPORTED: HRESULT = 2003;

/// The requested transformation operation is not supported.
pub const ERROR_TRANSFORM_NOT_SUPPORTED: HRESULT = 2004;

/// The requested clipping operation is not supported.
pub const ERROR_CLIPPING_NOT_SUPPORTED: HRESULT = 2005;

/// The specified color management module is invalid.
pub const ERROR_INVALID_CMM: HRESULT = 2010;

/// The specified color profile is invalid.
pub const ERROR_INVALID_PROFILE: HRESULT = 2011;

/// The specified tag was not found.
pub const ERROR_TAG_NOT_FOUND: HRESULT = 2012;

/// A required tag is not present.
pub const ERROR_TAG_NOT_PRESENT: HRESULT = 2013;

/// The specified tag is already present.
pub const ERROR_DUPLICATE_TAG: HRESULT = 2014;

/// The specified color profile is not associated with the specified device.
pub const ERROR_PROFILE_NOT_ASSOCIATED_WITH_DEVICE: HRESULT = 2015;

/// The specified color profile was not found.
pub const ERROR_PROFILE_NOT_FOUND: HRESULT = 2016;

/// The specified color space is invalid.
pub const ERROR_INVALID_COLORSPACE: HRESULT = 2017;

/// Image Color Management is not enabled.
pub const ERROR_ICM_NOT_ENABLED: HRESULT = 2018;

/// There was an error while deleting the color transform.
pub const ERROR_DELETING_ICM_XFORM: HRESULT = 2019;

/// The specified color transform is invalid.
pub const ERROR_INVALID_TRANSFORM: HRESULT = 2020;

/// The specified transform does not match the bitmap's color space.
pub const ERROR_COLORSPACE_MISMATCH: HRESULT = 2021;

/// The specified named color index is not present in the profile.
pub const ERROR_INVALID_COLORINDEX: HRESULT = 2022;

/// The specified profile is intended for a device of a different type than the specified device.
pub const ERROR_PROFILE_DOES_NOT_MATCH_DEVICE: HRESULT = 2023;

/// The network connection was made successfully, but the user had to be prompted for a password other than the one originally specified.
pub const ERROR_CONNECTED_OTHER_PASSWORD: HRESULT = 2108;

/// The network connection was made successfully using default credentials.
pub const ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT: HRESULT = 2109;

/// The specified username is invalid.
pub const ERROR_BAD_USERNAME: HRESULT = 2202;

/// This network connection does not exist.
pub const ERROR_NOT_CONNECTED: HRESULT = 2250;

/// This network connection has files open or requests pending.
pub const ERROR_OPEN_FILES: HRESULT = 2401;

/// Active connections still exist.
pub const ERROR_ACTIVE_CONNECTIONS: HRESULT = 2402;

/// The device is in use by an active process and cannot be disconnected.
pub const ERROR_DEVICE_IN_USE: HRESULT = 2404;

/// The specified print monitor is unknown.
pub const ERROR_UNKNOWN_PRINT_MONITOR: HRESULT = 3000;

/// The specified printer driver is currently in use.
pub const ERROR_PRINTER_DRIVER_IN_USE: HRESULT = 3001;

/// The spool file was not found.
pub const ERROR_SPOOL_FILE_NOT_FOUND: HRESULT = 3002;

/// A StartDocPrinter call was not issued.
pub const ERROR_SPL_NO_STARTDOC: HRESULT = 3003;

/// An AddJob call was not issued.
pub const ERROR_SPL_NO_ADDJOB: HRESULT = 3004;

/// The specified print processor has already been installed.
pub const ERROR_PRINT_PROCESSOR_ALREADY_INSTALLED: HRESULT = 3005;

/// The specified print monitor has already been installed.
pub const ERROR_PRINT_MONITOR_ALREADY_INSTALLED: HRESULT = 3006;

/// The specified print monitor does not have the required functions.
pub const ERROR_INVALID_PRINT_MONITOR: HRESULT = 3007;

/// The specified print monitor is currently in use.
pub const ERROR_PRINT_MONITOR_IN_USE: HRESULT = 3008;

/// The requested operation is not allowed when there are jobs queued to the printer.
pub const ERROR_PRINTER_HAS_JOBS_QUEUED: HRESULT = 3009;

/// The requested operation is successful. Changes will not be effective until the system is rebooted.
pub const ERROR_SUCCESS_REBOOT_REQUIRED: HRESULT = 3010;

/// The requested operation is successful. Changes will not be effective until the service is restarted.
pub const ERROR_SUCCESS_RESTART_REQUIRED: HRESULT = 3011;

/// No printers were found.
pub const ERROR_PRINTER_NOT_FOUND: HRESULT = 3012;

/// The printer driver is known to be unreliable.
pub const ERROR_PRINTER_DRIVER_WARNED: HRESULT = 3013;

/// The printer driver is known to harm the system.
pub const ERROR_PRINTER_DRIVER_BLOCKED: HRESULT = 3014;

/// The specified printer driver package is currently in use.
pub const ERROR_PRINTER_DRIVER_PACKAGE_IN_USE: HRESULT = 3015;

/// Unable to find a core driver package that is required by the printer driver package.
pub const ERROR_CORE_DRIVER_PACKAGE_NOT_FOUND: HRESULT = 3016;

/// The requested operation failed. A system reboot is required to roll back changes made.
pub const ERROR_FAIL_REBOOT_REQUIRED: HRESULT = 3017;

/// The requested operation failed. A system reboot has been initiated to roll back changes made.
pub const ERROR_FAIL_REBOOT_INITIATED: HRESULT = 3018;

/// The specified printer driver was not found on the system and needs to be downloaded.
pub const ERROR_PRINTER_DRIVER_DOWNLOAD_NEEDED: HRESULT = 3019;

/// The requested print job has failed to print. A print system update requires the job to be resubmitted.
pub const ERROR_PRINT_JOB_RESTART_REQUIRED: HRESULT = 3020;

/// The printer driver does not contain a valid manifest, or contains too many manifests.
pub const ERROR_INVALID_PRINTER_DRIVER_MANIFEST: HRESULT = 3021;

/// The specified printer cannot be shared.
pub const ERROR_PRINTER_NOT_SHAREABLE: HRESULT = 3022;

/// The requested function requires SMB1 to be present and enabled.
pub const ERROR_SERVER_SERVICE_CALL_REQUIRES_SMB1: HRESULT = 3023;

/// The user canceled the authentication prompt to a remote server.
pub const ERROR_NETWORK_AUTHENTICATION_PROMPT_CANCELED: HRESULT = 3024;

/// The operation was paused.
pub const ERROR_REQUEST_PAUSED: HRESULT = 3050;

/// The condition supplied for the app execution request was not satisfied, so the request was not performed.
pub const ERROR_APPEXEC_CONDITION_NOT_SATISFIED: HRESULT = 3060;

/// The supplied handle has been invalidated and may not be used for the requested operation.
pub const ERROR_APPEXEC_HANDLE_INVALIDATED: HRESULT = 3061;

/// The supplied host generation has been invalidated and may not be used for the requested operation.
pub const ERROR_APPEXEC_INVALID_HOST_GENERATION: HRESULT = 3062;

/// An attempt to register a process failed because the target host was not in a valid state to receive process registrations.
pub const ERROR_APPEXEC_UNEXPECTED_PROCESS_REGISTRATION: HRESULT = 3063;

/// The host is not in a valid state to support the execution request.
pub const ERROR_APPEXEC_INVALID_HOST_STATE: HRESULT = 3064;

/// The operation was not completed because a required resource donor was not found for the host.
pub const ERROR_APPEXEC_NO_DONOR: HRESULT = 3065;

/// The operation was not completed because an unexpected host ID was encountered.
pub const ERROR_APPEXEC_HOST_ID_MISMATCH: HRESULT = 3066;

/// The operation was not completed because the specified user was not known to the service.
pub const ERROR_APPEXEC_UNKNOWN_USER: HRESULT = 3067;

/// The application is blocked by app compat policy.
pub const ERROR_APPEXEC_APP_COMPAT_BLOCK: HRESULT = 3068;

/// The caller specified wait timed out before the operation completed.
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT: HRESULT = 3069;

/// The caller specified wait timed out before the operation completed because a host termination is in queued.
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_TERMINATION: HRESULT = 3070;

/// The caller specified wait timed out before the operation completed because a licensing operation is being performed.
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_LICENSING: HRESULT = 3071;

/// The caller specified wait timed out before the operation completed because resources are being acquired.
pub const ERROR_APPEXEC_CALLER_WAIT_TIMEOUT_RESOURCES: HRESULT = 3072;

/// Enabling driver verification from volatile command is currently not supported when both CFG and IO are enabled.
pub const ERROR_VRF_VOLATILE_CFG_AND_IO_ENABLED: HRESULT = 3080;

/// Removal of current driver verification is not supported from volatile command.
pub const ERROR_VRF_VOLATILE_NOT_STOPPABLE: HRESULT = 3081;

/// Enabling driver verification is not supported in safe mode.
pub const ERROR_VRF_VOLATILE_SAFE_MODE: HRESULT = 3082;

/// Enabling driver verification is not supported from volatile mode in current system.
pub const ERROR_VRF_VOLATILE_NOT_RUNNABLE_SYSTEM: HRESULT = 3083;

/// The specified rule class (a.k.a. flag) is not supported from volatile mode.
pub const ERROR_VRF_VOLATILE_NOT_SUPPORTED_RULECLASS: HRESULT = 3084;

/// The specified driver is protected and volatile verification is currently not supported.
pub const ERROR_VRF_VOLATILE_PROTECTED_DRIVER: HRESULT = 3085;

/// Enabling driver verification is not supported for a driver with  NMI callback(s) registered.
pub const ERROR_VRF_VOLATILE_NMI_REGISTERED: HRESULT = 3086;

/// Volatile verification settings cannot be changed when verification is enabled from boot or DIF volatile verification is enabled.
pub const ERROR_VRF_VOLATILE_SETTINGS_CONFLICT: HRESULT = 3087;

/// The specified driver is not associated with driver object or driver extension.
pub const ERROR_DIF_IOCALLBACK_NOT_REPLACED: HRESULT = 3190;

/// Verifier's internal data size exceeds the limit of live dump secondary data.
pub const ERROR_DIF_LIVEDUMP_LIMIT_EXCEEDED: HRESULT = 3191;

/// Verification cannot start because an attempt to lock code or data section failed.
pub const ERROR_DIF_VOLATILE_SECTION_NOT_LOCKED: HRESULT = 3192;

/// DIF volatile verification is not supported for hotpatched driver.
pub const ERROR_DIF_VOLATILE_DRIVER_HOTPATCHED: HRESULT = 3193;

/// The passed system DIF information is invalid.
pub const ERROR_DIF_VOLATILE_INVALID_INFO: HRESULT = 3194;

/// DIF volatile verification only supports on loaded drivers.
pub const ERROR_DIF_VOLATILE_DRIVER_IS_NOT_RUNNING: HRESULT = 3195;

/// Currently no plugin is running.
pub const ERROR_DIF_VOLATILE_PLUGIN_IS_NOT_RUNNING: HRESULT = 3196;

/// Currently running plugin must be removed before applying a new plugin.
pub const ERROR_DIF_VOLATILE_PLUGIN_CHANGE_NOT_ALLOWED: HRESULT = 3197;

/// The plugin is not allowed to run in volatile mode.
pub const ERROR_DIF_VOLATILE_NOT_ALLOWED: HRESULT = 3198;

/// One or more DDI is not yet supported by DIF.
pub const ERROR_DIF_BINDING_API_NOT_FOUND: HRESULT = 3199;

/// Reissue the given operation as a cached IO operation
pub const ERROR_IO_REISSUE_AS_CACHED: HRESULT = 3950;

/// WINS encountered an error while processing the command.
pub const ERROR_WINS_INTERNAL: HRESULT = 4000;

/// The local WINS cannot be deleted.
pub const ERROR_CAN_NOT_DEL_LOCAL_WINS: HRESULT = 4001;

/// The importation from the file failed.
pub const ERROR_STATIC_INIT: HRESULT = 4002;

/// The backup failed. Was a full backup done before?
pub const ERROR_INC_BACKUP: HRESULT = 4003;

/// The backup failed. Check the directory to which you are backing the database.
pub const ERROR_FULL_BACKUP: HRESULT = 4004;

/// The name does not exist in the WINS database.
pub const ERROR_REC_NON_EXISTENT: HRESULT = 4005;

/// Replication with a nonconfigured partner is not allowed.
pub const ERROR_RPL_NOT_ALLOWED: HRESULT = 4006;

/// The version of the supplied content information is not supported.
pub const PEERDIST_ERROR_CONTENTINFO_VERSION_UNSUPPORTED: HRESULT = 4050;

/// The supplied content information is malformed.
pub const PEERDIST_ERROR_CANNOT_PARSE_CONTENTINFO: HRESULT = 4051;

/// The requested data cannot be found in local or peer caches.
pub const PEERDIST_ERROR_MISSING_DATA: HRESULT = 4052;

/// No more data is available or required.
pub const PEERDIST_ERROR_NO_MORE: HRESULT = 4053;

/// The supplied object has not been initialized.
pub const PEERDIST_ERROR_NOT_INITIALIZED: HRESULT = 4054;

/// The supplied object has already been initialized.
pub const PEERDIST_ERROR_ALREADY_INITIALIZED: HRESULT = 4055;

/// A shutdown operation is already in progress.
pub const PEERDIST_ERROR_SHUTDOWN_IN_PROGRESS: HRESULT = 4056;

/// The supplied object has already been invalidated.
pub const PEERDIST_ERROR_INVALIDATED: HRESULT = 4057;

/// An element already exists and was not replaced.
pub const PEERDIST_ERROR_ALREADY_EXISTS: HRESULT = 4058;

/// Can not cancel the requested operation as it has already been completed.
pub const PEERDIST_ERROR_OPERATION_NOTFOUND: HRESULT = 4059;

/// Can not perform the requested operation because it has already been carried out.
pub const PEERDIST_ERROR_ALREADY_COMPLETED: HRESULT = 4060;

/// An operation accessed data beyond the bounds of valid data.
pub const PEERDIST_ERROR_OUT_OF_BOUNDS: HRESULT = 4061;

/// The requested version is not supported.
pub const PEERDIST_ERROR_VERSION_UNSUPPORTED: HRESULT = 4062;

/// A configuration value is invalid.
pub const PEERDIST_ERROR_INVALID_CONFIGURATION: HRESULT = 4063;

/// The SKU is not licensed.
pub const PEERDIST_ERROR_NOT_LICENSED: HRESULT = 4064;

/// PeerDist Service is still initializing and will be available shortly.
pub const PEERDIST_ERROR_SERVICE_UNAVAILABLE: HRESULT = 4065;

/// Communication with one or more computers will be temporarily blocked due to recent errors.
pub const PEERDIST_ERROR_TRUST_FAILURE: HRESULT = 4066;

/// The DHCP client has obtained an IP address that is already in use on the network. The local interface will be disabled until the DHCP client can obtain a new address.
pub const ERROR_DHCP_ADDRESS_CONFLICT: HRESULT = 4100;

/// The GUID passed was not recognized as valid by a WMI data provider.
pub const ERROR_WMI_GUID_NOT_FOUND: HRESULT = 4200;

/// The instance name passed was not recognized as valid by a WMI data provider.
pub const ERROR_WMI_INSTANCE_NOT_FOUND: HRESULT = 4201;

/// The data item ID passed was not recognized as valid by a WMI data provider.
pub const ERROR_WMI_ITEMID_NOT_FOUND: HRESULT = 4202;

/// The WMI request could not be completed and should be retried.
pub const ERROR_WMI_TRY_AGAIN: HRESULT = 4203;

/// The WMI data provider could not be located.
pub const ERROR_WMI_DP_NOT_FOUND: HRESULT = 4204;

/// The WMI data provider references an instance set that has not been registered.
pub const ERROR_WMI_UNRESOLVED_INSTANCE_REF: HRESULT = 4205;

/// The WMI data block or event notification has already been enabled.
pub const ERROR_WMI_ALREADY_ENABLED: HRESULT = 4206;

/// The WMI data block is no longer available.
pub const ERROR_WMI_GUID_DISCONNECTED: HRESULT = 4207;

/// The WMI data service is not available.
pub const ERROR_WMI_SERVER_UNAVAILABLE: HRESULT = 4208;

/// The WMI data provider failed to carry out the request.
pub const ERROR_WMI_DP_FAILED: HRESULT = 4209;

/// The WMI MOF information is not valid.
pub const ERROR_WMI_INVALID_MOF: HRESULT = 4210;

/// The WMI registration information is not valid.
pub const ERROR_WMI_INVALID_REGINFO: HRESULT = 4211;

/// The WMI data block or event notification has already been disabled.
pub const ERROR_WMI_ALREADY_DISABLED: HRESULT = 4212;

/// The WMI data item or data block is read only.
pub const ERROR_WMI_READ_ONLY: HRESULT = 4213;

/// The WMI data item or data block could not be changed.
pub const ERROR_WMI_SET_FAILURE: HRESULT = 4214;

/// This operation is only valid in the context of an app container.
pub const ERROR_NOT_APPCONTAINER: HRESULT = 4250;

/// This application can only run in the context of an app container.
pub const ERROR_APPCONTAINER_REQUIRED: HRESULT = 4251;

/// This functionality is not supported in the context of an app container.
pub const ERROR_NOT_SUPPORTED_IN_APPCONTAINER: HRESULT = 4252;

/// The length of the SID supplied is not a valid length for app container SIDs.
pub const ERROR_INVALID_PACKAGE_SID_LENGTH: HRESULT = 4253;

/// The media identifier does not represent a valid medium.
pub const ERROR_INVALID_MEDIA: HRESULT = 4300;

/// The library identifier does not represent a valid library.
pub const ERROR_INVALID_LIBRARY: HRESULT = 4301;

/// The media pool identifier does not represent a valid media pool.
pub const ERROR_INVALID_MEDIA_POOL: HRESULT = 4302;

/// The drive and medium are not compatible or exist in different libraries.
pub const ERROR_DRIVE_MEDIA_MISMATCH: HRESULT = 4303;

/// The medium currently exists in an offline library and must be online to perform this operation.
pub const ERROR_MEDIA_OFFLINE: HRESULT = 4304;

/// The operation cannot be performed on an offline library.
pub const ERROR_LIBRARY_OFFLINE: HRESULT = 4305;

/// The library, drive, or media pool is empty.
pub const ERROR_EMPTY: HRESULT = 4306;

/// The library, drive, or media pool must be empty to perform this operation.
pub const ERROR_NOT_EMPTY: HRESULT = 4307;

/// No media is currently available in this media pool or library.
pub const ERROR_MEDIA_UNAVAILABLE: HRESULT = 4308;

/// A resource required for this operation is disabled.
pub const ERROR_RESOURCE_DISABLED: HRESULT = 4309;

/// The media identifier does not represent a valid cleaner.
pub const ERROR_INVALID_CLEANER: HRESULT = 4310;

/// The drive cannot be cleaned or does not support cleaning.
pub const ERROR_UNABLE_TO_CLEAN: HRESULT = 4311;

/// The object identifier does not represent a valid object.
pub const ERROR_OBJECT_NOT_FOUND: HRESULT = 4312;

/// Unable to read from or write to the database.
pub const ERROR_DATABASE_FAILURE: HRESULT = 4313;

/// The database is full.
pub const ERROR_DATABASE_FULL: HRESULT = 4314;

/// The medium is not compatible with the device or media pool.
pub const ERROR_MEDIA_INCOMPATIBLE: HRESULT = 4315;

/// The resource required for this operation does not exist.
pub const ERROR_RESOURCE_NOT_PRESENT: HRESULT = 4316;

/// The operation identifier is not valid.
pub const ERROR_INVALID_OPERATION: HRESULT = 4317;

/// The media is not mounted or ready for use.
pub const ERROR_MEDIA_NOT_AVAILABLE: HRESULT = 4318;

/// The device is not ready for use.
pub const ERROR_DEVICE_NOT_AVAILABLE: HRESULT = 4319;

/// The operator or administrator has refused the request.
pub const ERROR_REQUEST_REFUSED: HRESULT = 4320;

/// The drive identifier does not represent a valid drive.
pub const ERROR_INVALID_DRIVE_OBJECT: HRESULT = 4321;

/// Library is full. No slot is available for use.
pub const ERROR_LIBRARY_FULL: HRESULT = 4322;

/// The transport cannot access the medium.
pub const ERROR_MEDIUM_NOT_ACCESSIBLE: HRESULT = 4323;

/// Unable to load the medium into the drive.
pub const ERROR_UNABLE_TO_LOAD_MEDIUM: HRESULT = 4324;

/// Unable to retrieve the drive status.
pub const ERROR_UNABLE_TO_INVENTORY_DRIVE: HRESULT = 4325;

/// Unable to retrieve the slot status.
pub const ERROR_UNABLE_TO_INVENTORY_SLOT: HRESULT = 4326;

/// Unable to retrieve status about the transport.
pub const ERROR_UNABLE_TO_INVENTORY_TRANSPORT: HRESULT = 4327;

/// Cannot use the transport because it is already in use.
pub const ERROR_TRANSPORT_FULL: HRESULT = 4328;

/// Unable to open or close the inject/eject port.
pub const ERROR_CONTROLLING_IEPORT: HRESULT = 4329;

/// Unable to eject the medium because it is in a drive.
pub const ERROR_UNABLE_TO_EJECT_MOUNTED_MEDIA: HRESULT = 4330;

/// A cleaner slot is already reserved.
pub const ERROR_CLEANER_SLOT_SET: HRESULT = 4331;

/// A cleaner slot is not reserved.
pub const ERROR_CLEANER_SLOT_NOT_SET: HRESULT = 4332;

/// The cleaner cartridge has performed the maximum number of drive cleanings.
pub const ERROR_CLEANER_CARTRIDGE_SPENT: HRESULT = 4333;

/// Unexpected on-medium identifier.
pub const ERROR_UNEXPECTED_OMID: HRESULT = 4334;

/// The last remaining item in this group or resource cannot be deleted.
pub const ERROR_CANT_DELETE_LAST_ITEM: HRESULT = 4335;

/// The message provided exceeds the maximum size allowed for this parameter.
pub const ERROR_MESSAGE_EXCEEDS_MAX_SIZE: HRESULT = 4336;

/// The volume contains system or paging files.
pub const ERROR_VOLUME_CONTAINS_SYS_FILES: HRESULT = 4337;

/// The media type cannot be removed from this library since at least one drive in the library reports it can support this media type.
pub const ERROR_INDIGENOUS_TYPE: HRESULT = 4338;

/// This offline media cannot be mounted on this system since no enabled drives are present which can be used.
pub const ERROR_NO_SUPPORTING_DRIVES: HRESULT = 4339;

/// A cleaner cartridge is present in the tape library.
pub const ERROR_CLEANER_CARTRIDGE_INSTALLED: HRESULT = 4340;

/// Cannot use the inject/eject port because it is not empty.
pub const ERROR_IEPORT_FULL: HRESULT = 4341;

/// This file is currently not available for use on this computer.
pub const ERROR_FILE_OFFLINE: HRESULT = 4350;

/// The remote storage service is not operational at this time.
pub const ERROR_REMOTE_STORAGE_NOT_ACTIVE: HRESULT = 4351;

/// The remote storage service encountered a media error.
pub const ERROR_REMOTE_STORAGE_MEDIA_ERROR: HRESULT = 4352;

/// The file or directory is not a reparse point.
pub const ERROR_NOT_A_REPARSE_POINT: HRESULT = 4390;

/// The reparse point attribute cannot be set because it conflicts with an existing attribute.
pub const ERROR_REPARSE_ATTRIBUTE_CONFLICT: HRESULT = 4391;

/// The data present in the reparse point buffer is invalid.
pub const ERROR_INVALID_REPARSE_DATA: HRESULT = 4392;

/// The tag present in the reparse point buffer is invalid.
pub const ERROR_REPARSE_TAG_INVALID: HRESULT = 4393;

/// There is a mismatch between the tag specified in the request and the tag present in the reparse point.
pub const ERROR_REPARSE_TAG_MISMATCH: HRESULT = 4394;

/// The object manager encountered a reparse point while retrieving an object.
pub const ERROR_REPARSE_POINT_ENCOUNTERED: HRESULT = 4395;

/// Fast Cache data not found.
pub const ERROR_APP_DATA_NOT_FOUND: HRESULT = 4400;

/// Fast Cache data expired.
pub const ERROR_APP_DATA_EXPIRED: HRESULT = 4401;

/// Fast Cache data corrupt.
pub const ERROR_APP_DATA_CORRUPT: HRESULT = 4402;

/// Fast Cache data has exceeded its max size and cannot be updated.
pub const ERROR_APP_DATA_LIMIT_EXCEEDED: HRESULT = 4403;

/// Fast Cache has been ReArmed and requires a reboot until it can be updated.
pub const ERROR_APP_DATA_REBOOT_REQUIRED: HRESULT = 4404;

/// Secure Boot detected that rollback of protected data has been attempted.
pub const ERROR_SECUREBOOT_ROLLBACK_DETECTED: HRESULT = 4420;

/// The value is protected by Secure Boot policy and cannot be modified or deleted.
pub const ERROR_SECUREBOOT_POLICY_VIOLATION: HRESULT = 4421;

/// The Secure Boot policy is invalid.
pub const ERROR_SECUREBOOT_INVALID_POLICY: HRESULT = 4422;

/// A new Secure Boot policy did not contain the current publisher on its update list.
pub const ERROR_SECUREBOOT_POLICY_PUBLISHER_NOT_FOUND: HRESULT = 4423;

/// The Secure Boot policy is either not signed or is signed by a non-trusted signer.
pub const ERROR_SECUREBOOT_POLICY_NOT_SIGNED: HRESULT = 4424;

/// Secure Boot is not enabled on this machine.
pub const ERROR_SECUREBOOT_NOT_ENABLED: HRESULT = 4425;

/// Secure Boot requires that certain files and drivers are not replaced by other files or drivers.
pub const ERROR_SECUREBOOT_FILE_REPLACED: HRESULT = 4426;

/// The Secure Boot Supplemental Policy file was not authorized on this machine.
pub const ERROR_SECUREBOOT_POLICY_NOT_AUTHORIZED: HRESULT = 4427;

/// The Supplemental Policy is not recognized on this device.
pub const ERROR_SECUREBOOT_POLICY_UNKNOWN: HRESULT = 4428;

/// The Antirollback version was not found in the Secure Boot Policy.
pub const ERROR_SECUREBOOT_POLICY_MISSING_ANTIROLLBACKVERSION: HRESULT = 4429;

/// The Platform ID specified in the Secure Boot policy does not match the Platform ID on this device.
pub const ERROR_SECUREBOOT_PLATFORM_ID_MISMATCH: HRESULT = 4430;

/// The Secure Boot policy file has an older Antirollback Version than this device.
pub const ERROR_SECUREBOOT_POLICY_ROLLBACK_DETECTED: HRESULT = 4431;

/// The Secure Boot policy file does not match the upgraded legacy policy.
pub const ERROR_SECUREBOOT_POLICY_UPGRADE_MISMATCH: HRESULT = 4432;

/// The Secure Boot policy file is required but could not be found.
pub const ERROR_SECUREBOOT_REQUIRED_POLICY_FILE_MISSING: HRESULT = 4433;

/// Supplemental Secure Boot policy file can not be loaded as a base Secure Boot policy.
pub const ERROR_SECUREBOOT_NOT_BASE_POLICY: HRESULT = 4434;

/// Base Secure Boot policy file can not be loaded as a Supplemental Secure Boot policy.
pub const ERROR_SECUREBOOT_NOT_SUPPLEMENTAL_POLICY: HRESULT = 4435;

/// The copy offload read operation is not supported by a filter.
pub const ERROR_OFFLOAD_READ_FLT_NOT_SUPPORTED: HRESULT = 4440;

/// The copy offload write operation is not supported by a filter.
pub const ERROR_OFFLOAD_WRITE_FLT_NOT_SUPPORTED: HRESULT = 4441;

/// The copy offload read operation is not supported for the file.
pub const ERROR_OFFLOAD_READ_FILE_NOT_SUPPORTED: HRESULT = 4442;

/// The copy offload write operation is not supported for the file.
pub const ERROR_OFFLOAD_WRITE_FILE_NOT_SUPPORTED: HRESULT = 4443;

/// This file is currently associated with a different stream id.
pub const ERROR_ALREADY_HAS_STREAM_ID: HRESULT = 4444;

/// The volume must undergo garbage collection.
pub const ERROR_SMR_GARBAGE_COLLECTION_REQUIRED: HRESULT = 4445;

/// The WOF driver encountered a corruption in WIM image's Header.
pub const ERROR_WOF_WIM_HEADER_CORRUPT: HRESULT = 4446;

/// The WOF driver encountered a corruption in WIM image's Resource Table.
pub const ERROR_WOF_WIM_RESOURCE_TABLE_CORRUPT: HRESULT = 4447;

/// The WOF driver encountered a corruption in the compressed file's Resource Table.
pub const ERROR_WOF_FILE_RESOURCE_TABLE_CORRUPT: HRESULT = 4448;

/// The request cannot be completed as it requires modifying an immutable object.
pub const ERROR_OBJECT_IS_IMMUTABLE: HRESULT = 4449;

/// Single Instance Storage is not available on this volume.
pub const ERROR_VOLUME_NOT_SIS_ENABLED: HRESULT = 4500;

/// System Integrity detected that policy rollback has been attempted.
pub const ERROR_SYSTEM_INTEGRITY_ROLLBACK_DETECTED: HRESULT = 4550;

/// Your organization used Device Guard to block this app. Contact your support person for more info.
pub const ERROR_SYSTEM_INTEGRITY_POLICY_VIOLATION: HRESULT = 4551;

/// The System Integrity policy is invalid.
pub const ERROR_SYSTEM_INTEGRITY_INVALID_POLICY: HRESULT = 4552;

/// The System Integrity policy is either not signed or is signed by a non-trusted signer.
pub const ERROR_SYSTEM_INTEGRITY_POLICY_NOT_SIGNED: HRESULT = 4553;

/// The number of System Integrity policies is out of limit.
pub const ERROR_SYSTEM_INTEGRITY_TOO_MANY_POLICIES: HRESULT = 4554;

/// The Code Integrity supplemental policy is not authorized by a Code Integrity base policy.
pub const ERROR_SYSTEM_INTEGRITY_SUPPLEMENTAL_POLICY_NOT_AUTHORIZED: HRESULT = 4555;

/// System Integrity policy has been violated.  Malicious binary reputation.
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_MALICIOUS: HRESULT = 4556;

/// System Integrity policy has been violated.  Potentially unwanted application.
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_PUA: HRESULT = 4557;

/// System Integrity policy has been violated.  Dangerous file extension from the web.
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_DANGEROUS_EXT: HRESULT = 4558;

/// System Integrity policy has been violated.  Unable to contact reputation service for unknown file.
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_OFFLINE: HRESULT = 4559;

/// Virtual Secure Mode (VSM) is not initialized. The hypervisor or VSM may not be present or enabled.
pub const ERROR_VSM_NOT_INITIALIZED: HRESULT = 4560;

/// The hypervisor is not protecting DMA because an IOMMU is not present or not enabled in the BIOS.
pub const ERROR_VSM_DMA_PROTECTION_NOT_IN_USE: HRESULT = 4561;

/// The Platform Manifest file was not authorized on this machine.
pub const ERROR_PLATFORM_MANIFEST_NOT_AUTHORIZED: HRESULT = 4570;

/// The Platform Manifest file was not valid.
pub const ERROR_PLATFORM_MANIFEST_INVALID: HRESULT = 4571;

/// The file is not authorized on this platform because an entry was not found in the Platform Manifest.
pub const ERROR_PLATFORM_MANIFEST_FILE_NOT_AUTHORIZED: HRESULT = 4572;

/// The catalog is not authorized on this platform because an entry was not found in the Platform Manifest.
pub const ERROR_PLATFORM_MANIFEST_CATALOG_NOT_AUTHORIZED: HRESULT = 4573;

/// The file is not authorized on this platform because a Binary ID was not found in the embedded signature.
pub const ERROR_PLATFORM_MANIFEST_BINARY_ID_NOT_FOUND: HRESULT = 4574;

/// No active Platform Manifest exists on this system.
pub const ERROR_PLATFORM_MANIFEST_NOT_ACTIVE: HRESULT = 4575;

/// The Platform Manifest file was not properly signed.
pub const ERROR_PLATFORM_MANIFEST_NOT_SIGNED: HRESULT = 4576;

/// System Integrity policy has been violated.  Unfriendly file.
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_UNFRIENDLY_FILE: HRESULT = 4580;

/// System Integrity policy has been violated.  Failed to obtain file reputation because an infrastructure issue occurred. Try again later.
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_UNATTAINABLE: HRESULT = 4581;

/// System Integrity policy has been violated.  Explicit denied file.
pub const ERROR_SYSTEM_INTEGRITY_REPUTATION_EXPLICIT_DENY_FILE: HRESULT = 4582;

/// The operation cannot be completed because other resources are dependent on this resource.
pub const ERROR_DEPENDENT_RESOURCE_EXISTS: HRESULT = 5001;

/// The cluster resource dependency cannot be found.
pub const ERROR_DEPENDENCY_NOT_FOUND: HRESULT = 5002;

/// The cluster resource cannot be made dependent on the specified resource because it is already dependent.
pub const ERROR_DEPENDENCY_ALREADY_EXISTS: HRESULT = 5003;

/// The cluster resource is not online.
pub const ERROR_RESOURCE_NOT_ONLINE: HRESULT = 5004;

/// A cluster node is not available for this operation.
pub const ERROR_HOST_NODE_NOT_AVAILABLE: HRESULT = 5005;

/// The cluster resource is not available.
pub const ERROR_RESOURCE_NOT_AVAILABLE: HRESULT = 5006;

/// The cluster resource could not be found.
pub const ERROR_RESOURCE_NOT_FOUND: HRESULT = 5007;

/// The cluster is being shut down.
pub const ERROR_SHUTDOWN_CLUSTER: HRESULT = 5008;

/// A cluster node cannot be evicted from the cluster unless the node is down or it is the last node.
pub const ERROR_CANT_EVICT_ACTIVE_NODE: HRESULT = 5009;

/// The object already exists.
pub const ERROR_OBJECT_ALREADY_EXISTS: HRESULT = 5010;

/// The object is already in the list.
pub const ERROR_OBJECT_IN_LIST: HRESULT = 5011;

/// The cluster group is not available for any new requests.
pub const ERROR_GROUP_NOT_AVAILABLE: HRESULT = 5012;

/// The cluster group could not be found.
pub const ERROR_GROUP_NOT_FOUND: HRESULT = 5013;

/// The operation could not be completed because the cluster group is not online.
pub const ERROR_GROUP_NOT_ONLINE: HRESULT = 5014;

/// The operation failed because either the specified cluster node is not the owner of the resource, or the node is not a possible owner of the resource.
pub const ERROR_HOST_NODE_NOT_RESOURCE_OWNER: HRESULT = 5015;

/// The operation failed because either the specified cluster node is not the owner of the group, or the node is not a possible owner of the group.
pub const ERROR_HOST_NODE_NOT_GROUP_OWNER: HRESULT = 5016;

/// The cluster resource could not be created in the specified resource monitor.
pub const ERROR_RESMON_CREATE_FAILED: HRESULT = 5017;

/// The cluster resource could not be brought online by the resource monitor.
pub const ERROR_RESMON_ONLINE_FAILED: HRESULT = 5018;

/// The operation could not be completed because the cluster resource is online.
pub const ERROR_RESOURCE_ONLINE: HRESULT = 5019;

/// The cluster resource could not be deleted or brought offline because it is the quorum resource.
pub const ERROR_QUORUM_RESOURCE: HRESULT = 5020;

/// The cluster could not make the specified resource a quorum resource because it is not capable of being a quorum resource.
pub const ERROR_NOT_QUORUM_CAPABLE: HRESULT = 5021;

/// The cluster software is shutting down.
pub const ERROR_CLUSTER_SHUTTING_DOWN: HRESULT = 5022;

/// The group or resource is not in the correct state to perform the requested operation.
pub const ERROR_INVALID_STATE: HRESULT = 5023;

/// The properties were stored but not all changes will take effect until the next time the resource is brought online.
pub const ERROR_RESOURCE_PROPERTIES_STORED: HRESULT = 5024;

/// The cluster could not make the specified resource a quorum resource because it does not belong to a shared storage class.
pub const ERROR_NOT_QUORUM_CLASS: HRESULT = 5025;

/// The cluster resource could not be deleted since it is a core resource.
pub const ERROR_CORE_RESOURCE: HRESULT = 5026;

/// The quorum resource failed to come online.
pub const ERROR_QUORUM_RESOURCE_ONLINE_FAILED: HRESULT = 5027;

/// The quorum log could not be created or mounted successfully.
pub const ERROR_QUORUMLOG_OPEN_FAILED: HRESULT = 5028;

/// The cluster log is corrupt.
pub const ERROR_CLUSTERLOG_CORRUPT: HRESULT = 5029;

/// The record could not be written to the cluster log since it exceeds the maximum size.
pub const ERROR_CLUSTERLOG_RECORD_EXCEEDS_MAXSIZE: HRESULT = 5030;

/// The cluster log exceeds its maximum size.
pub const ERROR_CLUSTERLOG_EXCEEDS_MAXSIZE: HRESULT = 5031;

/// No checkpoint record was found in the cluster log.
pub const ERROR_CLUSTERLOG_CHKPOINT_NOT_FOUND: HRESULT = 5032;

/// The minimum required disk space needed for logging is not available.
pub const ERROR_CLUSTERLOG_NOT_ENOUGH_SPACE: HRESULT = 5033;

/// The cluster node failed to take control of the quorum resource because the resource is owned by another active node.
pub const ERROR_QUORUM_OWNER_ALIVE: HRESULT = 5034;

/// A cluster network is not available for this operation.
pub const ERROR_NETWORK_NOT_AVAILABLE: HRESULT = 5035;

/// A cluster node is not available for this operation.
pub const ERROR_NODE_NOT_AVAILABLE: HRESULT = 5036;

/// All cluster nodes must be running to perform this operation.
pub const ERROR_ALL_NODES_NOT_AVAILABLE: HRESULT = 5037;

/// A cluster resource failed.
pub const ERROR_RESOURCE_FAILED: HRESULT = 5038;

/// The cluster node is not valid.
pub const ERROR_CLUSTER_INVALID_NODE: HRESULT = 5039;

/// The cluster node already exists.
pub const ERROR_CLUSTER_NODE_EXISTS: HRESULT = 5040;

/// A node is in the process of joining the cluster.
pub const ERROR_CLUSTER_JOIN_IN_PROGRESS: HRESULT = 5041;

/// The cluster node was not found.
pub const ERROR_CLUSTER_NODE_NOT_FOUND: HRESULT = 5042;

/// The cluster local node information was not found.
pub const ERROR_CLUSTER_LOCAL_NODE_NOT_FOUND: HRESULT = 5043;

/// The cluster network already exists.
pub const ERROR_CLUSTER_NETWORK_EXISTS: HRESULT = 5044;

/// The cluster network was not found.
pub const ERROR_CLUSTER_NETWORK_NOT_FOUND: HRESULT = 5045;

/// The cluster network interface already exists.
pub const ERROR_CLUSTER_NETINTERFACE_EXISTS: HRESULT = 5046;

/// The cluster network interface was not found.
pub const ERROR_CLUSTER_NETINTERFACE_NOT_FOUND: HRESULT = 5047;

/// The cluster request is not valid for this object.
pub const ERROR_CLUSTER_INVALID_REQUEST: HRESULT = 5048;

/// The cluster network provider is not valid.
pub const ERROR_CLUSTER_INVALID_NETWORK_PROVIDER: HRESULT = 5049;

/// The cluster node is down.
pub const ERROR_CLUSTER_NODE_DOWN: HRESULT = 5050;

/// The cluster node is not reachable.
pub const ERROR_CLUSTER_NODE_UNREACHABLE: HRESULT = 5051;

/// The cluster node is not a member of the cluster.
pub const ERROR_CLUSTER_NODE_NOT_MEMBER: HRESULT = 5052;

/// A cluster join operation is not in progress.
pub const ERROR_CLUSTER_JOIN_NOT_IN_PROGRESS: HRESULT = 5053;

/// The cluster network is not valid.
pub const ERROR_CLUSTER_INVALID_NETWORK: HRESULT = 5054;

/// The cluster node is up.
pub const ERROR_CLUSTER_NODE_UP: HRESULT = 5056;

/// The cluster IP address is already in use.
pub const ERROR_CLUSTER_IPADDR_IN_USE: HRESULT = 5057;

/// The cluster node is not paused.
pub const ERROR_CLUSTER_NODE_NOT_PAUSED: HRESULT = 5058;

/// No cluster security context is available.
pub const ERROR_CLUSTER_NO_SECURITY_CONTEXT: HRESULT = 5059;

/// The cluster network is not configured for internal cluster communication.
pub const ERROR_CLUSTER_NETWORK_NOT_INTERNAL: HRESULT = 5060;

/// The cluster node is already up.
pub const ERROR_CLUSTER_NODE_ALREADY_UP: HRESULT = 5061;

/// The cluster node is already down.
pub const ERROR_CLUSTER_NODE_ALREADY_DOWN: HRESULT = 5062;

/// The cluster network is already online.
pub const ERROR_CLUSTER_NETWORK_ALREADY_ONLINE: HRESULT = 5063;

/// The cluster network is already offline.
pub const ERROR_CLUSTER_NETWORK_ALREADY_OFFLINE: HRESULT = 5064;

/// The cluster node is already a member of the cluster.
pub const ERROR_CLUSTER_NODE_ALREADY_MEMBER: HRESULT = 5065;

/// The cluster network is the only one configured for internal cluster communication between two or more active cluster nodes. The internal communication capability cannot be removed from the network.
pub const ERROR_CLUSTER_LAST_INTERNAL_NETWORK: HRESULT = 5066;

/// One or more cluster resources depend on the network to provide service to clients. The client access capability cannot be removed from the network.
pub const ERROR_CLUSTER_NETWORK_HAS_DEPENDENTS: HRESULT = 5067;

/// This operation cannot currently be performed on the cluster group containing the quorum resource.
pub const ERROR_INVALID_OPERATION_ON_QUORUM: HRESULT = 5068;

/// The cluster quorum resource is not allowed to have any dependencies.
pub const ERROR_DEPENDENCY_NOT_ALLOWED: HRESULT = 5069;

/// The cluster node is paused.
pub const ERROR_CLUSTER_NODE_PAUSED: HRESULT = 5070;

/// The cluster resource cannot be brought online. The owner node cannot run this resource.
pub const ERROR_NODE_CANT_HOST_RESOURCE: HRESULT = 5071;

/// The cluster node is not ready to perform the requested operation.
pub const ERROR_CLUSTER_NODE_NOT_READY: HRESULT = 5072;

/// The cluster node is shutting down.
pub const ERROR_CLUSTER_NODE_SHUTTING_DOWN: HRESULT = 5073;

/// The cluster join operation was aborted.
pub const ERROR_CLUSTER_JOIN_ABORTED: HRESULT = 5074;

/// The node failed to join the cluster because the joining node and other nodes in the cluster have incompatible operating system versions. To get more information about operating system versions of the cluster, run the Validate a Configuration Wizard or the Test-Cluster Windows PowerShell cmdlet.
pub const ERROR_CLUSTER_INCOMPATIBLE_VERSIONS: HRESULT = 5075;

/// This resource cannot be created because the cluster has reached the limit on the number of resources it can monitor.
pub const ERROR_CLUSTER_MAXNUM_OF_RESOURCES_EXCEEDED: HRESULT = 5076;

/// The system configuration changed during the cluster join or form operation. The join or form operation was aborted.
pub const ERROR_CLUSTER_SYSTEM_CONFIG_CHANGED: HRESULT = 5077;

/// The specified resource type was not found.
pub const ERROR_CLUSTER_RESOURCE_TYPE_NOT_FOUND: HRESULT = 5078;

/// The specified node does not support a resource of this type. This may be due to version inconsistencies or due to the absence of the resource DLL on this node.
pub const ERROR_CLUSTER_RESTYPE_NOT_SUPPORTED: HRESULT = 5079;

/// The specified resource name is not supported by this resource DLL. This may be due to a bad (or changed) name supplied to the resource DLL.
pub const ERROR_CLUSTER_RESNAME_NOT_FOUND: HRESULT = 5080;

/// No authentication package could be registered with the RPC server.
pub const ERROR_CLUSTER_NO_RPC_PACKAGES_REGISTERED: HRESULT = 5081;

/// You cannot bring the group online because the owner of the group is not in the preferred list for the group. To change the owner node for the group, move the group.
pub const ERROR_CLUSTER_OWNER_NOT_IN_PREFLIST: HRESULT = 5082;

/// The join operation failed because the cluster database sequence number has changed or is incompatible with the locker node. This may happen during a join operation if the cluster database was changing during the join.
pub const ERROR_CLUSTER_DATABASE_SEQMISMATCH: HRESULT = 5083;

/// The resource monitor will not allow the fail operation to be performed while the resource is in its current state. This may happen if the resource is in a pending state.
pub const ERROR_RESMON_INVALID_STATE: HRESULT = 5084;

/// A non locker code got a request to reserve the lock for making global updates.
pub const ERROR_CLUSTER_GUM_NOT_LOCKER: HRESULT = 5085;

/// The quorum disk could not be located by the cluster service.
pub const ERROR_QUORUM_DISK_NOT_FOUND: HRESULT = 5086;

/// The backed up cluster database is possibly corrupt.
pub const ERROR_DATABASE_BACKUP_CORRUPT: HRESULT = 5087;

/// A DFS root already exists in this cluster node.
pub const ERROR_CLUSTER_NODE_ALREADY_HAS_DFS_ROOT: HRESULT = 5088;

/// An attempt to modify a resource property failed because it conflicts with another existing property.
pub const ERROR_RESOURCE_PROPERTY_UNCHANGEABLE: HRESULT = 5089;

/// This operation is not supported on a cluster without an Administrator Access Point.
pub const ERROR_NO_ADMIN_ACCESS_POINT: HRESULT = 5090;

/// An operation was attempted that is incompatible with the current membership state of the node.
pub const ERROR_CLUSTER_MEMBERSHIP_INVALID_STATE: HRESULT = 5890;

/// The quorum resource does not contain the quorum log.
pub const ERROR_CLUSTER_QUORUMLOG_NOT_FOUND: HRESULT = 5891;

/// The membership engine requested shutdown of the cluster service on this node.
pub const ERROR_CLUSTER_MEMBERSHIP_HALT: HRESULT = 5892;

/// The join operation failed because the cluster instance ID of the joining node does not match the cluster instance ID of the sponsor node.
pub const ERROR_CLUSTER_INSTANCE_ID_MISMATCH: HRESULT = 5893;

/// A matching cluster network for the specified IP address could not be found.
pub const ERROR_CLUSTER_NETWORK_NOT_FOUND_FOR_IP: HRESULT = 5894;

/// The actual data type of the property did not match the expected data type of the property.
pub const ERROR_CLUSTER_PROPERTY_DATA_TYPE_MISMATCH: HRESULT = 5895;

/// The cluster node was evicted from the cluster successfully, but the node was not cleaned up. To determine what cleanup steps failed and how to recover, see the Failover Clustering application event log using Event Viewer.
pub const ERROR_CLUSTER_EVICT_WITHOUT_CLEANUP: HRESULT = 5896;

/// Two or more parameter values specified for a resource's properties are in conflict.
pub const ERROR_CLUSTER_PARAMETER_MISMATCH: HRESULT = 5897;

/// This computer cannot be made a member of a cluster.
pub const ERROR_NODE_CANNOT_BE_CLUSTERED: HRESULT = 5898;

/// This computer cannot be made a member of a cluster because it does not have the correct version of Windows installed.
pub const ERROR_CLUSTER_WRONG_OS_VERSION: HRESULT = 5899;

/// A cluster cannot be created with the specified cluster name because that cluster name is already in use. Specify a different name for the cluster.
pub const ERROR_CLUSTER_CANT_CREATE_DUP_CLUSTER_NAME: HRESULT = 5900;

/// The cluster configuration action has already been committed.
pub const ERROR_CLUSCFG_ALREADY_COMMITTED: HRESULT = 5901;

/// The cluster configuration action could not be rolled back.
pub const ERROR_CLUSCFG_ROLLBACK_FAILED: HRESULT = 5902;

/// The drive letter assigned to a system disk on one node conflicted with the drive letter assigned to a disk on another node.
pub const ERROR_CLUSCFG_SYSTEM_DISK_DRIVE_LETTER_CONFLICT: HRESULT = 5903;

/// One or more nodes in the cluster are running a version of Windows that does not support this operation.
pub const ERROR_CLUSTER_OLD_VERSION: HRESULT = 5904;

/// The name of the corresponding computer account doesn't match the Network Name for this resource.
pub const ERROR_CLUSTER_MISMATCHED_COMPUTER_ACCT_NAME: HRESULT = 5905;

/// No network adapters are available.
pub const ERROR_CLUSTER_NO_NET_ADAPTERS: HRESULT = 5906;

/// The cluster node has been poisoned.
pub const ERROR_CLUSTER_POISONED: HRESULT = 5907;

/// The group is unable to accept the request since it is moving to another node.
pub const ERROR_CLUSTER_GROUP_MOVING: HRESULT = 5908;

/// The resource type cannot accept the request since is too busy performing another operation.
pub const ERROR_CLUSTER_RESOURCE_TYPE_BUSY: HRESULT = 5909;

/// The call to the cluster resource DLL timed out.
pub const ERROR_RESOURCE_CALL_TIMED_OUT: HRESULT = 5910;

/// The address is not valid for an IPv6 Address resource. A global IPv6 address is required, and it must match a cluster network. Compatibility addresses are not permitted.
pub const ERROR_INVALID_CLUSTER_IPV6_ADDRESS: HRESULT = 5911;

/// An internal cluster error occurred. A call to an invalid function was attempted.
pub const ERROR_CLUSTER_INTERNAL_INVALID_FUNCTION: HRESULT = 5912;

/// A parameter value is out of acceptable range.
pub const ERROR_CLUSTER_PARAMETER_OUT_OF_BOUNDS: HRESULT = 5913;

/// A network error occurred while sending data to another node in the cluster. The number of bytes transmitted was less than required.
pub const ERROR_CLUSTER_PARTIAL_SEND: HRESULT = 5914;

/// An invalid cluster registry operation was attempted.
pub const ERROR_CLUSTER_REGISTRY_INVALID_FUNCTION: HRESULT = 5915;

/// An input string of characters is not properly terminated.
pub const ERROR_CLUSTER_INVALID_STRING_TERMINATION: HRESULT = 5916;

/// An input string of characters is not in a valid format for the data it represents.
pub const ERROR_CLUSTER_INVALID_STRING_FORMAT: HRESULT = 5917;

/// An internal cluster error occurred. A cluster database transaction was attempted while a transaction was already in progress.
pub const ERROR_CLUSTER_DATABASE_TRANSACTION_IN_PROGRESS: HRESULT = 5918;

/// An internal cluster error occurred. There was an attempt to commit a cluster database transaction while no transaction was in progress.
pub const ERROR_CLUSTER_DATABASE_TRANSACTION_NOT_IN_PROGRESS: HRESULT = 5919;

/// An internal cluster error occurred. Data was not properly initialized.
pub const ERROR_CLUSTER_NULL_DATA: HRESULT = 5920;

/// An error occurred while reading from a stream of data. An unexpected number of bytes was returned.
pub const ERROR_CLUSTER_PARTIAL_READ: HRESULT = 5921;

/// An error occurred while writing to a stream of data. The required number of bytes could not be written.
pub const ERROR_CLUSTER_PARTIAL_WRITE: HRESULT = 5922;

/// An error occurred while deserializing a stream of cluster data.
pub const ERROR_CLUSTER_CANT_DESERIALIZE_DATA: HRESULT = 5923;

/// One or more property values for this resource are in conflict with one or more property values associated with its dependent resource(s).
pub const ERROR_DEPENDENT_RESOURCE_PROPERTY_CONFLICT: HRESULT = 5924;

/// A quorum of cluster nodes was not present to form a cluster.
pub const ERROR_CLUSTER_NO_QUORUM: HRESULT = 5925;

/// The cluster network is not valid for an IPv6 Address resource, or it does not match the configured address.
pub const ERROR_CLUSTER_INVALID_IPV6_NETWORK: HRESULT = 5926;

/// The cluster network is not valid for an IPv6 Tunnel resource. Check the configuration of the IP Address resource on which the IPv6 Tunnel resource depends.
pub const ERROR_CLUSTER_INVALID_IPV6_TUNNEL_NETWORK: HRESULT = 5927;

/// Quorum resource cannot reside in the Available Storage group.
pub const ERROR_QUORUM_NOT_ALLOWED_IN_THIS_GROUP: HRESULT = 5928;

/// The dependencies for this resource are nested too deeply.
pub const ERROR_DEPENDENCY_TREE_TOO_COMPLEX: HRESULT = 5929;

/// The call into the resource DLL raised an unhandled exception.
pub const ERROR_EXCEPTION_IN_RESOURCE_CALL: HRESULT = 5930;

/// The RHS process failed to initialize.
pub const ERROR_CLUSTER_RHS_FAILED_INITIALIZATION: HRESULT = 5931;

/// The Failover Clustering feature is not installed on this node.
pub const ERROR_CLUSTER_NOT_INSTALLED: HRESULT = 5932;

/// The resources must be online on the same node for this operation
pub const ERROR_CLUSTER_RESOURCES_MUST_BE_ONLINE_ON_THE_SAME_NODE: HRESULT = 5933;

/// A new node can not be added since this cluster is already at its maximum number of nodes.
pub const ERROR_CLUSTER_MAX_NODES_IN_CLUSTER: HRESULT = 5934;

/// This cluster can not be created since the specified number of nodes exceeds the maximum allowed limit.
pub const ERROR_CLUSTER_TOO_MANY_NODES: HRESULT = 5935;

/// An attempt to use the specified cluster name failed because an enabled computer object with the given name already exists in the domain.
pub const ERROR_CLUSTER_OBJECT_ALREADY_USED: HRESULT = 5936;

/// This cluster cannot be destroyed. It has non-core application groups which must be deleted before the cluster can be destroyed.
pub const ERROR_NONCORE_GROUPS_FOUND: HRESULT = 5937;

/// File share associated with file share witness resource cannot be hosted by this cluster or any of its nodes.
pub const ERROR_FILE_SHARE_RESOURCE_CONFLICT: HRESULT = 5938;

///  Eviction of this node is invalid at this time. Due to quorum requirements node eviction will result in cluster shutdown.
///
/// If it is the last node in the cluster, destroy cluster command should be used.
pub const ERROR_CLUSTER_EVICT_INVALID_REQUEST: HRESULT = 5939;

/// Only one instance of this resource type is allowed in the cluster.
pub const ERROR_CLUSTER_SINGLETON_RESOURCE: HRESULT = 5940;

/// Only one instance of this resource type is allowed per resource group.
pub const ERROR_CLUSTER_GROUP_SINGLETON_RESOURCE: HRESULT = 5941;

/// The resource failed to come online due to the failure of one or more provider resources.
pub const ERROR_CLUSTER_RESOURCE_PROVIDER_FAILED: HRESULT = 5942;

/// The resource has indicated that it cannot come online on any node.
pub const ERROR_CLUSTER_RESOURCE_CONFIGURATION_ERROR: HRESULT = 5943;

/// The current operation cannot be performed on this group at this time.
pub const ERROR_CLUSTER_GROUP_BUSY: HRESULT = 5944;

/// The directory or file is not located on a cluster shared volume.
pub const ERROR_CLUSTER_NOT_SHARED_VOLUME: HRESULT = 5945;

/// The Security Descriptor does not meet the requirements for a cluster.
pub const ERROR_CLUSTER_INVALID_SECURITY_DESCRIPTOR: HRESULT = 5946;

///  There is one or more shared volumes resources configured in the cluster.
///
/// Those resources must be moved to available storage in order for operation to succeed.
pub const ERROR_CLUSTER_SHARED_VOLUMES_IN_USE: HRESULT = 5947;

///  This group or resource cannot be directly manipulated.
///
/// Use shared volume APIs to perform desired operation.
pub const ERROR_CLUSTER_USE_SHARED_VOLUMES_API: HRESULT = 5948;

/// Back up is in progress. Please wait for backup completion before trying this operation again.
pub const ERROR_CLUSTER_BACKUP_IN_PROGRESS: HRESULT = 5949;

/// The path does not belong to a cluster shared volume.
pub const ERROR_NON_CSV_PATH: HRESULT = 5950;

/// The cluster shared volume is not locally mounted on this node.
pub const ERROR_CSV_VOLUME_NOT_LOCAL: HRESULT = 5951;

/// The cluster watchdog is terminating.
pub const ERROR_CLUSTER_WATCHDOG_TERMINATING: HRESULT = 5952;

/// A resource vetoed a move between two nodes because they are incompatible.
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_INCOMPATIBLE_NODES: HRESULT = 5953;

/// The request is invalid either because node weight cannot be changed while the cluster is in disk-only quorum mode, or because changing the node weight would violate the minimum cluster quorum requirements.
pub const ERROR_CLUSTER_INVALID_NODE_WEIGHT: HRESULT = 5954;

/// The resource vetoed the call.
pub const ERROR_CLUSTER_RESOURCE_VETOED_CALL: HRESULT = 5955;

/// Resource could not start or run because it could not reserve sufficient system resources.
pub const ERROR_RESMON_SYSTEM_RESOURCES_LACKING: HRESULT = 5956;

/// A resource vetoed a move between two nodes because the destination currently does not have enough resources to complete the operation.
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_DESTINATION: HRESULT = 5957;

/// A resource vetoed a move between two nodes because the source currently does not have enough resources to complete the operation.
pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_SOURCE: HRESULT = 5958;

/// The requested operation can not be completed because the group is queued for an operation.
pub const ERROR_CLUSTER_GROUP_QUEUED: HRESULT = 5959;

/// The requested operation can not be completed because a resource has locked status.
pub const ERROR_CLUSTER_RESOURCE_LOCKED_STATUS: HRESULT = 5960;

/// The resource cannot move to another node because a cluster shared volume vetoed the operation.
pub const ERROR_CLUSTER_SHARED_VOLUME_FAILOVER_NOT_ALLOWED: HRESULT = 5961;

/// A node drain is already in progress.
pub const ERROR_CLUSTER_NODE_DRAIN_IN_PROGRESS: HRESULT = 5962;

/// Clustered storage is not connected to the node.
pub const ERROR_CLUSTER_DISK_NOT_CONNECTED: HRESULT = 5963;

/// The disk is not configured in a way to be used with CSV. CSV disks must have at least one partition that is formatted with NTFS or REFS.
pub const ERROR_DISK_NOT_CSV_CAPABLE: HRESULT = 5964;

/// The resource must be part of the Available Storage group to complete this action.
pub const ERROR_RESOURCE_NOT_IN_AVAILABLE_STORAGE: HRESULT = 5965;

/// CSVFS failed operation as volume is in redirected mode.
pub const ERROR_CLUSTER_SHARED_VOLUME_REDIRECTED: HRESULT = 5966;

/// CSVFS failed operation as volume is not in redirected mode.
pub const ERROR_CLUSTER_SHARED_VOLUME_NOT_REDIRECTED: HRESULT = 5967;

/// Cluster properties cannot be returned at this time.
pub const ERROR_CLUSTER_CANNOT_RETURN_PROPERTIES: HRESULT = 5968;

/// The clustered disk resource contains software snapshot diff area that are not supported for Cluster Shared Volumes.
pub const ERROR_CLUSTER_RESOURCE_CONTAINS_UNSUPPORTED_DIFF_AREA_FOR_SHARED_VOLUMES: HRESULT = 5969;

/// The operation cannot be completed because the resource is in maintenance mode.
pub const ERROR_CLUSTER_RESOURCE_IS_IN_MAINTENANCE_MODE: HRESULT = 5970;

/// The operation cannot be completed because of cluster affinity conflicts
pub const ERROR_CLUSTER_AFFINITY_CONFLICT: HRESULT = 5971;

/// The operation cannot be completed because the resource is a replica virtual machine.
pub const ERROR_CLUSTER_RESOURCE_IS_REPLICA_VIRTUAL_MACHINE: HRESULT = 5972;

/// The Cluster Functional Level could not be increased because not all nodes in the cluster support the updated version.
pub const ERROR_CLUSTER_UPGRADE_INCOMPATIBLE_VERSIONS: HRESULT = 5973;

///  Updating the cluster functional level failed because the cluster is running in fix quorum mode.
///
/// Start additional nodes which are members of the cluster until the cluster reaches quorum and the cluster will automatically
///  switch out of fix quorum mode, or stop and restart the cluster without the FixQuorum switch. Once the cluster is out
///
/// of fix quorum mode retry the Update-ClusterFunctionalLevel PowerShell cmdlet to update the cluster functional level.
pub const ERROR_CLUSTER_UPGRADE_FIX_QUORUM_NOT_SUPPORTED: HRESULT = 5974;

///  The cluster functional level has been successfully updated but not all features are available yet. Restart the cluster by
///
/// using the Stop-Cluster PowerShell cmdlet followed by the Start-Cluster PowerShell cmdlet and all cluster features will
/// be available.
pub const ERROR_CLUSTER_UPGRADE_RESTART_REQUIRED: HRESULT = 5975;

/// The cluster is currently performing a version upgrade.
pub const ERROR_CLUSTER_UPGRADE_IN_PROGRESS: HRESULT = 5976;

/// The cluster did not successfully complete the version upgrade.
pub const ERROR_CLUSTER_UPGRADE_INCOMPLETE: HRESULT = 5977;

/// The cluster node is in grace period.
pub const ERROR_CLUSTER_NODE_IN_GRACE_PERIOD: HRESULT = 5978;

/// The operation has failed because CSV volume was not able to recover in time specified on this file object.
pub const ERROR_CLUSTER_CSV_IO_PAUSE_TIMEOUT: HRESULT = 5979;

/// The operation failed because the requested node is not currently part of active cluster membership.
pub const ERROR_NODE_NOT_ACTIVE_CLUSTER_MEMBER: HRESULT = 5980;

/// The operation failed because the requested cluster resource is currently unmonitored.
pub const ERROR_CLUSTER_RESOURCE_NOT_MONITORED: HRESULT = 5981;

/// The operation failed because a resource does not support running in an unmonitored state.
pub const ERROR_CLUSTER_RESOURCE_DOES_NOT_SUPPORT_UNMONITORED: HRESULT = 5982;

/// The operation cannot be completed because a resource participates in replication.
pub const ERROR_CLUSTER_RESOURCE_IS_REPLICATED: HRESULT = 5983;

/// The operation failed because the requested cluster node has been isolated
pub const ERROR_CLUSTER_NODE_ISOLATED: HRESULT = 5984;

/// The operation failed because the requested cluster node has been quarantined
pub const ERROR_CLUSTER_NODE_QUARANTINED: HRESULT = 5985;

/// The operation failed because the specified database update condition was not met
pub const ERROR_CLUSTER_DATABASE_UPDATE_CONDITION_FAILED: HRESULT = 5986;

/// A clustered space is in a degraded condition and the requested action cannot be completed at this time.
pub const ERROR_CLUSTER_SPACE_DEGRADED: HRESULT = 5987;

/// The operation failed because token delegation for this control is not supported.
pub const ERROR_CLUSTER_TOKEN_DELEGATION_NOT_SUPPORTED: HRESULT = 5988;

/// The operation has failed because CSV has invalidated this file object.
pub const ERROR_CLUSTER_CSV_INVALID_HANDLE: HRESULT = 5989;

/// This operation is supported only on the CSV coordinator node.
pub const ERROR_CLUSTER_CSV_SUPPORTED_ONLY_ON_COORDINATOR: HRESULT = 5990;

/// The cluster group set is not available for any further requests.
pub const ERROR_GROUPSET_NOT_AVAILABLE: HRESULT = 5991;

/// The cluster group set could not be found.
pub const ERROR_GROUPSET_NOT_FOUND: HRESULT = 5992;

/// The action cannot be completed at this time because the cluster group set would fall below quorum and not be able to act as a provider.
pub const ERROR_GROUPSET_CANT_PROVIDE: HRESULT = 5993;

/// The specified parent fault domain is not found.
pub const ERROR_CLUSTER_FAULT_DOMAIN_PARENT_NOT_FOUND: HRESULT = 5994;

/// The fault domain cannot be a child of the parent specified.
pub const ERROR_CLUSTER_FAULT_DOMAIN_INVALID_HIERARCHY: HRESULT = 5995;

/// Storage Spaces Direct has rejected the proposed fault domain changes because it impacts the fault tolerance of the storage.
pub const ERROR_CLUSTER_FAULT_DOMAIN_FAILED_S2D_VALIDATION: HRESULT = 5996;

/// Storage Spaces Direct has rejected the proposed fault domain changes because it reduces the storage connected to the system.
pub const ERROR_CLUSTER_FAULT_DOMAIN_S2D_CONNECTIVITY_LOSS: HRESULT = 5997;

/// Cluster infrastructure file server creation failed because a valid non-empty file server name was not provided.
pub const ERROR_CLUSTER_INVALID_INFRASTRUCTURE_FILESERVER_NAME: HRESULT = 5998;

/// The action cannot be completed because the cluster set management cluster is unreachable.
pub const ERROR_CLUSTERSET_MANAGEMENT_CLUSTER_UNREACHABLE: HRESULT = 5999;

/// The specified file could not be encrypted.
pub const ERROR_ENCRYPTION_FAILED: HRESULT = 6000;

/// The specified file could not be decrypted.
pub const ERROR_DECRYPTION_FAILED: HRESULT = 6001;

/// The specified file is encrypted and the user does not have the ability to decrypt it.
pub const ERROR_FILE_ENCRYPTED: HRESULT = 6002;

/// There is no valid encryption recovery policy configured for this system.
pub const ERROR_NO_RECOVERY_POLICY: HRESULT = 6003;

/// The required encryption driver is not loaded for this system.
pub const ERROR_NO_EFS: HRESULT = 6004;

/// The file was encrypted with a different encryption driver than is currently loaded.
pub const ERROR_WRONG_EFS: HRESULT = 6005;

/// There are no EFS keys defined for the user.
pub const ERROR_NO_USER_KEYS: HRESULT = 6006;

/// The specified file is not encrypted.
pub const ERROR_FILE_NOT_ENCRYPTED: HRESULT = 6007;

/// The specified file is not in the defined EFS export format.
pub const ERROR_NOT_EXPORT_FORMAT: HRESULT = 6008;

/// The specified file is read only.
pub const ERROR_FILE_READ_ONLY: HRESULT = 6009;

/// The directory has been disabled for encryption.
pub const ERROR_DIR_EFS_DISALLOWED: HRESULT = 6010;

/// The server is not trusted for remote encryption operation.
pub const ERROR_EFS_SERVER_NOT_TRUSTED: HRESULT = 6011;

/// Recovery policy configured for this system contains invalid recovery certificate.
pub const ERROR_BAD_RECOVERY_POLICY: HRESULT = 6012;

/// The encryption algorithm used on the source file needs a bigger key buffer than the one on the destination file.
pub const ERROR_EFS_ALG_BLOB_TOO_BIG: HRESULT = 6013;

/// The disk partition does not support file encryption.
pub const ERROR_VOLUME_NOT_SUPPORT_EFS: HRESULT = 6014;

/// This machine is disabled for file encryption.
pub const ERROR_EFS_DISABLED: HRESULT = 6015;

/// A newer system is required to decrypt this encrypted file.
pub const ERROR_EFS_VERSION_NOT_SUPPORT: HRESULT = 6016;

/// The remote server sent an invalid response for a file being opened with Client Side Encryption.
pub const ERROR_CS_ENCRYPTION_INVALID_SERVER_RESPONSE: HRESULT = 6017;

/// Client Side Encryption is not supported by the remote server even though it claims to support it.
pub const ERROR_CS_ENCRYPTION_UNSUPPORTED_SERVER: HRESULT = 6018;

/// File is encrypted and should be opened in Client Side Encryption mode.
pub const ERROR_CS_ENCRYPTION_EXISTING_ENCRYPTED_FILE: HRESULT = 6019;

/// A new encrypted file is being created and a $EFS needs to be provided.
pub const ERROR_CS_ENCRYPTION_NEW_ENCRYPTED_FILE: HRESULT = 6020;

/// The SMB client requested a CSE FSCTL on a non-CSE file.
pub const ERROR_CS_ENCRYPTION_FILE_NOT_CSE: HRESULT = 6021;

/// The requested operation was blocked by policy. For more information, contact your system administrator.
pub const ERROR_ENCRYPTION_POLICY_DENIES_OPERATION: HRESULT = 6022;

/// The specified file could not be encrypted with Windows Information Protection.
pub const ERROR_WIP_ENCRYPTION_FAILED: HRESULT = 6023;

/// The list of servers for this workgroup is not currently available
pub const ERROR_NO_BROWSER_SERVERS_FOUND: HRESULT = 6118;

/// The Task Scheduler service must be configured to run in the System account to function properly. Individual tasks may be configured to run in other accounts.
pub const SCHED_E_SERVICE_NOT_LOCALSYSTEM: HRESULT = 6200;

/// The object cannot be deleted from the local cluster because it is registered with the cluster set.
pub const ERROR_CLUSTER_OBJECT_IS_CLUSTER_SET_VM: HRESULT = 6250;

/// Log service encountered an invalid log sector.
pub const ERROR_LOG_SECTOR_INVALID: HRESULT = 6600;

/// Log service encountered a log sector with invalid block parity.
pub const ERROR_LOG_SECTOR_PARITY_INVALID: HRESULT = 6601;

/// Log service encountered a remapped log sector.
pub const ERROR_LOG_SECTOR_REMAPPED: HRESULT = 6602;

/// Log service encountered a partial or incomplete log block.
pub const ERROR_LOG_BLOCK_INCOMPLETE: HRESULT = 6603;

/// Log service encountered an attempt access data outside the active log range.
pub const ERROR_LOG_INVALID_RANGE: HRESULT = 6604;

/// Log service user marshalling buffers are exhausted.
pub const ERROR_LOG_BLOCKS_EXHAUSTED: HRESULT = 6605;

/// Log service encountered an attempt read from a marshalling area with an invalid read context.
pub const ERROR_LOG_READ_CONTEXT_INVALID: HRESULT = 6606;

/// Log service encountered an invalid log restart area.
pub const ERROR_LOG_RESTART_INVALID: HRESULT = 6607;

/// Log service encountered an invalid log block version.
pub const ERROR_LOG_BLOCK_VERSION: HRESULT = 6608;

/// Log service encountered an invalid log block.
pub const ERROR_LOG_BLOCK_INVALID: HRESULT = 6609;

/// Log service encountered an attempt to read the log with an invalid read mode.
pub const ERROR_LOG_READ_MODE_INVALID: HRESULT = 6610;

/// Log service encountered a log stream with no restart area.
pub const ERROR_LOG_NO_RESTART: HRESULT = 6611;

/// Log service encountered a corrupted metadata file.
pub const ERROR_LOG_METADATA_CORRUPT: HRESULT = 6612;

/// Log service encountered a metadata file that could not be created by the log file system.
pub const ERROR_LOG_METADATA_INVALID: HRESULT = 6613;

/// Log service encountered a metadata file with inconsistent data.
pub const ERROR_LOG_METADATA_INCONSISTENT: HRESULT = 6614;

/// Log service encountered an attempt to erroneous allocate or dispose reservation space.
pub const ERROR_LOG_RESERVATION_INVALID: HRESULT = 6615;

/// Log service cannot delete log file or file system container.
pub const ERROR_LOG_CANT_DELETE: HRESULT = 6616;

/// Log service has reached the maximum allowable containers allocated to a log file.
pub const ERROR_LOG_CONTAINER_LIMIT_EXCEEDED: HRESULT = 6617;

/// Log service has attempted to read or write backward past the start of the log.
pub const ERROR_LOG_START_OF_LOG: HRESULT = 6618;

/// Log policy could not be installed because a policy of the same type is already present.
pub const ERROR_LOG_POLICY_ALREADY_INSTALLED: HRESULT = 6619;

/// Log policy in question was not installed at the time of the request.
pub const ERROR_LOG_POLICY_NOT_INSTALLED: HRESULT = 6620;

/// The installed set of policies on the log is invalid.
pub const ERROR_LOG_POLICY_INVALID: HRESULT = 6621;

/// A policy on the log in question prevented the operation from completing.
pub const ERROR_LOG_POLICY_CONFLICT: HRESULT = 6622;

/// Log space cannot be reclaimed because the log is pinned by the archive tail.
pub const ERROR_LOG_PINNED_ARCHIVE_TAIL: HRESULT = 6623;

/// Log record is not a record in the log file.
pub const ERROR_LOG_RECORD_NONEXISTENT: HRESULT = 6624;

/// Number of reserved log records or the adjustment of the number of reserved log records is invalid.
pub const ERROR_LOG_RECORDS_RESERVED_INVALID: HRESULT = 6625;

/// Reserved log space or the adjustment of the log space is invalid.
pub const ERROR_LOG_SPACE_RESERVED_INVALID: HRESULT = 6626;

/// An new or existing archive tail or base of the active log is invalid.
pub const ERROR_LOG_TAIL_INVALID: HRESULT = 6627;

/// Log space is exhausted.
pub const ERROR_LOG_FULL: HRESULT = 6628;

/// The log could not be set to the requested size.
pub const ERROR_COULD_NOT_RESIZE_LOG: HRESULT = 6629;

/// Log is multiplexed, no direct writes to the physical log is allowed.
pub const ERROR_LOG_MULTIPLEXED: HRESULT = 6630;

/// The operation failed because the log is a dedicated log.
pub const ERROR_LOG_DEDICATED: HRESULT = 6631;

/// The operation requires an archive context.
pub const ERROR_LOG_ARCHIVE_NOT_IN_PROGRESS: HRESULT = 6632;

/// Log archival is in progress.
pub const ERROR_LOG_ARCHIVE_IN_PROGRESS: HRESULT = 6633;

/// The operation requires a non-ephemeral log, but the log is ephemeral.
pub const ERROR_LOG_EPHEMERAL: HRESULT = 6634;

/// The log must have at least two containers before it can be read from or written to.
pub const ERROR_LOG_NOT_ENOUGH_CONTAINERS: HRESULT = 6635;

/// A log client has already registered on the stream.
pub const ERROR_LOG_CLIENT_ALREADY_REGISTERED: HRESULT = 6636;

/// A log client has not been registered on the stream.
pub const ERROR_LOG_CLIENT_NOT_REGISTERED: HRESULT = 6637;

/// A request has already been made to handle the log full condition.
pub const ERROR_LOG_FULL_HANDLER_IN_PROGRESS: HRESULT = 6638;

/// Log service encountered an error when attempting to read from a log container.
pub const ERROR_LOG_CONTAINER_READ_FAILED: HRESULT = 6639;

/// Log service encountered an error when attempting to write to a log container.
pub const ERROR_LOG_CONTAINER_WRITE_FAILED: HRESULT = 6640;

/// Log service encountered an error when attempting open a log container.
pub const ERROR_LOG_CONTAINER_OPEN_FAILED: HRESULT = 6641;

/// Log service encountered an invalid container state when attempting a requested action.
pub const ERROR_LOG_CONTAINER_STATE_INVALID: HRESULT = 6642;

/// Log service is not in the correct state to perform a requested action.
pub const ERROR_LOG_STATE_INVALID: HRESULT = 6643;

/// Log space cannot be reclaimed because the log is pinned.
pub const ERROR_LOG_PINNED: HRESULT = 6644;

/// Log metadata flush failed.
pub const ERROR_LOG_METADATA_FLUSH_FAILED: HRESULT = 6645;

/// Security on the log and its containers is inconsistent.
pub const ERROR_LOG_INCONSISTENT_SECURITY: HRESULT = 6646;

/// Records were appended to the log or reservation changes were made, but the log could not be flushed.
pub const ERROR_LOG_APPENDED_FLUSH_FAILED: HRESULT = 6647;

/// The log is pinned due to reservation consuming most of the log space. Free some reserved records to make space available.
pub const ERROR_LOG_PINNED_RESERVATION: HRESULT = 6648;

/// The transaction handle associated with this operation is not valid.
pub const ERROR_INVALID_TRANSACTION: HRESULT = 6700;

/// The requested operation was made in the context of a transaction that is no longer active.
pub const ERROR_TRANSACTION_NOT_ACTIVE: HRESULT = 6701;

/// The requested operation is not valid on the Transaction object in its current state.
pub const ERROR_TRANSACTION_REQUEST_NOT_VALID: HRESULT = 6702;

/// The caller has called a response API, but the response is not expected because the TM did not issue the corresponding request to the caller.
pub const ERROR_TRANSACTION_NOT_REQUESTED: HRESULT = 6703;

/// It is too late to perform the requested operation, since the Transaction has already been aborted.
pub const ERROR_TRANSACTION_ALREADY_ABORTED: HRESULT = 6704;

/// It is too late to perform the requested operation, since the Transaction has already been committed.
pub const ERROR_TRANSACTION_ALREADY_COMMITTED: HRESULT = 6705;

/// The Transaction Manager was unable to be successfully initialized. Transacted operations are not supported.
pub const ERROR_TM_INITIALIZATION_FAILED: HRESULT = 6706;

/// The specified ResourceManager made no changes or updates to the resource under this transaction.
pub const ERROR_RESOURCEMANAGER_READ_ONLY: HRESULT = 6707;

/// The resource manager has attempted to prepare a transaction that it has not successfully joined.
pub const ERROR_TRANSACTION_NOT_JOINED: HRESULT = 6708;

/// The Transaction object already has a superior enlistment, and the caller attempted an operation that would have created a new superior. Only a single superior enlistment is allow.
pub const ERROR_TRANSACTION_SUPERIOR_EXISTS: HRESULT = 6709;

/// The RM tried to register a protocol that already exists.
pub const ERROR_CRM_PROTOCOL_ALREADY_EXISTS: HRESULT = 6710;

/// The attempt to propagate the Transaction failed.
pub const ERROR_TRANSACTION_PROPAGATION_FAILED: HRESULT = 6711;

/// The requested propagation protocol was not registered as a CRM.
pub const ERROR_CRM_PROTOCOL_NOT_FOUND: HRESULT = 6712;

/// The buffer passed in to PushTransaction or PullTransaction is not in a valid format.
pub const ERROR_TRANSACTION_INVALID_MARSHALL_BUFFER: HRESULT = 6713;

/// The current transaction context associated with the thread is not a valid handle to a transaction object.
pub const ERROR_CURRENT_TRANSACTION_NOT_VALID: HRESULT = 6714;

/// The specified Transaction object could not be opened, because it was not found.
pub const ERROR_TRANSACTION_NOT_FOUND: HRESULT = 6715;

/// The specified ResourceManager object could not be opened, because it was not found.
pub const ERROR_RESOURCEMANAGER_NOT_FOUND: HRESULT = 6716;

/// The specified Enlistment object could not be opened, because it was not found.
pub const ERROR_ENLISTMENT_NOT_FOUND: HRESULT = 6717;

/// The specified TransactionManager object could not be opened, because it was not found.
pub const ERROR_TRANSACTIONMANAGER_NOT_FOUND: HRESULT = 6718;

/// The object specified could not be created or opened, because its associated TransactionManager is not online.  The TransactionManager must be brought fully Online by calling RecoverTransactionManager to recover to the end of its LogFile before objects in its Transaction or ResourceManager namespaces can be opened.  In addition, errors in writing records to its LogFile can cause a TransactionManager to go offline.
pub const ERROR_TRANSACTIONMANAGER_NOT_ONLINE: HRESULT = 6719;

/// The specified TransactionManager was unable to create the objects contained in its logfile in the Ob namespace. Therefore, the TransactionManager was unable to recover.
pub const ERROR_TRANSACTIONMANAGER_RECOVERY_NAME_COLLISION: HRESULT = 6720;

/// The call to create a superior Enlistment on this Transaction object could not be completed, because the Transaction object specified for the enlistment is a subordinate branch of the Transaction. Only the root of the Transaction can be enlisted on as a superior.
pub const ERROR_TRANSACTION_NOT_ROOT: HRESULT = 6721;

/// Because the associated transaction manager or resource manager has been closed, the handle is no longer valid.
pub const ERROR_TRANSACTION_OBJECT_EXPIRED: HRESULT = 6722;

/// The specified operation could not be performed on this Superior enlistment, because the enlistment was not created with the corresponding completion response in the NotificationMask.
pub const ERROR_TRANSACTION_RESPONSE_NOT_ENLISTED: HRESULT = 6723;

/// The specified operation could not be performed, because the record that would be logged was too long. This can occur because of two conditions: either there are too many Enlistments on this Transaction, or the combined RecoveryInformation being logged on behalf of those Enlistments is too long.
pub const ERROR_TRANSACTION_RECORD_TOO_LONG: HRESULT = 6724;

/// Implicit transaction are not supported.
pub const ERROR_IMPLICIT_TRANSACTION_NOT_SUPPORTED: HRESULT = 6725;

/// The kernel transaction manager had to abort or forget the transaction because it blocked forward progress.
pub const ERROR_TRANSACTION_INTEGRITY_VIOLATED: HRESULT = 6726;

/// The TransactionManager identity that was supplied did not match the one recorded in the TransactionManager's log file.
pub const ERROR_TRANSACTIONMANAGER_IDENTITY_MISMATCH: HRESULT = 6727;

/// This snapshot operation cannot continue because a transactional resource manager cannot be frozen in its current state.  Please try again.
pub const ERROR_RM_CANNOT_BE_FROZEN_FOR_SNAPSHOT: HRESULT = 6728;

/// The transaction cannot be enlisted on with the specified EnlistmentMask, because the transaction has already completed the PrePrepare phase.  In order to ensure correctness, the ResourceManager must switch to a write-through mode and cease caching data within this transaction.  Enlisting for only subsequent transaction phases may still succeed.
pub const ERROR_TRANSACTION_MUST_WRITETHROUGH: HRESULT = 6729;

/// The transaction does not have a superior enlistment.
pub const ERROR_TRANSACTION_NO_SUPERIOR: HRESULT = 6730;

/// The attempt to commit the Transaction completed, but it is possible that some portion of the transaction tree did not commit successfully due to heuristics.  Therefore it is possible that some data modified in the transaction may not have committed, resulting in transactional inconsistency.  If possible, check the consistency of the associated data.
pub const ERROR_HEURISTIC_DAMAGE_POSSIBLE: HRESULT = 6731;

/// The function attempted to use a name that is reserved for use by another transaction.
pub const ERROR_TRANSACTIONAL_CONFLICT: HRESULT = 6800;

/// Transaction support within the specified resource manager is not started or was shut down due to an error.
pub const ERROR_RM_NOT_ACTIVE: HRESULT = 6801;

/// The metadata of the RM has been corrupted. The RM will not function.
pub const ERROR_RM_METADATA_CORRUPT: HRESULT = 6802;

/// The specified directory does not contain a resource manager.
pub const ERROR_DIRECTORY_NOT_RM: HRESULT = 6803;

/// The remote server or share does not support transacted file operations.
pub const ERROR_TRANSACTIONS_UNSUPPORTED_REMOTE: HRESULT = 6805;

/// The requested log size is invalid.
pub const ERROR_LOG_RESIZE_INVALID_SIZE: HRESULT = 6806;

/// The object (file, stream, link) corresponding to the handle has been deleted by a Transaction Savepoint Rollback.
pub const ERROR_OBJECT_NO_LONGER_EXISTS: HRESULT = 6807;

/// The specified file miniversion was not found for this transacted file open.
pub const ERROR_STREAM_MINIVERSION_NOT_FOUND: HRESULT = 6808;

/// The specified file miniversion was found but has been invalidated. Most likely cause is a transaction savepoint rollback.
pub const ERROR_STREAM_MINIVERSION_NOT_VALID: HRESULT = 6809;

/// A miniversion may only be opened in the context of the transaction that created it.
pub const ERROR_MINIVERSION_INACCESSIBLE_FROM_SPECIFIED_TRANSACTION: HRESULT = 6810;

/// It is not possible to open a miniversion with modify access.
pub const ERROR_CANT_OPEN_MINIVERSION_WITH_MODIFY_INTENT: HRESULT = 6811;

/// It is not possible to create any more miniversions for this stream.
pub const ERROR_CANT_CREATE_MORE_STREAM_MINIVERSIONS: HRESULT = 6812;

/// The remote server sent mismatching version number or Fid for a file opened with transactions.
pub const ERROR_REMOTE_FILE_VERSION_MISMATCH: HRESULT = 6814;

/// The handle has been invalidated by a transaction. The most likely cause is the presence of memory mapping on a file or an open handle when the transaction ended or rolled back to savepoint.
pub const ERROR_HANDLE_NO_LONGER_VALID: HRESULT = 6815;

/// There is no transaction metadata on the file.
pub const ERROR_NO_TXF_METADATA: HRESULT = 6816;

/// The log data is corrupt.
pub const ERROR_LOG_CORRUPTION_DETECTED: HRESULT = 6817;

/// The file can't be recovered because there is a handle still open on it.
pub const ERROR_CANT_RECOVER_WITH_HANDLE_OPEN: HRESULT = 6818;

/// The transaction outcome is unavailable because the resource manager responsible for it has disconnected.
pub const ERROR_RM_DISCONNECTED: HRESULT = 6819;

/// The request was rejected because the enlistment in question is not a superior enlistment.
pub const ERROR_ENLISTMENT_NOT_SUPERIOR: HRESULT = 6820;

/// The transactional resource manager is already consistent. Recovery is not needed.
pub const ERROR_RECOVERY_NOT_NEEDED: HRESULT = 6821;

/// The transactional resource manager has already been started.
pub const ERROR_RM_ALREADY_STARTED: HRESULT = 6822;

/// The file cannot be opened transactionally, because its identity depends on the outcome of an unresolved transaction.
pub const ERROR_FILE_IDENTITY_NOT_PERSISTENT: HRESULT = 6823;

/// The operation cannot be performed because another transaction is depending on the fact that this property will not change.
pub const ERROR_CANT_BREAK_TRANSACTIONAL_DEPENDENCY: HRESULT = 6824;

/// The operation would involve a single file with two transactional resource managers and is therefore not allowed.
pub const ERROR_CANT_CROSS_RM_BOUNDARY: HRESULT = 6825;

/// The $Txf directory must be empty for this operation to succeed.
pub const ERROR_TXF_DIR_NOT_EMPTY: HRESULT = 6826;

/// The operation would leave a transactional resource manager in an inconsistent state and is therefore not allowed.
pub const ERROR_INDOUBT_TRANSACTIONS_EXIST: HRESULT = 6827;

/// The operation could not be completed because the transaction manager does not have a log.
pub const ERROR_TM_VOLATILE: HRESULT = 6828;

/// A rollback could not be scheduled because a previously scheduled rollback has already executed or been queued for execution.
pub const ERROR_ROLLBACK_TIMER_EXPIRED: HRESULT = 6829;

/// The transactional metadata attribute on the file or directory is corrupt and unreadable.
pub const ERROR_TXF_ATTRIBUTE_CORRUPT: HRESULT = 6830;

/// The encryption operation could not be completed because a transaction is active.
pub const ERROR_EFS_NOT_ALLOWED_IN_TRANSACTION: HRESULT = 6831;

/// This object is not allowed to be opened in a transaction.
pub const ERROR_TRANSACTIONAL_OPEN_NOT_ALLOWED: HRESULT = 6832;

/// An attempt to create space in the transactional resource manager's log failed. The failure status has been recorded in the event log.
pub const ERROR_LOG_GROWTH_FAILED: HRESULT = 6833;

/// Memory mapping (creating a mapped section) a remote file under a transaction is not supported.
pub const ERROR_TRANSACTED_MAPPING_UNSUPPORTED_REMOTE: HRESULT = 6834;

/// Transaction metadata is already present on this file and cannot be superseded.
pub const ERROR_TXF_METADATA_ALREADY_PRESENT: HRESULT = 6835;

/// A transaction scope could not be entered because the scope handler has not been initialized.
pub const ERROR_TRANSACTION_SCOPE_CALLBACKS_NOT_SET: HRESULT = 6836;

/// Promotion was required in order to allow the resource manager to enlist, but the transaction was set to disallow it.
pub const ERROR_TRANSACTION_REQUIRED_PROMOTION: HRESULT = 6837;

/// This file is open for modification in an unresolved transaction and may be opened for execute only by a transacted reader.
pub const ERROR_CANNOT_EXECUTE_FILE_IN_TRANSACTION: HRESULT = 6838;

/// The request to thaw frozen transactions was ignored because transactions had not previously been frozen.
pub const ERROR_TRANSACTIONS_NOT_FROZEN: HRESULT = 6839;

/// Transactions cannot be frozen because a freeze is already in progress.
pub const ERROR_TRANSACTION_FREEZE_IN_PROGRESS: HRESULT = 6840;

/// The target volume is not a snapshot volume. This operation is only valid on a volume mounted as a snapshot.
pub const ERROR_NOT_SNAPSHOT_VOLUME: HRESULT = 6841;

/// The savepoint operation failed because files are open on the transaction. This is not permitted.
pub const ERROR_NO_SAVEPOINT_WITH_OPEN_FILES: HRESULT = 6842;

/// Windows has discovered corruption in a file, and that file has since been repaired. Data loss may have occurred.
pub const ERROR_DATA_LOST_REPAIR: HRESULT = 6843;

/// The sparse operation could not be completed because a transaction is active on the file.
pub const ERROR_SPARSE_NOT_ALLOWED_IN_TRANSACTION: HRESULT = 6844;

/// The call to create a TransactionManager object failed because the Tm Identity stored in the logfile does not match the Tm Identity that was passed in as an argument.
pub const ERROR_TM_IDENTITY_MISMATCH: HRESULT = 6845;

/// I/O was attempted on a section object that has been floated as a result of a transaction ending. There is no valid data.
pub const ERROR_FLOATED_SECTION: HRESULT = 6846;

/// The transactional resource manager cannot currently accept transacted work due to a transient condition such as low resources.
pub const ERROR_CANNOT_ACCEPT_TRANSACTED_WORK: HRESULT = 6847;

/// The transactional resource manager had too many transactions outstanding that could not be aborted. The transactional resource manger has been shut down.
pub const ERROR_CANNOT_ABORT_TRANSACTIONS: HRESULT = 6848;

/// The operation could not be completed due to bad clusters on disk.
pub const ERROR_BAD_CLUSTERS: HRESULT = 6849;

/// The compression operation could not be completed because a transaction is active on the file.
pub const ERROR_COMPRESSION_NOT_ALLOWED_IN_TRANSACTION: HRESULT = 6850;

/// The operation could not be completed because the volume is dirty. Please run chkdsk and try again.
pub const ERROR_VOLUME_DIRTY: HRESULT = 6851;

/// The link tracking operation could not be completed because a transaction is active.
pub const ERROR_NO_LINK_TRACKING_IN_TRANSACTION: HRESULT = 6852;

/// This operation cannot be performed in a transaction.
pub const ERROR_OPERATION_NOT_SUPPORTED_IN_TRANSACTION: HRESULT = 6853;

/// The handle is no longer properly associated with its transaction.  It may have been opened in a transactional resource manager that was subsequently forced to restart.  Please close the handle and open a new one.
pub const ERROR_EXPIRED_HANDLE: HRESULT = 6854;

/// The specified operation could not be performed because the resource manager is not enlisted in the transaction.
pub const ERROR_TRANSACTION_NOT_ENLISTED: HRESULT = 6855;

/// The specified session name is invalid.
pub const ERROR_CTX_WINSTATION_NAME_INVALID: HRESULT = 7001;

/// The specified protocol driver is invalid.
pub const ERROR_CTX_INVALID_PD: HRESULT = 7002;

/// The specified protocol driver was not found in the system path.
pub const ERROR_CTX_PD_NOT_FOUND: HRESULT = 7003;

/// The specified terminal connection driver was not found in the system path.
pub const ERROR_CTX_WD_NOT_FOUND: HRESULT = 7004;

/// A registry key for event logging could not be created for this session.
pub const ERROR_CTX_CANNOT_MAKE_EVENTLOG_ENTRY: HRESULT = 7005;

/// A service with the same name already exists on the system.
pub const ERROR_CTX_SERVICE_NAME_COLLISION: HRESULT = 7006;

/// A close operation is pending on the session.
pub const ERROR_CTX_CLOSE_PENDING: HRESULT = 7007;

/// There are no free output buffers available.
pub const ERROR_CTX_NO_OUTBUF: HRESULT = 7008;

/// The MODEM.INF file was not found.
pub const ERROR_CTX_MODEM_INF_NOT_FOUND: HRESULT = 7009;

/// The modem name was not found in MODEM.INF.
pub const ERROR_CTX_INVALID_MODEMNAME: HRESULT = 7010;

/// The modem did not accept the command sent to it. Verify that the configured modem name matches the attached modem.
pub const ERROR_CTX_MODEM_RESPONSE_ERROR: HRESULT = 7011;

/// The modem did not respond to the command sent to it. Verify that the modem is properly cabled and powered on.
pub const ERROR_CTX_MODEM_RESPONSE_TIMEOUT: HRESULT = 7012;

/// Carrier detect has failed or carrier has been dropped due to disconnect.
pub const ERROR_CTX_MODEM_RESPONSE_NO_CARRIER: HRESULT = 7013;

/// Dial tone not detected within the required time. Verify that the phone cable is properly attached and functional.
pub const ERROR_CTX_MODEM_RESPONSE_NO_DIALTONE: HRESULT = 7014;

/// Busy signal detected at remote site on callback.
pub const ERROR_CTX_MODEM_RESPONSE_BUSY: HRESULT = 7015;

/// Voice detected at remote site on callback.
pub const ERROR_CTX_MODEM_RESPONSE_VOICE: HRESULT = 7016;

/// Transport driver error
pub const ERROR_CTX_TD_ERROR: HRESULT = 7017;

/// The specified session cannot be found.
pub const ERROR_CTX_WINSTATION_NOT_FOUND: HRESULT = 7022;

/// The specified session name is already in use.
pub const ERROR_CTX_WINSTATION_ALREADY_EXISTS: HRESULT = 7023;

/// The task you are trying to do can't be completed because Remote Desktop Services is currently busy. Please try again in a few minutes. Other users should still be able to log on.
pub const ERROR_CTX_WINSTATION_BUSY: HRESULT = 7024;

/// An attempt has been made to connect to a session whose video mode is not supported by the current client.
pub const ERROR_CTX_BAD_VIDEO_MODE: HRESULT = 7025;

/// The application attempted to enable DOS graphics mode. DOS graphics mode is not supported.
pub const ERROR_CTX_GRAPHICS_INVALID: HRESULT = 7035;

/// Your interactive logon privilege has been disabled. Please contact your administrator.
pub const ERROR_CTX_LOGON_DISABLED: HRESULT = 7037;

/// The requested operation can be performed only on the system console. This is most often the result of a driver or system DLL requiring direct console access.
pub const ERROR_CTX_NOT_CONSOLE: HRESULT = 7038;

/// The client failed to respond to the server connect message.
pub const ERROR_CTX_CLIENT_QUERY_TIMEOUT: HRESULT = 7040;

/// Disconnecting the console session is not supported.
pub const ERROR_CTX_CONSOLE_DISCONNECT: HRESULT = 7041;

/// Reconnecting a disconnected session to the console is not supported.
pub const ERROR_CTX_CONSOLE_CONNECT: HRESULT = 7042;

/// The request to control another session remotely was denied.
pub const ERROR_CTX_SHADOW_DENIED: HRESULT = 7044;

/// The requested session access is denied.
pub const ERROR_CTX_WINSTATION_ACCESS_DENIED: HRESULT = 7045;

/// The specified terminal connection driver is invalid.
pub const ERROR_CTX_INVALID_WD: HRESULT = 7049;

///  The requested session cannot be controlled remotely.
///
/// This may be because the session is disconnected or does not currently have a user logged on.
pub const ERROR_CTX_SHADOW_INVALID: HRESULT = 7050;

/// The requested session is not configured to allow remote control.
pub const ERROR_CTX_SHADOW_DISABLED: HRESULT = 7051;

/// Your request to connect to this Terminal Server has been rejected. Your Terminal Server client license number is currently being used by another user. Please call your system administrator to obtain a unique license number.
pub const ERROR_CTX_CLIENT_LICENSE_IN_USE: HRESULT = 7052;

/// Your request to connect to this Terminal Server has been rejected. Your Terminal Server client license number has not been entered for this copy of the Terminal Server client. Please contact your system administrator.
pub const ERROR_CTX_CLIENT_LICENSE_NOT_SET: HRESULT = 7053;

/// The number of connections to this computer is limited and all connections are in use right now. Try connecting later or contact your system administrator.
pub const ERROR_CTX_LICENSE_NOT_AVAILABLE: HRESULT = 7054;

/// The client you are using is not licensed to use this system. Your logon request is denied.
pub const ERROR_CTX_LICENSE_CLIENT_INVALID: HRESULT = 7055;

/// The system license has expired. Your logon request is denied.
pub const ERROR_CTX_LICENSE_EXPIRED: HRESULT = 7056;

/// Remote control could not be terminated because the specified session is not currently being remotely controlled.
pub const ERROR_CTX_SHADOW_NOT_RUNNING: HRESULT = 7057;

/// The remote control of the console was terminated because the display mode was changed. Changing the display mode in a remote control session is not supported.
pub const ERROR_CTX_SHADOW_ENDED_BY_MODE_CHANGE: HRESULT = 7058;

/// Activation has already been reset the maximum number of times for this installation. Your activation timer will not be cleared.
pub const ERROR_ACTIVATION_COUNT_EXCEEDED: HRESULT = 7059;

/// Remote logins are currently disabled.
pub const ERROR_CTX_WINSTATIONS_DISABLED: HRESULT = 7060;

/// You do not have the proper encryption level to access this Session.
pub const ERROR_CTX_ENCRYPTION_LEVEL_REQUIRED: HRESULT = 7061;

/// The user %s\\%s is currently logged on to this computer. Only the current user or an administrator can log on to this computer.
pub const ERROR_CTX_SESSION_IN_USE: HRESULT = 7062;

/// The user %s\\%s is already logged on to the console of this computer. You do not have permission to log in at this time. To resolve this issue, contact %s\\%s and have them log off.
pub const ERROR_CTX_NO_FORCE_LOGOFF: HRESULT = 7063;

/// Unable to log you on because of an account restriction.
pub const ERROR_CTX_ACCOUNT_RESTRICTION: HRESULT = 7064;

/// The RDP protocol component %2 detected an error in the protocol stream and has disconnected the client.
pub const ERROR_RDP_PROTOCOL_ERROR: HRESULT = 7065;

/// The Client Drive Mapping Service Has Connected on Terminal Connection.
pub const ERROR_CTX_CDM_CONNECT: HRESULT = 7066;

/// The Client Drive Mapping Service Has Disconnected on Terminal Connection.
pub const ERROR_CTX_CDM_DISCONNECT: HRESULT = 7067;

/// The Terminal Server security layer detected an error in the protocol stream and has disconnected the client.
pub const ERROR_CTX_SECURITY_LAYER_ERROR: HRESULT = 7068;

/// The target session is incompatible with the current session.
pub const ERROR_TS_INCOMPATIBLE_SESSIONS: HRESULT = 7069;

/// Windows can't connect to your session because a problem occurred in the Windows video subsystem. Try connecting again later, or contact the server administrator for assistance.
pub const ERROR_TS_VIDEO_SUBSYSTEM_ERROR: HRESULT = 7070;

/// The file replication service API was called incorrectly.
pub const FRS_ERR_INVALID_API_SEQUENCE: HRESULT = 8001;

/// The file replication service cannot be started.
pub const FRS_ERR_STARTING_SERVICE: HRESULT = 8002;

/// The file replication service cannot be stopped.
pub const FRS_ERR_STOPPING_SERVICE: HRESULT = 8003;

/// The file replication service API terminated the request. The event log may have more information.
pub const FRS_ERR_INTERNAL_API: HRESULT = 8004;

/// The file replication service terminated the request. The event log may have more information.
pub const FRS_ERR_INTERNAL: HRESULT = 8005;

/// The file replication service cannot be contacted. The event log may have more information.
pub const FRS_ERR_SERVICE_COMM: HRESULT = 8006;

/// The file replication service cannot satisfy the request because the user has insufficient privileges. The event log may have more information.
pub const FRS_ERR_INSUFFICIENT_PRIV: HRESULT = 8007;

/// The file replication service cannot satisfy the request because authenticated RPC is not available. The event log may have more information.
pub const FRS_ERR_AUTHENTICATION: HRESULT = 8008;

/// The file replication service cannot satisfy the request because the user has insufficient privileges on the domain controller. The event log may have more information.
pub const FRS_ERR_PARENT_INSUFFICIENT_PRIV: HRESULT = 8009;

/// The file replication service cannot satisfy the request because authenticated RPC is not available on the domain controller. The event log may have more information.
pub const FRS_ERR_PARENT_AUTHENTICATION: HRESULT = 8010;

/// The file replication service cannot communicate with the file replication service on the domain controller. The event log may have more information.
pub const FRS_ERR_CHILD_TO_PARENT_COMM: HRESULT = 8011;

/// The file replication service on the domain controller cannot communicate with the file replication service on this computer. The event log may have more information.
pub const FRS_ERR_PARENT_TO_CHILD_COMM: HRESULT = 8012;

/// The file replication service cannot populate the system volume because of an internal error. The event log may have more information.
pub const FRS_ERR_SYSVOL_POPULATE: HRESULT = 8013;

/// The file replication service cannot populate the system volume because of an internal timeout. The event log may have more information.
pub const FRS_ERR_SYSVOL_POPULATE_TIMEOUT: HRESULT = 8014;

/// The file replication service cannot process the request. The system volume is busy with a previous request.
pub const FRS_ERR_SYSVOL_IS_BUSY: HRESULT = 8015;

/// The file replication service cannot stop replicating the system volume because of an internal error. The event log may have more information.
pub const FRS_ERR_SYSVOL_DEMOTE: HRESULT = 8016;

/// The file replication service detected an invalid parameter.
pub const FRS_ERR_INVALID_SERVICE_PARAMETER: HRESULT = 8017;

/// An error occurred while installing the directory service. For more information, see the event log.
pub const ERROR_DS_NOT_INSTALLED: HRESULT = 8200;

/// The directory service evaluated group memberships locally.
pub const ERROR_DS_MEMBERSHIP_EVALUATED_LOCALLY: HRESULT = 8201;

/// The specified directory service attribute or value does not exist.
pub const ERROR_DS_NO_ATTRIBUTE_OR_VALUE: HRESULT = 8202;

/// The attribute syntax specified to the directory service is invalid.
pub const ERROR_DS_INVALID_ATTRIBUTE_SYNTAX: HRESULT = 8203;

/// The attribute type specified to the directory service is not defined.
pub const ERROR_DS_ATTRIBUTE_TYPE_UNDEFINED: HRESULT = 8204;

/// The specified directory service attribute or value already exists.
pub const ERROR_DS_ATTRIBUTE_OR_VALUE_EXISTS: HRESULT = 8205;

/// The directory service is busy.
pub const ERROR_DS_BUSY: HRESULT = 8206;

/// The directory service is unavailable.
pub const ERROR_DS_UNAVAILABLE: HRESULT = 8207;

/// The directory service was unable to allocate a relative identifier.
pub const ERROR_DS_NO_RIDS_ALLOCATED: HRESULT = 8208;

/// The directory service has exhausted the pool of relative identifiers.
pub const ERROR_DS_NO_MORE_RIDS: HRESULT = 8209;

/// The requested operation could not be performed because the directory service is not the master for that type of operation.
pub const ERROR_DS_INCORRECT_ROLE_OWNER: HRESULT = 8210;

/// The directory service was unable to initialize the subsystem that allocates relative identifiers.
pub const ERROR_DS_RIDMGR_INIT_ERROR: HRESULT = 8211;

/// The requested operation did not satisfy one or more constraints associated with the class of the object.
pub const ERROR_DS_OBJ_CLASS_VIOLATION: HRESULT = 8212;

/// The directory service can perform the requested operation only on a leaf object.
pub const ERROR_DS_CANT_ON_NON_LEAF: HRESULT = 8213;

/// The directory service cannot perform the requested operation on the RDN attribute of an object.
pub const ERROR_DS_CANT_ON_RDN: HRESULT = 8214;

/// The directory service detected an attempt to modify the object class of an object.
pub const ERROR_DS_CANT_MOD_OBJ_CLASS: HRESULT = 8215;

/// The requested cross-domain move operation could not be performed.
pub const ERROR_DS_CROSS_DOM_MOVE_ERROR: HRESULT = 8216;

/// Unable to contact the global catalog server.
pub const ERROR_DS_GC_NOT_AVAILABLE: HRESULT = 8217;

/// The policy object is shared and can only be modified at the root.
pub const ERROR_SHARED_POLICY: HRESULT = 8218;

/// The policy object does not exist.
pub const ERROR_POLICY_OBJECT_NOT_FOUND: HRESULT = 8219;

/// The requested policy information is only in the directory service.
pub const ERROR_POLICY_ONLY_IN_DS: HRESULT = 8220;

/// A domain controller promotion is currently active.
pub const ERROR_PROMOTION_ACTIVE: HRESULT = 8221;

/// A domain controller promotion is not currently active
pub const ERROR_NO_PROMOTION_ACTIVE: HRESULT = 8222;

/// An operations error occurred.
pub const ERROR_DS_OPERATIONS_ERROR: HRESULT = 8224;

/// A protocol error occurred.
pub const ERROR_DS_PROTOCOL_ERROR: HRESULT = 8225;

/// The time limit for this request was exceeded.
pub const ERROR_DS_TIMELIMIT_EXCEEDED: HRESULT = 8226;

/// The size limit for this request was exceeded.
pub const ERROR_DS_SIZELIMIT_EXCEEDED: HRESULT = 8227;

/// The administrative limit for this request was exceeded.
pub const ERROR_DS_ADMIN_LIMIT_EXCEEDED: HRESULT = 8228;

/// The compare response was false.
pub const ERROR_DS_COMPARE_FALSE: HRESULT = 8229;

/// The compare response was true.
pub const ERROR_DS_COMPARE_TRUE: HRESULT = 8230;

/// The requested authentication method is not supported by the server.
pub const ERROR_DS_AUTH_METHOD_NOT_SUPPORTED: HRESULT = 8231;

/// A more secure authentication method is required for this server.
pub const ERROR_DS_STRONG_AUTH_REQUIRED: HRESULT = 8232;

/// Inappropriate authentication.
pub const ERROR_DS_INAPPROPRIATE_AUTH: HRESULT = 8233;

/// The authentication mechanism is unknown.
pub const ERROR_DS_AUTH_UNKNOWN: HRESULT = 8234;

/// A referral was returned from the server.
pub const ERROR_DS_REFERRAL: HRESULT = 8235;

/// The server does not support the requested critical extension.
pub const ERROR_DS_UNAVAILABLE_CRIT_EXTENSION: HRESULT = 8236;

/// This request requires a secure connection.
pub const ERROR_DS_CONFIDENTIALITY_REQUIRED: HRESULT = 8237;

/// Inappropriate matching.
pub const ERROR_DS_INAPPROPRIATE_MATCHING: HRESULT = 8238;

/// A constraint violation occurred.
pub const ERROR_DS_CONSTRAINT_VIOLATION: HRESULT = 8239;

/// There is no such object on the server.
pub const ERROR_DS_NO_SUCH_OBJECT: HRESULT = 8240;

/// There is an alias problem.
pub const ERROR_DS_ALIAS_PROBLEM: HRESULT = 8241;

/// An invalid dn syntax has been specified.
pub const ERROR_DS_INVALID_DN_SYNTAX: HRESULT = 8242;

/// The object is a leaf object.
pub const ERROR_DS_IS_LEAF: HRESULT = 8243;

/// There is an alias dereferencing problem.
pub const ERROR_DS_ALIAS_DEREF_PROBLEM: HRESULT = 8244;

/// The server is unwilling to process the request.
pub const ERROR_DS_UNWILLING_TO_PERFORM: HRESULT = 8245;

/// A loop has been detected.
pub const ERROR_DS_LOOP_DETECT: HRESULT = 8246;

/// There is a naming violation.
pub const ERROR_DS_NAMING_VIOLATION: HRESULT = 8247;

/// The result set is too large.
pub const ERROR_DS_OBJECT_RESULTS_TOO_LARGE: HRESULT = 8248;

/// The operation affects multiple DSAs
pub const ERROR_DS_AFFECTS_MULTIPLE_DSAS: HRESULT = 8249;

/// The server is not operational.
pub const ERROR_DS_SERVER_DOWN: HRESULT = 8250;

/// A local error has occurred.
pub const ERROR_DS_LOCAL_ERROR: HRESULT = 8251;

/// An encoding error has occurred.
pub const ERROR_DS_ENCODING_ERROR: HRESULT = 8252;

/// A decoding error has occurred.
pub const ERROR_DS_DECODING_ERROR: HRESULT = 8253;

/// The search filter cannot be recognized.
pub const ERROR_DS_FILTER_UNKNOWN: HRESULT = 8254;

/// One or more parameters are illegal.
pub const ERROR_DS_PARAM_ERROR: HRESULT = 8255;

/// The specified method is not supported.
pub const ERROR_DS_NOT_SUPPORTED: HRESULT = 8256;

/// No results were returned.
pub const ERROR_DS_NO_RESULTS_RETURNED: HRESULT = 8257;

/// The specified control is not supported by the server.
pub const ERROR_DS_CONTROL_NOT_FOUND: HRESULT = 8258;

/// A referral loop was detected by the client.
pub const ERROR_DS_CLIENT_LOOP: HRESULT = 8259;

/// The preset referral limit was exceeded.
pub const ERROR_DS_REFERRAL_LIMIT_EXCEEDED: HRESULT = 8260;

/// The search requires a SORT control.
pub const ERROR_DS_SORT_CONTROL_MISSING: HRESULT = 8261;

/// The search results exceed the offset range specified.
pub const ERROR_DS_OFFSET_RANGE_ERROR: HRESULT = 8262;

/// The directory service detected the subsystem that allocates relative identifiers is disabled. This can occur as a protective mechanism when the system determines a significant portion of relative identifiers (RIDs) have been exhausted. Please see <http://go.microsoft.com/fwlink/?LinkId=228610> for recommended diagnostic steps and the procedure to re-enable account creation.
pub const ERROR_DS_RIDMGR_DISABLED: HRESULT = 8263;

/// The root object must be the head of a naming context. The root object cannot have an instantiated parent.
pub const ERROR_DS_ROOT_MUST_BE_NC: HRESULT = 8301;

/// The add replica operation cannot be performed. The naming context must be writeable in order to create the replica.
pub const ERROR_DS_ADD_REPLICA_INHIBITED: HRESULT = 8302;

/// A reference to an attribute that is not defined in the schema occurred.
pub const ERROR_DS_ATT_NOT_DEF_IN_SCHEMA: HRESULT = 8303;

/// The maximum size of an object has been exceeded.
pub const ERROR_DS_MAX_OBJ_SIZE_EXCEEDED: HRESULT = 8304;

/// An attempt was made to add an object to the directory with a name that is already in use.
pub const ERROR_DS_OBJ_STRING_NAME_EXISTS: HRESULT = 8305;

/// An attempt was made to add an object of a class that does not have an RDN defined in the schema.
pub const ERROR_DS_NO_RDN_DEFINED_IN_SCHEMA: HRESULT = 8306;

/// An attempt was made to add an object using an RDN that is not the RDN defined in the schema.
pub const ERROR_DS_RDN_DOESNT_MATCH_SCHEMA: HRESULT = 8307;

/// None of the requested attributes were found on the objects.
pub const ERROR_DS_NO_REQUESTED_ATTS_FOUND: HRESULT = 8308;

/// The user buffer is too small.
pub const ERROR_DS_USER_BUFFER_TO_SMALL: HRESULT = 8309;

/// The attribute specified in the operation is not present on the object.
pub const ERROR_DS_ATT_IS_NOT_ON_OBJ: HRESULT = 8310;

/// Illegal modify operation. Some aspect of the modification is not permitted.
pub const ERROR_DS_ILLEGAL_MOD_OPERATION: HRESULT = 8311;

/// The specified object is too large.
pub const ERROR_DS_OBJ_TOO_LARGE: HRESULT = 8312;

/// The specified instance type is not valid.
pub const ERROR_DS_BAD_INSTANCE_TYPE: HRESULT = 8313;

/// The operation must be performed at a master DSA.
pub const ERROR_DS_MASTERDSA_REQUIRED: HRESULT = 8314;

/// The object class attribute must be specified.
pub const ERROR_DS_OBJECT_CLASS_REQUIRED: HRESULT = 8315;

/// A required attribute is missing.
pub const ERROR_DS_MISSING_REQUIRED_ATT: HRESULT = 8316;

/// An attempt was made to modify an object to include an attribute that is not legal for its class.
pub const ERROR_DS_ATT_NOT_DEF_FOR_CLASS: HRESULT = 8317;

/// The specified attribute is already present on the object.
pub const ERROR_DS_ATT_ALREADY_EXISTS: HRESULT = 8318;

/// The specified attribute is not present, or has no values.
pub const ERROR_DS_CANT_ADD_ATT_VALUES: HRESULT = 8320;

/// Multiple values were specified for an attribute that can have only one value.
pub const ERROR_DS_SINGLE_VALUE_CONSTRAINT: HRESULT = 8321;

/// A value for the attribute was not in the acceptable range of values.
pub const ERROR_DS_RANGE_CONSTRAINT: HRESULT = 8322;

/// The specified value already exists.
pub const ERROR_DS_ATT_VAL_ALREADY_EXISTS: HRESULT = 8323;

/// The attribute cannot be removed because it is not present on the object.
pub const ERROR_DS_CANT_REM_MISSING_ATT: HRESULT = 8324;

/// The attribute value cannot be removed because it is not present on the object.
pub const ERROR_DS_CANT_REM_MISSING_ATT_VAL: HRESULT = 8325;

/// The specified root object cannot be a subref.
pub const ERROR_DS_ROOT_CANT_BE_SUBREF: HRESULT = 8326;

/// Chaining is not permitted.
pub const ERROR_DS_NO_CHAINING: HRESULT = 8327;

/// Chained evaluation is not permitted.
pub const ERROR_DS_NO_CHAINED_EVAL: HRESULT = 8328;

/// The operation could not be performed because the object's parent is either uninstantiated or deleted.
pub const ERROR_DS_NO_PARENT_OBJECT: HRESULT = 8329;

/// Having a parent that is an alias is not permitted. Aliases are leaf objects.
pub const ERROR_DS_PARENT_IS_AN_ALIAS: HRESULT = 8330;

/// The object and parent must be of the same type, either both masters or both replicas.
pub const ERROR_DS_CANT_MIX_MASTER_AND_REPS: HRESULT = 8331;

/// The operation cannot be performed because child objects exist. This operation can only be performed on a leaf object.
pub const ERROR_DS_CHILDREN_EXIST: HRESULT = 8332;

/// Directory object not found.
pub const ERROR_DS_OBJ_NOT_FOUND: HRESULT = 8333;

/// The aliased object is missing.
pub const ERROR_DS_ALIASED_OBJ_MISSING: HRESULT = 8334;

/// The object name has bad syntax.
pub const ERROR_DS_BAD_NAME_SYNTAX: HRESULT = 8335;

/// It is not permitted for an alias to refer to another alias.
pub const ERROR_DS_ALIAS_POINTS_TO_ALIAS: HRESULT = 8336;

/// The alias cannot be dereferenced.
pub const ERROR_DS_CANT_DEREF_ALIAS: HRESULT = 8337;

/// The operation is out of scope.
pub const ERROR_DS_OUT_OF_SCOPE: HRESULT = 8338;

/// The operation cannot continue because the object is in the process of being removed.
pub const ERROR_DS_OBJECT_BEING_REMOVED: HRESULT = 8339;

/// The DSA object cannot be deleted.
pub const ERROR_DS_CANT_DELETE_DSA_OBJ: HRESULT = 8340;

/// A directory service error has occurred.
pub const ERROR_DS_GENERIC_ERROR: HRESULT = 8341;

/// The operation can only be performed on an internal master DSA object.
pub const ERROR_DS_DSA_MUST_BE_INT_MASTER: HRESULT = 8342;

/// The object must be of class DSA.
pub const ERROR_DS_CLASS_NOT_DSA: HRESULT = 8343;

/// Insufficient access rights to perform the operation.
pub const ERROR_DS_INSUFF_ACCESS_RIGHTS: HRESULT = 8344;

/// The object cannot be added because the parent is not on the list of possible superiors.
pub const ERROR_DS_ILLEGAL_SUPERIOR: HRESULT = 8345;

/// Access to the attribute is not permitted because the attribute is owned by the Security Accounts Manager (SAM).
pub const ERROR_DS_ATTRIBUTE_OWNED_BY_SAM: HRESULT = 8346;

/// The name has too many parts.
pub const ERROR_DS_NAME_TOO_MANY_PARTS: HRESULT = 8347;

/// The name is too long.
pub const ERROR_DS_NAME_TOO_LONG: HRESULT = 8348;

/// The name value is too long.
pub const ERROR_DS_NAME_VALUE_TOO_LONG: HRESULT = 8349;

/// The directory service encountered an error parsing a name.
pub const ERROR_DS_NAME_UNPARSEABLE: HRESULT = 8350;

/// The directory service cannot get the attribute type for a name.
pub const ERROR_DS_NAME_TYPE_UNKNOWN: HRESULT = 8351;

/// The name does not identify an object; the name identifies a phantom.
pub const ERROR_DS_NOT_AN_OBJECT: HRESULT = 8352;

/// The security descriptor is too short.
pub const ERROR_DS_SEC_DESC_TOO_SHORT: HRESULT = 8353;

/// The security descriptor is invalid.
pub const ERROR_DS_SEC_DESC_INVALID: HRESULT = 8354;

/// Failed to create name for deleted object.
pub const ERROR_DS_NO_DELETED_NAME: HRESULT = 8355;

/// The parent of a new subref must exist.
pub const ERROR_DS_SUBREF_MUST_HAVE_PARENT: HRESULT = 8356;

/// The object must be a naming context.
pub const ERROR_DS_NCNAME_MUST_BE_NC: HRESULT = 8357;

/// It is not permitted to add an attribute which is owned by the system.
pub const ERROR_DS_CANT_ADD_SYSTEM_ONLY: HRESULT = 8358;

/// The class of the object must be structural; you cannot instantiate an abstract class.
pub const ERROR_DS_CLASS_MUST_BE_CONCRETE: HRESULT = 8359;

/// The schema object could not be found.
pub const ERROR_DS_INVALID_DMD: HRESULT = 8360;

/// A local object with this GUID (dead or alive) already exists.
pub const ERROR_DS_OBJ_GUID_EXISTS: HRESULT = 8361;

/// The operation cannot be performed on a back link.
pub const ERROR_DS_NOT_ON_BACKLINK: HRESULT = 8362;

/// The cross reference for the specified naming context could not be found.
pub const ERROR_DS_NO_CROSSREF_FOR_NC: HRESULT = 8363;

/// The operation could not be performed because the directory service is shutting down.
pub const ERROR_DS_SHUTTING_DOWN: HRESULT = 8364;

/// The directory service request is invalid.
pub const ERROR_DS_UNKNOWN_OPERATION: HRESULT = 8365;

/// The role owner attribute could not be read.
pub const ERROR_DS_INVALID_ROLE_OWNER: HRESULT = 8366;

/// The requested FSMO operation failed. The current FSMO holder could not be contacted.
pub const ERROR_DS_COULDNT_CONTACT_FSMO: HRESULT = 8367;

/// Modification of a DN across a naming context is not permitted.
pub const ERROR_DS_CROSS_NC_DN_RENAME: HRESULT = 8368;

/// The attribute cannot be modified because it is owned by the system.
pub const ERROR_DS_CANT_MOD_SYSTEM_ONLY: HRESULT = 8369;

/// Only the replicator can perform this function.
pub const ERROR_DS_REPLICATOR_ONLY: HRESULT = 8370;

/// The specified class is not defined.
pub const ERROR_DS_OBJ_CLASS_NOT_DEFINED: HRESULT = 8371;

/// The specified class is not a subclass.
pub const ERROR_DS_OBJ_CLASS_NOT_SUBCLASS: HRESULT = 8372;

/// The name reference is invalid.
pub const ERROR_DS_NAME_REFERENCE_INVALID: HRESULT = 8373;

/// A cross reference already exists.
pub const ERROR_DS_CROSS_REF_EXISTS: HRESULT = 8374;

/// It is not permitted to delete a master cross reference.
pub const ERROR_DS_CANT_DEL_MASTER_CROSSREF: HRESULT = 8375;

/// Subtree notifications are only supported on NC heads.
pub const ERROR_DS_SUBTREE_NOTIFY_NOT_NC_HEAD: HRESULT = 8376;

/// Notification filter is too complex.
pub const ERROR_DS_NOTIFY_FILTER_TOO_COMPLEX: HRESULT = 8377;

/// Schema update failed: duplicate RDN.
pub const ERROR_DS_DUP_RDN: HRESULT = 8378;

/// Schema update failed: duplicate OID.
pub const ERROR_DS_DUP_OID: HRESULT = 8379;

/// Schema update failed: duplicate MAPI identifier.
pub const ERROR_DS_DUP_MAPI_ID: HRESULT = 8380;

/// Schema update failed: duplicate schema-id GUID.
pub const ERROR_DS_DUP_SCHEMA_ID_GUID: HRESULT = 8381;

/// Schema update failed: duplicate LDAP display name.
pub const ERROR_DS_DUP_LDAP_DISPLAY_NAME: HRESULT = 8382;

/// Schema update failed: range-lower less than range upper.
pub const ERROR_DS_SEMANTIC_ATT_TEST: HRESULT = 8383;

/// Schema update failed: syntax mismatch.
pub const ERROR_DS_SYNTAX_MISMATCH: HRESULT = 8384;

/// Schema deletion failed: attribute is used in must-contain.
pub const ERROR_DS_EXISTS_IN_MUST_HAVE: HRESULT = 8385;

/// Schema deletion failed: attribute is used in may-contain.
pub const ERROR_DS_EXISTS_IN_MAY_HAVE: HRESULT = 8386;

/// Schema update failed: attribute in may-contain does not exist.
pub const ERROR_DS_NONEXISTENT_MAY_HAVE: HRESULT = 8387;

/// Schema update failed: attribute in must-contain does not exist.
pub const ERROR_DS_NONEXISTENT_MUST_HAVE: HRESULT = 8388;

/// Schema update failed: class in aux-class list does not exist or is not an auxiliary class.
pub const ERROR_DS_AUX_CLS_TEST_FAIL: HRESULT = 8389;

/// Schema update failed: class in poss-superiors does not exist.
pub const ERROR_DS_NONEXISTENT_POSS_SUP: HRESULT = 8390;

/// Schema update failed: class in subclassof list does not exist or does not satisfy hierarchy rules.
pub const ERROR_DS_SUB_CLS_TEST_FAIL: HRESULT = 8391;

/// Schema update failed: Rdn-Att-Id has wrong syntax.
pub const ERROR_DS_BAD_RDN_ATT_ID_SYNTAX: HRESULT = 8392;

/// Schema deletion failed: class is used as auxiliary class.
pub const ERROR_DS_EXISTS_IN_AUX_CLS: HRESULT = 8393;

/// Schema deletion failed: class is used as sub class.
pub const ERROR_DS_EXISTS_IN_SUB_CLS: HRESULT = 8394;

/// Schema deletion failed: class is used as poss superior.
pub const ERROR_DS_EXISTS_IN_POSS_SUP: HRESULT = 8395;

/// Schema update failed in recalculating validation cache.
pub const ERROR_DS_RECALCSCHEMA_FAILED: HRESULT = 8396;

/// The tree deletion is not finished. The request must be made again to continue deleting the tree.
pub const ERROR_DS_TREE_DELETE_NOT_FINISHED: HRESULT = 8397;

/// The requested delete operation could not be performed.
pub const ERROR_DS_CANT_DELETE: HRESULT = 8398;

/// Cannot read the governs class identifier for the schema record.
pub const ERROR_DS_ATT_SCHEMA_REQ_ID: HRESULT = 8399;

/// The attribute schema has bad syntax.
pub const ERROR_DS_BAD_ATT_SCHEMA_SYNTAX: HRESULT = 8400;

/// The attribute could not be cached.
pub const ERROR_DS_CANT_CACHE_ATT: HRESULT = 8401;

/// The class could not be cached.
pub const ERROR_DS_CANT_CACHE_CLASS: HRESULT = 8402;

/// The attribute could not be removed from the cache.
pub const ERROR_DS_CANT_REMOVE_ATT_CACHE: HRESULT = 8403;

/// The class could not be removed from the cache.
pub const ERROR_DS_CANT_REMOVE_CLASS_CACHE: HRESULT = 8404;

/// The distinguished name attribute could not be read.
pub const ERROR_DS_CANT_RETRIEVE_DN: HRESULT = 8405;

/// No superior reference has been configured for the directory service. The directory service is therefore unable to issue referrals to objects outside this forest.
pub const ERROR_DS_MISSING_SUPREF: HRESULT = 8406;

/// The instance type attribute could not be retrieved.
pub const ERROR_DS_CANT_RETRIEVE_INSTANCE: HRESULT = 8407;

/// An internal error has occurred.
pub const ERROR_DS_CODE_INCONSISTENCY: HRESULT = 8408;

/// A database error has occurred.
pub const ERROR_DS_DATABASE_ERROR: HRESULT = 8409;

/// The attribute GOVERNSID is missing.
pub const ERROR_DS_GOVERNSID_MISSING: HRESULT = 8410;

/// An expected attribute is missing.
pub const ERROR_DS_MISSING_EXPECTED_ATT: HRESULT = 8411;

/// The specified naming context is missing a cross reference.
pub const ERROR_DS_NCNAME_MISSING_CR_REF: HRESULT = 8412;

/// A security checking error has occurred.
pub const ERROR_DS_SECURITY_CHECKING_ERROR: HRESULT = 8413;

/// The schema is not loaded.
pub const ERROR_DS_SCHEMA_NOT_LOADED: HRESULT = 8414;

/// Schema allocation failed. Please check if the machine is running low on memory.
pub const ERROR_DS_SCHEMA_ALLOC_FAILED: HRESULT = 8415;

/// Failed to obtain the required syntax for the attribute schema.
pub const ERROR_DS_ATT_SCHEMA_REQ_SYNTAX: HRESULT = 8416;

/// The global catalog verification failed. The global catalog is not available or does not support the operation. Some part of the directory is currently not available.
pub const ERROR_DS_GCVERIFY_ERROR: HRESULT = 8417;

/// The replication operation failed because of a schema mismatch between the servers involved.
pub const ERROR_DS_DRA_SCHEMA_MISMATCH: HRESULT = 8418;

/// The DSA object could not be found.
pub const ERROR_DS_CANT_FIND_DSA_OBJ: HRESULT = 8419;

/// The naming context could not be found.
pub const ERROR_DS_CANT_FIND_EXPECTED_NC: HRESULT = 8420;

/// The naming context could not be found in the cache.
pub const ERROR_DS_CANT_FIND_NC_IN_CACHE: HRESULT = 8421;

/// The child object could not be retrieved.
pub const ERROR_DS_CANT_RETRIEVE_CHILD: HRESULT = 8422;

/// The modification was not permitted for security reasons.
pub const ERROR_DS_SECURITY_ILLEGAL_MODIFY: HRESULT = 8423;

/// The operation cannot replace the hidden record.
pub const ERROR_DS_CANT_REPLACE_HIDDEN_REC: HRESULT = 8424;

/// The hierarchy file is invalid.
pub const ERROR_DS_BAD_HIERARCHY_FILE: HRESULT = 8425;

/// The attempt to build the hierarchy table failed.
pub const ERROR_DS_BUILD_HIERARCHY_TABLE_FAILED: HRESULT = 8426;

/// The directory configuration parameter is missing from the registry.
pub const ERROR_DS_CONFIG_PARAM_MISSING: HRESULT = 8427;

/// The attempt to count the address book indices failed.
pub const ERROR_DS_COUNTING_AB_INDICES_FAILED: HRESULT = 8428;

/// The allocation of the hierarchy table failed.
pub const ERROR_DS_HIERARCHY_TABLE_MALLOC_FAILED: HRESULT = 8429;

/// The directory service encountered an internal failure.
pub const ERROR_DS_INTERNAL_FAILURE: HRESULT = 8430;

/// The directory service encountered an unknown failure.
pub const ERROR_DS_UNKNOWN_ERROR: HRESULT = 8431;

/// A root object requires a class of 'top'.
pub const ERROR_DS_ROOT_REQUIRES_CLASS_TOP: HRESULT = 8432;

/// This directory server is shutting down, and cannot take ownership of new floating single-master operation roles.
pub const ERROR_DS_REFUSING_FSMO_ROLES: HRESULT = 8433;

/// The directory service is missing mandatory configuration information, and is unable to determine the ownership of floating single-master operation roles.
pub const ERROR_DS_MISSING_FSMO_SETTINGS: HRESULT = 8434;

/// The directory service was unable to transfer ownership of one or more floating single-master operation roles to other servers.
pub const ERROR_DS_UNABLE_TO_SURRENDER_ROLES: HRESULT = 8435;

/// The replication operation failed.
pub const ERROR_DS_DRA_GENERIC: HRESULT = 8436;

/// An invalid parameter was specified for this replication operation.
pub const ERROR_DS_DRA_INVALID_PARAMETER: HRESULT = 8437;

/// The directory service is too busy to complete the replication operation at this time.
pub const ERROR_DS_DRA_BUSY: HRESULT = 8438;

/// The distinguished name specified for this replication operation is invalid.
pub const ERROR_DS_DRA_BAD_DN: HRESULT = 8439;

/// The naming context specified for this replication operation is invalid.
pub const ERROR_DS_DRA_BAD_NC: HRESULT = 8440;

/// The distinguished name specified for this replication operation already exists.
pub const ERROR_DS_DRA_DN_EXISTS: HRESULT = 8441;

/// The replication system encountered an internal error.
pub const ERROR_DS_DRA_INTERNAL_ERROR: HRESULT = 8442;

/// The replication operation encountered a database inconsistency.
pub const ERROR_DS_DRA_INCONSISTENT_DIT: HRESULT = 8443;

/// The server specified for this replication operation could not be contacted.
pub const ERROR_DS_DRA_CONNECTION_FAILED: HRESULT = 8444;

/// The replication operation encountered an object with an invalid instance type.
pub const ERROR_DS_DRA_BAD_INSTANCE_TYPE: HRESULT = 8445;

/// The replication operation failed to allocate memory.
pub const ERROR_DS_DRA_OUT_OF_MEM: HRESULT = 8446;

/// The replication operation encountered an error with the mail system.
pub const ERROR_DS_DRA_MAIL_PROBLEM: HRESULT = 8447;

/// The replication reference information for the target server already exists.
pub const ERROR_DS_DRA_REF_ALREADY_EXISTS: HRESULT = 8448;

/// The replication reference information for the target server does not exist.
pub const ERROR_DS_DRA_REF_NOT_FOUND: HRESULT = 8449;

/// The naming context cannot be removed because it is replicated to another server.
pub const ERROR_DS_DRA_OBJ_IS_REP_SOURCE: HRESULT = 8450;

/// The replication operation encountered a database error.
pub const ERROR_DS_DRA_DB_ERROR: HRESULT = 8451;

/// The naming context is in the process of being removed or is not replicated from the specified server.
pub const ERROR_DS_DRA_NO_REPLICA: HRESULT = 8452;

/// Replication access was denied.
pub const ERROR_DS_DRA_ACCESS_DENIED: HRESULT = 8453;

/// The requested operation is not supported by this version of the directory service.
pub const ERROR_DS_DRA_NOT_SUPPORTED: HRESULT = 8454;

/// The replication remote procedure call was cancelled.
pub const ERROR_DS_DRA_RPC_CANCELLED: HRESULT = 8455;

/// The source server is currently rejecting replication requests.
pub const ERROR_DS_DRA_SOURCE_DISABLED: HRESULT = 8456;

/// The destination server is currently rejecting replication requests.
pub const ERROR_DS_DRA_SINK_DISABLED: HRESULT = 8457;

/// The replication operation failed due to a collision of object names.
pub const ERROR_DS_DRA_NAME_COLLISION: HRESULT = 8458;

/// The replication source has been reinstalled.
pub const ERROR_DS_DRA_SOURCE_REINSTALLED: HRESULT = 8459;

/// The replication operation failed because a required parent object is missing.
pub const ERROR_DS_DRA_MISSING_PARENT: HRESULT = 8460;

/// The replication operation was preempted.
pub const ERROR_DS_DRA_PREEMPTED: HRESULT = 8461;

/// The replication synchronization attempt was abandoned because of a lack of updates.
pub const ERROR_DS_DRA_ABANDON_SYNC: HRESULT = 8462;

/// The replication operation was terminated because the system is shutting down.
pub const ERROR_DS_DRA_SHUTDOWN: HRESULT = 8463;

/// Synchronization attempt failed because the destination DC is currently waiting to synchronize new partial attributes from source. This condition is normal if a recent schema change modified the partial attribute set. The destination partial attribute set is not a subset of source partial attribute set.
pub const ERROR_DS_DRA_INCOMPATIBLE_PARTIAL_SET: HRESULT = 8464;

/// The replication synchronization attempt failed because a master replica attempted to sync from a partial replica.
pub const ERROR_DS_DRA_SOURCE_IS_PARTIAL_REPLICA: HRESULT = 8465;

/// The server specified for this replication operation was contacted, but that server was unable to contact an additional server needed to complete the operation.
pub const ERROR_DS_DRA_EXTN_CONNECTION_FAILED: HRESULT = 8466;

/// The version of the directory service schema of the source forest is not compatible with the version of directory service on this computer.
pub const ERROR_DS_INSTALL_SCHEMA_MISMATCH: HRESULT = 8467;

/// Schema update failed: An attribute with the same link identifier already exists.
pub const ERROR_DS_DUP_LINK_ID: HRESULT = 8468;

/// Name translation: Generic processing error.
pub const ERROR_DS_NAME_ERROR_RESOLVING: HRESULT = 8469;

/// Name translation: Could not find the name or insufficient right to see name.
pub const ERROR_DS_NAME_ERROR_NOT_FOUND: HRESULT = 8470;

/// Name translation: Input name mapped to more than one output name.
pub const ERROR_DS_NAME_ERROR_NOT_UNIQUE: HRESULT = 8471;

/// Name translation: Input name found, but not the associated output format.
pub const ERROR_DS_NAME_ERROR_NO_MAPPING: HRESULT = 8472;

/// Name translation: Unable to resolve completely, only the domain was found.
pub const ERROR_DS_NAME_ERROR_DOMAIN_ONLY: HRESULT = 8473;

/// Name translation: Unable to perform purely syntactical mapping at the client without going out to the wire.
pub const ERROR_DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: HRESULT = 8474;

/// Modification of a constructed attribute is not allowed.
pub const ERROR_DS_CONSTRUCTED_ATT_MOD: HRESULT = 8475;

/// The OM-Object-Class specified is incorrect for an attribute with the specified syntax.
pub const ERROR_DS_WRONG_OM_OBJ_CLASS: HRESULT = 8476;

/// The replication request has been posted; waiting for reply.
pub const ERROR_DS_DRA_REPL_PENDING: HRESULT = 8477;

/// The requested operation requires a directory service, and none was available.
pub const ERROR_DS_DS_REQUIRED: HRESULT = 8478;

/// The LDAP display name of the class or attribute contains non-ASCII characters.
pub const ERROR_DS_INVALID_LDAP_DISPLAY_NAME: HRESULT = 8479;

/// The requested search operation is only supported for base searches.
pub const ERROR_DS_NON_BASE_SEARCH: HRESULT = 8480;

/// The search failed to retrieve attributes from the database.
pub const ERROR_DS_CANT_RETRIEVE_ATTS: HRESULT = 8481;

/// The schema update operation tried to add a backward link attribute that has no corresponding forward link.
pub const ERROR_DS_BACKLINK_WITHOUT_LINK: HRESULT = 8482;

/// Source and destination of a cross-domain move do not agree on the object's epoch number. Either source or destination does not have the latest version of the object.
pub const ERROR_DS_EPOCH_MISMATCH: HRESULT = 8483;

/// Source and destination of a cross-domain move do not agree on the object's current name. Either source or destination does not have the latest version of the object.
pub const ERROR_DS_SRC_NAME_MISMATCH: HRESULT = 8484;

/// Source and destination for the cross-domain move operation are identical. Caller should use local move operation instead of cross-domain move operation.
pub const ERROR_DS_SRC_AND_DST_NC_IDENTICAL: HRESULT = 8485;

/// Source and destination for a cross-domain move are not in agreement on the naming contexts in the forest. Either source or destination does not have the latest version of the Partitions container.
pub const ERROR_DS_DST_NC_MISMATCH: HRESULT = 8486;

/// Destination of a cross-domain move is not authoritative for the destination naming context.
pub const ERROR_DS_NOT_AUTHORITIVE_FOR_DST_NC: HRESULT = 8487;

/// Source and destination of a cross-domain move do not agree on the identity of the source object. Either source or destination does not have the latest version of the source object.
pub const ERROR_DS_SRC_GUID_MISMATCH: HRESULT = 8488;

/// Object being moved across-domains is already known to be deleted by the destination server. The source server does not have the latest version of the source object.
pub const ERROR_DS_CANT_MOVE_DELETED_OBJECT: HRESULT = 8489;

/// Another operation which requires exclusive access to the PDC FSMO is already in progress.
pub const ERROR_DS_PDC_OPERATION_IN_PROGRESS: HRESULT = 8490;

/// A cross-domain move operation failed such that two versions of the moved object exist - one each in the source and destination domains. The destination object needs to be removed to restore the system to a consistent state.
pub const ERROR_DS_CROSS_DOMAIN_CLEANUP_REQD: HRESULT = 8491;

/// This object may not be moved across domain boundaries either because cross-domain moves for this class are disallowed, or the object has some special characteristics, e.g.: trust account or restricted RID, which prevent its move.
pub const ERROR_DS_ILLEGAL_XDOM_MOVE_OPERATION: HRESULT = 8492;

/// Can't move objects with memberships across domain boundaries as once moved, this would violate the membership conditions of the account group. Remove the object from any account group memberships and retry.
pub const ERROR_DS_CANT_WITH_ACCT_GROUP_MEMBERSHPS: HRESULT = 8493;

/// A naming context head must be the immediate child of another naming context head, not of an interior node.
pub const ERROR_DS_NC_MUST_HAVE_NC_PARENT: HRESULT = 8494;

/// The directory cannot validate the proposed naming context name because it does not hold a replica of the naming context above the proposed naming context. Please ensure that the domain naming master role is held by a server that is configured as a global catalog server, and that the server is up to date with its replication partners. (Applies only to Windows 2000 Domain Naming masters)
pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE: HRESULT = 8495;

/// Destination domain must be in native mode.
pub const ERROR_DS_DST_DOMAIN_NOT_NATIVE: HRESULT = 8496;

/// The operation cannot be performed because the server does not have an infrastructure container in the domain of interest.
pub const ERROR_DS_MISSING_INFRASTRUCTURE_CONTAINER: HRESULT = 8497;

/// Cross-domain move of non-empty account groups is not allowed.
pub const ERROR_DS_CANT_MOVE_ACCOUNT_GROUP: HRESULT = 8498;

/// Cross-domain move of non-empty resource groups is not allowed.
pub const ERROR_DS_CANT_MOVE_RESOURCE_GROUP: HRESULT = 8499;

/// The search flags for the attribute are invalid. The ANR bit is valid only on attributes of Unicode or Teletex strings.
pub const ERROR_DS_INVALID_SEARCH_FLAG: HRESULT = 8500;

/// Tree deletions starting at an object which has an NC head as a descendant are not allowed.
pub const ERROR_DS_NO_TREE_DELETE_ABOVE_NC: HRESULT = 8501;

/// The directory service failed to lock a tree in preparation for a tree deletion because the tree was in use.
pub const ERROR_DS_COULDNT_LOCK_TREE_FOR_DELETE: HRESULT = 8502;

/// The directory service failed to identify the list of objects to delete while attempting a tree deletion.
pub const ERROR_DS_COULDNT_IDENTIFY_OBJECTS_FOR_TREE_DELETE: HRESULT = 8503;

///  Security Accounts Manager initialization failed because of the following error: %1.
///
/// Error Status: 0x%2. Please shutdown this system and reboot into Directory Services Restore Mode, check the event log for more detailed information.
pub const ERROR_DS_SAM_INIT_FAILURE: HRESULT = 8504;

/// Only an administrator can modify the membership list of an administrative group.
pub const ERROR_DS_SENSITIVE_GROUP_VIOLATION: HRESULT = 8505;

/// Cannot change the primary group ID of a domain controller account.
pub const ERROR_DS_CANT_MOD_PRIMARYGROUPID: HRESULT = 8506;

/// An attempt is made to modify the base schema.
pub const ERROR_DS_ILLEGAL_BASE_SCHEMA_MOD: HRESULT = 8507;

/// Adding a new mandatory attribute to an existing class, deleting a mandatory attribute from an existing class, or adding an optional attribute to the special class Top that is not a backlink attribute (directly or through inheritance, for example, by adding or deleting an auxiliary class) is not allowed.
pub const ERROR_DS_NONSAFE_SCHEMA_CHANGE: HRESULT = 8508;

/// Schema update is not allowed on this DC because the DC is not the schema FSMO Role Owner.
pub const ERROR_DS_SCHEMA_UPDATE_DISALLOWED: HRESULT = 8509;

/// An object of this class cannot be created under the schema container. You can only create attribute-schema and class-schema objects under the schema container.
pub const ERROR_DS_CANT_CREATE_UNDER_SCHEMA: HRESULT = 8510;

/// The replica/child install failed to get the objectVersion attribute on the schema container on the source DC. Either the attribute is missing on the schema container or the credentials supplied do not have permission to read it.
pub const ERROR_DS_INSTALL_NO_SRC_SCH_VERSION: HRESULT = 8511;

/// The replica/child install failed to read the objectVersion attribute in the SCHEMA section of the file schema.ini in the system32 directory.
pub const ERROR_DS_INSTALL_NO_SCH_VERSION_IN_INIFILE: HRESULT = 8512;

/// The specified group type is invalid.
pub const ERROR_DS_INVALID_GROUP_TYPE: HRESULT = 8513;

/// You cannot nest global groups in a mixed domain if the group is security-enabled.
pub const ERROR_DS_NO_NEST_GLOBALGROUP_IN_MIXEDDOMAIN: HRESULT = 8514;

/// You cannot nest local groups in a mixed domain if the group is security-enabled.
pub const ERROR_DS_NO_NEST_LOCALGROUP_IN_MIXEDDOMAIN: HRESULT = 8515;

/// A global group cannot have a local group as a member.
pub const ERROR_DS_GLOBAL_CANT_HAVE_LOCAL_MEMBER: HRESULT = 8516;

/// A global group cannot have a universal group as a member.
pub const ERROR_DS_GLOBAL_CANT_HAVE_UNIVERSAL_MEMBER: HRESULT = 8517;

/// A universal group cannot have a local group as a member.
pub const ERROR_DS_UNIVERSAL_CANT_HAVE_LOCAL_MEMBER: HRESULT = 8518;

/// A global group cannot have a cross-domain member.
pub const ERROR_DS_GLOBAL_CANT_HAVE_CROSSDOMAIN_MEMBER: HRESULT = 8519;

/// A local group cannot have another cross domain local group as a member.
pub const ERROR_DS_LOCAL_CANT_HAVE_CROSSDOMAIN_LOCAL_MEMBER: HRESULT = 8520;

/// A group with primary members cannot change to a security-disabled group.
pub const ERROR_DS_HAVE_PRIMARY_MEMBERS: HRESULT = 8521;

/// The schema cache load failed to convert the string default SD on a class-schema object.
pub const ERROR_DS_STRING_SD_CONVERSION_FAILED: HRESULT = 8522;

/// Only DSAs configured to be Global Catalog servers should be allowed to hold the Domain Naming Master FSMO role. (Applies only to Windows 2000 servers)
pub const ERROR_DS_NAMING_MASTER_GC: HRESULT = 8523;

/// The DSA operation is unable to proceed because of a DNS lookup failure.
pub const ERROR_DS_DNS_LOOKUP_FAILURE: HRESULT = 8524;

/// While processing a change to the DNS Host Name for an object, the Service Principal Name values could not be kept in sync.
pub const ERROR_DS_COULDNT_UPDATE_SPNS: HRESULT = 8525;

/// The Security Descriptor attribute could not be read.
pub const ERROR_DS_CANT_RETRIEVE_SD: HRESULT = 8526;

/// The object requested was not found, but an object with that key was found.
pub const ERROR_DS_KEY_NOT_UNIQUE: HRESULT = 8527;

/// The syntax of the linked attribute being added is incorrect. Forward links can only have syntax 2.5.5.1, 2.5.5.7, and 2.5.5.14, and backlinks can only have syntax 2.5.5.1
pub const ERROR_DS_WRONG_LINKED_ATT_SYNTAX: HRESULT = 8528;

/// Security Account Manager needs to get the boot password.
pub const ERROR_DS_SAM_NEED_BOOTKEY_PASSWORD: HRESULT = 8529;

/// Security Account Manager needs to get the boot key from floppy disk.
pub const ERROR_DS_SAM_NEED_BOOTKEY_FLOPPY: HRESULT = 8530;

/// Directory Service cannot start.
pub const ERROR_DS_CANT_START: HRESULT = 8531;

/// Directory Services could not start.
pub const ERROR_DS_INIT_FAILURE: HRESULT = 8532;

/// The connection between client and server requires packet privacy or better.
pub const ERROR_DS_NO_PKT_PRIVACY_ON_CONNECTION: HRESULT = 8533;

/// The source domain may not be in the same forest as destination.
pub const ERROR_DS_SOURCE_DOMAIN_IN_FOREST: HRESULT = 8534;

/// The destination domain must be in the forest.
pub const ERROR_DS_DESTINATION_DOMAIN_NOT_IN_FOREST: HRESULT = 8535;

/// The operation requires that destination domain auditing be enabled.
pub const ERROR_DS_DESTINATION_AUDITING_NOT_ENABLED: HRESULT = 8536;

/// The operation couldn't locate a DC for the source domain.
pub const ERROR_DS_CANT_FIND_DC_FOR_SRC_DOMAIN: HRESULT = 8537;

/// The source object must be a group or user.
pub const ERROR_DS_SRC_OBJ_NOT_GROUP_OR_USER: HRESULT = 8538;

/// The source object's SID already exists in destination forest.
pub const ERROR_DS_SRC_SID_EXISTS_IN_FOREST: HRESULT = 8539;

/// The source and destination object must be of the same type.
pub const ERROR_DS_SRC_AND_DST_OBJECT_CLASS_MISMATCH: HRESULT = 8540;

///  Security Accounts Manager initialization failed because of the following error: %1.
///
/// Error Status: 0x%2. Click OK to shut down the system and reboot into Safe Mode. Check the event log for detailed information.
pub const ERROR_SAM_INIT_FAILURE: HRESULT = 8541;

/// Schema information could not be included in the replication request.
pub const ERROR_DS_DRA_SCHEMA_INFO_SHIP: HRESULT = 8542;

/// The replication operation could not be completed due to a schema incompatibility.
pub const ERROR_DS_DRA_SCHEMA_CONFLICT: HRESULT = 8543;

/// The replication operation could not be completed due to a previous schema incompatibility.
pub const ERROR_DS_DRA_EARLIER_SCHEMA_CONFLICT: HRESULT = 8544;

/// The replication update could not be applied because either the source or the destination has not yet received information regarding a recent cross-domain move operation.
pub const ERROR_DS_DRA_OBJ_NC_MISMATCH: HRESULT = 8545;

/// The requested domain could not be deleted because there exist domain controllers that still host this domain.
pub const ERROR_DS_NC_STILL_HAS_DSAS: HRESULT = 8546;

/// The requested operation can be performed only on a global catalog server.
pub const ERROR_DS_GC_REQUIRED: HRESULT = 8547;

/// A local group can only be a member of other local groups in the same domain.
pub const ERROR_DS_LOCAL_MEMBER_OF_LOCAL_ONLY: HRESULT = 8548;

/// Foreign security principals cannot be members of universal groups.
pub const ERROR_DS_NO_FPO_IN_UNIVERSAL_GROUPS: HRESULT = 8549;

/// The attribute is not allowed to be replicated to the GC because of security reasons.
pub const ERROR_DS_CANT_ADD_TO_GC: HRESULT = 8550;

/// The checkpoint with the PDC could not be taken because there too many modifications being processed currently.
pub const ERROR_DS_NO_CHECKPOINT_WITH_PDC: HRESULT = 8551;

/// The operation requires that source domain auditing be enabled.
pub const ERROR_DS_SOURCE_AUDITING_NOT_ENABLED: HRESULT = 8552;

/// Security principal objects can only be created inside domain naming contexts.
pub const ERROR_DS_CANT_CREATE_IN_NONDOMAIN_NC: HRESULT = 8553;

/// A Service Principal Name (SPN) could not be constructed because the provided hostname is not in the necessary format.
pub const ERROR_DS_INVALID_NAME_FOR_SPN: HRESULT = 8554;

/// A Filter was passed that uses constructed attributes.
pub const ERROR_DS_FILTER_USES_CONTRUCTED_ATTRS: HRESULT = 8555;

/// The unicodePwd attribute value must be enclosed in double quotes.
pub const ERROR_DS_UNICODEPWD_NOT_IN_QUOTES: HRESULT = 8556;

/// Your computer could not be joined to the domain. You have exceeded the maximum number of computer accounts you are allowed to create in this domain. Contact your system administrator to have this limit reset or increased.
pub const ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED: HRESULT = 8557;

/// For security reasons, the operation must be run on the destination DC.
pub const ERROR_DS_MUST_BE_RUN_ON_DST_DC: HRESULT = 8558;

/// For security reasons, the source DC must be NT4SP4 or greater.
pub const ERROR_DS_SRC_DC_MUST_BE_SP4_OR_GREATER: HRESULT = 8559;

/// Critical Directory Service System objects cannot be deleted during tree delete operations. The tree delete may have been partially performed.
pub const ERROR_DS_CANT_TREE_DELETE_CRITICAL_OBJ: HRESULT = 8560;

///  Directory Services could not start because of the following error: %1.
///
/// Error Status: 0x%2. Please click OK to shutdown the system. You can use the recovery console to diagnose the system further.
pub const ERROR_DS_INIT_FAILURE_CONSOLE: HRESULT = 8561;

///  Security Accounts Manager initialization failed because of the following error: %1.
///
/// Error Status: 0x%2. Please click OK to shutdown the system. You can use the recovery console to diagnose the system further.
pub const ERROR_DS_SAM_INIT_FAILURE_CONSOLE: HRESULT = 8562;

/// The version of the operating system is incompatible with the current AD DS forest functional level or AD LDS Configuration Set functional level. You must upgrade to a new version of the operating system before this server can become an AD DS Domain Controller or add an AD LDS Instance in this AD DS Forest or AD LDS Configuration Set.
pub const ERROR_DS_FOREST_VERSION_TOO_HIGH: HRESULT = 8563;

/// The version of the operating system installed is incompatible with the current domain functional level. You must upgrade to a new version of the operating system before this server can become a domain controller in this domain.
pub const ERROR_DS_DOMAIN_VERSION_TOO_HIGH: HRESULT = 8564;

/// The version of the operating system installed on this server no longer supports the current AD DS Forest functional level or AD LDS Configuration Set functional level. You must raise the AD DS Forest functional level or AD LDS Configuration Set functional level before this server can become an AD DS Domain Controller or an AD LDS Instance in this Forest or Configuration Set.
pub const ERROR_DS_FOREST_VERSION_TOO_LOW: HRESULT = 8565;

/// The version of the operating system installed on this server no longer supports the current domain functional level. You must raise the domain functional level before this server can become a domain controller in this domain.
pub const ERROR_DS_DOMAIN_VERSION_TOO_LOW: HRESULT = 8566;

/// The version of the operating system installed on this server is incompatible with the functional level of the domain or forest.
pub const ERROR_DS_INCOMPATIBLE_VERSION: HRESULT = 8567;

/// The functional level of the domain (or forest) cannot be raised to the requested value, because there exist one or more domain controllers in the domain (or forest) that are at a lower incompatible functional level.
pub const ERROR_DS_LOW_DSA_VERSION: HRESULT = 8568;

/// The forest functional level cannot be raised to the requested value since one or more domains are still in mixed domain mode. All domains in the forest must be in native mode, for you to raise the forest functional level.
pub const ERROR_DS_NO_BEHAVIOR_VERSION_IN_MIXEDDOMAIN: HRESULT = 8569;

/// The sort order requested is not supported.
pub const ERROR_DS_NOT_SUPPORTED_SORT_ORDER: HRESULT = 8570;

/// The requested name already exists as a unique identifier.
pub const ERROR_DS_NAME_NOT_UNIQUE: HRESULT = 8571;

/// The machine account was created pre-NT4. The account needs to be recreated.
pub const ERROR_DS_MACHINE_ACCOUNT_CREATED_PRENT4: HRESULT = 8572;

/// The database is out of version store.
pub const ERROR_DS_OUT_OF_VERSION_STORE: HRESULT = 8573;

/// Unable to continue operation because multiple conflicting controls were used.
pub const ERROR_DS_INCOMPATIBLE_CONTROLS_USED: HRESULT = 8574;

/// Unable to find a valid security descriptor reference domain for this partition.
pub const ERROR_DS_NO_REF_DOMAIN: HRESULT = 8575;

/// Schema update failed: The link identifier is reserved.
pub const ERROR_DS_RESERVED_LINK_ID: HRESULT = 8576;

/// Schema update failed: There are no link identifiers available.
pub const ERROR_DS_LINK_ID_NOT_AVAILABLE: HRESULT = 8577;

/// An account group cannot have a universal group as a member.
pub const ERROR_DS_AG_CANT_HAVE_UNIVERSAL_MEMBER: HRESULT = 8578;

/// Rename or move operations on naming context heads or read-only objects are not allowed.
pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_INSTANCE_TYPE: HRESULT = 8579;

/// Move operations on objects in the schema naming context are not allowed.
pub const ERROR_DS_NO_OBJECT_MOVE_IN_SCHEMA_NC: HRESULT = 8580;

/// A system flag has been set on the object and does not allow the object to be moved or renamed.
pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_FLAG: HRESULT = 8581;

/// This object is not allowed to change its grandparent container. Moves are not forbidden on this object, but are restricted to sibling containers.
pub const ERROR_DS_MODIFYDN_WRONG_GRANDPARENT: HRESULT = 8582;

/// Unable to resolve completely, a referral to another forest is generated.
pub const ERROR_DS_NAME_ERROR_TRUST_REFERRAL: HRESULT = 8583;

/// The requested action is not supported on standard server.
pub const ERROR_NOT_SUPPORTED_ON_STANDARD_SERVER: HRESULT = 8584;

/// Could not access a partition of the directory service located on a remote server. Make sure at least one server is running for the partition in question.
pub const ERROR_DS_CANT_ACCESS_REMOTE_PART_OF_AD: HRESULT = 8585;

/// The directory cannot validate the proposed naming context (or partition) name because it does not hold a replica nor can it contact a replica of the naming context above the proposed naming context. Please ensure that the parent naming context is properly registered in DNS, and at least one replica of this naming context is reachable by the Domain Naming master.
pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE_V2: HRESULT = 8586;

/// The thread limit for this request was exceeded.
pub const ERROR_DS_THREAD_LIMIT_EXCEEDED: HRESULT = 8587;

/// The Global catalog server is not in the closest site.
pub const ERROR_DS_NOT_CLOSEST: HRESULT = 8588;

/// The DS cannot derive a service principal name (SPN) with which to mutually authenticate the target server because the corresponding server object in the local DS database has no serverReference attribute.
pub const ERROR_DS_CANT_DERIVE_SPN_WITHOUT_SERVER_REF: HRESULT = 8589;

/// The Directory Service failed to enter single user mode.
pub const ERROR_DS_SINGLE_USER_MODE_FAILED: HRESULT = 8590;

/// The Directory Service cannot parse the script because of a syntax error.
pub const ERROR_DS_NTDSCRIPT_SYNTAX_ERROR: HRESULT = 8591;

/// The Directory Service cannot process the script because of an error.
pub const ERROR_DS_NTDSCRIPT_PROCESS_ERROR: HRESULT = 8592;

/// The directory service cannot perform the requested operation because the servers involved are of different replication epochs (which is usually related to a domain rename that is in progress).
pub const ERROR_DS_DIFFERENT_REPL_EPOCHS: HRESULT = 8593;

/// The directory service binding must be renegotiated due to a change in the server extensions information.
pub const ERROR_DS_DRS_EXTENSIONS_CHANGED: HRESULT = 8594;

/// Operation not allowed on a disabled cross ref.
pub const ERROR_DS_REPLICA_SET_CHANGE_NOT_ALLOWED_ON_DISABLED_CR: HRESULT = 8595;

/// Schema update failed: No values for msDS-IntId are available.
pub const ERROR_DS_NO_MSDS_INTID: HRESULT = 8596;

/// Schema update failed: Duplicate msDS-INtId. Retry the operation.
pub const ERROR_DS_DUP_MSDS_INTID: HRESULT = 8597;

/// Schema deletion failed: attribute is used in rDNAttID.
pub const ERROR_DS_EXISTS_IN_RDNATTID: HRESULT = 8598;

/// The directory service failed to authorize the request.
pub const ERROR_DS_AUTHORIZATION_FAILED: HRESULT = 8599;

/// The Directory Service cannot process the script because it is invalid.
pub const ERROR_DS_INVALID_SCRIPT: HRESULT = 8600;

/// The remote create cross reference operation failed on the Domain Naming Master FSMO. The operation's error is in the extended data.
pub const ERROR_DS_REMOTE_CROSSREF_OP_FAILED: HRESULT = 8601;

/// A cross reference is in use locally with the same name.
pub const ERROR_DS_CROSS_REF_BUSY: HRESULT = 8602;

/// The DS cannot derive a service principal name (SPN) with which to mutually authenticate the target server because the server's domain has been deleted from the forest.
pub const ERROR_DS_CANT_DERIVE_SPN_FOR_DELETED_DOMAIN: HRESULT = 8603;

/// Writeable NCs prevent this DC from demoting.
pub const ERROR_DS_CANT_DEMOTE_WITH_WRITEABLE_NC: HRESULT = 8604;

/// The requested object has a non-unique identifier and cannot be retrieved.
pub const ERROR_DS_DUPLICATE_ID_FOUND: HRESULT = 8605;

/// Insufficient attributes were given to create an object. This object may not exist because it may have been deleted and already garbage collected.
pub const ERROR_DS_INSUFFICIENT_ATTR_TO_CREATE_OBJECT: HRESULT = 8606;

/// The group cannot be converted due to attribute restrictions on the requested group type.
pub const ERROR_DS_GROUP_CONVERSION_ERROR: HRESULT = 8607;

/// Cross-domain move of non-empty basic application groups is not allowed.
pub const ERROR_DS_CANT_MOVE_APP_BASIC_GROUP: HRESULT = 8608;

/// Cross-domain move of non-empty query based application groups is not allowed.
pub const ERROR_DS_CANT_MOVE_APP_QUERY_GROUP: HRESULT = 8609;

/// The FSMO role ownership could not be verified because its directory partition has not replicated successfully with at least one replication partner.
pub const ERROR_DS_ROLE_NOT_VERIFIED: HRESULT = 8610;

/// The target container for a redirection of a well known object container cannot already be a special container.
pub const ERROR_DS_WKO_CONTAINER_CANNOT_BE_SPECIAL: HRESULT = 8611;

/// The Directory Service cannot perform the requested operation because a domain rename operation is in progress.
pub const ERROR_DS_DOMAIN_RENAME_IN_PROGRESS: HRESULT = 8612;

/// The directory service detected a child partition below the requested partition name. The partition hierarchy must be created in a top down method.
pub const ERROR_DS_EXISTING_AD_CHILD_NC: HRESULT = 8613;

/// The directory service cannot replicate with this server because the time since the last replication with this server has exceeded the tombstone lifetime.
pub const ERROR_DS_REPL_LIFETIME_EXCEEDED: HRESULT = 8614;

/// The requested operation is not allowed on an object under the system container.
pub const ERROR_DS_DISALLOWED_IN_SYSTEM_CONTAINER: HRESULT = 8615;

/// The LDAP servers network send queue has filled up because the client is not processing the results of its requests fast enough. No more requests will be processed until the client catches up. If the client does not catch up then it will be disconnected.
pub const ERROR_DS_LDAP_SEND_QUEUE_FULL: HRESULT = 8616;

/// The scheduled replication did not take place because the system was too busy to execute the request within the schedule window. The replication queue is overloaded. Consider reducing the number of partners or decreasing the scheduled replication frequency.
pub const ERROR_DS_DRA_OUT_SCHEDULE_WINDOW: HRESULT = 8617;

/// At this time, it cannot be determined if the branch replication policy is available on the hub domain controller. Please retry at a later time to account for replication latencies.
pub const ERROR_DS_POLICY_NOT_KNOWN: HRESULT = 8618;

/// The site settings object for the specified site does not exist.
pub const ERROR_NO_SITE_SETTINGS_OBJECT: HRESULT = 8619;

/// The local account store does not contain secret material for the specified account.
pub const ERROR_NO_SECRETS: HRESULT = 8620;

/// Could not find a writable domain controller in the domain.
pub const ERROR_NO_WRITABLE_DC_FOUND: HRESULT = 8621;

/// The server object for the domain controller does not exist.
pub const ERROR_DS_NO_SERVER_OBJECT: HRESULT = 8622;

/// The NTDS Settings object for the domain controller does not exist.
pub const ERROR_DS_NO_NTDSA_OBJECT: HRESULT = 8623;

/// The requested search operation is not supported for ASQ searches.
pub const ERROR_DS_NON_ASQ_SEARCH: HRESULT = 8624;

/// A required audit event could not be generated for the operation.
pub const ERROR_DS_AUDIT_FAILURE: HRESULT = 8625;

/// The search flags for the attribute are invalid. The subtree index bit is valid only on single valued attributes.
pub const ERROR_DS_INVALID_SEARCH_FLAG_SUBTREE: HRESULT = 8626;

/// The search flags for the attribute are invalid. The tuple index bit is valid only on attributes of Unicode strings.
pub const ERROR_DS_INVALID_SEARCH_FLAG_TUPLE: HRESULT = 8627;

/// The address books are nested too deeply. Failed to build the hierarchy table.
pub const ERROR_DS_HIERARCHY_TABLE_TOO_DEEP: HRESULT = 8628;

/// The specified up-to-date-ness vector is corrupt.
pub const ERROR_DS_DRA_CORRUPT_UTD_VECTOR: HRESULT = 8629;

/// The request to replicate secrets is denied.
pub const ERROR_DS_DRA_SECRETS_DENIED: HRESULT = 8630;

/// Schema update failed: The MAPI identifier is reserved.
pub const ERROR_DS_RESERVED_MAPI_ID: HRESULT = 8631;

/// Schema update failed: There are no MAPI identifiers available.
pub const ERROR_DS_MAPI_ID_NOT_AVAILABLE: HRESULT = 8632;

/// The replication operation failed because the required attributes of the local krbtgt object are missing.
pub const ERROR_DS_DRA_MISSING_KRBTGT_SECRET: HRESULT = 8633;

/// The domain name of the trusted domain already exists in the forest.
pub const ERROR_DS_DOMAIN_NAME_EXISTS_IN_FOREST: HRESULT = 8634;

/// The flat name of the trusted domain already exists in the forest.
pub const ERROR_DS_FLAT_NAME_EXISTS_IN_FOREST: HRESULT = 8635;

/// The User Principal Name (UPN) is invalid.
pub const ERROR_INVALID_USER_PRINCIPAL_NAME: HRESULT = 8636;

/// OID mapped groups cannot have members.
pub const ERROR_DS_OID_MAPPED_GROUP_CANT_HAVE_MEMBERS: HRESULT = 8637;

/// The specified OID cannot be found.
pub const ERROR_DS_OID_NOT_FOUND: HRESULT = 8638;

/// The replication operation failed because the target object referred by a link value is recycled.
pub const ERROR_DS_DRA_RECYCLED_TARGET: HRESULT = 8639;

/// The redirect operation failed because the target object is in a NC different from the domain NC of the current domain controller.
pub const ERROR_DS_DISALLOWED_NC_REDIRECT: HRESULT = 8640;

/// The functional level of the AD LDS configuration set cannot be lowered to the requested value.
pub const ERROR_DS_HIGH_ADLDS_FFL: HRESULT = 8641;

/// The functional level of the domain (or forest) cannot be lowered to the requested value.
pub const ERROR_DS_HIGH_DSA_VERSION: HRESULT = 8642;

/// The functional level of the AD LDS configuration set cannot be raised to the requested value, because there exist one or more ADLDS instances that are at a lower incompatible functional level.
pub const ERROR_DS_LOW_ADLDS_FFL: HRESULT = 8643;

/// The domain join cannot be completed because the SID of the domain you attempted to join was identical to the SID of this machine. This is a symptom of an improperly cloned operating system install.  You should run sysprep on this machine in order to generate a new machine SID. Please see <http://go.microsoft.com/fwlink/?LinkId=168895> for more information.
pub const ERROR_DOMAIN_SID_SAME_AS_LOCAL_WORKSTATION: HRESULT = 8644;

/// The undelete operation failed because the Sam Account Name or Additional Sam Account Name of the object being undeleted conflicts with an existing live object.
pub const ERROR_DS_UNDELETE_SAM_VALIDATION_FAILED: HRESULT = 8645;

/// The system is not authoritative for the specified account and therefore cannot complete the operation. Please retry the operation using the provider associated with this account. If this is an online provider please use the provider's online site.
pub const ERROR_INCORRECT_ACCOUNT_TYPE: HRESULT = 8646;

/// The operation failed because SPN value provided for addition/modification is not unique forest-wide.
pub const ERROR_DS_SPN_VALUE_NOT_UNIQUE_IN_FOREST: HRESULT = 8647;

/// The operation failed because UPN value provided for addition/modification is not unique forest-wide.
pub const ERROR_DS_UPN_VALUE_NOT_UNIQUE_IN_FOREST: HRESULT = 8648;

/// The operation failed because the addition/modification referenced an inbound forest-wide trust that is not present.
pub const ERROR_DS_MISSING_FOREST_TRUST: HRESULT = 8649;

/// The link value specified was not found, but a link value with that key was found.
pub const ERROR_DS_VALUE_KEY_NOT_UNIQUE: HRESULT = 8650;

/// The Security Account Manager blocked the use of a weak Windows Hello for Business key.
pub const ERROR_WEAK_WHFBKEY_BLOCKED: HRESULT = 8651;

/// The add object operation failed because the caller was not authorized to add one or more attributes included in the request.
pub const ERROR_DS_PER_ATTRIBUTE_AUTHZ_FAILED_DURING_ADD: HRESULT = 8652;

/// The local account policy modification request was rejected because the policy is controlled by a regional authority.
pub const ERROR_LOCAL_POLICY_MODIFICATION_NOT_SUPPORTED: HRESULT = 8653;

/// The account is controlled by external policy and cannot be modified.
pub const ERROR_POLICY_CONTROLLED_ACCOUNT: HRESULT = 8654;

/// The Local Administrator Password Solution password update operation failed because the legacy LAPS schema needs to be added to Active Directory.
pub const ERROR_LAPS_LEGACY_SCHEMA_MISSING: HRESULT = 8655;

/// The Local Administrator Password Solution password update operation failed because the Windows LAPS schema needs to be added to Active Directory.
pub const ERROR_LAPS_SCHEMA_MISSING: HRESULT = 8656;

/// The Local Administrator Password Solution encrypted password update operation failed because Active Directory is not yet running at the minimum required domain functional level (2016).
pub const ERROR_LAPS_ENCRYPTION_REQUIRES_2016_DFL: HRESULT = 8657;

/// DNS server unable to interpret format.
pub const DNS_ERROR_RCODE_FORMAT_ERROR: HRESULT = 9001;

/// DNS server failure.
pub const DNS_ERROR_RCODE_SERVER_FAILURE: HRESULT = 9002;

/// DNS name does not exist.
pub const DNS_ERROR_RCODE_NAME_ERROR: HRESULT = 9003;

/// DNS request not supported by name server.
pub const DNS_ERROR_RCODE_NOT_IMPLEMENTED: HRESULT = 9004;

/// DNS operation refused.
pub const DNS_ERROR_RCODE_REFUSED: HRESULT = 9005;

/// DNS name that ought not exist, does exist.
pub const DNS_ERROR_RCODE_YXDOMAIN: HRESULT = 9006;

/// DNS RR set that ought not exist, does exist.
pub const DNS_ERROR_RCODE_YXRRSET: HRESULT = 9007;

/// DNS RR set that ought to exist, does not exist.
pub const DNS_ERROR_RCODE_NXRRSET: HRESULT = 9008;

/// DNS server not authoritative for zone.
pub const DNS_ERROR_RCODE_NOTAUTH: HRESULT = 9009;

/// DNS name in update or prereq is not in zone.
pub const DNS_ERROR_RCODE_NOTZONE: HRESULT = 9010;

/// DNS signature failed to verify.
pub const DNS_ERROR_RCODE_BADSIG: HRESULT = 9016;

/// DNS bad key.
pub const DNS_ERROR_RCODE_BADKEY: HRESULT = 9017;

/// DNS signature validity expired.
pub const DNS_ERROR_RCODE_BADTIME: HRESULT = 9018;

/// DNS signature validity expired.
pub const DNS_ERROR_RCODE_LAST: HRESULT = DNS_ERROR_RCODE_BADTIME;

/// Only the DNS server acting as the key master for the zone may perform this operation.
pub const DNS_ERROR_KEYMASTER_REQUIRED: HRESULT = 9101;

/// This operation is not allowed on a zone that is signed or has signing keys.
pub const DNS_ERROR_NOT_ALLOWED_ON_SIGNED_ZONE: HRESULT = 9102;

/// NSEC3 is not compatible with the RSA-SHA-1 algorithm. Choose a different algorithm or use NSEC.
pub const DNS_ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1: HRESULT = 9103;

/// The zone does not have enough signing keys. There must be at least one key signing key (KSK) and at least one zone signing key (ZSK).
pub const DNS_ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS: HRESULT = 9104;

/// The specified algorithm is not supported.
pub const DNS_ERROR_UNSUPPORTED_ALGORITHM: HRESULT = 9105;

/// The specified key size is not supported.
pub const DNS_ERROR_INVALID_KEY_SIZE: HRESULT = 9106;

/// One or more of the signing keys for a zone are not accessible to the DNS server. Zone signing will not be operational until this error is resolved.
pub const DNS_ERROR_SIGNING_KEY_NOT_ACCESSIBLE: HRESULT = 9107;

/// The specified key storage provider does not support DPAPI++ data protection. Zone signing will not be operational until this error is resolved.
pub const DNS_ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION: HRESULT = 9108;

/// An unexpected DPAPI++ error was encountered. Zone signing will not be operational until this error is resolved.
pub const DNS_ERROR_UNEXPECTED_DATA_PROTECTION_ERROR: HRESULT = 9109;

/// An unexpected crypto error was encountered. Zone signing may not be operational until this error is resolved.
pub const DNS_ERROR_UNEXPECTED_CNG_ERROR: HRESULT = 9110;

/// The DNS server encountered a signing key with an unknown version. Zone signing will not be operational until this error is resolved.
pub const DNS_ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION: HRESULT = 9111;

/// The specified key service provider cannot be opened by the DNS server.
pub const DNS_ERROR_KSP_NOT_ACCESSIBLE: HRESULT = 9112;

/// The DNS server cannot accept any more signing keys with the specified algorithm and KSK flag value for this zone.
pub const DNS_ERROR_TOO_MANY_SKDS: HRESULT = 9113;

/// The specified rollover period is invalid.
pub const DNS_ERROR_INVALID_ROLLOVER_PERIOD: HRESULT = 9114;

/// The specified initial rollover offset is invalid.
pub const DNS_ERROR_INVALID_INITIAL_ROLLOVER_OFFSET: HRESULT = 9115;

/// The specified signing key is already in process of rolling over keys.
pub const DNS_ERROR_ROLLOVER_IN_PROGRESS: HRESULT = 9116;

/// The specified signing key does not have a standby key to revoke.
pub const DNS_ERROR_STANDBY_KEY_NOT_PRESENT: HRESULT = 9117;

/// This operation is not allowed on a zone signing key (ZSK).
pub const DNS_ERROR_NOT_ALLOWED_ON_ZSK: HRESULT = 9118;

/// This operation is not allowed on an active signing key.
pub const DNS_ERROR_NOT_ALLOWED_ON_ACTIVE_SKD: HRESULT = 9119;

/// The specified signing key is already queued for rollover.
pub const DNS_ERROR_ROLLOVER_ALREADY_QUEUED: HRESULT = 9120;

/// This operation is not allowed on an unsigned zone.
pub const DNS_ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE: HRESULT = 9121;

/// This operation could not be completed because the DNS server listed as the current key master for this zone is down or misconfigured. Resolve the problem on the current key master for this zone or use another DNS server to seize the key master role.
pub const DNS_ERROR_BAD_KEYMASTER: HRESULT = 9122;

/// The specified signature validity period is invalid.
pub const DNS_ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD: HRESULT = 9123;

/// The specified NSEC3 iteration count is higher than allowed by the minimum key length used in the zone.
pub const DNS_ERROR_INVALID_NSEC3_ITERATION_COUNT: HRESULT = 9124;

/// This operation could not be completed because the DNS server has been configured with DNSSEC features disabled. Enable DNSSEC on the DNS server.
pub const DNS_ERROR_DNSSEC_IS_DISABLED: HRESULT = 9125;

/// This operation could not be completed because the XML stream received is empty or syntactically invalid.
pub const DNS_ERROR_INVALID_XML: HRESULT = 9126;

/// This operation completed, but no trust anchors were added because all of the trust anchors received were either invalid, unsupported, expired, or would not become valid in less than 30 days.
pub const DNS_ERROR_NO_VALID_TRUST_ANCHORS: HRESULT = 9127;

/// The specified signing key is not waiting for parental DS update.
pub const DNS_ERROR_ROLLOVER_NOT_POKEABLE: HRESULT = 9128;

/// Hash collision detected during NSEC3 signing. Specify a different user-provided salt, or use a randomly generated salt, and attempt to sign the zone again.
pub const DNS_ERROR_NSEC3_NAME_COLLISION: HRESULT = 9129;

/// NSEC is not compatible with the NSEC3-RSA-SHA-1 algorithm. Choose a different algorithm or use NSEC3.
pub const DNS_ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1: HRESULT = 9130;

/// No records found for given DNS query.
pub const DNS_INFO_NO_RECORDS: HRESULT = 9501;

/// Bad DNS packet.
pub const DNS_ERROR_BAD_PACKET: HRESULT = 9502;

/// No DNS packet.
pub const DNS_ERROR_NO_PACKET: HRESULT = 9503;

/// DNS error, check rcode.
pub const DNS_ERROR_RCODE: HRESULT = 9504;

/// Unsecured DNS packet.
pub const DNS_ERROR_UNSECURE_PACKET: HRESULT = 9505;

/// Unsecured DNS packet.
pub const DNS_STATUS_PACKET_UNSECURE: HRESULT = DNS_ERROR_UNSECURE_PACKET;

/// DNS query request is pending.
pub const DNS_REQUEST_PENDING: HRESULT = 9506;

/// Not enough memory resources are available to complete this operation.
pub const DNS_ERROR_NO_MEMORY: HRESULT = ERROR_OUTOFMEMORY;

/// The filename, directory name, or volume label syntax is incorrect.
pub const DNS_ERROR_INVALID_NAME: HRESULT = ERROR_INVALID_NAME;

/// The data is invalid.
pub const DNS_ERROR_INVALID_DATA: HRESULT = ERROR_INVALID_DATA;

/// Invalid DNS type.
pub const DNS_ERROR_INVALID_TYPE: HRESULT = 9551;

/// Invalid IP address.
pub const DNS_ERROR_INVALID_IP_ADDRESS: HRESULT = 9552;

/// Invalid property.
pub const DNS_ERROR_INVALID_PROPERTY: HRESULT = 9553;

/// Try DNS operation again later.
pub const DNS_ERROR_TRY_AGAIN_LATER: HRESULT = 9554;

/// Record for given name and type is not unique.
pub const DNS_ERROR_NOT_UNIQUE: HRESULT = 9555;

/// DNS name does not comply with RFC specifications.
pub const DNS_ERROR_NON_RFC_NAME: HRESULT = 9556;

/// DNS name is a fully-qualified DNS name.
pub const DNS_STATUS_FQDN: HRESULT = 9557;

/// DNS name is dotted (multi-label).
pub const DNS_STATUS_DOTTED_NAME: HRESULT = 9558;

/// DNS name is a single-part name.
pub const DNS_STATUS_SINGLE_PART_NAME: HRESULT = 9559;

/// DNS name contains an invalid character.
pub const DNS_ERROR_INVALID_NAME_CHAR: HRESULT = 9560;

/// DNS name is entirely numeric.
pub const DNS_ERROR_NUMERIC_NAME: HRESULT = 9561;

/// The operation requested is not permitted on a DNS root server.
pub const DNS_ERROR_NOT_ALLOWED_ON_ROOT_SERVER: HRESULT = 9562;

/// The record could not be created because this part of the DNS namespace has been delegated to another server.
pub const DNS_ERROR_NOT_ALLOWED_UNDER_DELEGATION: HRESULT = 9563;

/// The DNS server could not find a set of root hints.
pub const DNS_ERROR_CANNOT_FIND_ROOT_HINTS: HRESULT = 9564;

/// The DNS server found root hints but they were not consistent across all adapters.
pub const DNS_ERROR_INCONSISTENT_ROOT_HINTS: HRESULT = 9565;

/// The specified value is too small for this parameter.
pub const DNS_ERROR_DWORD_VALUE_TOO_SMALL: HRESULT = 9566;

/// The specified value is too large for this parameter.
pub const DNS_ERROR_DWORD_VALUE_TOO_LARGE: HRESULT = 9567;

/// This operation is not allowed while the DNS server is loading zones in the background. Please try again later.
pub const DNS_ERROR_BACKGROUND_LOADING: HRESULT = 9568;

/// The operation requested is not permitted on against a DNS server running on a read-only DC.
pub const DNS_ERROR_NOT_ALLOWED_ON_RODC: HRESULT = 9569;

/// No data is allowed to exist underneath a DNAME record.
pub const DNS_ERROR_NOT_ALLOWED_UNDER_DNAME: HRESULT = 9570;

/// This operation requires credentials delegation.
pub const DNS_ERROR_DELEGATION_REQUIRED: HRESULT = 9571;

/// Name resolution policy table has been corrupted. DNS resolution will fail until it is fixed. Contact your network administrator.
pub const DNS_ERROR_INVALID_POLICY_TABLE: HRESULT = 9572;

/// Not allowed to remove all addresses.
pub const DNS_ERROR_ADDRESS_REQUIRED: HRESULT = 9573;

/// DNS zone does not exist.
pub const DNS_ERROR_ZONE_DOES_NOT_EXIST: HRESULT = 9601;

/// DNS zone information not available.
pub const DNS_ERROR_NO_ZONE_INFO: HRESULT = 9602;

/// Invalid operation for DNS zone.
pub const DNS_ERROR_INVALID_ZONE_OPERATION: HRESULT = 9603;

/// Invalid DNS zone configuration.
pub const DNS_ERROR_ZONE_CONFIGURATION_ERROR: HRESULT = 9604;

/// DNS zone has no start of authority (SOA) record.
pub const DNS_ERROR_ZONE_HAS_NO_SOA_RECORD: HRESULT = 9605;

/// DNS zone has no Name Server (NS) record.
pub const DNS_ERROR_ZONE_HAS_NO_NS_RECORDS: HRESULT = 9606;

/// DNS zone is locked.
pub const DNS_ERROR_ZONE_LOCKED: HRESULT = 9607;

/// DNS zone creation failed.
pub const DNS_ERROR_ZONE_CREATION_FAILED: HRESULT = 9608;

/// DNS zone already exists.
pub const DNS_ERROR_ZONE_ALREADY_EXISTS: HRESULT = 9609;

/// DNS automatic zone already exists.
pub const DNS_ERROR_AUTOZONE_ALREADY_EXISTS: HRESULT = 9610;

/// Invalid DNS zone type.
pub const DNS_ERROR_INVALID_ZONE_TYPE: HRESULT = 9611;

/// Secondary DNS zone requires master IP address.
pub const DNS_ERROR_SECONDARY_REQUIRES_MASTER_IP: HRESULT = 9612;

/// DNS zone not secondary.
pub const DNS_ERROR_ZONE_NOT_SECONDARY: HRESULT = 9613;

/// Need secondary IP address.
pub const DNS_ERROR_NEED_SECONDARY_ADDRESSES: HRESULT = 9614;

/// WINS initialization failed.
pub const DNS_ERROR_WINS_INIT_FAILED: HRESULT = 9615;

/// Need WINS servers.
pub const DNS_ERROR_NEED_WINS_SERVERS: HRESULT = 9616;

/// NBTSTAT initialization call failed.
pub const DNS_ERROR_NBSTAT_INIT_FAILED: HRESULT = 9617;

/// Invalid delete of start of authority (SOA)
pub const DNS_ERROR_SOA_DELETE_INVALID: HRESULT = 9618;

/// A conditional forwarding zone already exists for that name.
pub const DNS_ERROR_FORWARDER_ALREADY_EXISTS: HRESULT = 9619;

/// This zone must be configured with one or more master DNS server IP addresses.
pub const DNS_ERROR_ZONE_REQUIRES_MASTER_IP: HRESULT = 9620;

/// The operation cannot be performed because this zone is shut down.
pub const DNS_ERROR_ZONE_IS_SHUTDOWN: HRESULT = 9621;

/// This operation cannot be performed because the zone is currently being signed. Please try again later.
pub const DNS_ERROR_ZONE_LOCKED_FOR_SIGNING: HRESULT = 9622;

/// Primary DNS zone requires datafile.
pub const DNS_ERROR_PRIMARY_REQUIRES_DATAFILE: HRESULT = 9651;

/// Invalid datafile name for DNS zone.
pub const DNS_ERROR_INVALID_DATAFILE_NAME: HRESULT = 9652;

/// Failed to open datafile for DNS zone.
pub const DNS_ERROR_DATAFILE_OPEN_FAILURE: HRESULT = 9653;

/// Failed to write datafile for DNS zone.
pub const DNS_ERROR_FILE_WRITEBACK_FAILED: HRESULT = 9654;

/// Failure while reading datafile for DNS zone.
pub const DNS_ERROR_DATAFILE_PARSING: HRESULT = 9655;

/// DNS record does not exist.
pub const DNS_ERROR_RECORD_DOES_NOT_EXIST: HRESULT = 9701;

/// DNS record format error.
pub const DNS_ERROR_RECORD_FORMAT: HRESULT = 9702;

/// Node creation failure in DNS.
pub const DNS_ERROR_NODE_CREATION_FAILED: HRESULT = 9703;

/// Unknown DNS record type.
pub const DNS_ERROR_UNKNOWN_RECORD_TYPE: HRESULT = 9704;

/// DNS record timed out.
pub const DNS_ERROR_RECORD_TIMED_OUT: HRESULT = 9705;

/// Name not in DNS zone.
pub const DNS_ERROR_NAME_NOT_IN_ZONE: HRESULT = 9706;

/// CNAME loop detected.
pub const DNS_ERROR_CNAME_LOOP: HRESULT = 9707;

/// Node is a CNAME DNS record.
pub const DNS_ERROR_NODE_IS_CNAME: HRESULT = 9708;

/// A CNAME record already exists for given name.
pub const DNS_ERROR_CNAME_COLLISION: HRESULT = 9709;

/// Record only at DNS zone root.
pub const DNS_ERROR_RECORD_ONLY_AT_ZONE_ROOT: HRESULT = 9710;

/// DNS record already exists.
pub const DNS_ERROR_RECORD_ALREADY_EXISTS: HRESULT = 9711;

/// Secondary DNS zone data error.
pub const DNS_ERROR_SECONDARY_DATA: HRESULT = 9712;

/// Could not create DNS cache data.
pub const DNS_ERROR_NO_CREATE_CACHE_DATA: HRESULT = 9713;

/// DNS name does not exist.
pub const DNS_ERROR_NAME_DOES_NOT_EXIST: HRESULT = 9714;

/// Could not create pointer (PTR) record.
pub const DNS_WARNING_PTR_CREATE_FAILED: HRESULT = 9715;

/// DNS domain was undeleted.
pub const DNS_WARNING_DOMAIN_UNDELETED: HRESULT = 9716;

/// The directory service is unavailable.
pub const DNS_ERROR_DS_UNAVAILABLE: HRESULT = 9717;

/// DNS zone already exists in the directory service.
pub const DNS_ERROR_DS_ZONE_ALREADY_EXISTS: HRESULT = 9718;

/// DNS server not creating or reading the boot file for the directory service integrated DNS zone.
pub const DNS_ERROR_NO_BOOTFILE_IF_DS_ZONE: HRESULT = 9719;

/// Node is a DNAME DNS record.
pub const DNS_ERROR_NODE_IS_DNAME: HRESULT = 9720;

/// A DNAME record already exists for given name.
pub const DNS_ERROR_DNAME_COLLISION: HRESULT = 9721;

/// An alias loop has been detected with either CNAME or DNAME records.
pub const DNS_ERROR_ALIAS_LOOP: HRESULT = 9722;

/// DNS AXFR (zone transfer) complete.
pub const DNS_INFO_AXFR_COMPLETE: HRESULT = 9751;

/// DNS zone transfer failed.
pub const DNS_ERROR_AXFR: HRESULT = 9752;

/// Added local WINS server.
pub const DNS_INFO_ADDED_LOCAL_WINS: HRESULT = 9753;

/// Secure update call needs to continue update request.
pub const DNS_STATUS_CONTINUE_NEEDED: HRESULT = 9801;

/// TCP/IP network protocol not installed.
pub const DNS_ERROR_NO_TCPIP: HRESULT = 9851;

/// No DNS servers configured for local system.
pub const DNS_ERROR_NO_DNS_SERVERS: HRESULT = 9852;

/// The specified directory partition does not exist.
pub const DNS_ERROR_DP_DOES_NOT_EXIST: HRESULT = 9901;

/// The specified directory partition already exists.
pub const DNS_ERROR_DP_ALREADY_EXISTS: HRESULT = 9902;

/// This DNS server is not enlisted in the specified directory partition.
pub const DNS_ERROR_DP_NOT_ENLISTED: HRESULT = 9903;

/// This DNS server is already enlisted in the specified directory partition.
pub const DNS_ERROR_DP_ALREADY_ENLISTED: HRESULT = 9904;

/// The directory partition is not available at this time. Please wait a few minutes and try again.
pub const DNS_ERROR_DP_NOT_AVAILABLE: HRESULT = 9905;

/// The operation failed because the domain naming master FSMO role could not be reached. The domain controller holding the domain naming master FSMO role is down or unable to service the request or is not running Windows Server 2003 or later.
pub const DNS_ERROR_DP_FSMO_ERROR: HRESULT = 9906;

/// The RRL is not enabled.
pub const DNS_ERROR_RRL_NOT_ENABLED: HRESULT = 9911;

/// The window size parameter is invalid. It should be greater than or equal to 1.
pub const DNS_ERROR_RRL_INVALID_WINDOW_SIZE: HRESULT = 9912;

/// The IPv4 prefix length parameter is invalid. It should be less than or equal to 32.
pub const DNS_ERROR_RRL_INVALID_IPV4_PREFIX: HRESULT = 9913;

/// The IPv6 prefix length parameter is invalid. It should be less than or equal to 128.
pub const DNS_ERROR_RRL_INVALID_IPV6_PREFIX: HRESULT = 9914;

/// The TC Rate parameter is invalid. It should be less than 10.
pub const DNS_ERROR_RRL_INVALID_TC_RATE: HRESULT = 9915;

/// The Leak Rate parameter is invalid. It should be either 0, or between 2 and 10.
pub const DNS_ERROR_RRL_INVALID_LEAK_RATE: HRESULT = 9916;

/// The Leak Rate or TC Rate parameter is invalid. Leak Rate should be greater than TC Rate.
pub const DNS_ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE: HRESULT = 9917;

/// The virtualization instance already exists.
pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS: HRESULT = 9921;

/// The virtualization instance does not exist.
pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST: HRESULT = 9922;

/// The virtualization tree is locked.
pub const DNS_ERROR_VIRTUALIZATION_TREE_LOCKED: HRESULT = 9923;

/// Invalid virtualization instance name.
pub const DNS_ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME: HRESULT = 9924;

/// The default virtualization instance cannot be added, removed or modified.
pub const DNS_ERROR_DEFAULT_VIRTUALIZATION_INSTANCE: HRESULT = 9925;

/// The scope already exists for the zone.
pub const DNS_ERROR_ZONESCOPE_ALREADY_EXISTS: HRESULT = 9951;

/// The scope does not exist for the zone.
pub const DNS_ERROR_ZONESCOPE_DOES_NOT_EXIST: HRESULT = 9952;

/// The scope is the same as the default zone scope.
pub const DNS_ERROR_DEFAULT_ZONESCOPE: HRESULT = 9953;

/// The scope name contains invalid characters.
pub const DNS_ERROR_INVALID_ZONESCOPE_NAME: HRESULT = 9954;

/// Operation not allowed when the zone has scopes.
pub const DNS_ERROR_NOT_ALLOWED_WITH_ZONESCOPES: HRESULT = 9955;

/// Failed to load zone scope.
pub const DNS_ERROR_LOAD_ZONESCOPE_FAILED: HRESULT = 9956;

/// Failed to write data file for DNS zone scope. Please verify the file exists and is writable.
pub const DNS_ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED: HRESULT = 9957;

/// The scope name contains invalid characters.
pub const DNS_ERROR_INVALID_SCOPE_NAME: HRESULT = 9958;

/// The scope does not exist.
pub const DNS_ERROR_SCOPE_DOES_NOT_EXIST: HRESULT = 9959;

/// The scope is the same as the default scope.
pub const DNS_ERROR_DEFAULT_SCOPE: HRESULT = 9960;

/// The operation is invalid on the scope.
pub const DNS_ERROR_INVALID_SCOPE_OPERATION: HRESULT = 9961;

/// The scope is locked.
pub const DNS_ERROR_SCOPE_LOCKED: HRESULT = 9962;

/// The scope already exists.
pub const DNS_ERROR_SCOPE_ALREADY_EXISTS: HRESULT = 9963;

/// A policy with the same name already exists on this level (server level or zone level) on the DNS server.
pub const DNS_ERROR_POLICY_ALREADY_EXISTS: HRESULT = 9971;

/// No policy with this name exists on this level (server level or zone level) on the DNS server.
pub const DNS_ERROR_POLICY_DOES_NOT_EXIST: HRESULT = 9972;

/// The criteria provided in the policy are invalid.
pub const DNS_ERROR_POLICY_INVALID_CRITERIA: HRESULT = 9973;

/// At least one of the settings of this policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_SETTINGS: HRESULT = 9974;

/// The client subnet cannot be deleted while it is being accessed by a policy.
pub const DNS_ERROR_CLIENT_SUBNET_IS_ACCESSED: HRESULT = 9975;

/// The client subnet does not exist on the DNS server.
pub const DNS_ERROR_CLIENT_SUBNET_DOES_NOT_EXIST: HRESULT = 9976;

/// A client subnet with this name already exists on the DNS server.
pub const DNS_ERROR_CLIENT_SUBNET_ALREADY_EXISTS: HRESULT = 9977;

/// The IP subnet specified does not exist in the client subnet.
pub const DNS_ERROR_SUBNET_DOES_NOT_EXIST: HRESULT = 9978;

/// The IP subnet that is being added, already exists in the client subnet.
pub const DNS_ERROR_SUBNET_ALREADY_EXISTS: HRESULT = 9979;

/// The policy is locked.
pub const DNS_ERROR_POLICY_LOCKED: HRESULT = 9980;

/// The weight of the scope in the policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_WEIGHT: HRESULT = 9981;

/// The DNS policy name is invalid.
pub const DNS_ERROR_POLICY_INVALID_NAME: HRESULT = 9982;

/// The policy is missing criteria.
pub const DNS_ERROR_POLICY_MISSING_CRITERIA: HRESULT = 9983;

/// The name of the the client subnet record is invalid.
pub const DNS_ERROR_INVALID_CLIENT_SUBNET_NAME: HRESULT = 9984;

/// Invalid policy processing order.
pub const DNS_ERROR_POLICY_PROCESSING_ORDER_INVALID: HRESULT = 9985;

/// The scope information has not been provided for a policy that requires it.
pub const DNS_ERROR_POLICY_SCOPE_MISSING: HRESULT = 9986;

/// The scope information has been provided for a policy that does not require it.
pub const DNS_ERROR_POLICY_SCOPE_NOT_ALLOWED: HRESULT = 9987;

/// The server scope cannot be deleted because it is referenced by a DNS Policy.
pub const DNS_ERROR_SERVERSCOPE_IS_REFERENCED: HRESULT = 9988;

/// The zone scope cannot be deleted because it is referenced by a DNS Policy.
pub const DNS_ERROR_ZONESCOPE_IS_REFERENCED: HRESULT = 9989;

/// The criterion client subnet provided in the policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET: HRESULT = 9990;

/// The criterion transport protocol provided in the policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL: HRESULT = 9991;

/// The criterion network protocol provided in the policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL: HRESULT = 9992;

/// The criterion interface provided in the policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_INTERFACE: HRESULT = 9993;

/// The criterion FQDN provided in the policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_FQDN: HRESULT = 9994;

/// The criterion query type provided in the policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE: HRESULT = 9995;

/// The criterion time of day provided in the policy is invalid.
pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY: HRESULT = 9996;

/// A blocking operation was interrupted by a call to WSACancelBlockingCall.
pub const WSAEINTR: HRESULT = 10004;

/// The file handle supplied is not valid.
pub const WSAEBADF: HRESULT = 10009;

/// An attempt was made to access a socket in a way forbidden by its access permissions.
pub const WSAEACCES: HRESULT = 10013;

/// The system detected an invalid pointer address in attempting to use a pointer argument in a call.
pub const WSAEFAULT: HRESULT = 10014;

/// An invalid argument was supplied.
pub const WSAEINVAL: HRESULT = 10022;

/// Too many open sockets.
pub const WSAEMFILE: HRESULT = 10024;

/// A non-blocking socket operation could not be completed immediately.
pub const WSAEWOULDBLOCK: HRESULT = 10035;

/// A blocking operation is currently executing.
pub const WSAEINPROGRESS: HRESULT = 10036;

/// An operation was attempted on a non-blocking socket that already had an operation in progress.
pub const WSAEALREADY: HRESULT = 10037;

/// An operation was attempted on something that is not a socket.
pub const WSAENOTSOCK: HRESULT = 10038;

/// A required address was omitted from an operation on a socket.
pub const WSAEDESTADDRREQ: HRESULT = 10039;

/// A message sent on a datagram socket was larger than the internal message buffer or some other network limit, or the buffer used to receive a datagram into was smaller than the datagram itself.
pub const WSAEMSGSIZE: HRESULT = 10040;

/// A protocol was specified in the socket function call that does not support the semantics of the socket type requested.
pub const WSAEPROTOTYPE: HRESULT = 10041;

/// An unknown, invalid, or unsupported option or level was specified in a getsockopt or setsockopt call.
pub const WSAENOPROTOOPT: HRESULT = 10042;

/// The requested protocol has not been configured into the system, or no implementation for it exists.
pub const WSAEPROTONOSUPPORT: HRESULT = 10043;

/// The support for the specified socket type does not exist in this address family.
pub const WSAESOCKTNOSUPPORT: HRESULT = 10044;

/// The attempted operation is not supported for the type of object referenced.
pub const WSAEOPNOTSUPP: HRESULT = 10045;

/// The protocol family has not been configured into the system or no implementation for it exists.
pub const WSAEPFNOSUPPORT: HRESULT = 10046;

/// An address incompatible with the requested protocol was used.
pub const WSAEAFNOSUPPORT: HRESULT = 10047;

/// Only one usage of each socket address (protocol/network address/port) is normally permitted.
pub const WSAEADDRINUSE: HRESULT = 10048;

/// The requested address is not valid in its context.
pub const WSAEADDRNOTAVAIL: HRESULT = 10049;

/// A socket operation encountered a dead network.
pub const WSAENETDOWN: HRESULT = 10050;

/// A socket operation was attempted to an unreachable network.
pub const WSAENETUNREACH: HRESULT = 10051;

/// The connection has been broken due to keep-alive activity detecting a failure while the operation was in progress.
pub const WSAENETRESET: HRESULT = 10052;

/// An established connection was aborted by the software in your host machine.
pub const WSAECONNABORTED: HRESULT = 10053;

/// An existing connection was forcibly closed by the remote host.
pub const WSAECONNRESET: HRESULT = 10054;

/// An operation on a socket could not be performed because the system lacked sufficient buffer space or because a queue was full.
pub const WSAENOBUFS: HRESULT = 10055;

/// A connect request was made on an already connected socket.
pub const WSAEISCONN: HRESULT = 10056;

/// A request to send or receive data was disallowed because the socket is not connected and (when sending on a datagram socket using a sendto call) no address was supplied.
pub const WSAENOTCONN: HRESULT = 10057;

/// A request to send or receive data was disallowed because the socket had already been shut down in that direction with a previous shutdown call.
pub const WSAESHUTDOWN: HRESULT = 10058;

/// Too many references to some kernel object.
pub const WSAETOOMANYREFS: HRESULT = 10059;

/// A connection attempt failed because the connected party did not properly respond after a period of time, or established connection failed because connected host has failed to respond.
pub const WSAETIMEDOUT: HRESULT = 10060;

/// No connection could be made because the target machine actively refused it.
pub const WSAECONNREFUSED: HRESULT = 10061;

/// Cannot translate name.
pub const WSAELOOP: HRESULT = 10062;

/// Name component or name was too long.
pub const WSAENAMETOOLONG: HRESULT = 10063;

/// A socket operation failed because the destination host was down.
pub const WSAEHOSTDOWN: HRESULT = 10064;

/// A socket operation was attempted to an unreachable host.
pub const WSAEHOSTUNREACH: HRESULT = 10065;

/// Cannot remove a directory that is not empty.
pub const WSAENOTEMPTY: HRESULT = 10066;

/// A Windows Sockets implementation may have a limit on the number of applications that may use it simultaneously.
pub const WSAEPROCLIM: HRESULT = 10067;

/// Ran out of quota.
pub const WSAEUSERS: HRESULT = 10068;

/// Ran out of disk quota.
pub const WSAEDQUOT: HRESULT = 10069;

/// File handle reference is no longer available.
pub const WSAESTALE: HRESULT = 10070;

/// Item is not available locally.
pub const WSAEREMOTE: HRESULT = 10071;

/// WSAStartup cannot function at this time because the underlying system it uses to provide network services is currently unavailable.
pub const WSASYSNOTREADY: HRESULT = 10091;

/// The Windows Sockets version requested is not supported.
pub const WSAVERNOTSUPPORTED: HRESULT = 10092;

/// Either the application has not called WSAStartup, or WSAStartup failed.
pub const WSANOTINITIALISED: HRESULT = 10093;

/// Returned by WSARecv or WSARecvFrom to indicate the remote party has initiated a graceful shutdown sequence.
pub const WSAEDISCON: HRESULT = 10101;

/// No more results can be returned by WSALookupServiceNext.
pub const WSAENOMORE: HRESULT = 10102;

/// A call to WSALookupServiceEnd was made while this call was still processing. The call has been canceled.
pub const WSAECANCELLED: HRESULT = 10103;

/// The procedure call table is invalid.
pub const WSAEINVALIDPROCTABLE: HRESULT = 10104;

/// The requested service provider is invalid.
pub const WSAEINVALIDPROVIDER: HRESULT = 10105;

/// The requested service provider could not be loaded or initialized.
pub const WSAEPROVIDERFAILEDINIT: HRESULT = 10106;

/// A system call has failed.
pub const WSASYSCALLFAILURE: HRESULT = 10107;

/// No such service is known. The service cannot be found in the specified name space.
pub const WSASERVICE_NOT_FOUND: HRESULT = 10108;

/// The specified class was not found.
pub const WSATYPE_NOT_FOUND: HRESULT = 10109;

/// No more results can be returned by WSALookupServiceNext.
pub const WSA_E_NO_MORE: HRESULT = 10110;

/// A call to WSALookupServiceEnd was made while this call was still processing. The call has been canceled.
pub const WSA_E_CANCELLED: HRESULT = 10111;

/// A database query failed because it was actively refused.
pub const WSAEREFUSED: HRESULT = 10112;

/// No such host is known.
pub const WSAHOST_NOT_FOUND: HRESULT = 11001;

/// This is usually a temporary error during hostname resolution and means that the local server did not receive a response from an authoritative server.
pub const WSATRY_AGAIN: HRESULT = 11002;

/// A non-recoverable error occurred during a database lookup.
pub const WSANO_RECOVERY: HRESULT = 11003;

/// The requested name is valid, but no data of the requested type was found.
pub const WSANO_DATA: HRESULT = 11004;

/// At least one reserve has arrived.
pub const WSA_QOS_RECEIVERS: HRESULT = 11005;

/// At least one path has arrived.
pub const WSA_QOS_SENDERS: HRESULT = 11006;

/// There are no senders.
pub const WSA_QOS_NO_SENDERS: HRESULT = 11007;

/// There are no receivers.
pub const WSA_QOS_NO_RECEIVERS: HRESULT = 11008;

/// Reserve has been confirmed.
pub const WSA_QOS_REQUEST_CONFIRMED: HRESULT = 11009;

/// Error due to lack of resources.
pub const WSA_QOS_ADMISSION_FAILURE: HRESULT = 11010;

/// Rejected for administrative reasons - bad credentials.
pub const WSA_QOS_POLICY_FAILURE: HRESULT = 11011;

/// Unknown or conflicting style.
pub const WSA_QOS_BAD_STYLE: HRESULT = 11012;

/// Problem with some part of the filterspec or providerspecific buffer in general.
pub const WSA_QOS_BAD_OBJECT: HRESULT = 11013;

/// Problem with some part of the flowspec.
pub const WSA_QOS_TRAFFIC_CTRL_ERROR: HRESULT = 11014;

/// General QOS error.
pub const WSA_QOS_GENERIC_ERROR: HRESULT = 11015;

/// An invalid or unrecognized service type was found in the flowspec.
pub const WSA_QOS_ESERVICETYPE: HRESULT = 11016;

/// An invalid or inconsistent flowspec was found in the QOS structure.
pub const WSA_QOS_EFLOWSPEC: HRESULT = 11017;

/// Invalid QOS provider-specific buffer.
pub const WSA_QOS_EPROVSPECBUF: HRESULT = 11018;

/// An invalid QOS filter style was used.
pub const WSA_QOS_EFILTERSTYLE: HRESULT = 11019;

/// An invalid QOS filter type was used.
pub const WSA_QOS_EFILTERTYPE: HRESULT = 11020;

/// An incorrect number of QOS FILTERSPECs were specified in the FLOWDESCRIPTOR.
pub const WSA_QOS_EFILTERCOUNT: HRESULT = 11021;

/// An object with an invalid ObjectLength field was specified in the QOS provider-specific buffer.
pub const WSA_QOS_EOBJLENGTH: HRESULT = 11022;

/// An incorrect number of flow descriptors was specified in the QOS structure.
pub const WSA_QOS_EFLOWCOUNT: HRESULT = 11023;

/// An unrecognized object was found in the QOS provider-specific buffer.
pub const WSA_QOS_EUNKOWNPSOBJ: HRESULT = 11024;

/// An invalid policy object was found in the QOS provider-specific buffer.
pub const WSA_QOS_EPOLICYOBJ: HRESULT = 11025;

/// An invalid QOS flow descriptor was found in the flow descriptor list.
pub const WSA_QOS_EFLOWDESC: HRESULT = 11026;

/// An invalid or inconsistent flowspec was found in the QOS provider specific buffer.
pub const WSA_QOS_EPSFLOWSPEC: HRESULT = 11027;

/// An invalid FILTERSPEC was found in the QOS provider-specific buffer.
pub const WSA_QOS_EPSFILTERSPEC: HRESULT = 11028;

/// An invalid shape discard mode object was found in the QOS provider specific buffer.
pub const WSA_QOS_ESDMODEOBJ: HRESULT = 11029;

/// An invalid shaping rate object was found in the QOS provider-specific buffer.
pub const WSA_QOS_ESHAPERATEOBJ: HRESULT = 11030;

/// A reserved policy element was found in the QOS provider-specific buffer.
pub const WSA_QOS_RESERVED_PETYPE: HRESULT = 11031;

/// No such host is known securely.
pub const WSA_SECURE_HOST_NOT_FOUND: HRESULT = 11032;

/// Name based IPSEC policy could not be added.
pub const WSA_IPSEC_NAME_POLICY_ERROR: HRESULT = 11033;

/// The specified quick mode policy already exists.
pub const ERROR_IPSEC_QM_POLICY_EXISTS: HRESULT = 13000;

/// The specified quick mode policy was not found.
pub const ERROR_IPSEC_QM_POLICY_NOT_FOUND: HRESULT = 13001;

/// The specified quick mode policy is being used.
pub const ERROR_IPSEC_QM_POLICY_IN_USE: HRESULT = 13002;

/// The specified main mode policy already exists.
pub const ERROR_IPSEC_MM_POLICY_EXISTS: HRESULT = 13003;

/// The specified main mode policy was not found
pub const ERROR_IPSEC_MM_POLICY_NOT_FOUND: HRESULT = 13004;

/// The specified main mode policy is being used.
pub const ERROR_IPSEC_MM_POLICY_IN_USE: HRESULT = 13005;

/// The specified main mode filter already exists.
pub const ERROR_IPSEC_MM_FILTER_EXISTS: HRESULT = 13006;

/// The specified main mode filter was not found.
pub const ERROR_IPSEC_MM_FILTER_NOT_FOUND: HRESULT = 13007;

/// The specified transport mode filter already exists.
pub const ERROR_IPSEC_TRANSPORT_FILTER_EXISTS: HRESULT = 13008;

/// The specified transport mode filter does not exist.
pub const ERROR_IPSEC_TRANSPORT_FILTER_NOT_FOUND: HRESULT = 13009;

/// The specified main mode authentication list exists.
pub const ERROR_IPSEC_MM_AUTH_EXISTS: HRESULT = 13010;

/// The specified main mode authentication list was not found.
pub const ERROR_IPSEC_MM_AUTH_NOT_FOUND: HRESULT = 13011;

/// The specified main mode authentication list is being used.
pub const ERROR_IPSEC_MM_AUTH_IN_USE: HRESULT = 13012;

/// The specified default main mode policy was not found.
pub const ERROR_IPSEC_DEFAULT_MM_POLICY_NOT_FOUND: HRESULT = 13013;

/// The specified default main mode authentication list was not found.
pub const ERROR_IPSEC_DEFAULT_MM_AUTH_NOT_FOUND: HRESULT = 13014;

/// The specified default quick mode policy was not found.
pub const ERROR_IPSEC_DEFAULT_QM_POLICY_NOT_FOUND: HRESULT = 13015;

/// The specified tunnel mode filter exists.
pub const ERROR_IPSEC_TUNNEL_FILTER_EXISTS: HRESULT = 13016;

/// The specified tunnel mode filter was not found.
pub const ERROR_IPSEC_TUNNEL_FILTER_NOT_FOUND: HRESULT = 13017;

/// The Main Mode filter is pending deletion.
pub const ERROR_IPSEC_MM_FILTER_PENDING_DELETION: HRESULT = 13018;

/// The transport filter is pending deletion.
pub const ERROR_IPSEC_TRANSPORT_FILTER_PENDING_DELETION: HRESULT = 13019;

/// The tunnel filter is pending deletion.
pub const ERROR_IPSEC_TUNNEL_FILTER_PENDING_DELETION: HRESULT = 13020;

/// The Main Mode policy is pending deletion.
pub const ERROR_IPSEC_MM_POLICY_PENDING_DELETION: HRESULT = 13021;

/// The Main Mode authentication bundle is pending deletion.
pub const ERROR_IPSEC_MM_AUTH_PENDING_DELETION: HRESULT = 13022;

/// The Quick Mode policy is pending deletion.
pub const ERROR_IPSEC_QM_POLICY_PENDING_DELETION: HRESULT = 13023;

/// The Main Mode policy was successfully added, but some of the requested offers are not supported.
pub const WARNING_IPSEC_MM_POLICY_PRUNED: HRESULT = 13024;

/// The Quick Mode policy was successfully added, but some of the requested offers are not supported.
pub const WARNING_IPSEC_QM_POLICY_PRUNED: HRESULT = 13025;

///  ERROR_IPSEC_IKE_NEG_STATUS_BEGIN
pub const ERROR_IPSEC_IKE_NEG_STATUS_BEGIN: HRESULT = 13800;

/// IKE authentication credentials are unacceptable
pub const ERROR_IPSEC_IKE_AUTH_FAIL: HRESULT = 13801;

/// IKE security attributes are unacceptable
pub const ERROR_IPSEC_IKE_ATTRIB_FAIL: HRESULT = 13802;

/// IKE Negotiation in progress
pub const ERROR_IPSEC_IKE_NEGOTIATION_PENDING: HRESULT = 13803;

/// General processing error
pub const ERROR_IPSEC_IKE_GENERAL_PROCESSING_ERROR: HRESULT = 13804;

/// Negotiation timed out
pub const ERROR_IPSEC_IKE_TIMED_OUT: HRESULT = 13805;

/// IKE failed to find valid machine certificate. Contact your Network Security Administrator about installing a valid certificate in the appropriate Certificate Store.
pub const ERROR_IPSEC_IKE_NO_CERT: HRESULT = 13806;

/// IKE SA deleted by peer before establishment completed
pub const ERROR_IPSEC_IKE_SA_DELETED: HRESULT = 13807;

/// IKE SA deleted before establishment completed
pub const ERROR_IPSEC_IKE_SA_REAPED: HRESULT = 13808;

/// Negotiation request sat in Queue too long
pub const ERROR_IPSEC_IKE_MM_ACQUIRE_DROP: HRESULT = 13809;

/// Negotiation request sat in Queue too long
pub const ERROR_IPSEC_IKE_QM_ACQUIRE_DROP: HRESULT = 13810;

/// Negotiation request sat in Queue too long
pub const ERROR_IPSEC_IKE_QUEUE_DROP_MM: HRESULT = 13811;

/// Negotiation request sat in Queue too long
pub const ERROR_IPSEC_IKE_QUEUE_DROP_NO_MM: HRESULT = 13812;

/// No response from peer
pub const ERROR_IPSEC_IKE_DROP_NO_RESPONSE: HRESULT = 13813;

/// Negotiation took too long
pub const ERROR_IPSEC_IKE_MM_DELAY_DROP: HRESULT = 13814;

/// Negotiation took too long
pub const ERROR_IPSEC_IKE_QM_DELAY_DROP: HRESULT = 13815;

/// Unknown error occurred
pub const ERROR_IPSEC_IKE_ERROR: HRESULT = 13816;

/// Certificate Revocation Check failed
pub const ERROR_IPSEC_IKE_CRL_FAILED: HRESULT = 13817;

/// Invalid certificate key usage
pub const ERROR_IPSEC_IKE_INVALID_KEY_USAGE: HRESULT = 13818;

/// Invalid certificate type
pub const ERROR_IPSEC_IKE_INVALID_CERT_TYPE: HRESULT = 13819;

/// IKE negotiation failed because the machine certificate used does not have a private key. IPsec certificates require a private key. Contact your Network Security administrator about replacing with a certificate that has a private key.
pub const ERROR_IPSEC_IKE_NO_PRIVATE_KEY: HRESULT = 13820;

/// Simultaneous rekeys were detected.
pub const ERROR_IPSEC_IKE_SIMULTANEOUS_REKEY: HRESULT = 13821;

/// Failure in Diffie-Hellman computation
pub const ERROR_IPSEC_IKE_DH_FAIL: HRESULT = 13822;

/// Don't know how to process critical payload
pub const ERROR_IPSEC_IKE_CRITICAL_PAYLOAD_NOT_RECOGNIZED: HRESULT = 13823;

/// Invalid header
pub const ERROR_IPSEC_IKE_INVALID_HEADER: HRESULT = 13824;

/// No policy configured
pub const ERROR_IPSEC_IKE_NO_POLICY: HRESULT = 13825;

/// Failed to verify signature
pub const ERROR_IPSEC_IKE_INVALID_SIGNATURE: HRESULT = 13826;

/// Failed to authenticate using Kerberos
pub const ERROR_IPSEC_IKE_KERBEROS_ERROR: HRESULT = 13827;

/// Peer's certificate did not have a public key
pub const ERROR_IPSEC_IKE_NO_PUBLIC_KEY: HRESULT = 13828;

/// Error processing error payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR: HRESULT = 13829;

/// Error processing SA payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_SA: HRESULT = 13830;

/// Error processing Proposal payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_PROP: HRESULT = 13831;

/// Error processing Transform payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_TRANS: HRESULT = 13832;

/// Error processing KE payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_KE: HRESULT = 13833;

/// Error processing ID payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_ID: HRESULT = 13834;

/// Error processing Cert payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT: HRESULT = 13835;

/// Error processing Certificate Request payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT_REQ: HRESULT = 13836;

/// Error processing Hash payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_HASH: HRESULT = 13837;

/// Error processing Signature payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_SIG: HRESULT = 13838;

/// Error processing Nonce payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NONCE: HRESULT = 13839;

/// Error processing Notify payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NOTIFY: HRESULT = 13840;

/// Error processing Delete Payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_DELETE: HRESULT = 13841;

/// Error processing VendorId payload
pub const ERROR_IPSEC_IKE_PROCESS_ERR_VENDOR: HRESULT = 13842;

/// Invalid payload received
pub const ERROR_IPSEC_IKE_INVALID_PAYLOAD: HRESULT = 13843;

/// Soft SA loaded
pub const ERROR_IPSEC_IKE_LOAD_SOFT_SA: HRESULT = 13844;

/// Soft SA torn down
pub const ERROR_IPSEC_IKE_SOFT_SA_TORN_DOWN: HRESULT = 13845;

/// Invalid cookie received.
pub const ERROR_IPSEC_IKE_INVALID_COOKIE: HRESULT = 13846;

/// Peer failed to send valid machine certificate
pub const ERROR_IPSEC_IKE_NO_PEER_CERT: HRESULT = 13847;

/// Certification Revocation check of peer's certificate failed
pub const ERROR_IPSEC_IKE_PEER_CRL_FAILED: HRESULT = 13848;

/// New policy invalidated SAs formed with old policy
pub const ERROR_IPSEC_IKE_POLICY_CHANGE: HRESULT = 13849;

/// There is no available Main Mode IKE policy.
pub const ERROR_IPSEC_IKE_NO_MM_POLICY: HRESULT = 13850;

/// Failed to enabled TCB privilege.
pub const ERROR_IPSEC_IKE_NOTCBPRIV: HRESULT = 13851;

/// Failed to load SECURITY.DLL.
pub const ERROR_IPSEC_IKE_SECLOADFAIL: HRESULT = 13852;

/// Failed to obtain security function table dispatch address from SSPI.
pub const ERROR_IPSEC_IKE_FAILSSPINIT: HRESULT = 13853;

/// Failed to query Kerberos package to obtain max token size.
pub const ERROR_IPSEC_IKE_FAILQUERYSSP: HRESULT = 13854;

/// Failed to obtain Kerberos server credentials for ISAKMP/ERROR_IPSEC_IKE service. Kerberos authentication will not function. The most likely reason for this is lack of domain membership. This is normal if your computer is a member of a workgroup.
pub const ERROR_IPSEC_IKE_SRVACQFAIL: HRESULT = 13855;

/// Failed to determine SSPI principal name for ISAKMP/ERROR_IPSEC_IKE service (QueryCredentialsAttributes).
pub const ERROR_IPSEC_IKE_SRVQUERYCRED: HRESULT = 13856;

/// Failed to obtain new SPI for the inbound SA from IPsec driver. The most common cause for this is that the driver does not have the correct filter. Check your policy to verify the filters.
pub const ERROR_IPSEC_IKE_GETSPIFAIL: HRESULT = 13857;

/// Given filter is invalid
pub const ERROR_IPSEC_IKE_INVALID_FILTER: HRESULT = 13858;

/// Memory allocation failed.
pub const ERROR_IPSEC_IKE_OUT_OF_MEMORY: HRESULT = 13859;

/// Failed to add Security Association to IPsec Driver. The most common cause for this is if the IKE negotiation took too long to complete. If the problem persists, reduce the load on the faulting machine.
pub const ERROR_IPSEC_IKE_ADD_UPDATE_KEY_FAILED: HRESULT = 13860;

/// Invalid policy
pub const ERROR_IPSEC_IKE_INVALID_POLICY: HRESULT = 13861;

/// Invalid DOI
pub const ERROR_IPSEC_IKE_UNKNOWN_DOI: HRESULT = 13862;

/// Invalid situation
pub const ERROR_IPSEC_IKE_INVALID_SITUATION: HRESULT = 13863;

/// Diffie-Hellman failure
pub const ERROR_IPSEC_IKE_DH_FAILURE: HRESULT = 13864;

/// Invalid Diffie-Hellman group
pub const ERROR_IPSEC_IKE_INVALID_GROUP: HRESULT = 13865;

/// Error encrypting payload
pub const ERROR_IPSEC_IKE_ENCRYPT: HRESULT = 13866;

/// Error decrypting payload
pub const ERROR_IPSEC_IKE_DECRYPT: HRESULT = 13867;

/// Policy match error
pub const ERROR_IPSEC_IKE_POLICY_MATCH: HRESULT = 13868;

/// Unsupported ID
pub const ERROR_IPSEC_IKE_UNSUPPORTED_ID: HRESULT = 13869;

/// Hash verification failed
pub const ERROR_IPSEC_IKE_INVALID_HASH: HRESULT = 13870;

/// Invalid hash algorithm
pub const ERROR_IPSEC_IKE_INVALID_HASH_ALG: HRESULT = 13871;

/// Invalid hash size
pub const ERROR_IPSEC_IKE_INVALID_HASH_SIZE: HRESULT = 13872;

/// Invalid encryption algorithm
pub const ERROR_IPSEC_IKE_INVALID_ENCRYPT_ALG: HRESULT = 13873;

/// Invalid authentication algorithm
pub const ERROR_IPSEC_IKE_INVALID_AUTH_ALG: HRESULT = 13874;

/// Invalid certificate signature
pub const ERROR_IPSEC_IKE_INVALID_SIG: HRESULT = 13875;

/// Load failed
pub const ERROR_IPSEC_IKE_LOAD_FAILED: HRESULT = 13876;

/// Deleted via RPC call
pub const ERROR_IPSEC_IKE_RPC_DELETE: HRESULT = 13877;

/// Temporary state created to perform reinitialization. This is not a real failure.
pub const ERROR_IPSEC_IKE_BENIGN_REINIT: HRESULT = 13878;

/// The lifetime value received in the Responder Lifetime Notify is below the Windows 2000 configured minimum value. Please fix the policy on the peer machine.
pub const ERROR_IPSEC_IKE_INVALID_RESPONDER_LIFETIME_NOTIFY: HRESULT = 13879;

/// The recipient cannot handle version of IKE specified in the header.
pub const ERROR_IPSEC_IKE_INVALID_MAJOR_VERSION: HRESULT = 13880;

/// Key length in certificate is too small for configured security requirements.
pub const ERROR_IPSEC_IKE_INVALID_CERT_KEYLEN: HRESULT = 13881;

/// Max number of established MM SAs to peer exceeded.
pub const ERROR_IPSEC_IKE_MM_LIMIT: HRESULT = 13882;

/// IKE received a policy that disables negotiation.
pub const ERROR_IPSEC_IKE_NEGOTIATION_DISABLED: HRESULT = 13883;

/// Reached maximum quick mode limit for the main mode. New main mode will be started.
pub const ERROR_IPSEC_IKE_QM_LIMIT: HRESULT = 13884;

/// Main mode SA lifetime expired or peer sent a main mode delete.
pub const ERROR_IPSEC_IKE_MM_EXPIRED: HRESULT = 13885;

/// Main mode SA assumed to be invalid because peer stopped responding.
pub const ERROR_IPSEC_IKE_PEER_MM_ASSUMED_INVALID: HRESULT = 13886;

/// Certificate doesn't chain to a trusted root in IPsec policy.
pub const ERROR_IPSEC_IKE_CERT_CHAIN_POLICY_MISMATCH: HRESULT = 13887;

/// Received unexpected message ID.
pub const ERROR_IPSEC_IKE_UNEXPECTED_MESSAGE_ID: HRESULT = 13888;

/// Received invalid authentication offers.
pub const ERROR_IPSEC_IKE_INVALID_AUTH_PAYLOAD: HRESULT = 13889;

/// Sent DoS cookie notify to initiator.
pub const ERROR_IPSEC_IKE_DOS_COOKIE_SENT: HRESULT = 13890;

/// IKE service is shutting down.
pub const ERROR_IPSEC_IKE_SHUTTING_DOWN: HRESULT = 13891;

/// Could not verify binding between CGA address and certificate.
pub const ERROR_IPSEC_IKE_CGA_AUTH_FAILED: HRESULT = 13892;

/// Error processing NatOA payload.
pub const ERROR_IPSEC_IKE_PROCESS_ERR_NATOA: HRESULT = 13893;

/// Parameters of the main mode are invalid for this quick mode.
pub const ERROR_IPSEC_IKE_INVALID_MM_FOR_QM: HRESULT = 13894;

/// Quick mode SA was expired by IPsec driver.
pub const ERROR_IPSEC_IKE_QM_EXPIRED: HRESULT = 13895;

/// Too many dynamically added IKEEXT filters were detected.
pub const ERROR_IPSEC_IKE_TOO_MANY_FILTERS: HRESULT = 13896;

///  ERROR_IPSEC_IKE_NEG_STATUS_END
pub const ERROR_IPSEC_IKE_NEG_STATUS_END: HRESULT = 13897;

/// NAP reauth succeeded and must delete the dummy NAP IKEv2 tunnel.
pub const ERROR_IPSEC_IKE_KILL_DUMMY_NAP_TUNNEL: HRESULT = 13898;

/// Error in assigning inner IP address to initiator in tunnel mode.
pub const ERROR_IPSEC_IKE_INNER_IP_ASSIGNMENT_FAILURE: HRESULT = 13899;

/// Require configuration payload missing.
pub const ERROR_IPSEC_IKE_REQUIRE_CP_PAYLOAD_MISSING: HRESULT = 13900;

/// A negotiation running as the security principle who issued the connection is in progress
pub const ERROR_IPSEC_KEY_MODULE_IMPERSONATION_NEGOTIATION_PENDING: HRESULT = 13901;

/// SA was deleted due to IKEv1/AuthIP co-existence suppress check.
pub const ERROR_IPSEC_IKE_COEXISTENCE_SUPPRESS: HRESULT = 13902;

/// Incoming SA request was dropped due to peer IP address rate limiting.
pub const ERROR_IPSEC_IKE_RATELIMIT_DROP: HRESULT = 13903;

/// Peer does not support MOBIKE.
pub const ERROR_IPSEC_IKE_PEER_DOESNT_SUPPORT_MOBIKE: HRESULT = 13904;

/// SA establishment is not authorized.
pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE: HRESULT = 13905;

/// SA establishment is not authorized because there is not a sufficiently strong PKINIT-based credential.
pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_FAILURE: HRESULT = 13906;

/// SA establishment is not authorized.  You may need to enter updated or different credentials such as a smartcard.
pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE_WITH_OPTIONAL_RETRY: HRESULT = 13907;

/// SA establishment is not authorized because there is not a sufficiently strong PKINIT-based credential. This might be related to certificate-to-account mapping failure for the SA.
pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_AND_CERTMAP_FAILURE: HRESULT = 13908;

///  ERROR_IPSEC_IKE_NEG_STATUS_EXTENDED_END
pub const ERROR_IPSEC_IKE_NEG_STATUS_EXTENDED_END: HRESULT = 13909;

/// The SPI in the packet does not match a valid IPsec SA.
pub const ERROR_IPSEC_BAD_SPI: HRESULT = 13910;

/// Packet was received on an IPsec SA whose lifetime has expired.
pub const ERROR_IPSEC_SA_LIFETIME_EXPIRED: HRESULT = 13911;

/// Packet was received on an IPsec SA that does not match the packet characteristics.
pub const ERROR_IPSEC_WRONG_SA: HRESULT = 13912;

/// Packet sequence number replay check failed.
pub const ERROR_IPSEC_REPLAY_CHECK_FAILED: HRESULT = 13913;

/// IPsec header and/or trailer in the packet is invalid.
pub const ERROR_IPSEC_INVALID_PACKET: HRESULT = 13914;

/// IPsec integrity check failed.
pub const ERROR_IPSEC_INTEGRITY_CHECK_FAILED: HRESULT = 13915;

/// IPsec dropped a clear text packet.
pub const ERROR_IPSEC_CLEAR_TEXT_DROP: HRESULT = 13916;

/// IPsec dropped an incoming ESP packet in authenticated firewall mode. This drop is benign.
pub const ERROR_IPSEC_AUTH_FIREWALL_DROP: HRESULT = 13917;

/// IPsec dropped a packet due to DoS throttling.
pub const ERROR_IPSEC_THROTTLE_DROP: HRESULT = 13918;

/// IPsec DoS Protection matched an explicit block rule.
pub const ERROR_IPSEC_DOSP_BLOCK: HRESULT = 13925;

/// IPsec DoS Protection received an IPsec specific multicast packet which is not allowed.
pub const ERROR_IPSEC_DOSP_RECEIVED_MULTICAST: HRESULT = 13926;

/// IPsec DoS Protection received an incorrectly formatted packet.
pub const ERROR_IPSEC_DOSP_INVALID_PACKET: HRESULT = 13927;

/// IPsec DoS Protection failed to look up state.
pub const ERROR_IPSEC_DOSP_STATE_LOOKUP_FAILED: HRESULT = 13928;

/// IPsec DoS Protection failed to create state because the maximum number of entries allowed by policy has been reached.
pub const ERROR_IPSEC_DOSP_MAX_ENTRIES: HRESULT = 13929;

/// IPsec DoS Protection received an IPsec negotiation packet for a keying module which is not allowed by policy.
pub const ERROR_IPSEC_DOSP_KEYMOD_NOT_ALLOWED: HRESULT = 13930;

/// IPsec DoS Protection has not been enabled.
pub const ERROR_IPSEC_DOSP_NOT_INSTALLED: HRESULT = 13931;

/// IPsec DoS Protection failed to create a per internal IP rate limit queue because the maximum number of queues allowed by policy has been reached.
pub const ERROR_IPSEC_DOSP_MAX_PER_IP_RATELIMIT_QUEUES: HRESULT = 13932;

/// The requested section was not present in the activation context.
pub const ERROR_SXS_SECTION_NOT_FOUND: HRESULT = 14000;

/// The application has failed to start because its side-by-side configuration is incorrect. Please see the application event log or use the command-line sxstrace.exe tool for more detail.
pub const ERROR_SXS_CANT_GEN_ACTCTX: HRESULT = 14001;

/// The application binding data format is invalid.
pub const ERROR_SXS_INVALID_ACTCTXDATA_FORMAT: HRESULT = 14002;

/// The referenced assembly is not installed on your system.
pub const ERROR_SXS_ASSEMBLY_NOT_FOUND: HRESULT = 14003;

/// The manifest file does not begin with the required tag and format information.
pub const ERROR_SXS_MANIFEST_FORMAT_ERROR: HRESULT = 14004;

/// The manifest file contains one or more syntax errors.
pub const ERROR_SXS_MANIFEST_PARSE_ERROR: HRESULT = 14005;

/// The application attempted to activate a disabled activation context.
pub const ERROR_SXS_ACTIVATION_CONTEXT_DISABLED: HRESULT = 14006;

/// The requested lookup key was not found in any active activation context.
pub const ERROR_SXS_KEY_NOT_FOUND: HRESULT = 14007;

/// A component version required by the application conflicts with another component version already active.
pub const ERROR_SXS_VERSION_CONFLICT: HRESULT = 14008;

/// The type requested activation context section does not match the query API used.
pub const ERROR_SXS_WRONG_SECTION_TYPE: HRESULT = 14009;

/// Lack of system resources has required isolated activation to be disabled for the current thread of execution.
pub const ERROR_SXS_THREAD_QUERIES_DISABLED: HRESULT = 14010;

/// An attempt to set the process default activation context failed because the process default activation context was already set.
pub const ERROR_SXS_PROCESS_DEFAULT_ALREADY_SET: HRESULT = 14011;

/// The encoding group identifier specified is not recognized.
pub const ERROR_SXS_UNKNOWN_ENCODING_GROUP: HRESULT = 14012;

/// The encoding requested is not recognized.
pub const ERROR_SXS_UNKNOWN_ENCODING: HRESULT = 14013;

/// The manifest contains a reference to an invalid URI.
pub const ERROR_SXS_INVALID_XML_NAMESPACE_URI: HRESULT = 14014;

/// The application manifest contains a reference to a dependent assembly which is not installed
pub const ERROR_SXS_ROOT_MANIFEST_DEPENDENCY_NOT_INSTALLED: HRESULT = 14015;

/// The manifest for an assembly used by the application has a reference to a dependent assembly which is not installed
pub const ERROR_SXS_LEAF_MANIFEST_DEPENDENCY_NOT_INSTALLED: HRESULT = 14016;

/// The manifest contains an attribute for the assembly identity which is not valid.
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE: HRESULT = 14017;

/// The manifest is missing the required default namespace specification on the assembly element.
pub const ERROR_SXS_MANIFEST_MISSING_REQUIRED_DEFAULT_NAMESPACE: HRESULT = 14018;

/// The manifest has a default namespace specified on the assembly element but its value is not "urn:schemas-microsoft-com:asm.v1".
pub const ERROR_SXS_MANIFEST_INVALID_REQUIRED_DEFAULT_NAMESPACE: HRESULT = 14019;

/// The private manifest probed has crossed a path with an unsupported reparse point.
pub const ERROR_SXS_PRIVATE_MANIFEST_CROSS_PATH_WITH_REPARSE_POINT: HRESULT = 14020;

/// Two or more components referenced directly or indirectly by the application manifest have files by the same name.
pub const ERROR_SXS_DUPLICATE_DLL_NAME: HRESULT = 14021;

/// Two or more components referenced directly or indirectly by the application manifest have window classes with the same name.
pub const ERROR_SXS_DUPLICATE_WINDOWCLASS_NAME: HRESULT = 14022;

/// Two or more components referenced directly or indirectly by the application manifest have the same COM server CLSIDs.
pub const ERROR_SXS_DUPLICATE_CLSID: HRESULT = 14023;

/// Two or more components referenced directly or indirectly by the application manifest have proxies for the same COM interface IIDs.
pub const ERROR_SXS_DUPLICATE_IID: HRESULT = 14024;

/// Two or more components referenced directly or indirectly by the application manifest have the same COM type library TLBIDs.
pub const ERROR_SXS_DUPLICATE_TLBID: HRESULT = 14025;

/// Two or more components referenced directly or indirectly by the application manifest have the same COM ProgIDs.
pub const ERROR_SXS_DUPLICATE_PROGID: HRESULT = 14026;

/// Two or more components referenced directly or indirectly by the application manifest are different versions of the same component which is not permitted.
pub const ERROR_SXS_DUPLICATE_ASSEMBLY_NAME: HRESULT = 14027;

/// A component's file does not match the verification information present in the component manifest.
pub const ERROR_SXS_FILE_HASH_MISMATCH: HRESULT = 14028;

/// The policy manifest contains one or more syntax errors.
pub const ERROR_SXS_POLICY_PARSE_ERROR: HRESULT = 14029;

/// Manifest Parse Error : A string literal was expected, but no opening quote character was found.
pub const ERROR_SXS_XML_E_MISSINGQUOTE: HRESULT = 14030;

/// Manifest Parse Error : Incorrect syntax was used in a comment.
pub const ERROR_SXS_XML_E_COMMENTSYNTAX: HRESULT = 14031;

/// Manifest Parse Error : A name was started with an invalid character.
pub const ERROR_SXS_XML_E_BADSTARTNAMECHAR: HRESULT = 14032;

/// Manifest Parse Error : A name contained an invalid character.
pub const ERROR_SXS_XML_E_BADNAMECHAR: HRESULT = 14033;

/// Manifest Parse Error : A string literal contained an invalid character.
pub const ERROR_SXS_XML_E_BADCHARINSTRING: HRESULT = 14034;

/// Manifest Parse Error : Invalid syntax for an xml declaration.
pub const ERROR_SXS_XML_E_XMLDECLSYNTAX: HRESULT = 14035;

/// Manifest Parse Error : An Invalid character was found in text content.
pub const ERROR_SXS_XML_E_BADCHARDATA: HRESULT = 14036;

/// Manifest Parse Error : Required white space was missing.
pub const ERROR_SXS_XML_E_MISSINGWHITESPACE: HRESULT = 14037;

/// Manifest Parse Error : The character '>' was expected.
pub const ERROR_SXS_XML_E_EXPECTINGTAGEND: HRESULT = 14038;

/// Manifest Parse Error : A semi colon character was expected.
pub const ERROR_SXS_XML_E_MISSINGSEMICOLON: HRESULT = 14039;

/// Manifest Parse Error : Unbalanced parentheses.
pub const ERROR_SXS_XML_E_UNBALANCEDPAREN: HRESULT = 14040;

/// Manifest Parse Error : Internal error.
pub const ERROR_SXS_XML_E_INTERNALERROR: HRESULT = 14041;

/// Manifest Parse Error : Whitespace is not allowed at this location.
pub const ERROR_SXS_XML_E_UNEXPECTED_WHITESPACE: HRESULT = 14042;

/// Manifest Parse Error : End of file reached in invalid state for current encoding.
pub const ERROR_SXS_XML_E_INCOMPLETE_ENCODING: HRESULT = 14043;

/// Manifest Parse Error : Missing parenthesis.
pub const ERROR_SXS_XML_E_MISSING_PAREN: HRESULT = 14044;

/// Manifest Parse Error : A single or double closing quote character (\' or \") is missing.
pub const ERROR_SXS_XML_E_EXPECTINGCLOSEQUOTE: HRESULT = 14045;

/// Manifest Parse Error : Multiple colons are not allowed in a name.
pub const ERROR_SXS_XML_E_MULTIPLE_COLONS: HRESULT = 14046;

/// Manifest Parse Error : Invalid character for decimal digit.
pub const ERROR_SXS_XML_E_INVALID_DECIMAL: HRESULT = 14047;

/// Manifest Parse Error : Invalid character for hexadecimal digit.
pub const ERROR_SXS_XML_E_INVALID_HEXIDECIMAL: HRESULT = 14048;

/// Manifest Parse Error : Invalid unicode character value for this platform.
pub const ERROR_SXS_XML_E_INVALID_UNICODE: HRESULT = 14049;

/// Manifest Parse Error : Expecting whitespace or '?'.
pub const ERROR_SXS_XML_E_WHITESPACEORQUESTIONMARK: HRESULT = 14050;

/// Manifest Parse Error : End tag was not expected at this location.
pub const ERROR_SXS_XML_E_UNEXPECTEDENDTAG: HRESULT = 14051;

/// Manifest Parse Error : The following tags were not closed: %1.
pub const ERROR_SXS_XML_E_UNCLOSEDTAG: HRESULT = 14052;

/// Manifest Parse Error : Duplicate attribute.
pub const ERROR_SXS_XML_E_DUPLICATEATTRIBUTE: HRESULT = 14053;

/// Manifest Parse Error : Only one top level element is allowed in an XML document.
pub const ERROR_SXS_XML_E_MULTIPLEROOTS: HRESULT = 14054;

/// Manifest Parse Error : Invalid at the top level of the document.
pub const ERROR_SXS_XML_E_INVALIDATROOTLEVEL: HRESULT = 14055;

/// Manifest Parse Error : Invalid xml declaration.
pub const ERROR_SXS_XML_E_BADXMLDECL: HRESULT = 14056;

/// Manifest Parse Error : XML document must have a top level element.
pub const ERROR_SXS_XML_E_MISSINGROOT: HRESULT = 14057;

/// Manifest Parse Error : Unexpected end of file.
pub const ERROR_SXS_XML_E_UNEXPECTEDEOF: HRESULT = 14058;

/// Manifest Parse Error : Parameter entities cannot be used inside markup declarations in an internal subset.
pub const ERROR_SXS_XML_E_BADPEREFINSUBSET: HRESULT = 14059;

/// Manifest Parse Error : Element was not closed.
pub const ERROR_SXS_XML_E_UNCLOSEDSTARTTAG: HRESULT = 14060;

/// Manifest Parse Error : End element was missing the character '>'.
pub const ERROR_SXS_XML_E_UNCLOSEDENDTAG: HRESULT = 14061;

/// Manifest Parse Error : A string literal was not closed.
pub const ERROR_SXS_XML_E_UNCLOSEDSTRING: HRESULT = 14062;

/// Manifest Parse Error : A comment was not closed.
pub const ERROR_SXS_XML_E_UNCLOSEDCOMMENT: HRESULT = 14063;

/// Manifest Parse Error : A declaration was not closed.
pub const ERROR_SXS_XML_E_UNCLOSEDDECL: HRESULT = 14064;

/// Manifest Parse Error : A CDATA section was not closed.
pub const ERROR_SXS_XML_E_UNCLOSEDCDATA: HRESULT = 14065;

/// Manifest Parse Error : The namespace prefix is not allowed to start with the reserved string "xml".
pub const ERROR_SXS_XML_E_RESERVEDNAMESPACE: HRESULT = 14066;

/// Manifest Parse Error : System does not support the specified encoding.
pub const ERROR_SXS_XML_E_INVALIDENCODING: HRESULT = 14067;

/// Manifest Parse Error : Switch from current encoding to specified encoding not supported.
pub const ERROR_SXS_XML_E_INVALIDSWITCH: HRESULT = 14068;

/// Manifest Parse Error : The name 'xml' is reserved and must be lower case.
pub const ERROR_SXS_XML_E_BADXMLCASE: HRESULT = 14069;

/// Manifest Parse Error : The standalone attribute must have the value 'yes' or 'no'.
pub const ERROR_SXS_XML_E_INVALID_STANDALONE: HRESULT = 14070;

/// Manifest Parse Error : The standalone attribute cannot be used in external entities.
pub const ERROR_SXS_XML_E_UNEXPECTED_STANDALONE: HRESULT = 14071;

/// Manifest Parse Error : Invalid version number.
pub const ERROR_SXS_XML_E_INVALID_VERSION: HRESULT = 14072;

/// Manifest Parse Error : Missing equals sign between attribute and attribute value.
pub const ERROR_SXS_XML_E_MISSINGEQUALS: HRESULT = 14073;

/// Assembly Protection Error : Unable to recover the specified assembly.
pub const ERROR_SXS_PROTECTION_RECOVERY_FAILED: HRESULT = 14074;

/// Assembly Protection Error : The public key for an assembly was too short to be allowed.
pub const ERROR_SXS_PROTECTION_PUBLIC_KEY_TOO_SHORT: HRESULT = 14075;

/// Assembly Protection Error : The catalog for an assembly is not valid, or does not match the assembly's manifest.
pub const ERROR_SXS_PROTECTION_CATALOG_NOT_VALID: HRESULT = 14076;

/// An HRESULT could not be translated to a corresponding Win32 error code.
pub const ERROR_SXS_UNTRANSLATABLE_HRESULT: HRESULT = 14077;

/// Assembly Protection Error : The catalog for an assembly is missing.
pub const ERROR_SXS_PROTECTION_CATALOG_FILE_MISSING: HRESULT = 14078;

/// The supplied assembly identity is missing one or more attributes which must be present in this context.
pub const ERROR_SXS_MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE: HRESULT = 14079;

/// The supplied assembly identity has one or more attribute names that contain characters not permitted in XML names.
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME: HRESULT = 14080;

/// The referenced assembly could not be found.
pub const ERROR_SXS_ASSEMBLY_MISSING: HRESULT = 14081;

/// The activation context activation stack for the running thread of execution is corrupt.
pub const ERROR_SXS_CORRUPT_ACTIVATION_STACK: HRESULT = 14082;

/// The application isolation metadata for this process or thread has become corrupt.
pub const ERROR_SXS_CORRUPTION: HRESULT = 14083;

/// The activation context being deactivated is not the most recently activated one.
pub const ERROR_SXS_EARLY_DEACTIVATION: HRESULT = 14084;

/// The activation context being deactivated is not active for the current thread of execution.
pub const ERROR_SXS_INVALID_DEACTIVATION: HRESULT = 14085;

/// The activation context being deactivated has already been deactivated.
pub const ERROR_SXS_MULTIPLE_DEACTIVATION: HRESULT = 14086;

/// A component used by the isolation facility has requested to terminate the process.
pub const ERROR_SXS_PROCESS_TERMINATION_REQUESTED: HRESULT = 14087;

/// A kernel mode component is releasing a reference on an activation context.
pub const ERROR_SXS_RELEASE_ACTIVATION_CONTEXT: HRESULT = 14088;

/// The activation context of system default assembly could not be generated.
pub const ERROR_SXS_SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY: HRESULT = 14089;

/// The value of an attribute in an identity is not within the legal range.
pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_VALUE: HRESULT = 14090;

/// The name of an attribute in an identity is not within the legal range.
pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_NAME: HRESULT = 14091;

/// An identity contains two definitions for the same attribute.
pub const ERROR_SXS_IDENTITY_DUPLICATE_ATTRIBUTE: HRESULT = 14092;

/// The identity string is malformed. This may be due to a trailing comma, more than two unnamed attributes, missing attribute name or missing attribute value.
pub const ERROR_SXS_IDENTITY_PARSE_ERROR: HRESULT = 14093;

/// A string containing localized substitutable content was malformed. Either a dollar sign ($) was followed by something other than a left parenthesis or another dollar sign or an substitution's right parenthesis was not found.
pub const ERROR_MALFORMED_SUBSTITUTION_STRING: HRESULT = 14094;

/// The public key token does not correspond to the public key specified.
pub const ERROR_SXS_INCORRECT_PUBLIC_KEY_TOKEN: HRESULT = 14095;

/// A substitution string had no mapping.
pub const ERROR_UNMAPPED_SUBSTITUTION_STRING: HRESULT = 14096;

/// The component must be locked before making the request.
pub const ERROR_SXS_ASSEMBLY_NOT_LOCKED: HRESULT = 14097;

/// The component store has been corrupted.
pub const ERROR_SXS_COMPONENT_STORE_CORRUPT: HRESULT = 14098;

/// An advanced installer failed during setup or servicing.
pub const ERROR_ADVANCED_INSTALLER_FAILED: HRESULT = 14099;

/// The character encoding in the XML declaration did not match the encoding used in the document.
pub const ERROR_XML_ENCODING_MISMATCH: HRESULT = 14100;

/// The identities of the manifests are identical but their contents are different.
pub const ERROR_SXS_MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT: HRESULT = 14101;

/// The component identities are different.
pub const ERROR_SXS_IDENTITIES_DIFFERENT: HRESULT = 14102;

/// The assembly is not a deployment.
pub const ERROR_SXS_ASSEMBLY_IS_NOT_A_DEPLOYMENT: HRESULT = 14103;

/// The file is not a part of the assembly.
pub const ERROR_SXS_FILE_NOT_PART_OF_ASSEMBLY: HRESULT = 14104;

/// The size of the manifest exceeds the maximum allowed.
pub const ERROR_SXS_MANIFEST_TOO_BIG: HRESULT = 14105;

/// The setting is not registered.
pub const ERROR_SXS_SETTING_NOT_REGISTERED: HRESULT = 14106;

/// One or more required members of the transaction are not present.
pub const ERROR_SXS_TRANSACTION_CLOSURE_INCOMPLETE: HRESULT = 14107;

/// The SMI primitive installer failed during setup or servicing.
pub const ERROR_SMI_PRIMITIVE_INSTALLER_FAILED: HRESULT = 14108;

/// A generic command executable returned a result that indicates failure.
pub const ERROR_GENERIC_COMMAND_FAILED: HRESULT = 14109;

/// A component is missing file verification information in its manifest.
pub const ERROR_SXS_FILE_HASH_MISSING: HRESULT = 14110;

/// Two or more components referenced directly or indirectly by the application manifest have the same WinRT ActivatableClass IDs.
pub const ERROR_SXS_DUPLICATE_ACTIVATABLE_CLASS: HRESULT = 14111;

/// The specified channel path is invalid.
pub const ERROR_EVT_INVALID_CHANNEL_PATH: HRESULT = 15000;

/// The specified query is invalid.
pub const ERROR_EVT_INVALID_QUERY: HRESULT = 15001;

/// The publisher metadata cannot be found in the resource.
pub const ERROR_EVT_PUBLISHER_METADATA_NOT_FOUND: HRESULT = 15002;

/// The template for an event definition cannot be found in the resource (error = %1).
pub const ERROR_EVT_EVENT_TEMPLATE_NOT_FOUND: HRESULT = 15003;

/// The specified publisher name is invalid.
pub const ERROR_EVT_INVALID_PUBLISHER_NAME: HRESULT = 15004;

/// The event data raised by the publisher is not compatible with the event template definition in the publisher's manifest.
pub const ERROR_EVT_INVALID_EVENT_DATA: HRESULT = 15005;

/// The specified channel could not be found.
pub const ERROR_EVT_CHANNEL_NOT_FOUND: HRESULT = 15007;

/// The specified XML text was not well-formed. See Extended Error for more details.
pub const ERROR_EVT_MALFORMED_XML_TEXT: HRESULT = 15008;

/// The events for a direct channel go directly to a log file and cannot be subscribed to.
pub const ERROR_EVT_SUBSCRIPTION_TO_DIRECT_CHANNEL: HRESULT = 15009;

/// Configuration error.
pub const ERROR_EVT_CONFIGURATION_ERROR: HRESULT = 15010;

/// The query result is stale or invalid and must be recreated. This may be due to the log being cleared or rolling over after the query result was created.
pub const ERROR_EVT_QUERY_RESULT_STALE: HRESULT = 15011;

/// The query result is currently at an invalid position.
pub const ERROR_EVT_QUERY_RESULT_INVALID_POSITION: HRESULT = 15012;

/// Registered MSXML doesn't support validation.
pub const ERROR_EVT_NON_VALIDATING_MSXML: HRESULT = 15013;

/// An expression can only be followed by a change-of-scope operation if the expression evaluates to a node set and is not already part of another change-of-scope operation.
pub const ERROR_EVT_FILTER_ALREADYSCOPED: HRESULT = 15014;

/// Cannot perform a step operation from a term that does not represent an element set.
pub const ERROR_EVT_FILTER_NOTELTSET: HRESULT = 15015;

/// Left-hand side arguments to binary operators must be either attributes, nodes or variables. Right-hand side arguments must be constants.
pub const ERROR_EVT_FILTER_INVARG: HRESULT = 15016;

/// A step operation must involve a node test or, in the case of a predicate, an algebraic expression against which to test each node in the preceeding node set.
pub const ERROR_EVT_FILTER_INVTEST: HRESULT = 15017;

/// This data type is currently unsupported.
pub const ERROR_EVT_FILTER_INVTYPE: HRESULT = 15018;

/// A syntax error occurred at position %1!d!
pub const ERROR_EVT_FILTER_PARSEERR: HRESULT = 15019;

/// This operator is unsupported by this implementation of the filter.
pub const ERROR_EVT_FILTER_UNSUPPORTEDOP: HRESULT = 15020;

/// An unexpected token was encountered.
pub const ERROR_EVT_FILTER_UNEXPECTEDTOKEN: HRESULT = 15021;

/// The requested operation cannot be performed over an enabled direct channel. The channel must first be disabled.
pub const ERROR_EVT_INVALID_OPERATION_OVER_ENABLED_DIRECT_CHANNEL: HRESULT = 15022;

/// Channel property %1!s! contains an invalid value. The value has an invalid type, is outside of its valid range, cannot be changed, or is not supported by this type of channel.
pub const ERROR_EVT_INVALID_CHANNEL_PROPERTY_VALUE: HRESULT = 15023;

/// Publisher property %1!s! contains an invalid value. The value has an invalid type, is outside of its valid range, cannot be changed, or is not supported by this type of publisher.
pub const ERROR_EVT_INVALID_PUBLISHER_PROPERTY_VALUE: HRESULT = 15024;

/// The channel failed to activate.
pub const ERROR_EVT_CHANNEL_CANNOT_ACTIVATE: HRESULT = 15025;

/// The XPath expression exceeded the supported complexity. Simplify the expression or split it into multiple expressions.
pub const ERROR_EVT_FILTER_TOO_COMPLEX: HRESULT = 15026;

/// The message resource is present but the message was not found in the message table.
pub const ERROR_EVT_MESSAGE_NOT_FOUND: HRESULT = 15027;

/// The message ID for the desired message could not be found.
pub const ERROR_EVT_MESSAGE_ID_NOT_FOUND: HRESULT = 15028;

/// The substitution string for insert index (%1) could not be found.
pub const ERROR_EVT_UNRESOLVED_VALUE_INSERT: HRESULT = 15029;

/// The description string for parameter reference (%1) could not be found.
pub const ERROR_EVT_UNRESOLVED_PARAMETER_INSERT: HRESULT = 15030;

/// The maximum number of replacements has been reached.
pub const ERROR_EVT_MAX_INSERTS_REACHED: HRESULT = 15031;

/// The event definition could not be found for event ID (%1).
pub const ERROR_EVT_EVENT_DEFINITION_NOT_FOUND: HRESULT = 15032;

/// The locale specific resource for the desired message is not present.
pub const ERROR_EVT_MESSAGE_LOCALE_NOT_FOUND: HRESULT = 15033;

/// The resource is too old and is not supported.
pub const ERROR_EVT_VERSION_TOO_OLD: HRESULT = 15034;

/// The resource is too new and is not supported.
pub const ERROR_EVT_VERSION_TOO_NEW: HRESULT = 15035;

/// The channel at index %1!d! of the query can't be opened.
pub const ERROR_EVT_CANNOT_OPEN_CHANNEL_OF_QUERY: HRESULT = 15036;

/// The publisher has been disabled and its resource is not available. This usually occurs when the publisher is in the process of being uninstalled or upgraded.
pub const ERROR_EVT_PUBLISHER_DISABLED: HRESULT = 15037;

/// Attempted to create a numeric type that is outside of its valid range.
pub const ERROR_EVT_FILTER_OUT_OF_RANGE: HRESULT = 15038;

/// The subscription fails to activate.
pub const ERROR_EC_SUBSCRIPTION_CANNOT_ACTIVATE: HRESULT = 15080;

/// The log of the subscription is in disabled state, and can not be used to forward events to. The log must first be enabled before the subscription can be activated.
pub const ERROR_EC_LOG_DISABLED: HRESULT = 15081;

/// When forwarding events from local machine to itself, the query of the subscription can't contain target log of the subscription.
pub const ERROR_EC_CIRCULAR_FORWARDING: HRESULT = 15082;

/// The credential store that is used to save credentials is full.
pub const ERROR_EC_CREDSTORE_FULL: HRESULT = 15083;

/// The credential used by this subscription can't be found in credential store.
pub const ERROR_EC_CRED_NOT_FOUND: HRESULT = 15084;

/// No active channel is found for the query.
pub const ERROR_EC_NO_ACTIVE_CHANNEL: HRESULT = 15085;

/// The resource loader failed to find MUI file.
pub const ERROR_MUI_FILE_NOT_FOUND: HRESULT = 15100;

/// The resource loader failed to load MUI file because the file fail to pass validation.
pub const ERROR_MUI_INVALID_FILE: HRESULT = 15101;

/// The RC Manifest is corrupted with garbage data or unsupported version or missing required item.
pub const ERROR_MUI_INVALID_RC_CONFIG: HRESULT = 15102;

/// The RC Manifest has invalid culture name.
pub const ERROR_MUI_INVALID_LOCALE_NAME: HRESULT = 15103;

/// The RC Manifest has invalid ultimatefallback name.
pub const ERROR_MUI_INVALID_ULTIMATEFALLBACK_NAME: HRESULT = 15104;

/// The resource loader cache doesn't have loaded MUI entry.
pub const ERROR_MUI_FILE_NOT_LOADED: HRESULT = 15105;

/// User stopped resource enumeration.
pub const ERROR_RESOURCE_ENUM_USER_STOP: HRESULT = 15106;

/// UI language installation failed.
pub const ERROR_MUI_INTLSETTINGS_UILANG_NOT_INSTALLED: HRESULT = 15107;

/// Locale installation failed.
pub const ERROR_MUI_INTLSETTINGS_INVALID_LOCALE_NAME: HRESULT = 15108;

/// A resource does not have default or neutral value.
pub const ERROR_MRM_RUNTIME_NO_DEFAULT_OR_NEUTRAL_RESOURCE: HRESULT = 15110;

/// Invalid PRI config file.
pub const ERROR_MRM_INVALID_PRICONFIG: HRESULT = 15111;

/// Invalid file type.
pub const ERROR_MRM_INVALID_FILE_TYPE: HRESULT = 15112;

/// Unknown qualifier.
pub const ERROR_MRM_UNKNOWN_QUALIFIER: HRESULT = 15113;

/// Invalid qualifier value.
pub const ERROR_MRM_INVALID_QUALIFIER_VALUE: HRESULT = 15114;

/// No Candidate found.
pub const ERROR_MRM_NO_CANDIDATE: HRESULT = 15115;

/// The ResourceMap or NamedResource has an item that does not have default or neutral resource..
pub const ERROR_MRM_NO_MATCH_OR_DEFAULT_CANDIDATE: HRESULT = 15116;

/// Invalid ResourceCandidate type.
pub const ERROR_MRM_RESOURCE_TYPE_MISMATCH: HRESULT = 15117;

/// Duplicate Resource Map.
pub const ERROR_MRM_DUPLICATE_MAP_NAME: HRESULT = 15118;

/// Duplicate Entry.
pub const ERROR_MRM_DUPLICATE_ENTRY: HRESULT = 15119;

/// Invalid Resource Identifier.
pub const ERROR_MRM_INVALID_RESOURCE_IDENTIFIER: HRESULT = 15120;

/// Filepath too long.
pub const ERROR_MRM_FILEPATH_TOO_LONG: HRESULT = 15121;

/// Unsupported directory type.
pub const ERROR_MRM_UNSUPPORTED_DIRECTORY_TYPE: HRESULT = 15122;

/// Invalid PRI File.
pub const ERROR_MRM_INVALID_PRI_FILE: HRESULT = 15126;

/// NamedResource Not Found.
pub const ERROR_MRM_NAMED_RESOURCE_NOT_FOUND: HRESULT = 15127;

/// ResourceMap Not Found.
pub const ERROR_MRM_MAP_NOT_FOUND: HRESULT = 15135;

/// Unsupported MRT profile type.
pub const ERROR_MRM_UNSUPPORTED_PROFILE_TYPE: HRESULT = 15136;

/// Invalid qualifier operator.
pub const ERROR_MRM_INVALID_QUALIFIER_OPERATOR: HRESULT = 15137;

/// Unable to determine qualifier value or qualifier value has not been set.
pub const ERROR_MRM_INDETERMINATE_QUALIFIER_VALUE: HRESULT = 15138;

/// Automerge is enabled in the PRI file.
pub const ERROR_MRM_AUTOMERGE_ENABLED: HRESULT = 15139;

/// Too many resources defined for package.
pub const ERROR_MRM_TOO_MANY_RESOURCES: HRESULT = 15140;

/// Resource File can not be used for merge operation.
pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_MERGE: HRESULT = 15141;

/// Load/UnloadPriFiles cannot be used with resource packages.
pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_LOAD_UNLOAD_PRI_FILE: HRESULT = 15142;

/// Resource Contexts may not be created on threads that do not have a CoreWindow.
pub const ERROR_MRM_NO_CURRENT_VIEW_ON_THREAD: HRESULT = 15143;

/// The singleton Resource Manager with different profile is already created.
pub const ERROR_DIFFERENT_PROFILE_RESOURCE_MANAGER_EXIST: HRESULT = 15144;

/// The system component cannot operate given API operation
pub const ERROR_OPERATION_NOT_ALLOWED_FROM_SYSTEM_COMPONENT: HRESULT = 15145;

/// The resource is a direct reference to a non-default resource candidate.
pub const ERROR_MRM_DIRECT_REF_TO_NON_DEFAULT_RESOURCE: HRESULT = 15146;

/// Resource Map has been re-generated and the query string is not valid anymore.
pub const ERROR_MRM_GENERATION_COUNT_MISMATCH: HRESULT = 15147;

/// The PRI files to be merged have incompatible versions.
pub const ERROR_PRI_MERGE_VERSION_MISMATCH: HRESULT = 15148;

/// The primary PRI files to be merged does not contain a schema.
pub const ERROR_PRI_MERGE_MISSING_SCHEMA: HRESULT = 15149;

/// Unable to load one of the PRI files to be merged.
pub const ERROR_PRI_MERGE_LOAD_FILE_FAILED: HRESULT = 15150;

/// Unable to add one of the PRI files to the merged file.
pub const ERROR_PRI_MERGE_ADD_FILE_FAILED: HRESULT = 15151;

/// Unable to create the merged PRI file.
pub const ERROR_PRI_MERGE_WRITE_FILE_FAILED: HRESULT = 15152;

/// Packages for a PRI file merge must all be from the same package family.
pub const ERROR_PRI_MERGE_MULTIPLE_PACKAGE_FAMILIES_NOT_ALLOWED: HRESULT = 15153;

/// Packages for a PRI file merge must not include multiple main packages.
pub const ERROR_PRI_MERGE_MULTIPLE_MAIN_PACKAGES_NOT_ALLOWED: HRESULT = 15154;

/// Packages for a PRI file merge must not include bundle packages.
pub const ERROR_PRI_MERGE_BUNDLE_PACKAGES_NOT_ALLOWED: HRESULT = 15155;

/// Packages for a PRI file merge must include one main package.
pub const ERROR_PRI_MERGE_MAIN_PACKAGE_REQUIRED: HRESULT = 15156;

/// Packages for a PRI file merge must include at least one resource package.
pub const ERROR_PRI_MERGE_RESOURCE_PACKAGE_REQUIRED: HRESULT = 15157;

/// Invalid name supplied for a canonical merged PRI file.
pub const ERROR_PRI_MERGE_INVALID_FILE_NAME: HRESULT = 15158;

/// Unable to find the specified package.
pub const ERROR_MRM_PACKAGE_NOT_FOUND: HRESULT = 15159;

/// No default value for language was specified.
pub const ERROR_MRM_MISSING_DEFAULT_LANGUAGE: HRESULT = 15160;

/// An entity was defined as both resource and scope, which is not allowed.
pub const ERROR_MRM_SCOPE_ITEM_CONFLICT: HRESULT = 15161;

/// The monitor returned a DDC/CI capabilities string that did not comply with the ACCESS.bus 3.0, DDC/CI 1.1 or MCCS 2 Revision 1 specification.
pub const ERROR_MCA_INVALID_CAPABILITIES_STRING: HRESULT = 15200;

/// The monitor's VCP Version (0xDF) VCP code returned an invalid version value.
pub const ERROR_MCA_INVALID_VCP_VERSION: HRESULT = 15201;

/// The monitor does not comply with the MCCS specification it claims to support.
pub const ERROR_MCA_MONITOR_VIOLATES_MCCS_SPECIFICATION: HRESULT = 15202;

/// The MCCS version in a monitor's mccs_ver capability does not match the MCCS version the monitor reports when the VCP Version (0xDF) VCP code is used.
pub const ERROR_MCA_MCCS_VERSION_MISMATCH: HRESULT = 15203;

/// The Monitor Configuration API only works with monitors that support the MCCS 1.0 specification, MCCS 2.0 specification or the MCCS 2.0 Revision 1 specification.
pub const ERROR_MCA_UNSUPPORTED_MCCS_VERSION: HRESULT = 15204;

/// An internal Monitor Configuration API error occurred.
pub const ERROR_MCA_INTERNAL_ERROR: HRESULT = 15205;

/// The monitor returned an invalid monitor technology type. CRT, Plasma and LCD (TFT) are examples of monitor technology types. This error implies that the monitor violated the MCCS 2.0 or MCCS 2.0 Revision 1 specification.
pub const ERROR_MCA_INVALID_TECHNOLOGY_TYPE_RETURNED: HRESULT = 15206;

/// The caller of SetMonitorColorTemperature specified a color temperature that the current monitor did not support. This error implies that the monitor violated the MCCS 2.0 or MCCS 2.0 Revision 1 specification.
pub const ERROR_MCA_UNSUPPORTED_COLOR_TEMPERATURE: HRESULT = 15207;

/// The requested system device cannot be identified due to multiple indistinguishable devices potentially matching the identification criteria.
pub const ERROR_AMBIGUOUS_SYSTEM_DEVICE: HRESULT = 15250;

/// The requested system device cannot be found.
pub const ERROR_SYSTEM_DEVICE_NOT_FOUND: HRESULT = 15299;

/// Hash generation for the specified hash version and hash type is not enabled on the server.
pub const ERROR_HASH_NOT_SUPPORTED: HRESULT = 15300;

/// The hash requested from the server is not available or no longer valid.
pub const ERROR_HASH_NOT_PRESENT: HRESULT = 15301;

/// The secondary interrupt controller instance that manages the specified interrupt is not registered.
pub const ERROR_SECONDARY_IC_PROVIDER_NOT_REGISTERED: HRESULT = 15321;

/// The information supplied by the GPIO client driver is invalid.
pub const ERROR_GPIO_CLIENT_INFORMATION_INVALID: HRESULT = 15322;

/// The version specified by the GPIO client driver is not supported.
pub const ERROR_GPIO_VERSION_NOT_SUPPORTED: HRESULT = 15323;

/// The registration packet supplied by the GPIO client driver is not valid.
pub const ERROR_GPIO_INVALID_REGISTRATION_PACKET: HRESULT = 15324;

/// The requested operation is not supported for the specified handle.
pub const ERROR_GPIO_OPERATION_DENIED: HRESULT = 15325;

/// The requested connect mode conflicts with an existing mode on one or more of the specified pins.
pub const ERROR_GPIO_INCOMPATIBLE_CONNECT_MODE: HRESULT = 15326;

/// The interrupt requested to be unmasked is not masked.
pub const ERROR_GPIO_INTERRUPT_ALREADY_UNMASKED: HRESULT = 15327;

/// The requested run level switch cannot be completed successfully.
pub const ERROR_CANNOT_SWITCH_RUNLEVEL: HRESULT = 15400;

///  The service has an invalid run level setting. The run level for a service
///
/// must not be higher than the run level of its dependent services.
pub const ERROR_INVALID_RUNLEVEL_SETTING: HRESULT = 15401;

///  The requested run level switch cannot be completed successfully since
///
/// one or more services will not stop or restart within the specified timeout.
pub const ERROR_RUNLEVEL_SWITCH_TIMEOUT: HRESULT = 15402;

/// A run level switch agent did not respond within the specified timeout.
pub const ERROR_RUNLEVEL_SWITCH_AGENT_TIMEOUT: HRESULT = 15403;

/// A run level switch is currently in progress.
pub const ERROR_RUNLEVEL_SWITCH_IN_PROGRESS: HRESULT = 15404;

/// One or more services failed to start during the service startup phase of a run level switch.
pub const ERROR_SERVICES_FAILED_AUTOSTART: HRESULT = 15405;

///  The task stop request cannot be completed immediately since
///
/// task needs more time to shutdown.
pub const ERROR_COM_TASK_STOP_PENDING: HRESULT = 15501;

/// Package could not be opened.
pub const ERROR_INSTALL_OPEN_PACKAGE_FAILED: HRESULT = 15600;

/// Package was not found.
pub const ERROR_INSTALL_PACKAGE_NOT_FOUND: HRESULT = 15601;

/// Package data is invalid.
pub const ERROR_INSTALL_INVALID_PACKAGE: HRESULT = 15602;

/// Package failed updates, dependency or conflict validation.
pub const ERROR_INSTALL_RESOLVE_DEPENDENCY_FAILED: HRESULT = 15603;

/// There is not enough disk space on your computer. Please free up some space and try again.
pub const ERROR_INSTALL_OUT_OF_DISK_SPACE: HRESULT = 15604;

/// There was a problem downloading your product.
pub const ERROR_INSTALL_NETWORK_FAILURE: HRESULT = 15605;

/// Package could not be registered.
pub const ERROR_INSTALL_REGISTRATION_FAILURE: HRESULT = 15606;

/// Package could not be unregistered.
pub const ERROR_INSTALL_DEREGISTRATION_FAILURE: HRESULT = 15607;

/// User cancelled the install request.
pub const ERROR_INSTALL_CANCEL: HRESULT = 15608;

/// Install failed. Please contact your software vendor.
pub const ERROR_INSTALL_FAILED: HRESULT = 15609;

/// Removal failed. Please contact your software vendor.
pub const ERROR_REMOVE_FAILED: HRESULT = 15610;

/// The provided package is already installed, and reinstallation of the package was blocked. Check the AppXDeployment-Server event log for details.
pub const ERROR_PACKAGE_ALREADY_EXISTS: HRESULT = 15611;

/// The application cannot be started. Try reinstalling the application to fix the problem.
pub const ERROR_NEEDS_REMEDIATION: HRESULT = 15612;

/// A Prerequisite for an install could not be satisfied.
pub const ERROR_INSTALL_PREREQUISITE_FAILED: HRESULT = 15613;

/// The package repository is corrupted.
pub const ERROR_PACKAGE_REPOSITORY_CORRUPTED: HRESULT = 15614;

/// To install this application you need either a Windows developer license or a sideloading-enabled system.
pub const ERROR_INSTALL_POLICY_FAILURE: HRESULT = 15615;

/// The application cannot be started because it is currently updating.
pub const ERROR_PACKAGE_UPDATING: HRESULT = 15616;

/// The package deployment operation is blocked by policy. Please contact your system administrator.
pub const ERROR_DEPLOYMENT_BLOCKED_BY_POLICY: HRESULT = 15617;

/// The package could not be installed because resources it modifies are currently in use.
pub const ERROR_PACKAGES_IN_USE: HRESULT = 15618;

/// The package could not be recovered because necessary data for recovery have been corrupted.
pub const ERROR_RECOVERY_FILE_CORRUPT: HRESULT = 15619;

/// The signature is invalid. To register in developer mode, AppxSignature.p7x and AppxBlockMap.xml must be valid or should not be present.
pub const ERROR_INVALID_STAGED_SIGNATURE: HRESULT = 15620;

/// An error occurred while deleting the package's previously existing application data.
pub const ERROR_DELETING_EXISTING_APPLICATIONDATA_STORE_FAILED: HRESULT = 15621;

/// The package could not be installed because a higher version of this package is already installed.
pub const ERROR_INSTALL_PACKAGE_DOWNGRADE: HRESULT = 15622;

/// An error in a system binary was detected. Try refreshing the PC to fix the problem.
pub const ERROR_SYSTEM_NEEDS_REMEDIATION: HRESULT = 15623;

/// A corrupted CLR NGEN binary was detected on the system.
pub const ERROR_APPX_INTEGRITY_FAILURE_CLR_NGEN: HRESULT = 15624;

/// The operation could not be resumed because necessary data for recovery have been corrupted.
pub const ERROR_RESILIENCY_FILE_CORRUPT: HRESULT = 15625;

/// The package could not be installed because the Windows Firewall service is not running. Enable the Windows Firewall service and try again.
pub const ERROR_INSTALL_FIREWALL_SERVICE_NOT_RUNNING: HRESULT = 15626;

/// Package move failed.
pub const ERROR_PACKAGE_MOVE_FAILED: HRESULT = 15627;

/// The deployment operation failed because the volume is not empty.
pub const ERROR_INSTALL_VOLUME_NOT_EMPTY: HRESULT = 15628;

/// The deployment operation failed because the volume is offline. For a package update, the volume refers to the installed volume of all package versions.
pub const ERROR_INSTALL_VOLUME_OFFLINE: HRESULT = 15629;

/// The deployment operation failed because the specified volume is corrupt.
pub const ERROR_INSTALL_VOLUME_CORRUPT: HRESULT = 15630;

/// The deployment operation failed because the specified application needs to be registered first.
pub const ERROR_NEEDS_REGISTRATION: HRESULT = 15631;

/// The deployment operation failed because the package targets the wrong processor architecture.
pub const ERROR_INSTALL_WRONG_PROCESSOR_ARCHITECTURE: HRESULT = 15632;

/// You have reached the maximum number of developer sideloaded packages allowed on this device. Please uninstall a sideloaded package and try again.
pub const ERROR_DEV_SIDELOAD_LIMIT_EXCEEDED: HRESULT = 15633;

/// A main app package is required to install this optional package.  Install the main package first and try again.
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE: HRESULT = 15634;

/// This app package type is not supported on this filesystem
pub const ERROR_PACKAGE_NOT_SUPPORTED_ON_FILESYSTEM: HRESULT = 15635;

/// Package move operation is blocked until the application has finished streaming
pub const ERROR_PACKAGE_MOVE_BLOCKED_BY_STREAMING: HRESULT = 15636;

/// A main or another optional app package has the same application ID as this optional package.  Change the application ID for the optional package to avoid conflicts.
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_APPLICATIONID_NOT_UNIQUE: HRESULT = 15637;

/// This staging session has been held to allow another staging operation to be prioritized.
pub const ERROR_PACKAGE_STAGING_ONHOLD: HRESULT = 15638;

/// A related set cannot be updated because the updated set is invalid. All packages in the related set must be updated at the same time.
pub const ERROR_INSTALL_INVALID_RELATED_SET_UPDATE: HRESULT = 15639;

/// An optional package with a FullTrust entry point requires the main package to have the runFullTrust capability.
pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY: HRESULT =
    15640;

/// An error occurred because a user was logged off.
pub const ERROR_DEPLOYMENT_BLOCKED_BY_USER_LOG_OFF: HRESULT = 15641;

/// An optional package provision requires the dependency main package to also be provisioned.
pub const ERROR_PROVISION_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_PROVISIONED: HRESULT = 15642;

/// The packages failed the SmartScreen reputation check.
pub const ERROR_PACKAGES_REPUTATION_CHECK_FAILED: HRESULT = 15643;

/// The SmartScreen reputation check operation timed out.
pub const ERROR_PACKAGES_REPUTATION_CHECK_TIMEDOUT: HRESULT = 15644;

/// The current deployment option is not supported.
pub const ERROR_DEPLOYMENT_OPTION_NOT_SUPPORTED: HRESULT = 15645;

/// Activation is blocked due to the .appinstaller update settings for this app.
pub const ERROR_APPINSTALLER_ACTIVATION_BLOCKED: HRESULT = 15646;

/// Remote drives are not supported; use \\server\share to register a remote package.
pub const ERROR_REGISTRATION_FROM_REMOTE_DRIVE_NOT_SUPPORTED: HRESULT = 15647;

/// Failed to process and write downloaded APPX package data to disk.
pub const ERROR_APPX_RAW_DATA_WRITE_FAILED: HRESULT = 15648;

/// The deployment operation was blocked due to a per-package-family policy restricting deployments on a non-system volume. Per policy, this app must be installed to the system drive, but that's not set as the default. In Storage Settings, make the system drive the default location to save new content, then retry the install.
pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_PACKAGE: HRESULT = 15649;

/// The deployment operation was blocked due to a machine-wide policy restricting deployments on a non-system volume. Per policy, this app must be installed to the system drive, but that's not set as the default. In Storage Settings, make the system drive the default location to save new content, then retry the install.
pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_MACHINE: HRESULT = 15650;

/// The deployment operation was blocked because Special profile deployment is not allowed. Please try logging into an account that is not a Special profile. You can try logging out and logging back into the current account, or try logging into a different account.
pub const ERROR_DEPLOYMENT_BLOCKED_BY_PROFILE_POLICY: HRESULT = 15651;

/// The deployment operation failed due to a conflicting package's mutable package directory. To install this package remove the existing package with the conflicting mutable package directory.
pub const ERROR_DEPLOYMENT_FAILED_CONFLICTING_MUTABLE_PACKAGE_DIRECTORY: HRESULT = 15652;

/// The package installation failed because a singleton resource was specified and another user with that package installed is logged in. Please make sure that all active users with the package installed are logged out and retry installation.
pub const ERROR_SINGLETON_RESOURCE_INSTALLED_IN_ACTIVE_USER: HRESULT = 15653;

/// The package installation failed because a different version of the service is installed. Try installing a newer version of the package.
pub const ERROR_DIFFERENT_VERSION_OF_PACKAGED_SERVICE_INSTALLED: HRESULT = 15654;

/// The package installation failed because a version of the service exists outside of APPX packaging. Please contact your software vendor.
pub const ERROR_SERVICE_EXISTS_AS_NON_PACKAGED_SERVICE: HRESULT = 15655;

/// The package installation failed because administrator privileges are required. Please contact an administrator to install this package.
pub const ERROR_PACKAGED_SERVICE_REQUIRES_ADMIN_PRIVILEGES: HRESULT = 15656;

/// The package deployment failed because the operation would have redirected to default account, when the caller said not to do so.
pub const ERROR_REDIRECTION_TO_DEFAULT_ACCOUNT_NOT_ALLOWED: HRESULT = 15657;

/// The package deployment failed because the package requires a capability to natively target this host.
pub const ERROR_PACKAGE_LACKS_CAPABILITY_TO_DEPLOY_ON_HOST: HRESULT = 15658;

/// The package deployment failed because its content is not valid for an unsigned package.
pub const ERROR_UNSIGNED_PACKAGE_INVALID_CONTENT: HRESULT = 15659;

/// The package deployment failed because its publisher is not in the unsigned namespace.
pub const ERROR_UNSIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: HRESULT = 15660;

/// The package deployment failed because its publisher is not in the signed namespace.
pub const ERROR_SIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: HRESULT = 15661;

/// The package deployment failed because it must allow external content to be deployed with an external location.
pub const ERROR_PACKAGE_EXTERNAL_LOCATION_NOT_ALLOWED: HRESULT = 15662;

/// A host runtime dependency resolving to a package with full trust content requires the main package to have the runFullTrust capability.
pub const ERROR_INSTALL_FULLTRUST_HOSTRUNTIME_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY: HRESULT =
    15663;

/// The package deployment failed because the package requires a capability for mandatory startup tasks.
pub const ERROR_PACKAGE_LACKS_CAPABILITY_FOR_MANDATORY_STARTUPTASKS: HRESULT = 15664;

/// Package failed host runtime dependency or conflict validation.
pub const ERROR_INSTALL_RESOLVE_HOSTRUNTIME_DEPENDENCY_FAILED: HRESULT = 15665;

/// The package deployment failed because it uses a machine scope but doesn't have the required capability.
pub const ERROR_MACHINE_SCOPE_NOT_ALLOWED: HRESULT = 15666;

/// The package deployment failed because it uses classic compatmode but doesn't have the required capability.
pub const ERROR_CLASSIC_COMPAT_MODE_NOT_ALLOWED: HRESULT = 15667;

/// AppxUpdateAgent attempted to stage a package that is not applicable.
pub const ERROR_STAGEFROMUPDATEAGENT_PACKAGE_NOT_APPLICABLE: HRESULT = 15668;

/// The application cannot be started for the target user.  Please have the user explicitly install this package.
pub const ERROR_PACKAGE_NOT_REGISTERED_FOR_USER: HRESULT = 15669;

/// The provided package name does not match the expected package name. Check the AppXDeployment-Server event log for details.
pub const ERROR_PACKAGE_NAME_MISMATCH: HRESULT = 15670;

/// The provided .appinstaller URI is already being used by another package family. Check the AppXDeployment-Server event log for details.
pub const ERROR_APPINSTALLER_URI_IN_USE: HRESULT = 15671;

/// The package family's auto update settings are being managed at system priority and cannot be changed at default priority. Please contact your system administrator for help with the error.
pub const ERROR_APPINSTALLER_IS_MANAGED_BY_SYSTEM: HRESULT = 15672;

/// The process has no package identity.
pub const APPMODEL_ERROR_NO_PACKAGE: HRESULT = 15700;

/// The package runtime information is corrupted.
pub const APPMODEL_ERROR_PACKAGE_RUNTIME_CORRUPT: HRESULT = 15701;

/// The package identity is corrupted.
pub const APPMODEL_ERROR_PACKAGE_IDENTITY_CORRUPT: HRESULT = 15702;

/// The process has no application identity.
pub const APPMODEL_ERROR_NO_APPLICATION: HRESULT = 15703;

/// One or more AppModel Runtime group policy values could not be read. Please contact your system administrator with the contents of your AppModel Runtime event log.
pub const APPMODEL_ERROR_DYNAMIC_PROPERTY_READ_FAILED: HRESULT = 15704;

/// One or more AppModel Runtime group policy values are invalid. Please contact your system administrator with the contents of your AppModel Runtime event log.
pub const APPMODEL_ERROR_DYNAMIC_PROPERTY_INVALID: HRESULT = 15705;

/// The package is currently not available.
pub const APPMODEL_ERROR_PACKAGE_NOT_AVAILABLE: HRESULT = 15706;

/// The package does not have a mutable directory.
pub const APPMODEL_ERROR_NO_MUTABLE_DIRECTORY: HRESULT = 15707;

/// Loading the state store failed.
pub const ERROR_STATE_LOAD_STORE_FAILED: HRESULT = 15800;

/// Retrieving the state version for the application failed.
pub const ERROR_STATE_GET_VERSION_FAILED: HRESULT = 15801;

/// Setting the state version for the application failed.
pub const ERROR_STATE_SET_VERSION_FAILED: HRESULT = 15802;

/// Resetting the structured state of the application failed.
pub const ERROR_STATE_STRUCTURED_RESET_FAILED: HRESULT = 15803;

/// State Manager failed to open the container.
pub const ERROR_STATE_OPEN_CONTAINER_FAILED: HRESULT = 15804;

/// State Manager failed to create the container.
pub const ERROR_STATE_CREATE_CONTAINER_FAILED: HRESULT = 15805;

/// State Manager failed to delete the container.
pub const ERROR_STATE_DELETE_CONTAINER_FAILED: HRESULT = 15806;

/// State Manager failed to read the setting.
pub const ERROR_STATE_READ_SETTING_FAILED: HRESULT = 15807;

/// State Manager failed to write the setting.
pub const ERROR_STATE_WRITE_SETTING_FAILED: HRESULT = 15808;

/// State Manager failed to delete the setting.
pub const ERROR_STATE_DELETE_SETTING_FAILED: HRESULT = 15809;

/// State Manager failed to query the setting.
pub const ERROR_STATE_QUERY_SETTING_FAILED: HRESULT = 15810;

/// State Manager failed to read the composite setting.
pub const ERROR_STATE_READ_COMPOSITE_SETTING_FAILED: HRESULT = 15811;

/// State Manager failed to write the composite setting.
pub const ERROR_STATE_WRITE_COMPOSITE_SETTING_FAILED: HRESULT = 15812;

/// State Manager failed to enumerate the containers.
pub const ERROR_STATE_ENUMERATE_CONTAINER_FAILED: HRESULT = 15813;

/// State Manager failed to enumerate the settings.
pub const ERROR_STATE_ENUMERATE_SETTINGS_FAILED: HRESULT = 15814;

/// The size of the state manager composite setting value has exceeded the limit.
pub const ERROR_STATE_COMPOSITE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: HRESULT = 15815;

/// The size of the state manager setting value has exceeded the limit.
pub const ERROR_STATE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: HRESULT = 15816;

/// The length of the state manager setting name has exceeded the limit.
pub const ERROR_STATE_SETTING_NAME_SIZE_LIMIT_EXCEEDED: HRESULT = 15817;

/// The length of the state manager container name has exceeded the limit.
pub const ERROR_STATE_CONTAINER_NAME_SIZE_LIMIT_EXCEEDED: HRESULT = 15818;

/// This API cannot be used in the context of the caller's application type.
pub const ERROR_API_UNAVAILABLE: HRESULT = 15841;

/// This PC does not have a valid license for the application or product.
pub const STORE_ERROR_UNLICENSED: HRESULT = 15861;

/// The authenticated user does not have a valid license for the application or product.
pub const STORE_ERROR_UNLICENSED_USER: HRESULT = 15862;

/// The commerce transaction associated with this license is still pending verification.
pub const STORE_ERROR_PENDING_COM_TRANSACTION: HRESULT = 15863;

/// The license has been revoked for this user.
pub const STORE_ERROR_LICENSE_REVOKED: HRESULT = 15864;

/// Element not found.
pub const E_NOT_SET: HRESULT = ERROR_NOT_FOUND;

/// The group or resource is not in the correct state to perform the requested operation.
pub const E_NOT_VALID_STATE: HRESULT = ERROR_INVALID_STATE;

/// The data area passed to a system call is too small
pub const E_NOT_SUFFICIENT_BUFFER: HRESULT = ERROR_INSUFFICIENT_BUFFER;

/// Operation cannot be performed on a time critical thread
pub const E_TIME_SENSITIVE_THREAD: HRESULT = ERROR_TIME_SENSITIVE_THREAD;

/// A task queue is required for this operation but none is available
pub const E_NO_TASK_QUEUE: HRESULT = ERROR_NO_TASK_QUEUE;

/// Catastrophic failure
pub const E_UNEXPECTED: HRESULT = 0x8000FFFFu32 as HRESULT;

/// Not implemented
pub const E_NOTIMPL: HRESULT = 0x80004001u32 as HRESULT;

/// Ran out of memory
pub const E_OUTOFMEMORY: HRESULT = 0x8007000Eu32 as HRESULT;

/// One or more arguments are invalid
pub const E_INVALIDARG: HRESULT = 0x80070057u32 as HRESULT;

/// No such interface supported
pub const E_NOINTERFACE: HRESULT = 0x80004002u32 as HRESULT;

/// Invalid pointer
pub const E_POINTER: HRESULT = 0x80004003u32 as HRESULT;

/// Invalid handle
pub const E_HANDLE: HRESULT = 0x80070006u32 as HRESULT;

/// Operation aborted
pub const E_ABORT: HRESULT = 0x80004004u32 as HRESULT;

/// Unspecified error
pub const E_FAIL: HRESULT = 0x80004005u32 as HRESULT;

/// General access denied error
pub const E_ACCESSDENIED: HRESULT = 0x80070005u32 as HRESULT;

/// The data necessary to complete this operation is not yet available.
pub const E_PENDING: HRESULT = 0x8000000Au32 as HRESULT;

/// The operation attempted to access data outside the valid range
pub const E_BOUNDS: HRESULT = 0x8000000Bu32 as HRESULT;

/// A concurrent or interleaved operation changed the state of the object, invalidating this operation.
pub const E_CHANGED_STATE: HRESULT = 0x8000000Cu32 as HRESULT;

/// An illegal state change was requested.
pub const E_ILLEGAL_STATE_CHANGE: HRESULT = 0x8000000Du32 as HRESULT;

/// A method was called at an unexpected time.
pub const E_ILLEGAL_METHOD_CALL: HRESULT = 0x8000000Eu32 as HRESULT;

/// Typename or Namespace was not found in metadata file.
pub const RO_E_METADATA_NAME_NOT_FOUND: HRESULT = 0x8000000Fu32 as HRESULT;

/// Name is an existing namespace rather than a typename.
pub const RO_E_METADATA_NAME_IS_NAMESPACE: HRESULT = 0x80000010u32 as HRESULT;

/// Typename has an invalid format.
pub const RO_E_METADATA_INVALID_TYPE_FORMAT: HRESULT = 0x80000011u32 as HRESULT;

/// Metadata file is invalid or corrupted.
pub const RO_E_INVALID_METADATA_FILE: HRESULT = 0x80000012u32 as HRESULT;

/// The object has been closed.
pub const RO_E_CLOSED: HRESULT = 0x80000013u32 as HRESULT;

/// Only one thread may access the object during a write operation.
pub const RO_E_EXCLUSIVE_WRITE: HRESULT = 0x80000014u32 as HRESULT;

/// Operation is prohibited during change notification.
pub const RO_E_CHANGE_NOTIFICATION_IN_PROGRESS: HRESULT = 0x80000015u32 as HRESULT;

/// The text associated with this error code could not be found.
pub const RO_E_ERROR_STRING_NOT_FOUND: HRESULT = 0x80000016u32 as HRESULT;

/// String not null terminated.
pub const E_STRING_NOT_NULL_TERMINATED: HRESULT = 0x80000017u32 as HRESULT;

/// A delegate was assigned when not allowed.
pub const E_ILLEGAL_DELEGATE_ASSIGNMENT: HRESULT = 0x80000018u32 as HRESULT;

/// An async operation was not properly started.
pub const E_ASYNC_OPERATION_NOT_STARTED: HRESULT = 0x80000019u32 as HRESULT;

/// The application is exiting and cannot service this request
pub const E_APPLICATION_EXITING: HRESULT = 0x8000001Au32 as HRESULT;

/// The application view is exiting and cannot service this request
pub const E_APPLICATION_VIEW_EXITING: HRESULT = 0x8000001Bu32 as HRESULT;

/// The object must support the IAgileObject interface
pub const RO_E_MUST_BE_AGILE: HRESULT = 0x8000001Cu32 as HRESULT;

/// Activating a single-threaded class from MTA is not supported
pub const RO_E_UNSUPPORTED_FROM_MTA: HRESULT = 0x8000001Du32 as HRESULT;

/// The object has been committed.
pub const RO_E_COMMITTED: HRESULT = 0x8000001Eu32 as HRESULT;

/// A COM call to an ASTA was blocked because the call chain originated in or passed through another ASTA. This call pattern is deadlock-prone and disallowed by apartment call control.
pub const RO_E_BLOCKED_CROSS_ASTA_CALL: HRESULT = 0x8000001Fu32 as HRESULT;

/// A universal application process cannot activate a packaged WinRT server that is declared to run full trust.
pub const RO_E_CANNOT_ACTIVATE_FULL_TRUST_SERVER: HRESULT = 0x80000020u32 as HRESULT;

/// A full trust packaged application process cannot activate a packaged WinRT server unless it is also declared to run full trust.
pub const RO_E_CANNOT_ACTIVATE_UNIVERSAL_APPLICATION_SERVER: HRESULT = 0x80000021u32 as HRESULT;

/// Thread local storage failure
pub const CO_E_INIT_TLS: HRESULT = 0x80004006u32 as HRESULT;

/// Get shared memory allocator failure
pub const CO_E_INIT_SHARED_ALLOCATOR: HRESULT = 0x80004007u32 as HRESULT;

/// Get memory allocator failure
pub const CO_E_INIT_MEMORY_ALLOCATOR: HRESULT = 0x80004008u32 as HRESULT;

/// Unable to initialize class cache
pub const CO_E_INIT_CLASS_CACHE: HRESULT = 0x80004009u32 as HRESULT;

/// Unable to initialize RPC services
pub const CO_E_INIT_RPC_CHANNEL: HRESULT = 0x8000400Au32 as HRESULT;

/// Cannot set thread local storage channel control
pub const CO_E_INIT_TLS_SET_CHANNEL_CONTROL: HRESULT = 0x8000400Bu32 as HRESULT;

/// Could not allocate thread local storage channel control
pub const CO_E_INIT_TLS_CHANNEL_CONTROL: HRESULT = 0x8000400Cu32 as HRESULT;

/// The user supplied memory allocator is unacceptable
pub const CO_E_INIT_UNACCEPTED_USER_ALLOCATOR: HRESULT = 0x8000400Du32 as HRESULT;

/// The OLE service mutex already exists
pub const CO_E_INIT_SCM_MUTEX_EXISTS: HRESULT = 0x8000400Eu32 as HRESULT;

/// The OLE service file mapping already exists
pub const CO_E_INIT_SCM_FILE_MAPPING_EXISTS: HRESULT = 0x8000400Fu32 as HRESULT;

/// Unable to map view of file for OLE service
pub const CO_E_INIT_SCM_MAP_VIEW_OF_FILE: HRESULT = 0x80004010u32 as HRESULT;

/// Failure attempting to launch OLE service
pub const CO_E_INIT_SCM_EXEC_FAILURE: HRESULT = 0x80004011u32 as HRESULT;

/// There was an attempt to call CoInitialize a second time while single threaded
pub const CO_E_INIT_ONLY_SINGLE_THREADED: HRESULT = 0x80004012u32 as HRESULT;

/// A Remote activation was necessary but was not allowed
pub const CO_E_CANT_REMOTE: HRESULT = 0x80004013u32 as HRESULT;

/// A Remote activation was necessary but the server name provided was invalid
pub const CO_E_BAD_SERVER_NAME: HRESULT = 0x80004014u32 as HRESULT;

/// The class is configured to run as a security id different from the caller
pub const CO_E_WRONG_SERVER_IDENTITY: HRESULT = 0x80004015u32 as HRESULT;

/// Use of Ole1 services requiring DDE windows is disabled
pub const CO_E_OLE1DDE_DISABLED: HRESULT = 0x80004016u32 as HRESULT;

/// A RunAs specification must be `<domain name>\<user name>` or simply `<user name>`
pub const CO_E_RUNAS_SYNTAX: HRESULT = 0x80004017u32 as HRESULT;

/// The server process could not be started. The pathname may be incorrect.
pub const CO_E_CREATEPROCESS_FAILURE: HRESULT = 0x80004018u32 as HRESULT;

/// The server process could not be started as the configured identity. The pathname may be incorrect or unavailable.
pub const CO_E_RUNAS_CREATEPROCESS_FAILURE: HRESULT = 0x80004019u32 as HRESULT;

/// The server process could not be started because the configured identity is incorrect. Check the username and password.
pub const CO_E_RUNAS_LOGON_FAILURE: HRESULT = 0x8000401Au32 as HRESULT;

/// The client is not allowed to launch this server.
pub const CO_E_LAUNCH_PERMSSION_DENIED: HRESULT = 0x8000401Bu32 as HRESULT;

/// The service providing this server could not be started.
pub const CO_E_START_SERVICE_FAILURE: HRESULT = 0x8000401Cu32 as HRESULT;

/// This computer was unable to communicate with the computer providing the server.
pub const CO_E_REMOTE_COMMUNICATION_FAILURE: HRESULT = 0x8000401Du32 as HRESULT;

/// The server did not respond after being launched.
pub const CO_E_SERVER_START_TIMEOUT: HRESULT = 0x8000401Eu32 as HRESULT;

/// The registration information for this server is inconsistent or incomplete.
pub const CO_E_CLSREG_INCONSISTENT: HRESULT = 0x8000401Fu32 as HRESULT;

/// The registration information for this interface is inconsistent or incomplete.
pub const CO_E_IIDREG_INCONSISTENT: HRESULT = 0x80004020u32 as HRESULT;

/// The operation attempted is not supported.
pub const CO_E_NOT_SUPPORTED: HRESULT = 0x80004021u32 as HRESULT;

/// A dll must be loaded.
pub const CO_E_RELOAD_DLL: HRESULT = 0x80004022u32 as HRESULT;

/// A Microsoft Software Installer error was encountered.
pub const CO_E_MSI_ERROR: HRESULT = 0x80004023u32 as HRESULT;

/// The specified activation could not occur in the client context as specified.
pub const CO_E_ATTEMPT_TO_CREATE_OUTSIDE_CLIENT_CONTEXT: HRESULT = 0x80004024u32 as HRESULT;

/// Activations on the server are paused.
pub const CO_E_SERVER_PAUSED: HRESULT = 0x80004025u32 as HRESULT;

/// Activations on the server are not paused.
pub const CO_E_SERVER_NOT_PAUSED: HRESULT = 0x80004026u32 as HRESULT;

/// The component or application containing the component has been disabled.
pub const CO_E_CLASS_DISABLED: HRESULT = 0x80004027u32 as HRESULT;

/// The common language runtime is not available
pub const CO_E_CLRNOTAVAILABLE: HRESULT = 0x80004028u32 as HRESULT;

/// The thread-pool rejected the submitted asynchronous work.
pub const CO_E_ASYNC_WORK_REJECTED: HRESULT = 0x80004029u32 as HRESULT;

/// The server started, but did not finish initializing in a timely fashion.
pub const CO_E_SERVER_INIT_TIMEOUT: HRESULT = 0x8000402Au32 as HRESULT;

/// Unable to complete the call since there is no COM+ security context inside IObjectControl.Activate.
pub const CO_E_NO_SECCTX_IN_ACTIVATE: HRESULT = 0x8000402Bu32 as HRESULT;

/// The provided tracker configuration is invalid
pub const CO_E_TRACKER_CONFIG: HRESULT = 0x80004030u32 as HRESULT;

/// The provided thread pool configuration is invalid
pub const CO_E_THREADPOOL_CONFIG: HRESULT = 0x80004031u32 as HRESULT;

/// The provided side-by-side configuration is invalid
pub const CO_E_SXS_CONFIG: HRESULT = 0x80004032u32 as HRESULT;

/// The server principal name (SPN) obtained during security negotiation is malformed.
pub const CO_E_MALFORMED_SPN: HRESULT = 0x80004033u32 as HRESULT;

/// The caller failed to revoke a per-apartment registration before apartment shutdown.
pub const CO_E_UNREVOKED_REGISTRATION_ON_APARTMENT_SHUTDOWN: HRESULT = 0x80004034u32 as HRESULT;

/// The object has been rundown by the stub manager while there are external clients.
pub const CO_E_PREMATURE_STUB_RUNDOWN: HRESULT = 0x80004035u32 as HRESULT;

/// Invalid OLEVERB structure
pub const OLE_E_OLEVERB: HRESULT = 0x80040000u32 as HRESULT;

/// Invalid advise flags
pub const OLE_E_ADVF: HRESULT = 0x80040001u32 as HRESULT;

/// Can't enumerate any more, because the associated data is missing
pub const OLE_E_ENUM_NOMORE: HRESULT = 0x80040002u32 as HRESULT;

/// This implementation doesn't take advises
pub const OLE_E_ADVISENOTSUPPORTED: HRESULT = 0x80040003u32 as HRESULT;

/// There is no connection for this connection ID
pub const OLE_E_NOCONNECTION: HRESULT = 0x80040004u32 as HRESULT;

/// Need to run the object to perform this operation
pub const OLE_E_NOTRUNNING: HRESULT = 0x80040005u32 as HRESULT;

/// There is no cache to operate on
pub const OLE_E_NOCACHE: HRESULT = 0x80040006u32 as HRESULT;

/// Uninitialized object
pub const OLE_E_BLANK: HRESULT = 0x80040007u32 as HRESULT;

/// Linked object's source class has changed
pub const OLE_E_CLASSDIFF: HRESULT = 0x80040008u32 as HRESULT;

/// Not able to get the moniker of the object
pub const OLE_E_CANT_GETMONIKER: HRESULT = 0x80040009u32 as HRESULT;

/// Not able to bind to the source
pub const OLE_E_CANT_BINDTOSOURCE: HRESULT = 0x8004000Au32 as HRESULT;

/// Object is static; operation not allowed
pub const OLE_E_STATIC: HRESULT = 0x8004000Bu32 as HRESULT;

/// User canceled out of save dialog
pub const OLE_E_PROMPTSAVECANCELLED: HRESULT = 0x8004000Cu32 as HRESULT;

/// Invalid rectangle
pub const OLE_E_INVALIDRECT: HRESULT = 0x8004000Du32 as HRESULT;

/// compobj.dll is too old for the ole2.dll initialized
pub const OLE_E_WRONGCOMPOBJ: HRESULT = 0x8004000Eu32 as HRESULT;

/// Invalid window handle
pub const OLE_E_INVALIDHWND: HRESULT = 0x8004000Fu32 as HRESULT;

/// Object is not in any of the inplace active states
pub const OLE_E_NOT_INPLACEACTIVE: HRESULT = 0x80040010u32 as HRESULT;

/// Not able to convert object
pub const OLE_E_CANTCONVERT: HRESULT = 0x80040011u32 as HRESULT;

/// Not able to perform the operation because object is not given storage yet
pub const OLE_E_NOSTORAGE: HRESULT = 0x80040012u32 as HRESULT;

/// Invalid FORMATETC structure
pub const DV_E_FORMATETC: HRESULT = 0x80040064u32 as HRESULT;

/// Invalid DVTARGETDEVICE structure
pub const DV_E_DVTARGETDEVICE: HRESULT = 0x80040065u32 as HRESULT;

/// Invalid STDGMEDIUM structure
pub const DV_E_STGMEDIUM: HRESULT = 0x80040066u32 as HRESULT;

/// Invalid STATDATA structure
pub const DV_E_STATDATA: HRESULT = 0x80040067u32 as HRESULT;

/// Invalid lindex
pub const DV_E_LINDEX: HRESULT = 0x80040068u32 as HRESULT;

/// Invalid tymed
pub const DV_E_TYMED: HRESULT = 0x80040069u32 as HRESULT;

/// Invalid clipboard format
pub const DV_E_CLIPFORMAT: HRESULT = 0x8004006Au32 as HRESULT;

/// Invalid aspect(s)
pub const DV_E_DVASPECT: HRESULT = 0x8004006Bu32 as HRESULT;

/// tdSize parameter of the DVTARGETDEVICE structure is invalid
pub const DV_E_DVTARGETDEVICE_SIZE: HRESULT = 0x8004006Cu32 as HRESULT;

/// Object doesn't support IViewObject interface
pub const DV_E_NOIVIEWOBJECT: HRESULT = 0x8004006Du32 as HRESULT;

/// Trying to revoke a drop target that has not been registered
pub const DRAGDROP_E_NOTREGISTERED: HRESULT = 0x80040100u32 as HRESULT;

/// This window has already been registered as a drop target
pub const DRAGDROP_E_ALREADYREGISTERED: HRESULT = 0x80040101u32 as HRESULT;

/// Invalid window handle
pub const DRAGDROP_E_INVALIDHWND: HRESULT = 0x80040102u32 as HRESULT;

/// A drag operation is already in progress
pub const DRAGDROP_E_CONCURRENT_DRAG_ATTEMPTED: HRESULT = 0x80040103u32 as HRESULT;

/// Class does not support aggregation (or class object is remote)
pub const CLASS_E_NOAGGREGATION: HRESULT = 0x80040110u32 as HRESULT;

/// ClassFactory cannot supply requested class
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x80040111u32 as HRESULT;

/// Class is not licensed for use
pub const CLASS_E_NOTLICENSED: HRESULT = 0x80040112u32 as HRESULT;

/// Error drawing view
pub const VIEW_E_DRAW: HRESULT = 0x80040140u32 as HRESULT;

/// Could not read key from registry
pub const REGDB_E_READREGDB: HRESULT = 0x80040150u32 as HRESULT;

/// Could not write key to registry
pub const REGDB_E_WRITEREGDB: HRESULT = 0x80040151u32 as HRESULT;

/// Could not find the key in the registry
pub const REGDB_E_KEYMISSING: HRESULT = 0x80040152u32 as HRESULT;

/// Invalid value for registry
pub const REGDB_E_INVALIDVALUE: HRESULT = 0x80040153u32 as HRESULT;

/// Class not registered
pub const REGDB_E_CLASSNOTREG: HRESULT = 0x80040154u32 as HRESULT;

/// Interface not registered
pub const REGDB_E_IIDNOTREG: HRESULT = 0x80040155u32 as HRESULT;

/// Threading model entry is not valid
pub const REGDB_E_BADTHREADINGMODEL: HRESULT = 0x80040156u32 as HRESULT;

/// A registration in a package violates package-specific policies
pub const REGDB_E_PACKAGEPOLICYVIOLATION: HRESULT = 0x80040157u32 as HRESULT;

/// CATID does not exist
pub const CAT_E_CATIDNOEXIST: HRESULT = 0x80040160u32 as HRESULT;

/// Description not found
pub const CAT_E_NODESCRIPTION: HRESULT = 0x80040161u32 as HRESULT;

/// No package in the software installation data in the Active Directory meets this criteria.
pub const CS_E_PACKAGE_NOTFOUND: HRESULT = 0x80040164u32 as HRESULT;

/// Deleting this will break the referential integrity of the software installation data in the Active Directory.
pub const CS_E_NOT_DELETABLE: HRESULT = 0x80040165u32 as HRESULT;

/// The CLSID was not found in the software installation data in the Active Directory.
pub const CS_E_CLASS_NOTFOUND: HRESULT = 0x80040166u32 as HRESULT;

/// The software installation data in the Active Directory is corrupt.
pub const CS_E_INVALID_VERSION: HRESULT = 0x80040167u32 as HRESULT;

/// There is no software installation data in the Active Directory.
pub const CS_E_NO_CLASSSTORE: HRESULT = 0x80040168u32 as HRESULT;

/// There is no software installation data object in the Active Directory.
pub const CS_E_OBJECT_NOTFOUND: HRESULT = 0x80040169u32 as HRESULT;

/// The software installation data object in the Active Directory already exists.
pub const CS_E_OBJECT_ALREADY_EXISTS: HRESULT = 0x8004016Au32 as HRESULT;

/// The path to the software installation data in the Active Directory is not correct.
pub const CS_E_INVALID_PATH: HRESULT = 0x8004016Bu32 as HRESULT;

/// A network error interrupted the operation.
pub const CS_E_NETWORK_ERROR: HRESULT = 0x8004016Cu32 as HRESULT;

/// The size of this object exceeds the maximum size set by the Administrator.
pub const CS_E_ADMIN_LIMIT_EXCEEDED: HRESULT = 0x8004016Du32 as HRESULT;

/// The schema for the software installation data in the Active Directory does not match the required schema.
pub const CS_E_SCHEMA_MISMATCH: HRESULT = 0x8004016Eu32 as HRESULT;

/// An error occurred in the software installation data in the Active Directory.
pub const CS_E_INTERNAL_ERROR: HRESULT = 0x8004016Fu32 as HRESULT;

/// Cache not updated
pub const CACHE_E_NOCACHE_UPDATED: HRESULT = 0x80040170u32 as HRESULT;

/// No verbs for OLE object
pub const OLEOBJ_E_NOVERBS: HRESULT = 0x80040180u32 as HRESULT;

/// Invalid verb for OLE object
pub const OLEOBJ_E_INVALIDVERB: HRESULT = 0x80040181u32 as HRESULT;

/// Undo is not available
pub const INPLACE_E_NOTUNDOABLE: HRESULT = 0x800401A0u32 as HRESULT;

/// Space for tools is not available
pub const INPLACE_E_NOTOOLSPACE: HRESULT = 0x800401A1u32 as HRESULT;

/// OLESTREAM Get method failed
pub const CONVERT10_E_OLESTREAM_GET: HRESULT = 0x800401C0u32 as HRESULT;

/// OLESTREAM Put method failed
pub const CONVERT10_E_OLESTREAM_PUT: HRESULT = 0x800401C1u32 as HRESULT;

/// Contents of the OLESTREAM not in correct format
pub const CONVERT10_E_OLESTREAM_FMT: HRESULT = 0x800401C2u32 as HRESULT;

/// There was an error in a Windows GDI call while converting the bitmap to a DIB
pub const CONVERT10_E_OLESTREAM_BITMAP_TO_DIB: HRESULT = 0x800401C3u32 as HRESULT;

/// Contents of the IStorage not in correct format
pub const CONVERT10_E_STG_FMT: HRESULT = 0x800401C4u32 as HRESULT;

/// Contents of IStorage is missing one of the standard streams
pub const CONVERT10_E_STG_NO_STD_STREAM: HRESULT = 0x800401C5u32 as HRESULT;

/// There was an error in a Windows GDI call while converting the DIB to a bitmap.
pub const CONVERT10_E_STG_DIB_TO_BITMAP: HRESULT = 0x800401C6u32 as HRESULT;

/// OLE Links in OLESTREAM are disabled while converting the OLESTREAM to IStorage
pub const CONVERT10_E_OLELINK_DISABLED: HRESULT = 0x800401C7u32 as HRESULT;

/// OpenClipboard Failed
pub const CLIPBRD_E_CANT_OPEN: HRESULT = 0x800401D0u32 as HRESULT;

/// EmptyClipboard Failed
pub const CLIPBRD_E_CANT_EMPTY: HRESULT = 0x800401D1u32 as HRESULT;

/// SetClipboard Failed
pub const CLIPBRD_E_CANT_SET: HRESULT = 0x800401D2u32 as HRESULT;

/// Data on clipboard is invalid
pub const CLIPBRD_E_BAD_DATA: HRESULT = 0x800401D3u32 as HRESULT;

/// CloseClipboard Failed
pub const CLIPBRD_E_CANT_CLOSE: HRESULT = 0x800401D4u32 as HRESULT;

/// Moniker needs to be connected manually
pub const MK_E_CONNECTMANUALLY: HRESULT = 0x800401E0u32 as HRESULT;

/// Operation exceeded deadline
pub const MK_E_EXCEEDEDDEADLINE: HRESULT = 0x800401E1u32 as HRESULT;

/// Moniker needs to be generic
pub const MK_E_NEEDGENERIC: HRESULT = 0x800401E2u32 as HRESULT;

/// Operation unavailable
pub const MK_E_UNAVAILABLE: HRESULT = 0x800401E3u32 as HRESULT;

/// Invalid syntax
pub const MK_E_SYNTAX: HRESULT = 0x800401E4u32 as HRESULT;

/// No object for moniker
pub const MK_E_NOOBJECT: HRESULT = 0x800401E5u32 as HRESULT;

/// Bad extension for file
pub const MK_E_INVALIDEXTENSION: HRESULT = 0x800401E6u32 as HRESULT;

/// Intermediate operation failed
pub const MK_E_INTERMEDIATEINTERFACENOTSUPPORTED: HRESULT = 0x800401E7u32 as HRESULT;

/// Moniker is not bindable
pub const MK_E_NOTBINDABLE: HRESULT = 0x800401E8u32 as HRESULT;

/// Moniker is not bound
pub const MK_E_NOTBOUND: HRESULT = 0x800401E9u32 as HRESULT;

/// Moniker cannot open file
pub const MK_E_CANTOPENFILE: HRESULT = 0x800401EAu32 as HRESULT;

/// User input required for operation to succeed
pub const MK_E_MUSTBOTHERUSER: HRESULT = 0x800401EBu32 as HRESULT;

/// Moniker class has no inverse
pub const MK_E_NOINVERSE: HRESULT = 0x800401ECu32 as HRESULT;

/// Moniker does not refer to storage
pub const MK_E_NOSTORAGE: HRESULT = 0x800401EDu32 as HRESULT;

/// No common prefix
pub const MK_E_NOPREFIX: HRESULT = 0x800401EEu32 as HRESULT;

/// Moniker could not be enumerated
pub const MK_E_ENUMERATION_FAILED: HRESULT = 0x800401EFu32 as HRESULT;

/// CoInitialize has not been called.
pub const CO_E_NOTINITIALIZED: HRESULT = 0x800401F0u32 as HRESULT;

/// CoInitialize has already been called.
pub const CO_E_ALREADYINITIALIZED: HRESULT = 0x800401F1u32 as HRESULT;

/// Class of object cannot be determined
pub const CO_E_CANTDETERMINECLASS: HRESULT = 0x800401F2u32 as HRESULT;

/// Invalid class string
pub const CO_E_CLASSSTRING: HRESULT = 0x800401F3u32 as HRESULT;

/// Invalid interface string
pub const CO_E_IIDSTRING: HRESULT = 0x800401F4u32 as HRESULT;

/// Application not found
pub const CO_E_APPNOTFOUND: HRESULT = 0x800401F5u32 as HRESULT;

/// Application cannot be run more than once
pub const CO_E_APPSINGLEUSE: HRESULT = 0x800401F6u32 as HRESULT;

/// Some error in application program
pub const CO_E_ERRORINAPP: HRESULT = 0x800401F7u32 as HRESULT;

/// DLL for class not found
pub const CO_E_DLLNOTFOUND: HRESULT = 0x800401F8u32 as HRESULT;

/// Error in the DLL
pub const CO_E_ERRORINDLL: HRESULT = 0x800401F9u32 as HRESULT;

/// Wrong OS or OS version for application
pub const CO_E_WRONGOSFORAPP: HRESULT = 0x800401FAu32 as HRESULT;

/// Object is not registered
pub const CO_E_OBJNOTREG: HRESULT = 0x800401FBu32 as HRESULT;

/// Object is already registered
pub const CO_E_OBJISREG: HRESULT = 0x800401FCu32 as HRESULT;

/// Object is not connected to server
pub const CO_E_OBJNOTCONNECTED: HRESULT = 0x800401FDu32 as HRESULT;

/// Application was launched but it didn't register a class factory
pub const CO_E_APPDIDNTREG: HRESULT = 0x800401FEu32 as HRESULT;

/// Object has been released
pub const CO_E_RELEASED: HRESULT = 0x800401FFu32 as HRESULT;

/// An event was able to invoke some but not all of the subscribers
pub const EVENT_S_SOME_SUBSCRIBERS_FAILED: HRESULT = 0x00040200;

/// An event was unable to invoke any of the subscribers
pub const EVENT_E_ALL_SUBSCRIBERS_FAILED: HRESULT = 0x80040201u32 as HRESULT;

/// An event was delivered but there were no subscribers
pub const EVENT_S_NOSUBSCRIBERS: HRESULT = 0x00040202;

/// A syntax error occurred trying to evaluate a query string
pub const EVENT_E_QUERYSYNTAX: HRESULT = 0x80040203u32 as HRESULT;

/// An invalid field name was used in a query string
pub const EVENT_E_QUERYFIELD: HRESULT = 0x80040204u32 as HRESULT;

/// An unexpected exception was raised
pub const EVENT_E_INTERNALEXCEPTION: HRESULT = 0x80040205u32 as HRESULT;

/// An unexpected internal error was detected
pub const EVENT_E_INTERNALERROR: HRESULT = 0x80040206u32 as HRESULT;

/// The owner SID on a per-user subscription doesn't exist
pub const EVENT_E_INVALID_PER_USER_SID: HRESULT = 0x80040207u32 as HRESULT;

/// A user-supplied component or subscriber raised an exception
pub const EVENT_E_USER_EXCEPTION: HRESULT = 0x80040208u32 as HRESULT;

/// An interface has too many methods to fire events from
pub const EVENT_E_TOO_MANY_METHODS: HRESULT = 0x80040209u32 as HRESULT;

/// A subscription cannot be stored unless its event class already exists
pub const EVENT_E_MISSING_EVENTCLASS: HRESULT = 0x8004020Au32 as HRESULT;

/// Not all the objects requested could be removed
pub const EVENT_E_NOT_ALL_REMOVED: HRESULT = 0x8004020Bu32 as HRESULT;

/// COM+ is required for this operation, but is not installed
pub const EVENT_E_COMPLUS_NOT_INSTALLED: HRESULT = 0x8004020Cu32 as HRESULT;

/// Cannot modify or delete an object that was not added using the COM+ Admin SDK
pub const EVENT_E_CANT_MODIFY_OR_DELETE_UNCONFIGURED_OBJECT: HRESULT = 0x8004020Du32 as HRESULT;

/// Cannot modify or delete an object that was added using the COM+ Admin SDK
pub const EVENT_E_CANT_MODIFY_OR_DELETE_CONFIGURED_OBJECT: HRESULT = 0x8004020Eu32 as HRESULT;

/// The event class for this subscription is in an invalid partition
pub const EVENT_E_INVALID_EVENT_CLASS_PARTITION: HRESULT = 0x8004020Fu32 as HRESULT;

/// The owner of the PerUser subscription is not logged on to the system specified
pub const EVENT_E_PER_USER_SID_NOT_LOGGED_ON: HRESULT = 0x80040210u32 as HRESULT;

/// TabletPC inking error code. The property was not found, or supported by the recognizer
pub const TPC_E_INVALID_PROPERTY: HRESULT = 0x80040241u32 as HRESULT;

/// TabletPC inking error code. No default tablet
pub const TPC_E_NO_DEFAULT_TABLET: HRESULT = 0x80040212u32 as HRESULT;

/// TabletPC inking error code. Unknown property specified
pub const TPC_E_UNKNOWN_PROPERTY: HRESULT = 0x8004021Bu32 as HRESULT;

/// TabletPC inking error code. An invalid input rectangle was specified
pub const TPC_E_INVALID_INPUT_RECT: HRESULT = 0x80040219u32 as HRESULT;

/// TabletPC inking error code. The stroke object was deleted
pub const TPC_E_INVALID_STROKE: HRESULT = 0x80040222u32 as HRESULT;

/// TabletPC inking error code. Initialization failure
pub const TPC_E_INITIALIZE_FAIL: HRESULT = 0x80040223u32 as HRESULT;

/// TabletPC inking error code. The data required for the operation was not supplied
pub const TPC_E_NOT_RELEVANT: HRESULT = 0x80040232u32 as HRESULT;

/// TabletPC inking error code. Invalid packet description
pub const TPC_E_INVALID_PACKET_DESCRIPTION: HRESULT = 0x80040233u32 as HRESULT;

/// TabletPC inking error code. There are no handwriting recognizers registered
pub const TPC_E_RECOGNIZER_NOT_REGISTERED: HRESULT = 0x80040235u32 as HRESULT;

/// TabletPC inking error code. User does not have the necessary rights to read recognizer information
pub const TPC_E_INVALID_RIGHTS: HRESULT = 0x80040236u32 as HRESULT;

/// TabletPC inking error code. API calls were made in an incorrect order
pub const TPC_E_OUT_OF_ORDER_CALL: HRESULT = 0x80040237u32 as HRESULT;

/// TabletPC inking error code. Queue is full
pub const TPC_E_QUEUE_FULL: HRESULT = 0x80040238u32 as HRESULT;

/// TabletPC inking error code. RtpEnabled called multiple times
pub const TPC_E_INVALID_CONFIGURATION: HRESULT = 0x80040239u32 as HRESULT;

/// TabletPC inking error code. A recognizer returned invalid data
pub const TPC_E_INVALID_DATA_FROM_RECOGNIZER: HRESULT = 0x8004023Au32 as HRESULT;

/// TabletPC inking error code. String was truncated
pub const TPC_S_TRUNCATED: HRESULT = 0x00040252;

/// TabletPC inking error code. Recognition or training was interrupted
pub const TPC_S_INTERRUPTED: HRESULT = 0x00040253;

/// TabletPC inking error code. No personalization update to the recognizer because no training data found
pub const TPC_S_NO_DATA_TO_PROCESS: HRESULT = 0x00040254;

/// Another single phase resource manager has already been enlisted in this transaction.
pub const XACT_E_ALREADYOTHERSINGLEPHASE: HRESULT = 0x8004D000u32 as HRESULT;

/// A retaining commit or abort is not supported
pub const XACT_E_CANTRETAIN: HRESULT = 0x8004D001u32 as HRESULT;

/// The transaction failed to commit for an unknown reason. The transaction was aborted.
pub const XACT_E_COMMITFAILED: HRESULT = 0x8004D002u32 as HRESULT;

/// Cannot call commit on this transaction object because the calling application did not initiate the transaction.
pub const XACT_E_COMMITPREVENTED: HRESULT = 0x8004D003u32 as HRESULT;

/// Instead of committing, the resource heuristically aborted.
pub const XACT_E_HEURISTICABORT: HRESULT = 0x8004D004u32 as HRESULT;

/// Instead of aborting, the resource heuristically committed.
pub const XACT_E_HEURISTICCOMMIT: HRESULT = 0x8004D005u32 as HRESULT;

/// Some of the states of the resource were committed while others were aborted, likely because of heuristic decisions.
pub const XACT_E_HEURISTICDAMAGE: HRESULT = 0x8004D006u32 as HRESULT;

/// Some of the states of the resource may have been committed while others may have been aborted, likely because of heuristic decisions.
pub const XACT_E_HEURISTICDANGER: HRESULT = 0x8004D007u32 as HRESULT;

/// The requested isolation level is not valid or supported.
pub const XACT_E_ISOLATIONLEVEL: HRESULT = 0x8004D008u32 as HRESULT;

/// The transaction manager doesn't support an asynchronous operation for this method.
pub const XACT_E_NOASYNC: HRESULT = 0x8004D009u32 as HRESULT;

/// Unable to enlist in the transaction.
pub const XACT_E_NOENLIST: HRESULT = 0x8004D00Au32 as HRESULT;

/// The requested semantics of retention of isolation across retaining commit and abort boundaries cannot be supported by this transaction implementation, or isoFlags was not equal to zero.
pub const XACT_E_NOISORETAIN: HRESULT = 0x8004D00Bu32 as HRESULT;

/// There is no resource presently associated with this enlistment
pub const XACT_E_NORESOURCE: HRESULT = 0x8004D00Cu32 as HRESULT;

/// The transaction failed to commit due to the failure of optimistic concurrency control in at least one of the resource managers.
pub const XACT_E_NOTCURRENT: HRESULT = 0x8004D00Du32 as HRESULT;

/// The transaction has already been implicitly or explicitly committed or aborted
pub const XACT_E_NOTRANSACTION: HRESULT = 0x8004D00Eu32 as HRESULT;

/// An invalid combination of flags was specified
pub const XACT_E_NOTSUPPORTED: HRESULT = 0x8004D00Fu32 as HRESULT;

/// The resource manager id is not associated with this transaction or the transaction manager.
pub const XACT_E_UNKNOWNRMGRID: HRESULT = 0x8004D010u32 as HRESULT;

/// This method was called in the wrong state
pub const XACT_E_WRONGSTATE: HRESULT = 0x8004D011u32 as HRESULT;

/// The indicated unit of work does not match the unit of work expected by the resource manager.
pub const XACT_E_WRONGUOW: HRESULT = 0x8004D012u32 as HRESULT;

/// An enlistment in a transaction already exists.
pub const XACT_E_XTIONEXISTS: HRESULT = 0x8004D013u32 as HRESULT;

/// An import object for the transaction could not be found.
pub const XACT_E_NOIMPORTOBJECT: HRESULT = 0x8004D014u32 as HRESULT;

/// The transaction cookie is invalid.
pub const XACT_E_INVALIDCOOKIE: HRESULT = 0x8004D015u32 as HRESULT;

/// The transaction status is in doubt. A communication failure occurred, or a transaction manager or resource manager has failed
pub const XACT_E_INDOUBT: HRESULT = 0x8004D016u32 as HRESULT;

/// A time-out was specified, but time-outs are not supported.
pub const XACT_E_NOTIMEOUT: HRESULT = 0x8004D017u32 as HRESULT;

/// The requested operation is already in progress for the transaction.
pub const XACT_E_ALREADYINPROGRESS: HRESULT = 0x8004D018u32 as HRESULT;

/// The transaction has already been aborted.
pub const XACT_E_ABORTED: HRESULT = 0x8004D019u32 as HRESULT;

/// The Transaction Manager returned a log full error.
pub const XACT_E_LOGFULL: HRESULT = 0x8004D01Au32 as HRESULT;

/// The Transaction Manager is not available.
pub const XACT_E_TMNOTAVAILABLE: HRESULT = 0x8004D01Bu32 as HRESULT;

/// A connection with the transaction manager was lost.
pub const XACT_E_CONNECTION_DOWN: HRESULT = 0x8004D01Cu32 as HRESULT;

/// A request to establish a connection with the transaction manager was denied.
pub const XACT_E_CONNECTION_DENIED: HRESULT = 0x8004D01Du32 as HRESULT;

/// Resource manager reenlistment to determine transaction status timed out.
pub const XACT_E_REENLISTTIMEOUT: HRESULT = 0x8004D01Eu32 as HRESULT;

/// This transaction manager failed to establish a connection with another TIP transaction manager.
pub const XACT_E_TIP_CONNECT_FAILED: HRESULT = 0x8004D01Fu32 as HRESULT;

/// This transaction manager encountered a protocol error with another TIP transaction manager.
pub const XACT_E_TIP_PROTOCOL_ERROR: HRESULT = 0x8004D020u32 as HRESULT;

/// This transaction manager could not propagate a transaction from another TIP transaction manager.
pub const XACT_E_TIP_PULL_FAILED: HRESULT = 0x8004D021u32 as HRESULT;

/// The Transaction Manager on the destination machine is not available.
pub const XACT_E_DEST_TMNOTAVAILABLE: HRESULT = 0x8004D022u32 as HRESULT;

/// The Transaction Manager has disabled its support for TIP.
pub const XACT_E_TIP_DISABLED: HRESULT = 0x8004D023u32 as HRESULT;

/// The transaction manager has disabled its support for remote/network transactions.
pub const XACT_E_NETWORK_TX_DISABLED: HRESULT = 0x8004D024u32 as HRESULT;

/// The partner transaction manager has disabled its support for remote/network transactions.
pub const XACT_E_PARTNER_NETWORK_TX_DISABLED: HRESULT = 0x8004D025u32 as HRESULT;

/// The transaction manager has disabled its support for XA transactions.
pub const XACT_E_XA_TX_DISABLED: HRESULT = 0x8004D026u32 as HRESULT;

/// MSDTC was unable to read its configuration information.
pub const XACT_E_UNABLE_TO_READ_DTC_CONFIG: HRESULT = 0x8004D027u32 as HRESULT;

/// MSDTC was unable to load the dtc proxy dll.
pub const XACT_E_UNABLE_TO_LOAD_DTC_PROXY: HRESULT = 0x8004D028u32 as HRESULT;

/// The local transaction has aborted.
pub const XACT_E_ABORTING: HRESULT = 0x8004D029u32 as HRESULT;

/// The MSDTC transaction manager was unable to push the transaction to the destination transaction manager due to communication problems. Possible causes are: a firewall is present and it doesn't have an exception for the MSDTC process, the two machines cannot find each other by their NetBIOS names, or the support for network transactions is not enabled for one of the two transaction managers.
pub const XACT_E_PUSH_COMM_FAILURE: HRESULT = 0x8004D02Au32 as HRESULT;

/// The MSDTC transaction manager was unable to pull the transaction from the source transaction manager due to communication problems. Possible causes are: a firewall is present and it doesn't have an exception for the MSDTC process, the two machines cannot find each other by their NetBIOS names, or the support for network transactions is not enabled for one of the two transaction managers.
pub const XACT_E_PULL_COMM_FAILURE: HRESULT = 0x8004D02Bu32 as HRESULT;

/// The MSDTC transaction manager has disabled its support for SNA LU 6.2 transactions.
pub const XACT_E_LU_TX_DISABLED: HRESULT = 0x8004D02Cu32 as HRESULT;

///  XACT_E_CLERKNOTFOUND
pub const XACT_E_CLERKNOTFOUND: HRESULT = 0x8004D080u32 as HRESULT;

///  XACT_E_CLERKEXISTS
pub const XACT_E_CLERKEXISTS: HRESULT = 0x8004D081u32 as HRESULT;

///  XACT_E_RECOVERYINPROGRESS
pub const XACT_E_RECOVERYINPROGRESS: HRESULT = 0x8004D082u32 as HRESULT;

///  XACT_E_TRANSACTIONCLOSED
pub const XACT_E_TRANSACTIONCLOSED: HRESULT = 0x8004D083u32 as HRESULT;

///  XACT_E_INVALIDLSN
pub const XACT_E_INVALIDLSN: HRESULT = 0x8004D084u32 as HRESULT;

///  XACT_E_REPLAYREQUEST
pub const XACT_E_REPLAYREQUEST: HRESULT = 0x8004D085u32 as HRESULT;

/// An asynchronous operation was specified. The operation has begun, but its outcome is not known yet.
pub const XACT_S_ASYNC: HRESULT = 0x0004D000;

///  XACT_S_DEFECT
pub const XACT_S_DEFECT: HRESULT = 0x0004D001;

/// The method call succeeded because the transaction was read-only.
pub const XACT_S_READONLY: HRESULT = 0x0004D002;

/// The transaction was successfully aborted. However, this is a coordinated transaction, and some number of enlisted resources were aborted outright because they could not support abort-retaining semantics
pub const XACT_S_SOMENORETAIN: HRESULT = 0x0004D003;

/// No changes were made during this call, but the sink wants another chance to look if any other sinks make further changes.
pub const XACT_S_OKINFORM: HRESULT = 0x0004D004;

/// The sink is content and wishes the transaction to proceed. Changes were made to one or more resources during this call.
pub const XACT_S_MADECHANGESCONTENT: HRESULT = 0x0004D005;

/// The sink is for the moment and wishes the transaction to proceed, but if other changes are made following this return by other event sinks then this sink wants another chance to look
pub const XACT_S_MADECHANGESINFORM: HRESULT = 0x0004D006;

/// The transaction was successfully aborted. However, the abort was non-retaining.
pub const XACT_S_ALLNORETAIN: HRESULT = 0x0004D007;

/// An abort operation was already in progress.
pub const XACT_S_ABORTING: HRESULT = 0x0004D008;

/// The resource manager has performed a single-phase commit of the transaction.
pub const XACT_S_SINGLEPHASE: HRESULT = 0x0004D009;

/// The local transaction has not aborted.
pub const XACT_S_LOCALLY_OK: HRESULT = 0x0004D00A;

/// The resource manager has requested to be the coordinator (last resource manager) for the transaction.
pub const XACT_S_LASTRESOURCEMANAGER: HRESULT = 0x0004D010;

/// The root transaction wanted to commit, but transaction aborted
pub const CONTEXT_E_ABORTED: HRESULT = 0x8004E002u32 as HRESULT;

/// You made a method call on a COM+ component that has a transaction that has already aborted or in the process of aborting.
pub const CONTEXT_E_ABORTING: HRESULT = 0x8004E003u32 as HRESULT;

/// There is no MTS object context
pub const CONTEXT_E_NOCONTEXT: HRESULT = 0x8004E004u32 as HRESULT;

/// The component is configured to use synchronization and this method call would cause a deadlock to occur.
pub const CONTEXT_E_WOULD_DEADLOCK: HRESULT = 0x8004E005u32 as HRESULT;

/// The component is configured to use synchronization and a thread has timed out waiting to enter the context.
pub const CONTEXT_E_SYNCH_TIMEOUT: HRESULT = 0x8004E006u32 as HRESULT;

/// You made a method call on a COM+ component that has a transaction that has already committed or aborted.
pub const CONTEXT_E_OLDREF: HRESULT = 0x8004E007u32 as HRESULT;

/// The specified role was not configured for the application
pub const CONTEXT_E_ROLENOTFOUND: HRESULT = 0x8004E00Cu32 as HRESULT;

/// COM+ was unable to talk to the Microsoft Distributed Transaction Coordinator
pub const CONTEXT_E_TMNOTAVAILABLE: HRESULT = 0x8004E00Fu32 as HRESULT;

/// An unexpected error occurred during COM+ Activation.
pub const CO_E_ACTIVATIONFAILED: HRESULT = 0x8004E021u32 as HRESULT;

/// COM+ Activation failed. Check the event log for more information
pub const CO_E_ACTIVATIONFAILED_EVENTLOGGED: HRESULT = 0x8004E022u32 as HRESULT;

/// COM+ Activation failed due to a catalog or configuration error.
pub const CO_E_ACTIVATIONFAILED_CATALOGERROR: HRESULT = 0x8004E023u32 as HRESULT;

/// COM+ activation failed because the activation could not be completed in the specified amount of time.
pub const CO_E_ACTIVATIONFAILED_TIMEOUT: HRESULT = 0x8004E024u32 as HRESULT;

/// COM+ Activation failed because an initialization function failed. Check the event log for more information.
pub const CO_E_INITIALIZATIONFAILED: HRESULT = 0x8004E025u32 as HRESULT;

/// The requested operation requires that JIT be in the current context and it is not
pub const CONTEXT_E_NOJIT: HRESULT = 0x8004E026u32 as HRESULT;

/// The requested operation requires that the current context have a Transaction, and it does not
pub const CONTEXT_E_NOTRANSACTION: HRESULT = 0x8004E027u32 as HRESULT;

/// The components threading model has changed after install into a COM+ Application. Please re-install component.
pub const CO_E_THREADINGMODEL_CHANGED: HRESULT = 0x8004E028u32 as HRESULT;

/// IIS intrinsics not available. Start your work with IIS.
pub const CO_E_NOIISINTRINSICS: HRESULT = 0x8004E029u32 as HRESULT;

/// An attempt to write a cookie failed.
pub const CO_E_NOCOOKIES: HRESULT = 0x8004E02Au32 as HRESULT;

/// An attempt to use a database generated a database specific error.
pub const CO_E_DBERROR: HRESULT = 0x8004E02Bu32 as HRESULT;

/// The COM+ component you created must use object pooling to work.
pub const CO_E_NOTPOOLED: HRESULT = 0x8004E02Cu32 as HRESULT;

/// The COM+ component you created must use object construction to work correctly.
pub const CO_E_NOTCONSTRUCTED: HRESULT = 0x8004E02Du32 as HRESULT;

/// The COM+ component requires synchronization, and it is not configured for it.
pub const CO_E_NOSYNCHRONIZATION: HRESULT = 0x8004E02Eu32 as HRESULT;

/// The TxIsolation Level property for the COM+ component being created is stronger than the TxIsolationLevel for the "root" component for the transaction. The creation failed.
pub const CO_E_ISOLEVELMISMATCH: HRESULT = 0x8004E02Fu32 as HRESULT;

/// The component attempted to make a cross-context call between invocations of EnterTransactionScopeand ExitTransactionScope. This is not allowed. Cross-context calls cannot be made while inside of a transaction scope.
pub const CO_E_CALL_OUT_OF_TX_SCOPE_NOT_ALLOWED: HRESULT = 0x8004E030u32 as HRESULT;

/// The component made a call to EnterTransactionScope, but did not make a corresponding call to ExitTransactionScope before returning.
pub const CO_E_EXIT_TRANSACTION_SCOPE_NOT_CALLED: HRESULT = 0x8004E031u32 as HRESULT;

/// Use the registry database to provide the requested information
pub const OLE_S_USEREG: HRESULT = 0x00040000;

/// Success, but static
pub const OLE_S_STATIC: HRESULT = 0x00040001;

/// Macintosh clipboard format
pub const OLE_S_MAC_CLIPFORMAT: HRESULT = 0x00040002;

/// Successful drop took place
pub const DRAGDROP_S_DROP: HRESULT = 0x00040100;

/// Drag-drop operation canceled
pub const DRAGDROP_S_CANCEL: HRESULT = 0x00040101;

/// Use the default cursor
pub const DRAGDROP_S_USEDEFAULTCURSORS: HRESULT = 0x00040102;

/// Data has same FORMATETC
pub const DATA_S_SAMEFORMATETC: HRESULT = 0x00040130;

/// View is already frozen
pub const VIEW_S_ALREADY_FROZEN: HRESULT = 0x00040140;

/// FORMATETC not supported
pub const CACHE_S_FORMATETC_NOTSUPPORTED: HRESULT = 0x00040170;

/// Same cache
pub const CACHE_S_SAMECACHE: HRESULT = 0x00040171;

/// Some cache(s) not updated
pub const CACHE_S_SOMECACHES_NOTUPDATED: HRESULT = 0x00040172;

/// Invalid verb for OLE object
pub const OLEOBJ_S_INVALIDVERB: HRESULT = 0x00040180;

/// Verb number is valid but verb cannot be done now
pub const OLEOBJ_S_CANNOT_DOVERB_NOW: HRESULT = 0x00040181;

/// Invalid window handle passed
pub const OLEOBJ_S_INVALIDHWND: HRESULT = 0x00040182;

/// Message is too long; some of it had to be truncated before displaying
pub const INPLACE_S_TRUNCATED: HRESULT = 0x000401A0;

/// Unable to convert OLESTREAM to IStorage
pub const CONVERT10_S_NO_PRESENTATION: HRESULT = 0x000401C0;

/// Moniker reduced to itself
pub const MK_S_REDUCED_TO_SELF: HRESULT = 0x000401E2;

/// Common prefix is this moniker
pub const MK_S_ME: HRESULT = 0x000401E4;

/// Common prefix is input moniker
pub const MK_S_HIM: HRESULT = 0x000401E5;

/// Common prefix is both monikers
pub const MK_S_US: HRESULT = 0x000401E6;

/// Moniker is already registered in running object table
pub const MK_S_MONIKERALREADYREGISTERED: HRESULT = 0x000401E7;

/// The task is ready to run at its next scheduled time.
pub const SCHED_S_TASK_READY: HRESULT = 0x00041300;

/// The task is currently running.
pub const SCHED_S_TASK_RUNNING: HRESULT = 0x00041301;

/// The task will not run at the scheduled times because it has been disabled.
pub const SCHED_S_TASK_DISABLED: HRESULT = 0x00041302;

/// The task has not yet run.
pub const SCHED_S_TASK_HAS_NOT_RUN: HRESULT = 0x00041303;

/// There are no more runs scheduled for this task.
pub const SCHED_S_TASK_NO_MORE_RUNS: HRESULT = 0x00041304;

/// One or more of the properties that are needed to run this task on a schedule have not been set.
pub const SCHED_S_TASK_NOT_SCHEDULED: HRESULT = 0x00041305;

/// The last run of the task was terminated by the user.
pub const SCHED_S_TASK_TERMINATED: HRESULT = 0x00041306;

/// Either the task has no triggers or the existing triggers are disabled or not set.
pub const SCHED_S_TASK_NO_VALID_TRIGGERS: HRESULT = 0x00041307;

/// Event triggers don't have set run times.
pub const SCHED_S_EVENT_TRIGGER: HRESULT = 0x00041308;

/// Trigger not found.
pub const SCHED_E_TRIGGER_NOT_FOUND: HRESULT = 0x80041309u32 as HRESULT;

/// One or more of the properties that are needed to run this task have not been set.
pub const SCHED_E_TASK_NOT_READY: HRESULT = 0x8004130Au32 as HRESULT;

/// There is no running instance of the task.
pub const SCHED_E_TASK_NOT_RUNNING: HRESULT = 0x8004130Bu32 as HRESULT;

/// The Task Scheduler Service is not installed on this computer.
pub const SCHED_E_SERVICE_NOT_INSTALLED: HRESULT = 0x8004130Cu32 as HRESULT;

/// The task object could not be opened.
pub const SCHED_E_CANNOT_OPEN_TASK: HRESULT = 0x8004130Du32 as HRESULT;

/// The object is either an invalid task object or is not a task object.
pub const SCHED_E_INVALID_TASK: HRESULT = 0x8004130Eu32 as HRESULT;

/// No account information could be found in the Task Scheduler security database for the task indicated.
pub const SCHED_E_ACCOUNT_INFORMATION_NOT_SET: HRESULT = 0x8004130Fu32 as HRESULT;

/// Unable to establish existence of the account specified.
pub const SCHED_E_ACCOUNT_NAME_NOT_FOUND: HRESULT = 0x80041310u32 as HRESULT;

/// Corruption was detected in the Task Scheduler security database; the database has been reset.
pub const SCHED_E_ACCOUNT_DBASE_CORRUPT: HRESULT = 0x80041311u32 as HRESULT;

/// Task Scheduler security services are available only on Windows NT.
pub const SCHED_E_NO_SECURITY_SERVICES: HRESULT = 0x80041312u32 as HRESULT;

/// The task object version is either unsupported or invalid.
pub const SCHED_E_UNKNOWN_OBJECT_VERSION: HRESULT = 0x80041313u32 as HRESULT;

/// The task has been configured with an unsupported combination of account settings and run time options.
pub const SCHED_E_UNSUPPORTED_ACCOUNT_OPTION: HRESULT = 0x80041314u32 as HRESULT;

/// The Task Scheduler Service is not running.
pub const SCHED_E_SERVICE_NOT_RUNNING: HRESULT = 0x80041315u32 as HRESULT;

/// The task XML contains an unexpected node.
pub const SCHED_E_UNEXPECTEDNODE: HRESULT = 0x80041316u32 as HRESULT;

/// The task XML contains an element or attribute from an unexpected namespace.
pub const SCHED_E_NAMESPACE: HRESULT = 0x80041317u32 as HRESULT;

/// The task XML contains a value which is incorrectly formatted or out of range.
pub const SCHED_E_INVALIDVALUE: HRESULT = 0x80041318u32 as HRESULT;

/// The task XML is missing a required element or attribute.
pub const SCHED_E_MISSINGNODE: HRESULT = 0x80041319u32 as HRESULT;

/// The task XML is malformed.
pub const SCHED_E_MALFORMEDXML: HRESULT = 0x8004131Au32 as HRESULT;

/// The task is registered, but not all specified triggers will start the task, check task scheduler event log for detailed information.
pub const SCHED_S_SOME_TRIGGERS_FAILED: HRESULT = 0x0004131B;

/// The task is registered, but may fail to start. Batch logon privilege needs to be enabled for the task principal.
pub const SCHED_S_BATCH_LOGON_PROBLEM: HRESULT = 0x0004131C;

/// The task XML contains too many nodes of the same type.
pub const SCHED_E_TOO_MANY_NODES: HRESULT = 0x8004131Du32 as HRESULT;

/// The task cannot be started after the trigger's end boundary.
pub const SCHED_E_PAST_END_BOUNDARY: HRESULT = 0x8004131Eu32 as HRESULT;

/// An instance of this task is already running.
pub const SCHED_E_ALREADY_RUNNING: HRESULT = 0x8004131Fu32 as HRESULT;

/// The task will not run because the user is not logged on.
pub const SCHED_E_USER_NOT_LOGGED_ON: HRESULT = 0x80041320u32 as HRESULT;

/// The task image is corrupt or has been tampered with.
pub const SCHED_E_INVALID_TASK_HASH: HRESULT = 0x80041321u32 as HRESULT;

/// The Task Scheduler service is not available.
pub const SCHED_E_SERVICE_NOT_AVAILABLE: HRESULT = 0x80041322u32 as HRESULT;

/// The Task Scheduler service is too busy to handle your request. Please try again later.
pub const SCHED_E_SERVICE_TOO_BUSY: HRESULT = 0x80041323u32 as HRESULT;

/// The Task Scheduler service attempted to run the task, but the task did not run due to one of the constraints in the task definition.
pub const SCHED_E_TASK_ATTEMPTED: HRESULT = 0x80041324u32 as HRESULT;

/// The Task Scheduler service has asked the task to run.
pub const SCHED_S_TASK_QUEUED: HRESULT = 0x00041325;

/// The task is disabled.
pub const SCHED_E_TASK_DISABLED: HRESULT = 0x80041326u32 as HRESULT;

/// The task has properties that are not compatible with previous versions of Windows.
pub const SCHED_E_TASK_NOT_V1_COMPAT: HRESULT = 0x80041327u32 as HRESULT;

/// The task settings do not allow the task to start on demand.
pub const SCHED_E_START_ON_DEMAND: HRESULT = 0x80041328u32 as HRESULT;

/// The combination of properties that task is using is not compatible with the scheduling engine.
pub const SCHED_E_TASK_NOT_UBPM_COMPAT: HRESULT = 0x80041329u32 as HRESULT;

/// The task definition uses a deprecated feature.
pub const SCHED_E_DEPRECATED_FEATURE_USED: HRESULT = 0x80041330u32 as HRESULT;

/// Attempt to create a class object failed
pub const CO_E_CLASS_CREATE_FAILED: HRESULT = 0x80080001u32 as HRESULT;

/// OLE service could not bind object
pub const CO_E_SCM_ERROR: HRESULT = 0x80080002u32 as HRESULT;

/// RPC communication failed with OLE service
pub const CO_E_SCM_RPC_FAILURE: HRESULT = 0x80080003u32 as HRESULT;

/// Bad path to object
pub const CO_E_BAD_PATH: HRESULT = 0x80080004u32 as HRESULT;

/// Server execution failed
pub const CO_E_SERVER_EXEC_FAILURE: HRESULT = 0x80080005u32 as HRESULT;

/// OLE service could not communicate with the object server
pub const CO_E_OBJSRV_RPC_FAILURE: HRESULT = 0x80080006u32 as HRESULT;

/// Moniker path could not be normalized
pub const MK_E_NO_NORMALIZED: HRESULT = 0x80080007u32 as HRESULT;

/// Object server is stopping when OLE service contacts it
pub const CO_E_SERVER_STOPPING: HRESULT = 0x80080008u32 as HRESULT;

/// An invalid root block pointer was specified
pub const MEM_E_INVALID_ROOT: HRESULT = 0x80080009u32 as HRESULT;

/// An allocation chain contained an invalid link pointer
pub const MEM_E_INVALID_LINK: HRESULT = 0x80080010u32 as HRESULT;

/// The requested allocation size was too large
pub const MEM_E_INVALID_SIZE: HRESULT = 0x80080011u32 as HRESULT;

/// Not all the requested interfaces were available
pub const CO_S_NOTALLINTERFACES: HRESULT = 0x00080012;

/// The specified machine name was not found in the cache.
pub const CO_S_MACHINENAMENOTFOUND: HRESULT = 0x00080013;

/// The activation requires a display name to be present under the CLSID key.
pub const CO_E_MISSING_DISPLAYNAME: HRESULT = 0x80080015u32 as HRESULT;

/// The activation requires that the RunAs value for the application is Activate As Activator.
pub const CO_E_RUNAS_VALUE_MUST_BE_AAA: HRESULT = 0x80080016u32 as HRESULT;

/// The class is not configured to support Elevated activation.
pub const CO_E_ELEVATION_DISABLED: HRESULT = 0x80080017u32 as HRESULT;

/// Appx packaging API has encountered an internal error.
pub const APPX_E_PACKAGING_INTERNAL: HRESULT = 0x80080200u32 as HRESULT;

/// The file is not a valid Appx package because its contents are interleaved.
pub const APPX_E_INTERLEAVING_NOT_ALLOWED: HRESULT = 0x80080201u32 as HRESULT;

/// The file is not a valid Appx package because it contains OPC relationships.
pub const APPX_E_RELATIONSHIPS_NOT_ALLOWED: HRESULT = 0x80080202u32 as HRESULT;

/// The file is not a valid Appx package because it is missing a manifest or block map, or missing a signature file when the code integrity file is present.
pub const APPX_E_MISSING_REQUIRED_FILE: HRESULT = 0x80080203u32 as HRESULT;

/// The Appx package's manifest is invalid.
pub const APPX_E_INVALID_MANIFEST: HRESULT = 0x80080204u32 as HRESULT;

/// The Appx package's block map is invalid.
pub const APPX_E_INVALID_BLOCKMAP: HRESULT = 0x80080205u32 as HRESULT;

/// The Appx package's content cannot be read because it is corrupt.
pub const APPX_E_CORRUPT_CONTENT: HRESULT = 0x80080206u32 as HRESULT;

/// The computed hash value of the block does not match the one stored in the block map.
pub const APPX_E_BLOCK_HASH_INVALID: HRESULT = 0x80080207u32 as HRESULT;

/// The requested byte range is over 4GB when translated to byte range of blocks.
pub const APPX_E_REQUESTED_RANGE_TOO_LARGE: HRESULT = 0x80080208u32 as HRESULT;

/// The SIP_SUBJECTINFO structure used to sign the package didn't contain the required data.
pub const APPX_E_INVALID_SIP_CLIENT_DATA: HRESULT = 0x80080209u32 as HRESULT;

/// The APPX_KEY_INFO structure used to encrypt or decrypt the package contains invalid data.
pub const APPX_E_INVALID_KEY_INFO: HRESULT = 0x8008020Au32 as HRESULT;

/// The Appx package's content group map is invalid.
pub const APPX_E_INVALID_CONTENTGROUPMAP: HRESULT = 0x8008020Bu32 as HRESULT;

/// The .appinstaller file is invalid.
pub const APPX_E_INVALID_APPINSTALLER: HRESULT = 0x8008020Cu32 as HRESULT;

/// The baseline package version in delta package does not match the version in the baseline package to be updated.
pub const APPX_E_DELTA_BASELINE_VERSION_MISMATCH: HRESULT = 0x8008020Du32 as HRESULT;

/// The delta package is missing a file from the updated package.
pub const APPX_E_DELTA_PACKAGE_MISSING_FILE: HRESULT = 0x8008020Eu32 as HRESULT;

/// The delta package is invalid.
pub const APPX_E_INVALID_DELTA_PACKAGE: HRESULT = 0x8008020Fu32 as HRESULT;

/// The delta appended package is not allowed for the current operation.
pub const APPX_E_DELTA_APPENDED_PACKAGE_NOT_ALLOWED: HRESULT = 0x80080210u32 as HRESULT;

/// The packaging layout file is invalid.
pub const APPX_E_INVALID_PACKAGING_LAYOUT: HRESULT = 0x80080211u32 as HRESULT;

/// The packageSignConfig file is invalid.
pub const APPX_E_INVALID_PACKAGESIGNCONFIG: HRESULT = 0x80080212u32 as HRESULT;

/// The resources.pri file is not allowed when there are no resource elements in the package manifest.
pub const APPX_E_RESOURCESPRI_NOT_ALLOWED: HRESULT = 0x80080213u32 as HRESULT;

/// The compression state of file in baseline and updated package does not match.
pub const APPX_E_FILE_COMPRESSION_MISMATCH: HRESULT = 0x80080214u32 as HRESULT;

/// Non appx extensions are not allowed for payload packages targeting older platforms.
pub const APPX_E_INVALID_PAYLOAD_PACKAGE_EXTENSION: HRESULT = 0x80080215u32 as HRESULT;

/// The encryptionExclusionFileList file is invalid.
pub const APPX_E_INVALID_ENCRYPTION_EXCLUSION_FILE_LIST: HRESULT = 0x80080216u32 as HRESULT;

/// The package folder ACLs are invalid.
pub const APPX_E_INVALID_PACKAGE_FOLDER_ACLS: HRESULT = 0x80080217u32 as HRESULT;

/// The publisher bridging artifact is invalid.
pub const APPX_E_INVALID_PUBLISHER_BRIDGING: HRESULT = 0x80080218u32 as HRESULT;

/// The expected digest value did not match the actual digest value of the content.
pub const APPX_E_DIGEST_MISMATCH: HRESULT = 0x80080219u32 as HRESULT;

/// The background task activation is spurious.
pub const BT_E_SPURIOUS_ACTIVATION: HRESULT = 0x80080300u32 as HRESULT;

/// Unknown interface.
pub const DISP_E_UNKNOWNINTERFACE: HRESULT = 0x80020001u32 as HRESULT;

/// Member not found.
pub const DISP_E_MEMBERNOTFOUND: HRESULT = 0x80020003u32 as HRESULT;

/// Parameter not found.
pub const DISP_E_PARAMNOTFOUND: HRESULT = 0x80020004u32 as HRESULT;

/// Type mismatch.
pub const DISP_E_TYPEMISMATCH: HRESULT = 0x80020005u32 as HRESULT;

/// Unknown name.
pub const DISP_E_UNKNOWNNAME: HRESULT = 0x80020006u32 as HRESULT;

/// No named arguments.
pub const DISP_E_NONAMEDARGS: HRESULT = 0x80020007u32 as HRESULT;

/// Bad variable type.
pub const DISP_E_BADVARTYPE: HRESULT = 0x80020008u32 as HRESULT;

/// Exception occurred.
pub const DISP_E_EXCEPTION: HRESULT = 0x80020009u32 as HRESULT;

/// Out of present range.
pub const DISP_E_OVERFLOW: HRESULT = 0x8002000Au32 as HRESULT;

/// Invalid index.
pub const DISP_E_BADINDEX: HRESULT = 0x8002000Bu32 as HRESULT;

/// Unknown language.
pub const DISP_E_UNKNOWNLCID: HRESULT = 0x8002000Cu32 as HRESULT;

/// Memory is locked.
pub const DISP_E_ARRAYISLOCKED: HRESULT = 0x8002000Du32 as HRESULT;

/// Invalid number of parameters.
pub const DISP_E_BADPARAMCOUNT: HRESULT = 0x8002000Eu32 as HRESULT;

/// Parameter not optional.
pub const DISP_E_PARAMNOTOPTIONAL: HRESULT = 0x8002000Fu32 as HRESULT;

/// Invalid callee.
pub const DISP_E_BADCALLEE: HRESULT = 0x80020010u32 as HRESULT;

/// Does not support a collection.
pub const DISP_E_NOTACOLLECTION: HRESULT = 0x80020011u32 as HRESULT;

/// Division by zero.
pub const DISP_E_DIVBYZERO: HRESULT = 0x80020012u32 as HRESULT;

/// Buffer too small
pub const DISP_E_BUFFERTOOSMALL: HRESULT = 0x80020013u32 as HRESULT;

/// Buffer too small.
pub const TYPE_E_BUFFERTOOSMALL: HRESULT = 0x80028016u32 as HRESULT;

/// Field name not defined in the record.
pub const TYPE_E_FIELDNOTFOUND: HRESULT = 0x80028017u32 as HRESULT;

/// Old format or invalid type library.
pub const TYPE_E_INVDATAREAD: HRESULT = 0x80028018u32 as HRESULT;

/// Old format or invalid type library.
pub const TYPE_E_UNSUPFORMAT: HRESULT = 0x80028019u32 as HRESULT;

/// Error accessing the OLE registry.
pub const TYPE_E_REGISTRYACCESS: HRESULT = 0x8002801Cu32 as HRESULT;

/// Library not registered.
pub const TYPE_E_LIBNOTREGISTERED: HRESULT = 0x8002801Du32 as HRESULT;

/// Bound to unknown type.
pub const TYPE_E_UNDEFINEDTYPE: HRESULT = 0x80028027u32 as HRESULT;

/// Qualified name disallowed.
pub const TYPE_E_QUALIFIEDNAMEDISALLOWED: HRESULT = 0x80028028u32 as HRESULT;

/// Invalid forward reference, or reference to uncompiled type.
pub const TYPE_E_INVALIDSTATE: HRESULT = 0x80028029u32 as HRESULT;

/// Type mismatch.
pub const TYPE_E_WRONGTYPEKIND: HRESULT = 0x8002802Au32 as HRESULT;

/// Element not found.
pub const TYPE_E_ELEMENTNOTFOUND: HRESULT = 0x8002802Bu32 as HRESULT;

/// Ambiguous name.
pub const TYPE_E_AMBIGUOUSNAME: HRESULT = 0x8002802Cu32 as HRESULT;

/// Name already exists in the library.
pub const TYPE_E_NAMECONFLICT: HRESULT = 0x8002802Du32 as HRESULT;

/// Unknown LCID.
pub const TYPE_E_UNKNOWNLCID: HRESULT = 0x8002802Eu32 as HRESULT;

/// Function not defined in specified DLL.
pub const TYPE_E_DLLFUNCTIONNOTFOUND: HRESULT = 0x8002802Fu32 as HRESULT;

/// Wrong module kind for the operation.
pub const TYPE_E_BADMODULEKIND: HRESULT = 0x800288BDu32 as HRESULT;

/// Size may not exceed 64K.
pub const TYPE_E_SIZETOOBIG: HRESULT = 0x800288C5u32 as HRESULT;

/// Duplicate ID in inheritance hierarchy.
pub const TYPE_E_DUPLICATEID: HRESULT = 0x800288C6u32 as HRESULT;

/// Incorrect inheritance depth in standard OLE hmember.
pub const TYPE_E_INVALIDID: HRESULT = 0x800288CFu32 as HRESULT;

/// Type mismatch.
pub const TYPE_E_TYPEMISMATCH: HRESULT = 0x80028CA0u32 as HRESULT;

/// Invalid number of arguments.
pub const TYPE_E_OUTOFBOUNDS: HRESULT = 0x80028CA1u32 as HRESULT;

/// I/O Error.
pub const TYPE_E_IOERROR: HRESULT = 0x80028CA2u32 as HRESULT;

/// Error creating unique tmp file.
pub const TYPE_E_CANTCREATETMPFILE: HRESULT = 0x80028CA3u32 as HRESULT;

/// Error loading type library/DLL.
pub const TYPE_E_CANTLOADLIBRARY: HRESULT = 0x80029C4Au32 as HRESULT;

/// Inconsistent property functions.
pub const TYPE_E_INCONSISTENTPROPFUNCS: HRESULT = 0x80029C83u32 as HRESULT;

/// Circular dependency between types/modules.
pub const TYPE_E_CIRCULARTYPE: HRESULT = 0x80029C84u32 as HRESULT;

/// Unable to perform requested operation.
pub const STG_E_INVALIDFUNCTION: HRESULT = 0x80030001u32 as HRESULT;

/// %1 could not be found.
pub const STG_E_FILENOTFOUND: HRESULT = 0x80030002u32 as HRESULT;

/// The path %1 could not be found.
pub const STG_E_PATHNOTFOUND: HRESULT = 0x80030003u32 as HRESULT;

/// There are insufficient resources to open another file.
pub const STG_E_TOOMANYOPENFILES: HRESULT = 0x80030004u32 as HRESULT;

/// Access Denied.
pub const STG_E_ACCESSDENIED: HRESULT = 0x80030005u32 as HRESULT;

/// Attempted an operation on an invalid object.
pub const STG_E_INVALIDHANDLE: HRESULT = 0x80030006u32 as HRESULT;

/// There is insufficient memory available to complete operation.
pub const STG_E_INSUFFICIENTMEMORY: HRESULT = 0x80030008u32 as HRESULT;

/// Invalid pointer error.
pub const STG_E_INVALIDPOINTER: HRESULT = 0x80030009u32 as HRESULT;

/// There are no more entries to return.
pub const STG_E_NOMOREFILES: HRESULT = 0x80030012u32 as HRESULT;

/// Disk is write-protected.
pub const STG_E_DISKISWRITEPROTECTED: HRESULT = 0x80030013u32 as HRESULT;

/// An error occurred during a seek operation.
pub const STG_E_SEEKERROR: HRESULT = 0x80030019u32 as HRESULT;

/// A disk error occurred during a write operation.
pub const STG_E_WRITEFAULT: HRESULT = 0x8003001Du32 as HRESULT;

/// A disk error occurred during a read operation.
pub const STG_E_READFAULT: HRESULT = 0x8003001Eu32 as HRESULT;

/// A share violation has occurred.
pub const STG_E_SHAREVIOLATION: HRESULT = 0x80030020u32 as HRESULT;

/// A lock violation has occurred.
pub const STG_E_LOCKVIOLATION: HRESULT = 0x80030021u32 as HRESULT;

/// %1 already exists.
pub const STG_E_FILEALREADYEXISTS: HRESULT = 0x80030050u32 as HRESULT;

/// Invalid parameter error.
pub const STG_E_INVALIDPARAMETER: HRESULT = 0x80030057u32 as HRESULT;

/// There is insufficient disk space to complete operation.
pub const STG_E_MEDIUMFULL: HRESULT = 0x80030070u32 as HRESULT;

/// Illegal write of non-simple property to simple property set.
pub const STG_E_PROPSETMISMATCHED: HRESULT = 0x800300F0u32 as HRESULT;

/// An API call exited abnormally.
pub const STG_E_ABNORMALAPIEXIT: HRESULT = 0x800300FAu32 as HRESULT;

/// The file %1 is not a valid compound file.
pub const STG_E_INVALIDHEADER: HRESULT = 0x800300FBu32 as HRESULT;

/// The name %1 is not valid.
pub const STG_E_INVALIDNAME: HRESULT = 0x800300FCu32 as HRESULT;

/// An unexpected error occurred.
pub const STG_E_UNKNOWN: HRESULT = 0x800300FDu32 as HRESULT;

/// That function is not implemented.
pub const STG_E_UNIMPLEMENTEDFUNCTION: HRESULT = 0x800300FEu32 as HRESULT;

/// Invalid flag error.
pub const STG_E_INVALIDFLAG: HRESULT = 0x800300FFu32 as HRESULT;

/// Attempted to use an object that is busy.
pub const STG_E_INUSE: HRESULT = 0x80030100u32 as HRESULT;

/// The storage has been changed since the last commit.
pub const STG_E_NOTCURRENT: HRESULT = 0x80030101u32 as HRESULT;

/// Attempted to use an object that has ceased to exist.
pub const STG_E_REVERTED: HRESULT = 0x80030102u32 as HRESULT;

/// Can't save.
pub const STG_E_CANTSAVE: HRESULT = 0x80030103u32 as HRESULT;

/// The compound file %1 was produced with an incompatible version of storage.
pub const STG_E_OLDFORMAT: HRESULT = 0x80030104u32 as HRESULT;

/// The compound file %1 was produced with a newer version of storage.
pub const STG_E_OLDDLL: HRESULT = 0x80030105u32 as HRESULT;

/// Share.exe or equivalent is required for operation.
pub const STG_E_SHAREREQUIRED: HRESULT = 0x80030106u32 as HRESULT;

/// Illegal operation called on non-file based storage.
pub const STG_E_NOTFILEBASEDSTORAGE: HRESULT = 0x80030107u32 as HRESULT;

/// Illegal operation called on object with extant marshallings.
pub const STG_E_EXTANTMARSHALLINGS: HRESULT = 0x80030108u32 as HRESULT;

/// The docfile has been corrupted.
pub const STG_E_DOCFILECORRUPT: HRESULT = 0x80030109u32 as HRESULT;

/// OLE32.DLL has been loaded at the wrong address.
pub const STG_E_BADBASEADDRESS: HRESULT = 0x80030110u32 as HRESULT;

/// The compound file is too large for the current implementation
pub const STG_E_DOCFILETOOLARGE: HRESULT = 0x80030111u32 as HRESULT;

/// The compound file was not created with the STGM_SIMPLE flag
pub const STG_E_NOTSIMPLEFORMAT: HRESULT = 0x80030112u32 as HRESULT;

/// The file download was aborted abnormally. The file is incomplete.
pub const STG_E_INCOMPLETE: HRESULT = 0x80030201u32 as HRESULT;

/// The file download has been terminated.
pub const STG_E_TERMINATED: HRESULT = 0x80030202u32 as HRESULT;

/// The underlying file was converted to compound file format.
pub const STG_S_CONVERTED: HRESULT = 0x00030200;

/// The storage operation should block until more data is available.
pub const STG_S_BLOCK: HRESULT = 0x00030201;

/// The storage operation should retry immediately.
pub const STG_S_RETRYNOW: HRESULT = 0x00030202;

/// The notified event sink will not influence the storage operation.
pub const STG_S_MONITORING: HRESULT = 0x00030203;

/// Multiple opens prevent consolidated. (commit succeeded).
pub const STG_S_MULTIPLEOPENS: HRESULT = 0x00030204;

/// Consolidation of the storage file failed. (commit succeeded).
pub const STG_S_CONSOLIDATIONFAILED: HRESULT = 0x00030205;

/// Consolidation of the storage file is inappropriate. (commit succeeded).
pub const STG_S_CANNOTCONSOLIDATE: HRESULT = 0x00030206;

/// The device needs to be power cycled. (commit succeeded).
pub const STG_S_POWER_CYCLE_REQUIRED: HRESULT = 0x00030207;

/// The specified firmware slot is invalid.
pub const STG_E_FIRMWARE_SLOT_INVALID: HRESULT = 0x80030208u32 as HRESULT;

/// The specified firmware image is invalid.
pub const STG_E_FIRMWARE_IMAGE_INVALID: HRESULT = 0x80030209u32 as HRESULT;

/// The storage device is unresponsive.
pub const STG_E_DEVICE_UNRESPONSIVE: HRESULT = 0x8003020Au32 as HRESULT;

/*++

 MessageId's 0x0305 - 0x031f (inclusive) are reserved for **STORAGE**
 copy protection errors.

--*/
/// Generic Copy Protection Error.
pub const STG_E_STATUS_COPY_PROTECTION_FAILURE: HRESULT = 0x80030305u32 as HRESULT;

/// Copy Protection Error - DVD CSS Authentication failed.
pub const STG_E_CSS_AUTHENTICATION_FAILURE: HRESULT = 0x80030306u32 as HRESULT;

/// Copy Protection Error - The given sector does not have a valid CSS key.
pub const STG_E_CSS_KEY_NOT_PRESENT: HRESULT = 0x80030307u32 as HRESULT;

/// Copy Protection Error - DVD session key not established.
pub const STG_E_CSS_KEY_NOT_ESTABLISHED: HRESULT = 0x80030308u32 as HRESULT;

/// Copy Protection Error - The read failed because the sector is encrypted.
pub const STG_E_CSS_SCRAMBLED_SECTOR: HRESULT = 0x80030309u32 as HRESULT;

/// Copy Protection Error - The current DVD's region does not correspond to the region setting of the drive.
pub const STG_E_CSS_REGION_MISMATCH: HRESULT = 0x8003030Au32 as HRESULT;

/// Copy Protection Error - The drive's region setting may be permanent or the number of user resets has been exhausted.
pub const STG_E_RESETS_EXHAUSTED: HRESULT = 0x8003030Bu32 as HRESULT;

/*++

 MessageId's 0x0305 - 0x031f (inclusive) are reserved for **STORAGE**
 copy protection errors.

--*/

/// Call was rejected by callee.
pub const RPC_E_CALL_REJECTED: HRESULT = 0x80010001u32 as HRESULT;

/// Call was canceled by the message filter.
pub const RPC_E_CALL_CANCELED: HRESULT = 0x80010002u32 as HRESULT;

/// The caller is dispatching an intertask SendMessage call and cannot call out via PostMessage.
pub const RPC_E_CANTPOST_INSENDCALL: HRESULT = 0x80010003u32 as HRESULT;

/// The caller is dispatching an asynchronous call and cannot make an outgoing call on behalf of this call.
pub const RPC_E_CANTCALLOUT_INASYNCCALL: HRESULT = 0x80010004u32 as HRESULT;

/// It is illegal to call out while inside message filter.
pub const RPC_E_CANTCALLOUT_INEXTERNALCALL: HRESULT = 0x80010005u32 as HRESULT;

/// The connection terminated or is in a bogus state and cannot be used any more. Other connections are still valid.
pub const RPC_E_CONNECTION_TERMINATED: HRESULT = 0x80010006u32 as HRESULT;

/// The callee (server [not server application]) is not available and disappeared; all connections are invalid. The call may have executed.
pub const RPC_E_SERVER_DIED: HRESULT = 0x80010007u32 as HRESULT;

/// The caller (client) disappeared while the callee (server) was processing a call.
pub const RPC_E_CLIENT_DIED: HRESULT = 0x80010008u32 as HRESULT;

/// The data packet with the marshalled parameter data is incorrect.
pub const RPC_E_INVALID_DATAPACKET: HRESULT = 0x80010009u32 as HRESULT;

/// The call was not transmitted properly; the message queue was full and was not emptied after yielding.
pub const RPC_E_CANTTRANSMIT_CALL: HRESULT = 0x8001000Au32 as HRESULT;

/// The client (caller) cannot marshall the parameter data - low memory, etc.
pub const RPC_E_CLIENT_CANTMARSHAL_DATA: HRESULT = 0x8001000Bu32 as HRESULT;

/// The client (caller) cannot unmarshall the return data - low memory, etc.
pub const RPC_E_CLIENT_CANTUNMARSHAL_DATA: HRESULT = 0x8001000Cu32 as HRESULT;

/// The server (callee) cannot marshall the return data - low memory, etc.
pub const RPC_E_SERVER_CANTMARSHAL_DATA: HRESULT = 0x8001000Du32 as HRESULT;

/// The server (callee) cannot unmarshall the parameter data - low memory, etc.
pub const RPC_E_SERVER_CANTUNMARSHAL_DATA: HRESULT = 0x8001000Eu32 as HRESULT;

/// Received data is invalid; could be server or client data.
pub const RPC_E_INVALID_DATA: HRESULT = 0x8001000Fu32 as HRESULT;

/// A particular parameter is invalid and cannot be (un)marshalled.
pub const RPC_E_INVALID_PARAMETER: HRESULT = 0x80010010u32 as HRESULT;

/// There is no second outgoing call on same channel in DDE conversation.
pub const RPC_E_CANTCALLOUT_AGAIN: HRESULT = 0x80010011u32 as HRESULT;

/// The callee (server [not server application]) is not available and disappeared; all connections are invalid. The call did not execute.
pub const RPC_E_SERVER_DIED_DNE: HRESULT = 0x80010012u32 as HRESULT;

/// System call failed.
pub const RPC_E_SYS_CALL_FAILED: HRESULT = 0x80010100u32 as HRESULT;

/// Could not allocate some required resource (memory, events, ...)
pub const RPC_E_OUT_OF_RESOURCES: HRESULT = 0x80010101u32 as HRESULT;

/// Attempted to make calls on more than one thread in single threaded mode.
pub const RPC_E_ATTEMPTED_MULTITHREAD: HRESULT = 0x80010102u32 as HRESULT;

/// The requested interface is not registered on the server object.
pub const RPC_E_NOT_REGISTERED: HRESULT = 0x80010103u32 as HRESULT;

/// RPC could not call the server or could not return the results of calling the server.
pub const RPC_E_FAULT: HRESULT = 0x80010104u32 as HRESULT;

/// The server threw an exception.
pub const RPC_E_SERVERFAULT: HRESULT = 0x80010105u32 as HRESULT;

/// Cannot change thread mode after it is set.
pub const RPC_E_CHANGED_MODE: HRESULT = 0x80010106u32 as HRESULT;

/// The method called does not exist on the server.
pub const RPC_E_INVALIDMETHOD: HRESULT = 0x80010107u32 as HRESULT;

/// The object invoked has disconnected from its clients.
pub const RPC_E_DISCONNECTED: HRESULT = 0x80010108u32 as HRESULT;

/// The object invoked chose not to process the call now. Try again later.
pub const RPC_E_RETRY: HRESULT = 0x80010109u32 as HRESULT;

/// The message filter indicated that the application is busy.
pub const RPC_E_SERVERCALL_RETRYLATER: HRESULT = 0x8001010Au32 as HRESULT;

/// The message filter rejected the call.
pub const RPC_E_SERVERCALL_REJECTED: HRESULT = 0x8001010Bu32 as HRESULT;

/// A call control interfaces was called with invalid data.
pub const RPC_E_INVALID_CALLDATA: HRESULT = 0x8001010Cu32 as HRESULT;

/// An outgoing call cannot be made since the application is dispatching an input-synchronous call.
pub const RPC_E_CANTCALLOUT_ININPUTSYNCCALL: HRESULT = 0x8001010Du32 as HRESULT;

/// The application called an interface that was marshalled for a different thread.
pub const RPC_E_WRONG_THREAD: HRESULT = 0x8001010Eu32 as HRESULT;

/// CoInitialize has not been called on the current thread.
pub const RPC_E_THREAD_NOT_INIT: HRESULT = 0x8001010Fu32 as HRESULT;

/// The version of OLE on the client and server machines does not match.
pub const RPC_E_VERSION_MISMATCH: HRESULT = 0x80010110u32 as HRESULT;

/// OLE received a packet with an invalid header.
pub const RPC_E_INVALID_HEADER: HRESULT = 0x80010111u32 as HRESULT;

/// OLE received a packet with an invalid extension.
pub const RPC_E_INVALID_EXTENSION: HRESULT = 0x80010112u32 as HRESULT;

/// The requested object or interface does not exist.
pub const RPC_E_INVALID_IPID: HRESULT = 0x80010113u32 as HRESULT;

/// The requested object does not exist.
pub const RPC_E_INVALID_OBJECT: HRESULT = 0x80010114u32 as HRESULT;

/// OLE has sent a request and is waiting for a reply.
pub const RPC_S_CALLPENDING: HRESULT = 0x80010115u32 as HRESULT;

/// OLE is waiting before retrying a request.
pub const RPC_S_WAITONTIMER: HRESULT = 0x80010116u32 as HRESULT;

/// Call context cannot be accessed after call completed.
pub const RPC_E_CALL_COMPLETE: HRESULT = 0x80010117u32 as HRESULT;

/// Impersonate on unsecure calls is not supported.
pub const RPC_E_UNSECURE_CALL: HRESULT = 0x80010118u32 as HRESULT;

/// Security must be initialized before any interfaces are marshalled or unmarshalled. It cannot be changed once initialized.
pub const RPC_E_TOO_LATE: HRESULT = 0x80010119u32 as HRESULT;

/// No security packages are installed on this machine or the user is not logged on or there are no compatible security packages between the client and server.
pub const RPC_E_NO_GOOD_SECURITY_PACKAGES: HRESULT = 0x8001011Au32 as HRESULT;

/// Access is denied.
pub const RPC_E_ACCESS_DENIED: HRESULT = 0x8001011Bu32 as HRESULT;

/// Remote calls are not allowed for this process.
pub const RPC_E_REMOTE_DISABLED: HRESULT = 0x8001011Cu32 as HRESULT;

/// The marshaled interface data packet (OBJREF) has an invalid or unknown format.
pub const RPC_E_INVALID_OBJREF: HRESULT = 0x8001011Du32 as HRESULT;

/// No context is associated with this call. This happens for some custom marshalled calls and on the client side of the call.
pub const RPC_E_NO_CONTEXT: HRESULT = 0x8001011Eu32 as HRESULT;

/// This operation returned because the timeout period expired.
pub const RPC_E_TIMEOUT: HRESULT = 0x8001011Fu32 as HRESULT;

/// There are no synchronize objects to wait on.
pub const RPC_E_NO_SYNC: HRESULT = 0x80010120u32 as HRESULT;

/// Full subject issuer chain SSL principal name expected from the server.
pub const RPC_E_FULLSIC_REQUIRED: HRESULT = 0x80010121u32 as HRESULT;

/// Principal name is not a valid MSSTD name.
pub const RPC_E_INVALID_STD_NAME: HRESULT = 0x80010122u32 as HRESULT;

/// Unable to impersonate DCOM client
pub const CO_E_FAILEDTOIMPERSONATE: HRESULT = 0x80010123u32 as HRESULT;

/// Unable to obtain server's security context
pub const CO_E_FAILEDTOGETSECCTX: HRESULT = 0x80010124u32 as HRESULT;

/// Unable to open the access token of the current thread
pub const CO_E_FAILEDTOOPENTHREADTOKEN: HRESULT = 0x80010125u32 as HRESULT;

/// Unable to obtain user info from an access token
pub const CO_E_FAILEDTOGETTOKENINFO: HRESULT = 0x80010126u32 as HRESULT;

/// The client who called IAccessControl::IsAccessPermitted was not the trustee provided to the method
pub const CO_E_TRUSTEEDOESNTMATCHCLIENT: HRESULT = 0x80010127u32 as HRESULT;

/// Unable to obtain the client's security blanket
pub const CO_E_FAILEDTOQUERYCLIENTBLANKET: HRESULT = 0x80010128u32 as HRESULT;

/// Unable to set a discretionary ACL into a security descriptor
pub const CO_E_FAILEDTOSETDACL: HRESULT = 0x80010129u32 as HRESULT;

/// The system function, AccessCheck, returned false
pub const CO_E_ACCESSCHECKFAILED: HRESULT = 0x8001012Au32 as HRESULT;

/// Either NetAccessDel or NetAccessAdd returned an error code.
pub const CO_E_NETACCESSAPIFAILED: HRESULT = 0x8001012Bu32 as HRESULT;

/// One of the trustee strings provided by the user did not conform to the `<Domain>\<Name>` syntax and it was not the "*" string
pub const CO_E_WRONGTRUSTEENAMESYNTAX: HRESULT = 0x8001012Cu32 as HRESULT;

/// One of the security identifiers provided by the user was invalid
pub const CO_E_INVALIDSID: HRESULT = 0x8001012Du32 as HRESULT;

/// Unable to convert a wide character trustee string to a multibyte trustee string
pub const CO_E_CONVERSIONFAILED: HRESULT = 0x8001012Eu32 as HRESULT;

/// Unable to find a security identifier that corresponds to a trustee string provided by the user
pub const CO_E_NOMATCHINGSIDFOUND: HRESULT = 0x8001012Fu32 as HRESULT;

/// The system function, LookupAccountSID, failed
pub const CO_E_LOOKUPACCSIDFAILED: HRESULT = 0x80010130u32 as HRESULT;

/// Unable to find a trustee name that corresponds to a security identifier provided by the user
pub const CO_E_NOMATCHINGNAMEFOUND: HRESULT = 0x80010131u32 as HRESULT;

/// The system function, LookupAccountName, failed
pub const CO_E_LOOKUPACCNAMEFAILED: HRESULT = 0x80010132u32 as HRESULT;

/// Unable to set or reset a serialization handle
pub const CO_E_SETSERLHNDLFAILED: HRESULT = 0x80010133u32 as HRESULT;

/// Unable to obtain the Windows directory
pub const CO_E_FAILEDTOGETWINDIR: HRESULT = 0x80010134u32 as HRESULT;

/// Path too long
pub const CO_E_PATHTOOLONG: HRESULT = 0x80010135u32 as HRESULT;

/// Unable to generate a uuid.
pub const CO_E_FAILEDTOGENUUID: HRESULT = 0x80010136u32 as HRESULT;

/// Unable to create file
pub const CO_E_FAILEDTOCREATEFILE: HRESULT = 0x80010137u32 as HRESULT;

/// Unable to close a serialization handle or a file handle.
pub const CO_E_FAILEDTOCLOSEHANDLE: HRESULT = 0x80010138u32 as HRESULT;

/// The number of ACEs in an ACL exceeds the system limit.
pub const CO_E_EXCEEDSYSACLLIMIT: HRESULT = 0x80010139u32 as HRESULT;

/// Not all the DENY_ACCESS ACEs are arranged in front of the GRANT_ACCESS ACEs in the stream.
pub const CO_E_ACESINWRONGORDER: HRESULT = 0x8001013Au32 as HRESULT;

/// The version of ACL format in the stream is not supported by this implementation of IAccessControl
pub const CO_E_INCOMPATIBLESTREAMVERSION: HRESULT = 0x8001013Bu32 as HRESULT;

/// Unable to open the access token of the server process
pub const CO_E_FAILEDTOOPENPROCESSTOKEN: HRESULT = 0x8001013Cu32 as HRESULT;

/// Unable to decode the ACL in the stream provided by the user
pub const CO_E_DECODEFAILED: HRESULT = 0x8001013Du32 as HRESULT;

/// The COM IAccessControl object is not initialized
pub const CO_E_ACNOTINITIALIZED: HRESULT = 0x8001013Fu32 as HRESULT;

/// Call Cancellation is disabled
pub const CO_E_CANCEL_DISABLED: HRESULT = 0x80010140u32 as HRESULT;

/// An internal error occurred.
pub const RPC_E_UNEXPECTED: HRESULT = 0x8001FFFFu32 as HRESULT;

/// The specified event is currently not being audited.
pub const ERROR_AUDITING_DISABLED: HRESULT = 0xC0090001u32 as HRESULT;

/// The SID filtering operation removed all SIDs.
pub const ERROR_ALL_SIDS_FILTERED: HRESULT = 0xC0090002u32 as HRESULT;

/// Business rule scripts are disabled for the calling application.
pub const ERROR_BIZRULES_NOT_ENABLED: HRESULT = 0xC0090003u32 as HRESULT;

/// Bad UID.
pub const NTE_BAD_UID: HRESULT = 0x80090001u32 as HRESULT;

/// Bad Hash.
pub const NTE_BAD_HASH: HRESULT = 0x80090002u32 as HRESULT;

/// Bad Key.
pub const NTE_BAD_KEY: HRESULT = 0x80090003u32 as HRESULT;

/// Bad Length.
pub const NTE_BAD_LEN: HRESULT = 0x80090004u32 as HRESULT;

/// Bad Data.
pub const NTE_BAD_DATA: HRESULT = 0x80090005u32 as HRESULT;

/// Invalid Signature.
pub const NTE_BAD_SIGNATURE: HRESULT = 0x80090006u32 as HRESULT;

/// Bad Version of provider.
pub const NTE_BAD_VER: HRESULT = 0x80090007u32 as HRESULT;

/// Invalid algorithm specified.
pub const NTE_BAD_ALGID: HRESULT = 0x80090008u32 as HRESULT;

/// Invalid flags specified.
pub const NTE_BAD_FLAGS: HRESULT = 0x80090009u32 as HRESULT;

/// Invalid type specified.
pub const NTE_BAD_TYPE: HRESULT = 0x8009000Au32 as HRESULT;

/// Key not valid for use in specified state.
pub const NTE_BAD_KEY_STATE: HRESULT = 0x8009000Bu32 as HRESULT;

/// Hash not valid for use in specified state.
pub const NTE_BAD_HASH_STATE: HRESULT = 0x8009000Cu32 as HRESULT;

/// Key does not exist.
pub const NTE_NO_KEY: HRESULT = 0x8009000Du32 as HRESULT;

/// Insufficient memory available for the operation.
pub const NTE_NO_MEMORY: HRESULT = 0x8009000Eu32 as HRESULT;

/// Object already exists.
pub const NTE_EXISTS: HRESULT = 0x8009000Fu32 as HRESULT;

/// Access denied.
pub const NTE_PERM: HRESULT = 0x80090010u32 as HRESULT;

/// Object was not found.
pub const NTE_NOT_FOUND: HRESULT = 0x80090011u32 as HRESULT;

/// Data already encrypted.
pub const NTE_DOUBLE_ENCRYPT: HRESULT = 0x80090012u32 as HRESULT;

/// Invalid provider specified.
pub const NTE_BAD_PROVIDER: HRESULT = 0x80090013u32 as HRESULT;

/// Invalid provider type specified.
pub const NTE_BAD_PROV_TYPE: HRESULT = 0x80090014u32 as HRESULT;

/// Provider's public key is invalid.
pub const NTE_BAD_PUBLIC_KEY: HRESULT = 0x80090015u32 as HRESULT;

/// Keyset does not exist
pub const NTE_BAD_KEYSET: HRESULT = 0x80090016u32 as HRESULT;

/// Provider type not defined.
pub const NTE_PROV_TYPE_NOT_DEF: HRESULT = 0x80090017u32 as HRESULT;

/// Provider type as registered is invalid.
pub const NTE_PROV_TYPE_ENTRY_BAD: HRESULT = 0x80090018u32 as HRESULT;

/// The keyset is not defined.
pub const NTE_KEYSET_NOT_DEF: HRESULT = 0x80090019u32 as HRESULT;

/// Keyset as registered is invalid.
pub const NTE_KEYSET_ENTRY_BAD: HRESULT = 0x8009001Au32 as HRESULT;

/// Provider type does not match registered value.
pub const NTE_PROV_TYPE_NO_MATCH: HRESULT = 0x8009001Bu32 as HRESULT;

/// The digital signature file is corrupt.
pub const NTE_SIGNATURE_FILE_BAD: HRESULT = 0x8009001Cu32 as HRESULT;

/// Provider DLL failed to initialize correctly.
pub const NTE_PROVIDER_DLL_FAIL: HRESULT = 0x8009001Du32 as HRESULT;

/// Provider DLL could not be found.
pub const NTE_PROV_DLL_NOT_FOUND: HRESULT = 0x8009001Eu32 as HRESULT;

/// The Keyset parameter is invalid.
pub const NTE_BAD_KEYSET_PARAM: HRESULT = 0x8009001Fu32 as HRESULT;

/// An internal error occurred.
pub const NTE_FAIL: HRESULT = 0x80090020u32 as HRESULT;

/// A base error occurred.
pub const NTE_SYS_ERR: HRESULT = 0x80090021u32 as HRESULT;

/// Provider could not perform the action since the context was acquired as silent.
pub const NTE_SILENT_CONTEXT: HRESULT = 0x80090022u32 as HRESULT;

/// The security token does not have storage space available for an additional container.
pub const NTE_TOKEN_KEYSET_STORAGE_FULL: HRESULT = 0x80090023u32 as HRESULT;

/// The profile for the user is a temporary profile.
pub const NTE_TEMPORARY_PROFILE: HRESULT = 0x80090024u32 as HRESULT;

/// The key parameters could not be set because the CSP uses fixed parameters.
pub const NTE_FIXEDPARAMETER: HRESULT = 0x80090025u32 as HRESULT;

/// The supplied handle is invalid.
pub const NTE_INVALID_HANDLE: HRESULT = 0x80090026u32 as HRESULT;

/// The parameter is incorrect.
pub const NTE_INVALID_PARAMETER: HRESULT = 0x80090027u32 as HRESULT;

/// The buffer supplied to a function was too small.
pub const NTE_BUFFER_TOO_SMALL: HRESULT = 0x80090028u32 as HRESULT;

/// The requested operation is not supported.
pub const NTE_NOT_SUPPORTED: HRESULT = 0x80090029u32 as HRESULT;

/// No more data is available.
pub const NTE_NO_MORE_ITEMS: HRESULT = 0x8009002Au32 as HRESULT;

/// The supplied buffers overlap incorrectly.
pub const NTE_BUFFERS_OVERLAP: HRESULT = 0x8009002Bu32 as HRESULT;

/// The specified data could not be decrypted.
pub const NTE_DECRYPTION_FAILURE: HRESULT = 0x8009002Cu32 as HRESULT;

/// An internal consistency check failed.
pub const NTE_INTERNAL_ERROR: HRESULT = 0x8009002Du32 as HRESULT;

/// This operation requires input from the user.
pub const NTE_UI_REQUIRED: HRESULT = 0x8009002Eu32 as HRESULT;

/// The cryptographic provider does not support HMAC.
pub const NTE_HMAC_NOT_SUPPORTED: HRESULT = 0x8009002Fu32 as HRESULT;

/// The device that is required by this cryptographic provider is not ready for use.
pub const NTE_DEVICE_NOT_READY: HRESULT = 0x80090030u32 as HRESULT;

/// The dictionary attack mitigation is triggered and the provided authorization was ignored by the provider.
pub const NTE_AUTHENTICATION_IGNORED: HRESULT = 0x80090031u32 as HRESULT;

/// The validation of the provided data failed the integrity or signature validation.
pub const NTE_VALIDATION_FAILED: HRESULT = 0x80090032u32 as HRESULT;

/// Incorrect password.
pub const NTE_INCORRECT_PASSWORD: HRESULT = 0x80090033u32 as HRESULT;

/// Encryption failed.
pub const NTE_ENCRYPTION_FAILURE: HRESULT = 0x80090034u32 as HRESULT;

/// The device that is required by this cryptographic provider is not found on this platform.
pub const NTE_DEVICE_NOT_FOUND: HRESULT = 0x80090035u32 as HRESULT;

/// The action was cancelled by the user.
pub const NTE_USER_CANCELLED: HRESULT = 0x80090036u32 as HRESULT;

/// The password is no longer valid and must be changed.
pub const NTE_PASSWORD_CHANGE_REQUIRED: HRESULT = 0x80090037u32 as HRESULT;

/// The operation cannot be completed from Terminal Server client sessions.
pub const NTE_NOT_ACTIVE_CONSOLE: HRESULT = 0x80090038u32 as HRESULT;

/// Not enough memory is available to complete this request
pub const SEC_E_INSUFFICIENT_MEMORY: HRESULT = 0x80090300u32 as HRESULT;

/// The handle specified is invalid
pub const SEC_E_INVALID_HANDLE: HRESULT = 0x80090301u32 as HRESULT;

/// The function requested is not supported
pub const SEC_E_UNSUPPORTED_FUNCTION: HRESULT = 0x80090302u32 as HRESULT;

/// The specified target is unknown or unreachable
pub const SEC_E_TARGET_UNKNOWN: HRESULT = 0x80090303u32 as HRESULT;

/// The Local Security Authority cannot be contacted
pub const SEC_E_INTERNAL_ERROR: HRESULT = 0x80090304u32 as HRESULT;

/// The requested security package does not exist
pub const SEC_E_SECPKG_NOT_FOUND: HRESULT = 0x80090305u32 as HRESULT;

/// The caller is not the owner of the desired credentials
pub const SEC_E_NOT_OWNER: HRESULT = 0x80090306u32 as HRESULT;

/// The security package failed to initialize, and cannot be installed
pub const SEC_E_CANNOT_INSTALL: HRESULT = 0x80090307u32 as HRESULT;

/// The token supplied to the function is invalid
pub const SEC_E_INVALID_TOKEN: HRESULT = 0x80090308u32 as HRESULT;

/// The security package is not able to marshall the logon buffer, so the logon attempt has failed
pub const SEC_E_CANNOT_PACK: HRESULT = 0x80090309u32 as HRESULT;

/// The per-message Quality of Protection is not supported by the security package
pub const SEC_E_QOP_NOT_SUPPORTED: HRESULT = 0x8009030Au32 as HRESULT;

/// The security context does not allow impersonation of the client
pub const SEC_E_NO_IMPERSONATION: HRESULT = 0x8009030Bu32 as HRESULT;

/// The logon attempt failed
pub const SEC_E_LOGON_DENIED: HRESULT = 0x8009030Cu32 as HRESULT;

/// The credentials supplied to the package were not recognized
pub const SEC_E_UNKNOWN_CREDENTIALS: HRESULT = 0x8009030Du32 as HRESULT;

/// No credentials are available in the security package
pub const SEC_E_NO_CREDENTIALS: HRESULT = 0x8009030Eu32 as HRESULT;

/// The message or signature supplied for verification has been altered
pub const SEC_E_MESSAGE_ALTERED: HRESULT = 0x8009030Fu32 as HRESULT;

/// The message supplied for verification is out of sequence
pub const SEC_E_OUT_OF_SEQUENCE: HRESULT = 0x80090310u32 as HRESULT;

/// No authority could be contacted for authentication.
pub const SEC_E_NO_AUTHENTICATING_AUTHORITY: HRESULT = 0x80090311u32 as HRESULT;

/// The function completed successfully, but must be called again to complete the context
pub const SEC_I_CONTINUE_NEEDED: HRESULT = 0x00090312;

/// The function completed successfully, but CompleteToken must be called
pub const SEC_I_COMPLETE_NEEDED: HRESULT = 0x00090313;

/// The function completed successfully, but both CompleteToken and this function must be called to complete the context
pub const SEC_I_COMPLETE_AND_CONTINUE: HRESULT = 0x00090314;

/// The logon was completed, but no network authority was available. The logon was made using locally known information
pub const SEC_I_LOCAL_LOGON: HRESULT = 0x00090315;

/// Schannel has received a TLS extension the SSPI caller subscribed to.
pub const SEC_I_GENERIC_EXTENSION_RECEIVED: HRESULT = 0x00090316;

/// The requested security package does not exist
pub const SEC_E_BAD_PKGID: HRESULT = 0x80090316u32 as HRESULT;

/// The context has expired and can no longer be used.
pub const SEC_E_CONTEXT_EXPIRED: HRESULT = 0x80090317u32 as HRESULT;

/// The context has expired and can no longer be used.
pub const SEC_I_CONTEXT_EXPIRED: HRESULT = 0x00090317;

/// The supplied message is incomplete. The signature was not verified.
pub const SEC_E_INCOMPLETE_MESSAGE: HRESULT = 0x80090318u32 as HRESULT;

/// The credentials supplied were not complete, and could not be verified. The context could not be initialized.
pub const SEC_E_INCOMPLETE_CREDENTIALS: HRESULT = 0x80090320u32 as HRESULT;

/// The buffers supplied to a function was too small.
pub const SEC_E_BUFFER_TOO_SMALL: HRESULT = 0x80090321u32 as HRESULT;

/// The credentials supplied were not complete, and could not be verified. Additional information can be returned from the context.
pub const SEC_I_INCOMPLETE_CREDENTIALS: HRESULT = 0x00090320;

/// The context data must be renegotiated with the peer.
pub const SEC_I_RENEGOTIATE: HRESULT = 0x00090321;

/// The target principal name is incorrect.
pub const SEC_E_WRONG_PRINCIPAL: HRESULT = 0x80090322u32 as HRESULT;

/// There is no LSA mode context associated with this context.
pub const SEC_I_NO_LSA_CONTEXT: HRESULT = 0x00090323;

/// The clocks on the client and server machines are skewed.
pub const SEC_E_TIME_SKEW: HRESULT = 0x80090324u32 as HRESULT;

/// The certificate chain was issued by an authority that is not trusted.
pub const SEC_E_UNTRUSTED_ROOT: HRESULT = 0x80090325u32 as HRESULT;

/// The message received was unexpected or badly formatted.
pub const SEC_E_ILLEGAL_MESSAGE: HRESULT = 0x80090326u32 as HRESULT;

/// An unknown error occurred while processing the certificate.
pub const SEC_E_CERT_UNKNOWN: HRESULT = 0x80090327u32 as HRESULT;

/// The received certificate has expired.
pub const SEC_E_CERT_EXPIRED: HRESULT = 0x80090328u32 as HRESULT;

/// The specified data could not be encrypted.
pub const SEC_E_ENCRYPT_FAILURE: HRESULT = 0x80090329u32 as HRESULT;

///  The specified data could not be decrypted.
///
///
pub const SEC_E_DECRYPT_FAILURE: HRESULT = 0x80090330u32 as HRESULT;

/// The client and server cannot communicate, because they do not possess a common algorithm.
pub const SEC_E_ALGORITHM_MISMATCH: HRESULT = 0x80090331u32 as HRESULT;

/// The security context could not be established due to a failure in the requested quality of service (e.g. mutual authentication or delegation).
pub const SEC_E_SECURITY_QOS_FAILED: HRESULT = 0x80090332u32 as HRESULT;

/// A security context was deleted before the context was completed. This is considered a logon failure.
pub const SEC_E_UNFINISHED_CONTEXT_DELETED: HRESULT = 0x80090333u32 as HRESULT;

/// The client is trying to negotiate a context and the server requires user-to-user but didn't send a TGT reply.
pub const SEC_E_NO_TGT_REPLY: HRESULT = 0x80090334u32 as HRESULT;

/// Unable to accomplish the requested task because the local machine does not have any IP addresses.
pub const SEC_E_NO_IP_ADDRESSES: HRESULT = 0x80090335u32 as HRESULT;

/// The supplied credential handle does not match the credential associated with the security context.
pub const SEC_E_WRONG_CREDENTIAL_HANDLE: HRESULT = 0x80090336u32 as HRESULT;

/// The crypto system or checksum function is invalid because a required function is unavailable.
pub const SEC_E_CRYPTO_SYSTEM_INVALID: HRESULT = 0x80090337u32 as HRESULT;

/// The number of maximum ticket referrals has been exceeded.
pub const SEC_E_MAX_REFERRALS_EXCEEDED: HRESULT = 0x80090338u32 as HRESULT;

/// The local machine must be a Kerberos KDC (domain controller) and it is not.
pub const SEC_E_MUST_BE_KDC: HRESULT = 0x80090339u32 as HRESULT;

/// The other end of the security negotiation is requires strong crypto but it is not supported on the local machine.
pub const SEC_E_STRONG_CRYPTO_NOT_SUPPORTED: HRESULT = 0x8009033Au32 as HRESULT;

/// The KDC reply contained more than one principal name.
pub const SEC_E_TOO_MANY_PRINCIPALS: HRESULT = 0x8009033Bu32 as HRESULT;

/// Expected to find PA data for a hint of what etype to use, but it was not found.
pub const SEC_E_NO_PA_DATA: HRESULT = 0x8009033Cu32 as HRESULT;

/// The client certificate does not contain a valid UPN, or does not match the client name in the logon request. Please contact your administrator.
pub const SEC_E_PKINIT_NAME_MISMATCH: HRESULT = 0x8009033Du32 as HRESULT;

/// Smartcard logon is required and was not used.
pub const SEC_E_SMARTCARD_LOGON_REQUIRED: HRESULT = 0x8009033Eu32 as HRESULT;

/// A system shutdown is in progress.
pub const SEC_E_SHUTDOWN_IN_PROGRESS: HRESULT = 0x8009033Fu32 as HRESULT;

/// An invalid request was sent to the KDC.
pub const SEC_E_KDC_INVALID_REQUEST: HRESULT = 0x80090340u32 as HRESULT;

/// The KDC was unable to generate a referral for the service requested.
pub const SEC_E_KDC_UNABLE_TO_REFER: HRESULT = 0x80090341u32 as HRESULT;

/// The encryption type requested is not supported by the KDC.
pub const SEC_E_KDC_UNKNOWN_ETYPE: HRESULT = 0x80090342u32 as HRESULT;

/// An unsupported preauthentication mechanism was presented to the Kerberos package.
pub const SEC_E_UNSUPPORTED_PREAUTH: HRESULT = 0x80090343u32 as HRESULT;

/// The requested operation cannot be completed. The computer must be trusted for delegation and the current user account must be configured to allow delegation.
pub const SEC_E_DELEGATION_REQUIRED: HRESULT = 0x80090345u32 as HRESULT;

/// Client's supplied SSPI channel bindings were incorrect.
pub const SEC_E_BAD_BINDINGS: HRESULT = 0x80090346u32 as HRESULT;

/// The received certificate was mapped to multiple accounts.
pub const SEC_E_MULTIPLE_ACCOUNTS: HRESULT = 0x80090347u32 as HRESULT;

///  SEC_E_NO_KERB_KEY
pub const SEC_E_NO_KERB_KEY: HRESULT = 0x80090348u32 as HRESULT;

/// The certificate is not valid for the requested usage.
pub const SEC_E_CERT_WRONG_USAGE: HRESULT = 0x80090349u32 as HRESULT;

/// The system cannot contact a domain controller to service the authentication request. Please try again later.
pub const SEC_E_DOWNGRADE_DETECTED: HRESULT = 0x80090350u32 as HRESULT;

/// The smartcard certificate used for authentication has been revoked. Please contact your system administrator. There may be additional information in the event log.
pub const SEC_E_SMARTCARD_CERT_REVOKED: HRESULT = 0x80090351u32 as HRESULT;

/// An untrusted certificate authority was detected while processing the smartcard certificate used for authentication. Please contact your system administrator.
pub const SEC_E_ISSUING_CA_UNTRUSTED: HRESULT = 0x80090352u32 as HRESULT;

/// The revocation status of the smartcard certificate used for authentication could not be determined. Please contact your system administrator.
pub const SEC_E_REVOCATION_OFFLINE_C: HRESULT = 0x80090353u32 as HRESULT;

/// The smartcard certificate used for authentication was not trusted. Please contact your system administrator.
pub const SEC_E_PKINIT_CLIENT_FAILURE: HRESULT = 0x80090354u32 as HRESULT;

/// The smartcard certificate used for authentication has expired. Please contact your system administrator.
pub const SEC_E_SMARTCARD_CERT_EXPIRED: HRESULT = 0x80090355u32 as HRESULT;

/// The Kerberos subsystem encountered an error. A service for user protocol request was made against a domain controller which does not support service for user.
pub const SEC_E_NO_S4U_PROT_SUPPORT: HRESULT = 0x80090356u32 as HRESULT;

/// An attempt was made by this server to make a Kerberos constrained delegation request for a target outside of the server's realm. This is not supported, and indicates a misconfiguration on this server's allowed to delegate to list. Please contact your administrator.
pub const SEC_E_CROSSREALM_DELEGATION_FAILURE: HRESULT = 0x80090357u32 as HRESULT;

/// The revocation status of the domain controller certificate used for smartcard authentication could not be determined. There is additional information in the system event log. Please contact your system administrator.
pub const SEC_E_REVOCATION_OFFLINE_KDC: HRESULT = 0x80090358u32 as HRESULT;

/// An untrusted certificate authority was detected while processing the domain controller certificate used for authentication. There is additional information in the system event log. Please contact your system administrator.
pub const SEC_E_ISSUING_CA_UNTRUSTED_KDC: HRESULT = 0x80090359u32 as HRESULT;

/// The domain controller certificate used for smartcard logon has expired. Please contact your system administrator with the contents of your system event log.
pub const SEC_E_KDC_CERT_EXPIRED: HRESULT = 0x8009035Au32 as HRESULT;

/// The domain controller certificate used for smartcard logon has been revoked. Please contact your system administrator with the contents of your system event log.
pub const SEC_E_KDC_CERT_REVOKED: HRESULT = 0x8009035Bu32 as HRESULT;

/// A signature operation must be performed before the user can authenticate.
pub const SEC_I_SIGNATURE_NEEDED: HRESULT = 0x0009035C;

/// One or more of the parameters passed to the function was invalid.
pub const SEC_E_INVALID_PARAMETER: HRESULT = 0x8009035Du32 as HRESULT;

/// Client policy does not allow credential delegation to target server.
pub const SEC_E_DELEGATION_POLICY: HRESULT = 0x8009035Eu32 as HRESULT;

/// Client policy does not allow credential delegation to target server with NLTM only authentication.
pub const SEC_E_POLICY_NLTM_ONLY: HRESULT = 0x8009035Fu32 as HRESULT;

/// The recipient rejected the renegotiation request.
pub const SEC_I_NO_RENEGOTIATION: HRESULT = 0x00090360;

/// The required security context does not exist.
pub const SEC_E_NO_CONTEXT: HRESULT = 0x80090361u32 as HRESULT;

/// The PKU2U protocol encountered an error while attempting to utilize the associated certificates.
pub const SEC_E_PKU2U_CERT_FAILURE: HRESULT = 0x80090362u32 as HRESULT;

/// The identity of the server computer could not be verified.
pub const SEC_E_MUTUAL_AUTH_FAILED: HRESULT = 0x80090363u32 as HRESULT;

/// The returned buffer is only a fragment of the message.  More fragments need to be returned.
pub const SEC_I_MESSAGE_FRAGMENT: HRESULT = 0x00090364;

/// Only https scheme is allowed.
pub const SEC_E_ONLY_HTTPS_ALLOWED: HRESULT = 0x80090365u32 as HRESULT;

/// The function completed successfully, but must be called again to complete the context.  Early start can be used.
pub const SEC_I_CONTINUE_NEEDED_MESSAGE_OK: HRESULT = 0x00090366;

/// No common application protocol exists between the client and the server. Application protocol negotiation failed.
pub const SEC_E_APPLICATION_PROTOCOL_MISMATCH: HRESULT = 0x80090367u32 as HRESULT;

/// An asynchronous SSPI routine has been called and the work is pending completion.
pub const SEC_I_ASYNC_CALL_PENDING: HRESULT = 0x00090368;

/// You can't sign in with a user ID in this format. Try using your email address instead.
pub const SEC_E_INVALID_UPN_NAME: HRESULT = 0x80090369u32 as HRESULT;

/// The buffer supplied by the SSPI caller to receive generic extensions is too small.
pub const SEC_E_EXT_BUFFER_TOO_SMALL: HRESULT = 0x8009036Au32 as HRESULT;

/// Not enough secbuffers were supplied to generate a token.
pub const SEC_E_INSUFFICIENT_BUFFERS: HRESULT = 0x8009036Bu32 as HRESULT;

/// The Local Security Authority cannot be contacted
pub const SEC_E_NO_SPM: HRESULT = SEC_E_INTERNAL_ERROR;

/// The function requested is not supported
pub const SEC_E_NOT_SUPPORTED: HRESULT = SEC_E_UNSUPPORTED_FUNCTION;

/// An error occurred while performing an operation on a cryptographic message.
pub const CRYPT_E_MSG_ERROR: HRESULT = 0x80091001u32 as HRESULT;

/// Unknown cryptographic algorithm.
pub const CRYPT_E_UNKNOWN_ALGO: HRESULT = 0x80091002u32 as HRESULT;

/// The object identifier is poorly formatted.
pub const CRYPT_E_OID_FORMAT: HRESULT = 0x80091003u32 as HRESULT;

/// Invalid cryptographic message type.
pub const CRYPT_E_INVALID_MSG_TYPE: HRESULT = 0x80091004u32 as HRESULT;

/// Unexpected cryptographic message encoding.
pub const CRYPT_E_UNEXPECTED_ENCODING: HRESULT = 0x80091005u32 as HRESULT;

/// The cryptographic message does not contain an expected authenticated attribute.
pub const CRYPT_E_AUTH_ATTR_MISSING: HRESULT = 0x80091006u32 as HRESULT;

/// The hash value is not correct.
pub const CRYPT_E_HASH_VALUE: HRESULT = 0x80091007u32 as HRESULT;

/// The index value is not valid.
pub const CRYPT_E_INVALID_INDEX: HRESULT = 0x80091008u32 as HRESULT;

/// The content of the cryptographic message has already been decrypted.
pub const CRYPT_E_ALREADY_DECRYPTED: HRESULT = 0x80091009u32 as HRESULT;

/// The content of the cryptographic message has not been decrypted yet.
pub const CRYPT_E_NOT_DECRYPTED: HRESULT = 0x8009100Au32 as HRESULT;

/// The enveloped-data message does not contain the specified recipient.
pub const CRYPT_E_RECIPIENT_NOT_FOUND: HRESULT = 0x8009100Bu32 as HRESULT;

/// Invalid control type.
pub const CRYPT_E_CONTROL_TYPE: HRESULT = 0x8009100Cu32 as HRESULT;

/// Invalid issuer and/or serial number.
pub const CRYPT_E_ISSUER_SERIALNUMBER: HRESULT = 0x8009100Du32 as HRESULT;

/// Cannot find the original signer.
pub const CRYPT_E_SIGNER_NOT_FOUND: HRESULT = 0x8009100Eu32 as HRESULT;

/// The cryptographic message does not contain all of the requested attributes.
pub const CRYPT_E_ATTRIBUTES_MISSING: HRESULT = 0x8009100Fu32 as HRESULT;

/// The streamed cryptographic message is not ready to return data.
pub const CRYPT_E_STREAM_MSG_NOT_READY: HRESULT = 0x80091010u32 as HRESULT;

/// The streamed cryptographic message requires more data to complete the decode operation.
pub const CRYPT_E_STREAM_INSUFFICIENT_DATA: HRESULT = 0x80091011u32 as HRESULT;

/// The protected data needs to be re-protected.
pub const CRYPT_I_NEW_PROTECTION_REQUIRED: HRESULT = 0x00091012;

/// The length specified for the output data was insufficient.
pub const CRYPT_E_BAD_LEN: HRESULT = 0x80092001u32 as HRESULT;

/// An error occurred during encode or decode operation.
pub const CRYPT_E_BAD_ENCODE: HRESULT = 0x80092002u32 as HRESULT;

/// An error occurred while reading or writing to a file.
pub const CRYPT_E_FILE_ERROR: HRESULT = 0x80092003u32 as HRESULT;

/// Cannot find object or property.
pub const CRYPT_E_NOT_FOUND: HRESULT = 0x80092004u32 as HRESULT;

/// The object or property already exists.
pub const CRYPT_E_EXISTS: HRESULT = 0x80092005u32 as HRESULT;

/// No provider was specified for the store or object.
pub const CRYPT_E_NO_PROVIDER: HRESULT = 0x80092006u32 as HRESULT;

/// The specified certificate is self signed.
pub const CRYPT_E_SELF_SIGNED: HRESULT = 0x80092007u32 as HRESULT;

/// The previous certificate or CRL context was deleted.
pub const CRYPT_E_DELETED_PREV: HRESULT = 0x80092008u32 as HRESULT;

/// Cannot find the requested object.
pub const CRYPT_E_NO_MATCH: HRESULT = 0x80092009u32 as HRESULT;

/// The certificate does not have a property that references a private key.
pub const CRYPT_E_UNEXPECTED_MSG_TYPE: HRESULT = 0x8009200Au32 as HRESULT;

/// Cannot find the certificate and private key for decryption.
pub const CRYPT_E_NO_KEY_PROPERTY: HRESULT = 0x8009200Bu32 as HRESULT;

/// Cannot find the certificate and private key to use for decryption.
pub const CRYPT_E_NO_DECRYPT_CERT: HRESULT = 0x8009200Cu32 as HRESULT;

/// Not a cryptographic message or the cryptographic message is not formatted correctly.
pub const CRYPT_E_BAD_MSG: HRESULT = 0x8009200Du32 as HRESULT;

/// The signed cryptographic message does not have a signer for the specified signer index.
pub const CRYPT_E_NO_SIGNER: HRESULT = 0x8009200Eu32 as HRESULT;

/// Final closure is pending until additional frees or closes.
pub const CRYPT_E_PENDING_CLOSE: HRESULT = 0x8009200Fu32 as HRESULT;

/// The certificate is revoked.
pub const CRYPT_E_REVOKED: HRESULT = 0x80092010u32 as HRESULT;

/// No Dll or exported function was found to verify revocation.
pub const CRYPT_E_NO_REVOCATION_DLL: HRESULT = 0x80092011u32 as HRESULT;

/// The revocation function was unable to check revocation for the certificate.
pub const CRYPT_E_NO_REVOCATION_CHECK: HRESULT = 0x80092012u32 as HRESULT;

/// The revocation function was unable to check revocation because the revocation server was offline.
pub const CRYPT_E_REVOCATION_OFFLINE: HRESULT = 0x80092013u32 as HRESULT;

/// The certificate is not in the revocation server's database.
pub const CRYPT_E_NOT_IN_REVOCATION_DATABASE: HRESULT = 0x80092014u32 as HRESULT;

/// The string contains a non-numeric character.
pub const CRYPT_E_INVALID_NUMERIC_STRING: HRESULT = 0x80092020u32 as HRESULT;

/// The string contains a non-printable character.
pub const CRYPT_E_INVALID_PRINTABLE_STRING: HRESULT = 0x80092021u32 as HRESULT;

/// The string contains a character not in the 7 bit ASCII character set.
pub const CRYPT_E_INVALID_IA5_STRING: HRESULT = 0x80092022u32 as HRESULT;

/// The string contains an invalid X500 name attribute key, oid, value or delimiter.
pub const CRYPT_E_INVALID_X500_STRING: HRESULT = 0x80092023u32 as HRESULT;

/// The dwValueType for the CERT_NAME_VALUE is not one of the character strings. Most likely it is either a CERT_RDN_ENCODED_BLOB or CERT_RDN_OCTET_STRING.
pub const CRYPT_E_NOT_CHAR_STRING: HRESULT = 0x80092024u32 as HRESULT;

/// The Put operation cannot continue. The file needs to be resized. However, there is already a signature present. A complete signing operation must be done.
pub const CRYPT_E_FILERESIZED: HRESULT = 0x80092025u32 as HRESULT;

/// The cryptographic operation failed due to a local security option setting.
pub const CRYPT_E_SECURITY_SETTINGS: HRESULT = 0x80092026u32 as HRESULT;

/// No DLL or exported function was found to verify subject usage.
pub const CRYPT_E_NO_VERIFY_USAGE_DLL: HRESULT = 0x80092027u32 as HRESULT;

/// The called function was unable to do a usage check on the subject.
pub const CRYPT_E_NO_VERIFY_USAGE_CHECK: HRESULT = 0x80092028u32 as HRESULT;

/// Since the server was offline, the called function was unable to complete the usage check.
pub const CRYPT_E_VERIFY_USAGE_OFFLINE: HRESULT = 0x80092029u32 as HRESULT;

/// The subject was not found in a Certificate Trust List (CTL).
pub const CRYPT_E_NOT_IN_CTL: HRESULT = 0x8009202Au32 as HRESULT;

/// None of the signers of the cryptographic message or certificate trust list is trusted.
pub const CRYPT_E_NO_TRUSTED_SIGNER: HRESULT = 0x8009202Bu32 as HRESULT;

/// The public key's algorithm parameters are missing.
pub const CRYPT_E_MISSING_PUBKEY_PARA: HRESULT = 0x8009202Cu32 as HRESULT;

/// An object could not be located using the object locator infrastructure with the given name.
pub const CRYPT_E_OBJECT_LOCATOR_OBJECT_NOT_FOUND: HRESULT = 0x8009202Du32 as HRESULT;

///  OSS Certificate encode/decode error code base
///
///
/// See asn1code.h for a definition of the OSS runtime errors. The OSS error values are offset by CRYPT_E_OSS_ERROR.
pub const CRYPT_E_OSS_ERROR: HRESULT = 0x80093000u32 as HRESULT;

/// OSS ASN.1 Error: Output Buffer is too small.
pub const OSS_MORE_BUF: HRESULT = 0x80093001u32 as HRESULT;

/// OSS ASN.1 Error: Signed integer is encoded as a unsigned integer.
pub const OSS_NEGATIVE_UINTEGER: HRESULT = 0x80093002u32 as HRESULT;

/// OSS ASN.1 Error: Unknown ASN.1 data type.
pub const OSS_PDU_RANGE: HRESULT = 0x80093003u32 as HRESULT;

/// OSS ASN.1 Error: Output buffer is too small, the decoded data has been truncated.
pub const OSS_MORE_INPUT: HRESULT = 0x80093004u32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_DATA_ERROR: HRESULT = 0x80093005u32 as HRESULT;

/// OSS ASN.1 Error: Invalid argument.
pub const OSS_BAD_ARG: HRESULT = 0x80093006u32 as HRESULT;

/// OSS ASN.1 Error: Encode/Decode version mismatch.
pub const OSS_BAD_VERSION: HRESULT = 0x80093007u32 as HRESULT;

/// OSS ASN.1 Error: Out of memory.
pub const OSS_OUT_MEMORY: HRESULT = 0x80093008u32 as HRESULT;

/// OSS ASN.1 Error: Encode/Decode Error.
pub const OSS_PDU_MISMATCH: HRESULT = 0x80093009u32 as HRESULT;

/// OSS ASN.1 Error: Internal Error.
pub const OSS_LIMITED: HRESULT = 0x8009300Au32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_BAD_PTR: HRESULT = 0x8009300Bu32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_BAD_TIME: HRESULT = 0x8009300Cu32 as HRESULT;

/// OSS ASN.1 Error: Unsupported BER indefinite-length encoding.
pub const OSS_INDEFINITE_NOT_SUPPORTED: HRESULT = 0x8009300Du32 as HRESULT;

/// OSS ASN.1 Error: Access violation.
pub const OSS_MEM_ERROR: HRESULT = 0x8009300Eu32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_BAD_TABLE: HRESULT = 0x8009300Fu32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_TOO_LONG: HRESULT = 0x80093010u32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_CONSTRAINT_VIOLATED: HRESULT = 0x80093011u32 as HRESULT;

/// OSS ASN.1 Error: Internal Error.
pub const OSS_FATAL_ERROR: HRESULT = 0x80093012u32 as HRESULT;

/// OSS ASN.1 Error: Multi-threading conflict.
pub const OSS_ACCESS_SERIALIZATION_ERROR: HRESULT = 0x80093013u32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_NULL_TBL: HRESULT = 0x80093014u32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_NULL_FCN: HRESULT = 0x80093015u32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_BAD_ENCRULES: HRESULT = 0x80093016u32 as HRESULT;

/// OSS ASN.1 Error: Encode/Decode function not implemented.
pub const OSS_UNAVAIL_ENCRULES: HRESULT = 0x80093017u32 as HRESULT;

/// OSS ASN.1 Error: Trace file error.
pub const OSS_CANT_OPEN_TRACE_WINDOW: HRESULT = 0x80093018u32 as HRESULT;

/// OSS ASN.1 Error: Function not implemented.
pub const OSS_UNIMPLEMENTED: HRESULT = 0x80093019u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_OID_DLL_NOT_LINKED: HRESULT = 0x8009301Au32 as HRESULT;

/// OSS ASN.1 Error: Trace file error.
pub const OSS_CANT_OPEN_TRACE_FILE: HRESULT = 0x8009301Bu32 as HRESULT;

/// OSS ASN.1 Error: Trace file error.
pub const OSS_TRACE_FILE_ALREADY_OPEN: HRESULT = 0x8009301Cu32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_TABLE_MISMATCH: HRESULT = 0x8009301Du32 as HRESULT;

/// OSS ASN.1 Error: Invalid data.
pub const OSS_TYPE_NOT_SUPPORTED: HRESULT = 0x8009301Eu32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_REAL_DLL_NOT_LINKED: HRESULT = 0x8009301Fu32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_REAL_CODE_NOT_LINKED: HRESULT = 0x80093020u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_OUT_OF_RANGE: HRESULT = 0x80093021u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_COPIER_DLL_NOT_LINKED: HRESULT = 0x80093022u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_CONSTRAINT_DLL_NOT_LINKED: HRESULT = 0x80093023u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_COMPARATOR_DLL_NOT_LINKED: HRESULT = 0x80093024u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_COMPARATOR_CODE_NOT_LINKED: HRESULT = 0x80093025u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_MEM_MGR_DLL_NOT_LINKED: HRESULT = 0x80093026u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_PDV_DLL_NOT_LINKED: HRESULT = 0x80093027u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_PDV_CODE_NOT_LINKED: HRESULT = 0x80093028u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_API_DLL_NOT_LINKED: HRESULT = 0x80093029u32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_BERDER_DLL_NOT_LINKED: HRESULT = 0x8009302Au32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_PER_DLL_NOT_LINKED: HRESULT = 0x8009302Bu32 as HRESULT;

/// OSS ASN.1 Error: Program link error.
pub const OSS_OPEN_TYPE_ERROR: HRESULT = 0x8009302Cu32 as HRESULT;

/// OSS ASN.1 Error: System resource error.
pub const OSS_MUTEX_NOT_CREATED: HRESULT = 0x8009302Du32 as HRESULT;

/// OSS ASN.1 Error: Trace file error.
pub const OSS_CANT_CLOSE_TRACE_FILE: HRESULT = 0x8009302Eu32 as HRESULT;

/// ASN1 Certificate encode/decode error code base. The ASN1 error values are offset by CRYPT_E_ASN1_ERROR.
pub const CRYPT_E_ASN1_ERROR: HRESULT = 0x80093100u32 as HRESULT;

/// ASN1 internal encode or decode error.
pub const CRYPT_E_ASN1_INTERNAL: HRESULT = 0x80093101u32 as HRESULT;

/// ASN1 unexpected end of data.
pub const CRYPT_E_ASN1_EOD: HRESULT = 0x80093102u32 as HRESULT;

/// ASN1 corrupted data.
pub const CRYPT_E_ASN1_CORRUPT: HRESULT = 0x80093103u32 as HRESULT;

/// ASN1 value too large.
pub const CRYPT_E_ASN1_LARGE: HRESULT = 0x80093104u32 as HRESULT;

/// ASN1 constraint violated.
pub const CRYPT_E_ASN1_CONSTRAINT: HRESULT = 0x80093105u32 as HRESULT;

/// ASN1 out of memory.
pub const CRYPT_E_ASN1_MEMORY: HRESULT = 0x80093106u32 as HRESULT;

/// ASN1 buffer overflow.
pub const CRYPT_E_ASN1_OVERFLOW: HRESULT = 0x80093107u32 as HRESULT;

/// ASN1 function not supported for this PDU.
pub const CRYPT_E_ASN1_BADPDU: HRESULT = 0x80093108u32 as HRESULT;

/// ASN1 bad arguments to function call.
pub const CRYPT_E_ASN1_BADARGS: HRESULT = 0x80093109u32 as HRESULT;

/// ASN1 bad real value.
pub const CRYPT_E_ASN1_BADREAL: HRESULT = 0x8009310Au32 as HRESULT;

/// ASN1 bad tag value met.
pub const CRYPT_E_ASN1_BADTAG: HRESULT = 0x8009310Bu32 as HRESULT;

/// ASN1 bad choice value.
pub const CRYPT_E_ASN1_CHOICE: HRESULT = 0x8009310Cu32 as HRESULT;

/// ASN1 bad encoding rule.
pub const CRYPT_E_ASN1_RULE: HRESULT = 0x8009310Du32 as HRESULT;

/// ASN1 bad unicode (UTF8).
pub const CRYPT_E_ASN1_UTF8: HRESULT = 0x8009310Eu32 as HRESULT;

/// ASN1 bad PDU type.
pub const CRYPT_E_ASN1_PDU_TYPE: HRESULT = 0x80093133u32 as HRESULT;

/// ASN1 not yet implemented.
pub const CRYPT_E_ASN1_NYI: HRESULT = 0x80093134u32 as HRESULT;

/// ASN1 skipped unknown extension(s).
pub const CRYPT_E_ASN1_EXTENDED: HRESULT = 0x80093201u32 as HRESULT;

/// ASN1 end of data expected
pub const CRYPT_E_ASN1_NOEOD: HRESULT = 0x80093202u32 as HRESULT;

/// The request subject name is invalid or too long.
pub const CERTSRV_E_BAD_REQUESTSUBJECT: HRESULT = 0x80094001u32 as HRESULT;

/// The request does not exist.
pub const CERTSRV_E_NO_REQUEST: HRESULT = 0x80094002u32 as HRESULT;

/// The request's current status does not allow this operation.
pub const CERTSRV_E_BAD_REQUESTSTATUS: HRESULT = 0x80094003u32 as HRESULT;

/// The requested property value is empty.
pub const CERTSRV_E_PROPERTY_EMPTY: HRESULT = 0x80094004u32 as HRESULT;

/// The certification authority's certificate contains invalid data.
pub const CERTSRV_E_INVALID_CA_CERTIFICATE: HRESULT = 0x80094005u32 as HRESULT;

/// Certificate service has been suspended for a database restore operation.
pub const CERTSRV_E_SERVER_SUSPENDED: HRESULT = 0x80094006u32 as HRESULT;

/// The certificate contains an encoded length that is potentially incompatible with older enrollment software.
pub const CERTSRV_E_ENCODING_LENGTH: HRESULT = 0x80094007u32 as HRESULT;

/// The operation is denied. The user has multiple roles assigned and the certification authority is configured to enforce role separation.
pub const CERTSRV_E_ROLECONFLICT: HRESULT = 0x80094008u32 as HRESULT;

/// The operation is denied. It can only be performed by a certificate manager that is allowed to manage certificates for the current requester.
pub const CERTSRV_E_RESTRICTEDOFFICER: HRESULT = 0x80094009u32 as HRESULT;

/// Cannot archive private key. The certification authority is not configured for key archival.
pub const CERTSRV_E_KEY_ARCHIVAL_NOT_CONFIGURED: HRESULT = 0x8009400Au32 as HRESULT;

/// Cannot archive private key. The certification authority could not verify one or more key recovery certificates.
pub const CERTSRV_E_NO_VALID_KRA: HRESULT = 0x8009400Bu32 as HRESULT;

/// The request is incorrectly formatted. The encrypted private key must be in an unauthenticated attribute in an outermost signature.
pub const CERTSRV_E_BAD_REQUEST_KEY_ARCHIVAL: HRESULT = 0x8009400Cu32 as HRESULT;

/// At least one security principal must have the permission to manage this CA.
pub const CERTSRV_E_NO_CAADMIN_DEFINED: HRESULT = 0x8009400Du32 as HRESULT;

/// The request contains an invalid renewal certificate attribute.
pub const CERTSRV_E_BAD_RENEWAL_CERT_ATTRIBUTE: HRESULT = 0x8009400Eu32 as HRESULT;

/// An attempt was made to open a Certification Authority database session, but there are already too many active sessions. The server may need to be configured to allow additional sessions.
pub const CERTSRV_E_NO_DB_SESSIONS: HRESULT = 0x8009400Fu32 as HRESULT;

/// A memory reference caused a data alignment fault.
pub const CERTSRV_E_ALIGNMENT_FAULT: HRESULT = 0x80094010u32 as HRESULT;

/// The permissions on this certification authority do not allow the current user to enroll for certificates.
pub const CERTSRV_E_ENROLL_DENIED: HRESULT = 0x80094011u32 as HRESULT;

/// The permissions on the certificate template do not allow the current user to enroll for this type of certificate.
pub const CERTSRV_E_TEMPLATE_DENIED: HRESULT = 0x80094012u32 as HRESULT;

/// The contacted domain controller cannot support signed LDAP traffic. Update the domain controller or configure Certificate Services to use SSL for Active Directory access.
pub const CERTSRV_E_DOWNLEVEL_DC_SSL_OR_UPGRADE: HRESULT = 0x80094013u32 as HRESULT;

/// The request was denied by a certificate manager or CA administrator.
pub const CERTSRV_E_ADMIN_DENIED_REQUEST: HRESULT = 0x80094014u32 as HRESULT;

/// An enrollment policy server cannot be located.
pub const CERTSRV_E_NO_POLICY_SERVER: HRESULT = 0x80094015u32 as HRESULT;

/// A signature algorithm or public key length does not meet the system's minimum required strength.
pub const CERTSRV_E_WEAK_SIGNATURE_OR_KEY: HRESULT = 0x80094016u32 as HRESULT;

/// Failed to create an attested key.  This computer or the cryptographic provider may not meet the hardware requirements to support key attestation.
pub const CERTSRV_E_KEY_ATTESTATION_NOT_SUPPORTED: HRESULT = 0x80094017u32 as HRESULT;

/// No encryption certificate was specified.
pub const CERTSRV_E_ENCRYPTION_CERT_REQUIRED: HRESULT = 0x80094018u32 as HRESULT;

/// The requested certificate template is not supported by this CA.
pub const CERTSRV_E_UNSUPPORTED_CERT_TYPE: HRESULT = 0x80094800u32 as HRESULT;

/// The request contains no certificate template information.
pub const CERTSRV_E_NO_CERT_TYPE: HRESULT = 0x80094801u32 as HRESULT;

/// The request contains conflicting template information.
pub const CERTSRV_E_TEMPLATE_CONFLICT: HRESULT = 0x80094802u32 as HRESULT;

/// The request is missing a required Subject Alternate name extension.
pub const CERTSRV_E_SUBJECT_ALT_NAME_REQUIRED: HRESULT = 0x80094803u32 as HRESULT;

/// The request is missing a required private key for archival by the server.
pub const CERTSRV_E_ARCHIVED_KEY_REQUIRED: HRESULT = 0x80094804u32 as HRESULT;

/// The request is missing a required SMIME capabilities extension.
pub const CERTSRV_E_SMIME_REQUIRED: HRESULT = 0x80094805u32 as HRESULT;

/// The request was made on behalf of a subject other than the caller. The certificate template must be configured to require at least one signature to authorize the request.
pub const CERTSRV_E_BAD_RENEWAL_SUBJECT: HRESULT = 0x80094806u32 as HRESULT;

/// The request template version is newer than the supported template version.
pub const CERTSRV_E_BAD_TEMPLATE_VERSION: HRESULT = 0x80094807u32 as HRESULT;

/// The template is missing a required signature policy attribute.
pub const CERTSRV_E_TEMPLATE_POLICY_REQUIRED: HRESULT = 0x80094808u32 as HRESULT;

/// The request is missing required signature policy information.
pub const CERTSRV_E_SIGNATURE_POLICY_REQUIRED: HRESULT = 0x80094809u32 as HRESULT;

/// The request is missing one or more required signatures.
pub const CERTSRV_E_SIGNATURE_COUNT: HRESULT = 0x8009480Au32 as HRESULT;

/// One or more signatures did not include the required application or issuance policies. The request is missing one or more required valid signatures.
pub const CERTSRV_E_SIGNATURE_REJECTED: HRESULT = 0x8009480Bu32 as HRESULT;

/// The request is missing one or more required signature issuance policies.
pub const CERTSRV_E_ISSUANCE_POLICY_REQUIRED: HRESULT = 0x8009480Cu32 as HRESULT;

/// The UPN is unavailable and cannot be added to the Subject Alternate name.
pub const CERTSRV_E_SUBJECT_UPN_REQUIRED: HRESULT = 0x8009480Du32 as HRESULT;

/// The Active Directory GUID is unavailable and cannot be added to the Subject Alternate name.
pub const CERTSRV_E_SUBJECT_DIRECTORY_GUID_REQUIRED: HRESULT = 0x8009480Eu32 as HRESULT;

/// The DNS name is unavailable and cannot be added to the Subject Alternate name.
pub const CERTSRV_E_SUBJECT_DNS_REQUIRED: HRESULT = 0x8009480Fu32 as HRESULT;

/// The request includes a private key for archival by the server, but key archival is not enabled for the specified certificate template.
pub const CERTSRV_E_ARCHIVED_KEY_UNEXPECTED: HRESULT = 0x80094810u32 as HRESULT;

/// The public key does not meet the minimum size required by the specified certificate template.
pub const CERTSRV_E_KEY_LENGTH: HRESULT = 0x80094811u32 as HRESULT;

/// The EMail name is unavailable and cannot be added to the Subject or Subject Alternate name.
pub const CERTSRV_E_SUBJECT_EMAIL_REQUIRED: HRESULT = 0x80094812u32 as HRESULT;

/// One or more certificate templates to be enabled on this certification authority could not be found.
pub const CERTSRV_E_UNKNOWN_CERT_TYPE: HRESULT = 0x80094813u32 as HRESULT;

/// The certificate template renewal period is longer than the certificate validity period. The template should be reconfigured or the CA certificate renewed.
pub const CERTSRV_E_CERT_TYPE_OVERLAP: HRESULT = 0x80094814u32 as HRESULT;

/// The certificate template requires too many RA signatures. Only one RA signature is allowed.
pub const CERTSRV_E_TOO_MANY_SIGNATURES: HRESULT = 0x80094815u32 as HRESULT;

/// The certificate template requires renewal with the same public key, but the request uses a different public key.
pub const CERTSRV_E_RENEWAL_BAD_PUBLIC_KEY: HRESULT = 0x80094816u32 as HRESULT;

/// The certification authority cannot interpret or verify the endorsement key information supplied in the request, or the information is inconsistent.
pub const CERTSRV_E_INVALID_EK: HRESULT = 0x80094817u32 as HRESULT;

/// The certification authority cannot validate the Attestation Identity Key Id Binding.
pub const CERTSRV_E_INVALID_IDBINDING: HRESULT = 0x80094818u32 as HRESULT;

/// The certification authority cannot validate the private key attestation data.
pub const CERTSRV_E_INVALID_ATTESTATION: HRESULT = 0x80094819u32 as HRESULT;

/// The request does not support private key attestation as defined in the certificate template.
pub const CERTSRV_E_KEY_ATTESTATION: HRESULT = 0x8009481Au32 as HRESULT;

/// The request public key is not consistent with the private key attestation data.
pub const CERTSRV_E_CORRUPT_KEY_ATTESTATION: HRESULT = 0x8009481Bu32 as HRESULT;

/// The private key attestation challenge cannot be validated because the encryption certificate has expired, or the certificate or key is unavailable.
pub const CERTSRV_E_EXPIRED_CHALLENGE: HRESULT = 0x8009481Cu32 as HRESULT;

/// The client's response could not be validated. It is either unexpected or incorrect.
pub const CERTSRV_E_INVALID_RESPONSE: HRESULT = 0x8009481Du32 as HRESULT;

/// A valid Request ID was not detected in the request attributes, or an invalid one was submitted.
pub const CERTSRV_E_INVALID_REQUESTID: HRESULT = 0x8009481Eu32 as HRESULT;

/// The request is not consistent with the previously generated precertificate.
pub const CERTSRV_E_REQUEST_PRECERTIFICATE_MISMATCH: HRESULT = 0x8009481Fu32 as HRESULT;

/// The request is locked against edits until a response is received from the client.
pub const CERTSRV_E_PENDING_CLIENT_RESPONSE: HRESULT = 0x80094820u32 as HRESULT;

/// The Active Directory SID is unavailable and cannot be added to the custom security extension.
pub const CERTSRV_E_SEC_EXT_DIRECTORY_SID_REQUIRED: HRESULT = 0x80094821u32 as HRESULT;

/// The key is not exportable.
pub const XENROLL_E_KEY_NOT_EXPORTABLE: HRESULT = 0x80095000u32 as HRESULT;

/// You cannot add the root CA certificate into your local store.
pub const XENROLL_E_CANNOT_ADD_ROOT_CERT: HRESULT = 0x80095001u32 as HRESULT;

/// The key archival hash attribute was not found in the response.
pub const XENROLL_E_RESPONSE_KA_HASH_NOT_FOUND: HRESULT = 0x80095002u32 as HRESULT;

/// An unexpected key archival hash attribute was found in the response.
pub const XENROLL_E_RESPONSE_UNEXPECTED_KA_HASH: HRESULT = 0x80095003u32 as HRESULT;

/// There is a key archival hash mismatch between the request and the response.
pub const XENROLL_E_RESPONSE_KA_HASH_MISMATCH: HRESULT = 0x80095004u32 as HRESULT;

/// Signing certificate cannot include SMIME extension.
pub const XENROLL_E_KEYSPEC_SMIME_MISMATCH: HRESULT = 0x80095005u32 as HRESULT;

/// A system-level error occurred while verifying trust.
pub const TRUST_E_SYSTEM_ERROR: HRESULT = 0x80096001u32 as HRESULT;

/// The certificate for the signer of the message is invalid or not found.
pub const TRUST_E_NO_SIGNER_CERT: HRESULT = 0x80096002u32 as HRESULT;

/// One of the counter signatures was invalid.
pub const TRUST_E_COUNTER_SIGNER: HRESULT = 0x80096003u32 as HRESULT;

/// The signature of the certificate cannot be verified.
pub const TRUST_E_CERT_SIGNATURE: HRESULT = 0x80096004u32 as HRESULT;

/// The timestamp signature and/or certificate could not be verified or is malformed.
pub const TRUST_E_TIME_STAMP: HRESULT = 0x80096005u32 as HRESULT;

/// The digital signature of the object did not verify.
pub const TRUST_E_BAD_DIGEST: HRESULT = 0x80096010u32 as HRESULT;

/// The digital signature of the object is malformed. For technical detail, see security bulletin MS13-098.
pub const TRUST_E_MALFORMED_SIGNATURE: HRESULT = 0x80096011u32 as HRESULT;

/// A certificate's basic constraint extension has not been observed.
pub const TRUST_E_BASIC_CONSTRAINTS: HRESULT = 0x80096019u32 as HRESULT;

/// The certificate does not meet or contain the Authenticode(tm) financial extensions.
pub const TRUST_E_FINANCIAL_CRITERIA: HRESULT = 0x8009601Eu32 as HRESULT;

/// Tried to reference a part of the file outside the proper range.
pub const MSSIPOTF_E_OUTOFMEMRANGE: HRESULT = 0x80097001u32 as HRESULT;

/// Could not retrieve an object from the file.
pub const MSSIPOTF_E_CANTGETOBJECT: HRESULT = 0x80097002u32 as HRESULT;

/// Could not find the head table in the file.
pub const MSSIPOTF_E_NOHEADTABLE: HRESULT = 0x80097003u32 as HRESULT;

/// The magic number in the head table is incorrect.
pub const MSSIPOTF_E_BAD_MAGICNUMBER: HRESULT = 0x80097004u32 as HRESULT;

/// The offset table has incorrect values.
pub const MSSIPOTF_E_BAD_OFFSET_TABLE: HRESULT = 0x80097005u32 as HRESULT;

/// Duplicate table tags or tags out of alphabetical order.
pub const MSSIPOTF_E_TABLE_TAGORDER: HRESULT = 0x80097006u32 as HRESULT;

/// A table does not start on a long word boundary.
pub const MSSIPOTF_E_TABLE_LONGWORD: HRESULT = 0x80097007u32 as HRESULT;

/// First table does not appear after header information.
pub const MSSIPOTF_E_BAD_FIRST_TABLE_PLACEMENT: HRESULT = 0x80097008u32 as HRESULT;

/// Two or more tables overlap.
pub const MSSIPOTF_E_TABLES_OVERLAP: HRESULT = 0x80097009u32 as HRESULT;

/// Too many pad bytes between tables or pad bytes are not 0.
pub const MSSIPOTF_E_TABLE_PADBYTES: HRESULT = 0x8009700Au32 as HRESULT;

/// File is too small to contain the last table.
pub const MSSIPOTF_E_FILETOOSMALL: HRESULT = 0x8009700Bu32 as HRESULT;

/// A table checksum is incorrect.
pub const MSSIPOTF_E_TABLE_CHECKSUM: HRESULT = 0x8009700Cu32 as HRESULT;

/// The file checksum is incorrect.
pub const MSSIPOTF_E_FILE_CHECKSUM: HRESULT = 0x8009700Du32 as HRESULT;

/// The signature does not have the correct attributes for the policy.
pub const MSSIPOTF_E_FAILED_POLICY: HRESULT = 0x80097010u32 as HRESULT;

/// The file did not pass the hints check.
pub const MSSIPOTF_E_FAILED_HINTS_CHECK: HRESULT = 0x80097011u32 as HRESULT;

/// The file is not an OpenType file.
pub const MSSIPOTF_E_NOT_OPENTYPE: HRESULT = 0x80097012u32 as HRESULT;

/// Failed on a file operation (open, map, read, write).
pub const MSSIPOTF_E_FILE: HRESULT = 0x80097013u32 as HRESULT;

/// A call to a CryptoAPI function failed.
pub const MSSIPOTF_E_CRYPT: HRESULT = 0x80097014u32 as HRESULT;

/// There is a bad version number in the file.
pub const MSSIPOTF_E_BADVERSION: HRESULT = 0x80097015u32 as HRESULT;

/// The structure of the DSIG table is incorrect.
pub const MSSIPOTF_E_DSIG_STRUCTURE: HRESULT = 0x80097016u32 as HRESULT;

/// A check failed in a partially constant table.
pub const MSSIPOTF_E_PCONST_CHECK: HRESULT = 0x80097017u32 as HRESULT;

/// Some kind of structural error.
pub const MSSIPOTF_E_STRUCTURE: HRESULT = 0x80097018u32 as HRESULT;

/// The requested credential requires confirmation.
pub const ERROR_CRED_REQUIRES_CONFIRMATION: HRESULT = 0x80097019u32 as HRESULT;

/// Unknown trust provider.
pub const TRUST_E_PROVIDER_UNKNOWN: HRESULT = 0x800B0001u32 as HRESULT;

/// The trust verification action specified is not supported by the specified trust provider.
pub const TRUST_E_ACTION_UNKNOWN: HRESULT = 0x800B0002u32 as HRESULT;

/// The form specified for the subject is not one supported or known by the specified trust provider.
pub const TRUST_E_SUBJECT_FORM_UNKNOWN: HRESULT = 0x800B0003u32 as HRESULT;

/// The subject is not trusted for the specified action.
pub const TRUST_E_SUBJECT_NOT_TRUSTED: HRESULT = 0x800B0004u32 as HRESULT;

/// Error due to problem in ASN.1 encoding process.
pub const DIGSIG_E_ENCODE: HRESULT = 0x800B0005u32 as HRESULT;

/// Error due to problem in ASN.1 decoding process.
pub const DIGSIG_E_DECODE: HRESULT = 0x800B0006u32 as HRESULT;

/// Reading / writing Extensions where Attributes are appropriate, and vice versa.
pub const DIGSIG_E_EXTENSIBILITY: HRESULT = 0x800B0007u32 as HRESULT;

/// Unspecified cryptographic failure.
pub const DIGSIG_E_CRYPTO: HRESULT = 0x800B0008u32 as HRESULT;

/// The size of the data could not be determined.
pub const PERSIST_E_SIZEDEFINITE: HRESULT = 0x800B0009u32 as HRESULT;

/// The size of the indefinite-sized data could not be determined.
pub const PERSIST_E_SIZEINDEFINITE: HRESULT = 0x800B000Au32 as HRESULT;

/// This object does not read and write self-sizing data.
pub const PERSIST_E_NOTSELFSIZING: HRESULT = 0x800B000Bu32 as HRESULT;

/// No signature was present in the subject.
pub const TRUST_E_NOSIGNATURE: HRESULT = 0x800B0100u32 as HRESULT;

/// A required certificate is not within its validity period when verifying against the current system clock or the timestamp in the signed file.
pub const CERT_E_EXPIRED: HRESULT = 0x800B0101u32 as HRESULT;

/// The validity periods of the certification chain do not nest correctly.
pub const CERT_E_VALIDITYPERIODNESTING: HRESULT = 0x800B0102u32 as HRESULT;

/// A certificate that can only be used as an end-entity is being used as a CA or vice versa.
pub const CERT_E_ROLE: HRESULT = 0x800B0103u32 as HRESULT;

/// A path length constraint in the certification chain has been violated.
pub const CERT_E_PATHLENCONST: HRESULT = 0x800B0104u32 as HRESULT;

/// A certificate contains an unknown extension that is marked 'critical'.
pub const CERT_E_CRITICAL: HRESULT = 0x800B0105u32 as HRESULT;

/// A certificate being used for a purpose other than the ones specified by its CA.
pub const CERT_E_PURPOSE: HRESULT = 0x800B0106u32 as HRESULT;

/// A parent of a given certificate in fact did not issue that child certificate.
pub const CERT_E_ISSUERCHAINING: HRESULT = 0x800B0107u32 as HRESULT;

/// A certificate is missing or has an empty value for an important field, such as a subject or issuer name.
pub const CERT_E_MALFORMED: HRESULT = 0x800B0108u32 as HRESULT;

/// A certificate chain processed, but terminated in a root certificate which is not trusted by the trust provider.
pub const CERT_E_UNTRUSTEDROOT: HRESULT = 0x800B0109u32 as HRESULT;

/// A certificate chain could not be built to a trusted root authority.
pub const CERT_E_CHAINING: HRESULT = 0x800B010Au32 as HRESULT;

/// Generic trust failure.
pub const TRUST_E_FAIL: HRESULT = 0x800B010Bu32 as HRESULT;

/// A certificate was explicitly revoked by its issuer.
pub const CERT_E_REVOKED: HRESULT = 0x800B010Cu32 as HRESULT;

/// The certification path terminates with the test root which is not trusted with the current policy settings.
pub const CERT_E_UNTRUSTEDTESTROOT: HRESULT = 0x800B010Du32 as HRESULT;

/// The revocation process could not continue - the certificate(s) could not be checked.
pub const CERT_E_REVOCATION_FAILURE: HRESULT = 0x800B010Eu32 as HRESULT;

/// The certificate's CN name does not match the passed value.
pub const CERT_E_CN_NO_MATCH: HRESULT = 0x800B010Fu32 as HRESULT;

/// The certificate is not valid for the requested usage.
pub const CERT_E_WRONG_USAGE: HRESULT = 0x800B0110u32 as HRESULT;

/// The certificate was explicitly marked as untrusted by the user.
pub const TRUST_E_EXPLICIT_DISTRUST: HRESULT = 0x800B0111u32 as HRESULT;

/// A certification chain processed correctly, but one of the CA certificates is not trusted by the policy provider.
pub const CERT_E_UNTRUSTEDCA: HRESULT = 0x800B0112u32 as HRESULT;

/// The certificate has invalid policy.
pub const CERT_E_INVALID_POLICY: HRESULT = 0x800B0113u32 as HRESULT;

/// The certificate has an invalid name. The name is not included in the permitted list or is explicitly excluded.
pub const CERT_E_INVALID_NAME: HRESULT = 0x800B0114u32 as HRESULT;

/// A non-empty line was encountered in the INF before the start of a section.
pub const SPAPI_E_EXPECTED_SECTION_NAME: HRESULT = 0x800F0000u32 as HRESULT;

/// A section name marker in the INF is not complete, or does not exist on a line by itself.
pub const SPAPI_E_BAD_SECTION_NAME_LINE: HRESULT = 0x800F0001u32 as HRESULT;

/// An INF section was encountered whose name exceeds the maximum section name length.
pub const SPAPI_E_SECTION_NAME_TOO_LONG: HRESULT = 0x800F0002u32 as HRESULT;

/// The syntax of the INF is invalid.
pub const SPAPI_E_GENERAL_SYNTAX: HRESULT = 0x800F0003u32 as HRESULT;

/// The style of the INF is different than what was requested.
pub const SPAPI_E_WRONG_INF_STYLE: HRESULT = 0x800F0100u32 as HRESULT;

/// The required section was not found in the INF.
pub const SPAPI_E_SECTION_NOT_FOUND: HRESULT = 0x800F0101u32 as HRESULT;

/// The required line was not found in the INF.
pub const SPAPI_E_LINE_NOT_FOUND: HRESULT = 0x800F0102u32 as HRESULT;

/// The files affected by the installation of this file queue have not been backed up for uninstall.
pub const SPAPI_E_NO_BACKUP: HRESULT = 0x800F0103u32 as HRESULT;

/// The INF or the device information set or element does not have an associated install class.
pub const SPAPI_E_NO_ASSOCIATED_CLASS: HRESULT = 0x800F0200u32 as HRESULT;

/// The INF or the device information set or element does not match the specified install class.
pub const SPAPI_E_CLASS_MISMATCH: HRESULT = 0x800F0201u32 as HRESULT;

/// An existing device was found that is a duplicate of the device being manually installed.
pub const SPAPI_E_DUPLICATE_FOUND: HRESULT = 0x800F0202u32 as HRESULT;

/// There is no driver selected for the device information set or element.
pub const SPAPI_E_NO_DRIVER_SELECTED: HRESULT = 0x800F0203u32 as HRESULT;

/// The requested device registry key does not exist.
pub const SPAPI_E_KEY_DOES_NOT_EXIST: HRESULT = 0x800F0204u32 as HRESULT;

/// The device instance name is invalid.
pub const SPAPI_E_INVALID_DEVINST_NAME: HRESULT = 0x800F0205u32 as HRESULT;

/// The install class is not present or is invalid.
pub const SPAPI_E_INVALID_CLASS: HRESULT = 0x800F0206u32 as HRESULT;

/// The device instance cannot be created because it already exists.
pub const SPAPI_E_DEVINST_ALREADY_EXISTS: HRESULT = 0x800F0207u32 as HRESULT;

/// The operation cannot be performed on a device information element that has not been registered.
pub const SPAPI_E_DEVINFO_NOT_REGISTERED: HRESULT = 0x800F0208u32 as HRESULT;

/// The device property code is invalid.
pub const SPAPI_E_INVALID_REG_PROPERTY: HRESULT = 0x800F0209u32 as HRESULT;

/// The INF from which a driver list is to be built does not exist.
pub const SPAPI_E_NO_INF: HRESULT = 0x800F020Au32 as HRESULT;

/// The device instance does not exist in the hardware tree.
pub const SPAPI_E_NO_SUCH_DEVINST: HRESULT = 0x800F020Bu32 as HRESULT;

/// The icon representing this install class cannot be loaded.
pub const SPAPI_E_CANT_LOAD_CLASS_ICON: HRESULT = 0x800F020Cu32 as HRESULT;

/// The class installer registry entry is invalid.
pub const SPAPI_E_INVALID_CLASS_INSTALLER: HRESULT = 0x800F020Du32 as HRESULT;

/// The class installer has indicated that the default action should be performed for this installation request.
pub const SPAPI_E_DI_DO_DEFAULT: HRESULT = 0x800F020Eu32 as HRESULT;

/// The operation does not require any files to be copied.
pub const SPAPI_E_DI_NOFILECOPY: HRESULT = 0x800F020Fu32 as HRESULT;

/// The specified hardware profile does not exist.
pub const SPAPI_E_INVALID_HWPROFILE: HRESULT = 0x800F0210u32 as HRESULT;

/// There is no device information element currently selected for this device information set.
pub const SPAPI_E_NO_DEVICE_SELECTED: HRESULT = 0x800F0211u32 as HRESULT;

/// The operation cannot be performed because the device information set is locked.
pub const SPAPI_E_DEVINFO_LIST_LOCKED: HRESULT = 0x800F0212u32 as HRESULT;

/// The operation cannot be performed because the device information element is locked.
pub const SPAPI_E_DEVINFO_DATA_LOCKED: HRESULT = 0x800F0213u32 as HRESULT;

/// The specified path does not contain any applicable device INFs.
pub const SPAPI_E_DI_BAD_PATH: HRESULT = 0x800F0214u32 as HRESULT;

/// No class installer parameters have been set for the device information set or element.
pub const SPAPI_E_NO_CLASSINSTALL_PARAMS: HRESULT = 0x800F0215u32 as HRESULT;

/// The operation cannot be performed because the file queue is locked.
pub const SPAPI_E_FILEQUEUE_LOCKED: HRESULT = 0x800F0216u32 as HRESULT;

/// A service installation section in this INF is invalid.
pub const SPAPI_E_BAD_SERVICE_INSTALLSECT: HRESULT = 0x800F0217u32 as HRESULT;

/// There is no class driver list for the device information element.
pub const SPAPI_E_NO_CLASS_DRIVER_LIST: HRESULT = 0x800F0218u32 as HRESULT;

/// The installation failed because a function driver was not specified for this device instance.
pub const SPAPI_E_NO_ASSOCIATED_SERVICE: HRESULT = 0x800F0219u32 as HRESULT;

/// There is presently no default device interface designated for this interface class.
pub const SPAPI_E_NO_DEFAULT_DEVICE_INTERFACE: HRESULT = 0x800F021Au32 as HRESULT;

/// The operation cannot be performed because the device interface is currently active.
pub const SPAPI_E_DEVICE_INTERFACE_ACTIVE: HRESULT = 0x800F021Bu32 as HRESULT;

/// The operation cannot be performed because the device interface has been removed from the system.
pub const SPAPI_E_DEVICE_INTERFACE_REMOVED: HRESULT = 0x800F021Cu32 as HRESULT;

/// An interface installation section in this INF is invalid.
pub const SPAPI_E_BAD_INTERFACE_INSTALLSECT: HRESULT = 0x800F021Du32 as HRESULT;

/// This interface class does not exist in the system.
pub const SPAPI_E_NO_SUCH_INTERFACE_CLASS: HRESULT = 0x800F021Eu32 as HRESULT;

/// The reference string supplied for this interface device is invalid.
pub const SPAPI_E_INVALID_REFERENCE_STRING: HRESULT = 0x800F021Fu32 as HRESULT;

/// The specified machine name does not conform to UNC naming conventions.
pub const SPAPI_E_INVALID_MACHINENAME: HRESULT = 0x800F0220u32 as HRESULT;

/// A general remote communication error occurred.
pub const SPAPI_E_REMOTE_COMM_FAILURE: HRESULT = 0x800F0221u32 as HRESULT;

/// The machine selected for remote communication is not available at this time.
pub const SPAPI_E_MACHINE_UNAVAILABLE: HRESULT = 0x800F0222u32 as HRESULT;

/// The Plug and Play service is not available on the remote machine.
pub const SPAPI_E_NO_CONFIGMGR_SERVICES: HRESULT = 0x800F0223u32 as HRESULT;

/// The property page provider registry entry is invalid.
pub const SPAPI_E_INVALID_PROPPAGE_PROVIDER: HRESULT = 0x800F0224u32 as HRESULT;

/// The requested device interface is not present in the system.
pub const SPAPI_E_NO_SUCH_DEVICE_INTERFACE: HRESULT = 0x800F0225u32 as HRESULT;

/// The device's co-installer has additional work to perform after installation is complete.
pub const SPAPI_E_DI_POSTPROCESSING_REQUIRED: HRESULT = 0x800F0226u32 as HRESULT;

/// The device's co-installer is invalid.
pub const SPAPI_E_INVALID_COINSTALLER: HRESULT = 0x800F0227u32 as HRESULT;

/// There are no compatible drivers for this device.
pub const SPAPI_E_NO_COMPAT_DRIVERS: HRESULT = 0x800F0228u32 as HRESULT;

/// There is no icon that represents this device or device type.
pub const SPAPI_E_NO_DEVICE_ICON: HRESULT = 0x800F0229u32 as HRESULT;

/// A logical configuration specified in this INF is invalid.
pub const SPAPI_E_INVALID_INF_LOGCONFIG: HRESULT = 0x800F022Au32 as HRESULT;

/// The class installer has denied the request to install or upgrade this device.
pub const SPAPI_E_DI_DONT_INSTALL: HRESULT = 0x800F022Bu32 as HRESULT;

/// One of the filter drivers installed for this device is invalid.
pub const SPAPI_E_INVALID_FILTER_DRIVER: HRESULT = 0x800F022Cu32 as HRESULT;

/// The driver selected for this device does not support this version of Windows.
pub const SPAPI_E_NON_WINDOWS_NT_DRIVER: HRESULT = 0x800F022Du32 as HRESULT;

/// The driver selected for this device does not support Windows.
pub const SPAPI_E_NON_WINDOWS_DRIVER: HRESULT = 0x800F022Eu32 as HRESULT;

/// The third-party INF does not contain digital signature information.
pub const SPAPI_E_NO_CATALOG_FOR_OEM_INF: HRESULT = 0x800F022Fu32 as HRESULT;

/// An invalid attempt was made to use a device installation file queue for verification of digital signatures relative to other platforms.
pub const SPAPI_E_DEVINSTALL_QUEUE_NONNATIVE: HRESULT = 0x800F0230u32 as HRESULT;

/// The device cannot be disabled.
pub const SPAPI_E_NOT_DISABLEABLE: HRESULT = 0x800F0231u32 as HRESULT;

/// The device could not be dynamically removed.
pub const SPAPI_E_CANT_REMOVE_DEVINST: HRESULT = 0x800F0232u32 as HRESULT;

/// Cannot copy to specified target.
pub const SPAPI_E_INVALID_TARGET: HRESULT = 0x800F0233u32 as HRESULT;

/// Driver is not intended for this platform.
pub const SPAPI_E_DRIVER_NONNATIVE: HRESULT = 0x800F0234u32 as HRESULT;

/// Operation not allowed in WOW64.
pub const SPAPI_E_IN_WOW64: HRESULT = 0x800F0235u32 as HRESULT;

/// The operation involving unsigned file copying was rolled back, so that a system restore point could be set.
pub const SPAPI_E_SET_SYSTEM_RESTORE_POINT: HRESULT = 0x800F0236u32 as HRESULT;

/// An INF was copied into the Windows INF directory in an improper manner.
pub const SPAPI_E_INCORRECTLY_COPIED_INF: HRESULT = 0x800F0237u32 as HRESULT;

/// The Security Configuration Editor (SCE) APIs have been disabled on this Embedded product.
pub const SPAPI_E_SCE_DISABLED: HRESULT = 0x800F0238u32 as HRESULT;

/// An unknown exception was encountered.
pub const SPAPI_E_UNKNOWN_EXCEPTION: HRESULT = 0x800F0239u32 as HRESULT;

/// A problem was encountered when accessing the Plug and Play registry database.
pub const SPAPI_E_PNP_REGISTRY_ERROR: HRESULT = 0x800F023Au32 as HRESULT;

/// The requested operation is not supported for a remote machine.
pub const SPAPI_E_REMOTE_REQUEST_UNSUPPORTED: HRESULT = 0x800F023Bu32 as HRESULT;

/// The specified file is not an installed OEM INF.
pub const SPAPI_E_NOT_AN_INSTALLED_OEM_INF: HRESULT = 0x800F023Cu32 as HRESULT;

/// One or more devices are presently installed using the specified INF.
pub const SPAPI_E_INF_IN_USE_BY_DEVICES: HRESULT = 0x800F023Du32 as HRESULT;

/// The requested device install operation is obsolete.
pub const SPAPI_E_DI_FUNCTION_OBSOLETE: HRESULT = 0x800F023Eu32 as HRESULT;

/// A file could not be verified because it does not have an associated catalog signed via Authenticode(tm).
pub const SPAPI_E_NO_AUTHENTICODE_CATALOG: HRESULT = 0x800F023Fu32 as HRESULT;

/// Authenticode(tm) signature verification is not supported for the specified INF.
pub const SPAPI_E_AUTHENTICODE_DISALLOWED: HRESULT = 0x800F0240u32 as HRESULT;

/// The INF was signed with an Authenticode(tm) catalog from a trusted publisher.
pub const SPAPI_E_AUTHENTICODE_TRUSTED_PUBLISHER: HRESULT = 0x800F0241u32 as HRESULT;

/// The publisher of an Authenticode(tm) signed catalog has not yet been established as trusted.
pub const SPAPI_E_AUTHENTICODE_TRUST_NOT_ESTABLISHED: HRESULT = 0x800F0242u32 as HRESULT;

/// The publisher of an Authenticode(tm) signed catalog was not established as trusted.
pub const SPAPI_E_AUTHENTICODE_PUBLISHER_NOT_TRUSTED: HRESULT = 0x800F0243u32 as HRESULT;

/// The software was tested for compliance with Windows Logo requirements on a different version of Windows, and may not be compatible with this version.
pub const SPAPI_E_SIGNATURE_OSATTRIBUTE_MISMATCH: HRESULT = 0x800F0244u32 as HRESULT;

/// The file may only be validated by a catalog signed via Authenticode(tm).
pub const SPAPI_E_ONLY_VALIDATE_VIA_AUTHENTICODE: HRESULT = 0x800F0245u32 as HRESULT;

/// One of the installers for this device cannot perform the installation at this time.
pub const SPAPI_E_DEVICE_INSTALLER_NOT_READY: HRESULT = 0x800F0246u32 as HRESULT;

/// A problem was encountered while attempting to add the driver to the store.
pub const SPAPI_E_DRIVER_STORE_ADD_FAILED: HRESULT = 0x800F0247u32 as HRESULT;

/// The installation of this device is forbidden by system policy. Contact your system administrator.
pub const SPAPI_E_DEVICE_INSTALL_BLOCKED: HRESULT = 0x800F0248u32 as HRESULT;

/// The installation of this driver is forbidden by system policy. Contact your system administrator.
pub const SPAPI_E_DRIVER_INSTALL_BLOCKED: HRESULT = 0x800F0249u32 as HRESULT;

/// The specified INF is the wrong type for this operation.
pub const SPAPI_E_WRONG_INF_TYPE: HRESULT = 0x800F024Au32 as HRESULT;

/// The hash for the file is not present in the specified catalog file. The file is likely corrupt or the victim of tampering.
pub const SPAPI_E_FILE_HASH_NOT_IN_CATALOG: HRESULT = 0x800F024Bu32 as HRESULT;

/// A problem was encountered while attempting to delete the driver from the store.
pub const SPAPI_E_DRIVER_STORE_DELETE_FAILED: HRESULT = 0x800F024Cu32 as HRESULT;

/// An unrecoverable stack overflow was encountered.
pub const SPAPI_E_UNRECOVERABLE_STACK_OVERFLOW: HRESULT = 0x800F0300u32 as HRESULT;

/// No installed components were detected.
pub const SPAPI_E_ERROR_NOT_INSTALLED: HRESULT = 0x800F1000u32 as HRESULT;

/// An internal consistency check failed.
pub const SCARD_F_INTERNAL_ERROR: HRESULT = 0x80100001u32 as HRESULT;

/// The action was cancelled by an SCardCancel request.
pub const SCARD_E_CANCELLED: HRESULT = 0x80100002u32 as HRESULT;

/// The supplied handle was invalid.
pub const SCARD_E_INVALID_HANDLE: HRESULT = 0x80100003u32 as HRESULT;

/// One or more of the supplied parameters could not be properly interpreted.
pub const SCARD_E_INVALID_PARAMETER: HRESULT = 0x80100004u32 as HRESULT;

/// Registry startup information is missing or invalid.
pub const SCARD_E_INVALID_TARGET: HRESULT = 0x80100005u32 as HRESULT;

/// Not enough memory available to complete this command.
pub const SCARD_E_NO_MEMORY: HRESULT = 0x80100006u32 as HRESULT;

/// An internal consistency timer has expired.
pub const SCARD_F_WAITED_TOO_LONG: HRESULT = 0x80100007u32 as HRESULT;

/// The data buffer to receive returned data is too small for the returned data.
pub const SCARD_E_INSUFFICIENT_BUFFER: HRESULT = 0x80100008u32 as HRESULT;

/// The specified reader name is not recognized.
pub const SCARD_E_UNKNOWN_READER: HRESULT = 0x80100009u32 as HRESULT;

/// The user-specified timeout value has expired.
pub const SCARD_E_TIMEOUT: HRESULT = 0x8010000Au32 as HRESULT;

/// The smart card cannot be accessed because of other connections outstanding.
pub const SCARD_E_SHARING_VIOLATION: HRESULT = 0x8010000Bu32 as HRESULT;

/// The operation requires a smart card, but no smart card is currently in the device.
pub const SCARD_E_NO_SMARTCARD: HRESULT = 0x8010000Cu32 as HRESULT;

/// The specified smart card name is not recognized.
pub const SCARD_E_UNKNOWN_CARD: HRESULT = 0x8010000Du32 as HRESULT;

/// The system could not dispose of the media in the requested manner.
pub const SCARD_E_CANT_DISPOSE: HRESULT = 0x8010000Eu32 as HRESULT;

/// The requested protocols are incompatible with the protocol currently in use with the smart card.
pub const SCARD_E_PROTO_MISMATCH: HRESULT = 0x8010000Fu32 as HRESULT;

/// The reader or smart card is not ready to accept commands.
pub const SCARD_E_NOT_READY: HRESULT = 0x80100010u32 as HRESULT;

/// One or more of the supplied parameters values could not be properly interpreted.
pub const SCARD_E_INVALID_VALUE: HRESULT = 0x80100011u32 as HRESULT;

/// The action was cancelled by the system, presumably to log off or shut down.
pub const SCARD_E_SYSTEM_CANCELLED: HRESULT = 0x80100012u32 as HRESULT;

/// An internal communications error has been detected.
pub const SCARD_F_COMM_ERROR: HRESULT = 0x80100013u32 as HRESULT;

/// An internal error has been detected, but the source is unknown.
pub const SCARD_F_UNKNOWN_ERROR: HRESULT = 0x80100014u32 as HRESULT;

/// An ATR obtained from the registry is not a valid ATR string.
pub const SCARD_E_INVALID_ATR: HRESULT = 0x80100015u32 as HRESULT;

/// An attempt was made to end a non-existent transaction.
pub const SCARD_E_NOT_TRANSACTED: HRESULT = 0x80100016u32 as HRESULT;

/// The specified reader is not currently available for use.
pub const SCARD_E_READER_UNAVAILABLE: HRESULT = 0x80100017u32 as HRESULT;

/// The operation has been aborted to allow the server application to exit.
pub const SCARD_P_SHUTDOWN: HRESULT = 0x80100018u32 as HRESULT;

/// The PCI Receive buffer was too small.
pub const SCARD_E_PCI_TOO_SMALL: HRESULT = 0x80100019u32 as HRESULT;

/// The reader driver does not meet minimal requirements for support.
pub const SCARD_E_READER_UNSUPPORTED: HRESULT = 0x8010001Au32 as HRESULT;

/// The reader driver did not produce a unique reader name.
pub const SCARD_E_DUPLICATE_READER: HRESULT = 0x8010001Bu32 as HRESULT;

/// The smart card does not meet minimal requirements for support.
pub const SCARD_E_CARD_UNSUPPORTED: HRESULT = 0x8010001Cu32 as HRESULT;

/// The Smart Card Resource Manager is not running.
pub const SCARD_E_NO_SERVICE: HRESULT = 0x8010001Du32 as HRESULT;

/// The Smart Card Resource Manager has shut down.
pub const SCARD_E_SERVICE_STOPPED: HRESULT = 0x8010001Eu32 as HRESULT;

/// An unexpected card error has occurred.
pub const SCARD_E_UNEXPECTED: HRESULT = 0x8010001Fu32 as HRESULT;

/// No Primary Provider can be found for the smart card.
pub const SCARD_E_ICC_INSTALLATION: HRESULT = 0x80100020u32 as HRESULT;

/// The requested order of object creation is not supported.
pub const SCARD_E_ICC_CREATEORDER: HRESULT = 0x80100021u32 as HRESULT;

/// This smart card does not support the requested feature.
pub const SCARD_E_UNSUPPORTED_FEATURE: HRESULT = 0x80100022u32 as HRESULT;

/// The identified directory does not exist in the smart card.
pub const SCARD_E_DIR_NOT_FOUND: HRESULT = 0x80100023u32 as HRESULT;

/// The identified file does not exist in the smart card.
pub const SCARD_E_FILE_NOT_FOUND: HRESULT = 0x80100024u32 as HRESULT;

/// The supplied path does not represent a smart card directory.
pub const SCARD_E_NO_DIR: HRESULT = 0x80100025u32 as HRESULT;

/// The supplied path does not represent a smart card file.
pub const SCARD_E_NO_FILE: HRESULT = 0x80100026u32 as HRESULT;

/// Access is denied to this file.
pub const SCARD_E_NO_ACCESS: HRESULT = 0x80100027u32 as HRESULT;

/// The smart card does not have enough memory to store the information.
pub const SCARD_E_WRITE_TOO_MANY: HRESULT = 0x80100028u32 as HRESULT;

/// There was an error trying to set the smart card file object pointer.
pub const SCARD_E_BAD_SEEK: HRESULT = 0x80100029u32 as HRESULT;

/// The supplied PIN is incorrect.
pub const SCARD_E_INVALID_CHV: HRESULT = 0x8010002Au32 as HRESULT;

/// An unrecognized error code was returned from a layered component.
pub const SCARD_E_UNKNOWN_RES_MNG: HRESULT = 0x8010002Bu32 as HRESULT;

/// The requested certificate does not exist.
pub const SCARD_E_NO_SUCH_CERTIFICATE: HRESULT = 0x8010002Cu32 as HRESULT;

/// The requested certificate could not be obtained.
pub const SCARD_E_CERTIFICATE_UNAVAILABLE: HRESULT = 0x8010002Du32 as HRESULT;

/// Cannot find a smart card reader.
pub const SCARD_E_NO_READERS_AVAILABLE: HRESULT = 0x8010002Eu32 as HRESULT;

/// A communications error with the smart card has been detected. Retry the operation.
pub const SCARD_E_COMM_DATA_LOST: HRESULT = 0x8010002Fu32 as HRESULT;

/// The requested key container does not exist on the smart card.
pub const SCARD_E_NO_KEY_CONTAINER: HRESULT = 0x80100030u32 as HRESULT;

/// The Smart Card Resource Manager is too busy to complete this operation.
pub const SCARD_E_SERVER_TOO_BUSY: HRESULT = 0x80100031u32 as HRESULT;

/// The smart card PIN cache has expired.
pub const SCARD_E_PIN_CACHE_EXPIRED: HRESULT = 0x80100032u32 as HRESULT;

/// The smart card PIN cannot be cached.
pub const SCARD_E_NO_PIN_CACHE: HRESULT = 0x80100033u32 as HRESULT;

/// The smart card is read only and cannot be written to.
pub const SCARD_E_READ_ONLY_CARD: HRESULT = 0x80100034u32 as HRESULT;

/// The reader cannot communicate with the smart card, due to ATR configuration conflicts.
pub const SCARD_W_UNSUPPORTED_CARD: HRESULT = 0x80100065u32 as HRESULT;

/// The smart card is not responding to a reset.
pub const SCARD_W_UNRESPONSIVE_CARD: HRESULT = 0x80100066u32 as HRESULT;

/// Power has been removed from the smart card, so that further communication is not possible.
pub const SCARD_W_UNPOWERED_CARD: HRESULT = 0x80100067u32 as HRESULT;

/// The smart card has been reset, so any shared state information is invalid.
pub const SCARD_W_RESET_CARD: HRESULT = 0x80100068u32 as HRESULT;

/// The smart card has been removed, so that further communication is not possible.
pub const SCARD_W_REMOVED_CARD: HRESULT = 0x80100069u32 as HRESULT;

/// Access was denied because of a security violation.
pub const SCARD_W_SECURITY_VIOLATION: HRESULT = 0x8010006Au32 as HRESULT;

/// The card cannot be accessed because the wrong PIN was presented.
pub const SCARD_W_WRONG_CHV: HRESULT = 0x8010006Bu32 as HRESULT;

/// The card cannot be accessed because the maximum number of PIN entry attempts has been reached.
pub const SCARD_W_CHV_BLOCKED: HRESULT = 0x8010006Cu32 as HRESULT;

/// The end of the smart card file has been reached.
pub const SCARD_W_EOF: HRESULT = 0x8010006Du32 as HRESULT;

/// The action was cancelled by the user.
pub const SCARD_W_CANCELLED_BY_USER: HRESULT = 0x8010006Eu32 as HRESULT;

/// No PIN was presented to the smart card.
pub const SCARD_W_CARD_NOT_AUTHENTICATED: HRESULT = 0x8010006Fu32 as HRESULT;

/// The requested item could not be found in the cache.
pub const SCARD_W_CACHE_ITEM_NOT_FOUND: HRESULT = 0x80100070u32 as HRESULT;

/// The requested cache item is too old and was deleted from the cache.
pub const SCARD_W_CACHE_ITEM_STALE: HRESULT = 0x80100071u32 as HRESULT;

/// The new cache item exceeds the maximum per-item size defined for the cache.
pub const SCARD_W_CACHE_ITEM_TOO_BIG: HRESULT = 0x80100072u32 as HRESULT;

/// Errors occurred accessing one or more objects - the ErrorInfo collection may have more detail
pub const COMADMIN_E_OBJECTERRORS: HRESULT = 0x80110401u32 as HRESULT;

/// One or more of the object's properties are missing or invalid
pub const COMADMIN_E_OBJECTINVALID: HRESULT = 0x80110402u32 as HRESULT;

/// The object was not found in the catalog
pub const COMADMIN_E_KEYMISSING: HRESULT = 0x80110403u32 as HRESULT;

/// The object is already registered
pub const COMADMIN_E_ALREADYINSTALLED: HRESULT = 0x80110404u32 as HRESULT;

/// Error occurred writing to the application file
pub const COMADMIN_E_APP_FILE_WRITEFAIL: HRESULT = 0x80110407u32 as HRESULT;

/// Error occurred reading the application file
pub const COMADMIN_E_APP_FILE_READFAIL: HRESULT = 0x80110408u32 as HRESULT;

/// Invalid version number in application file
pub const COMADMIN_E_APP_FILE_VERSION: HRESULT = 0x80110409u32 as HRESULT;

/// The file path is invalid
pub const COMADMIN_E_BADPATH: HRESULT = 0x8011040Au32 as HRESULT;

/// The application is already installed
pub const COMADMIN_E_APPLICATIONEXISTS: HRESULT = 0x8011040Bu32 as HRESULT;

/// The role already exists
pub const COMADMIN_E_ROLEEXISTS: HRESULT = 0x8011040Cu32 as HRESULT;

/// An error occurred copying the file
pub const COMADMIN_E_CANTCOPYFILE: HRESULT = 0x8011040Du32 as HRESULT;

/// One or more users are not valid
pub const COMADMIN_E_NOUSER: HRESULT = 0x8011040Fu32 as HRESULT;

/// One or more users in the application file are not valid
pub const COMADMIN_E_INVALIDUSERIDS: HRESULT = 0x80110410u32 as HRESULT;

/// The component's CLSID is missing or corrupt
pub const COMADMIN_E_NOREGISTRYCLSID: HRESULT = 0x80110411u32 as HRESULT;

/// The component's progID is missing or corrupt
pub const COMADMIN_E_BADREGISTRYPROGID: HRESULT = 0x80110412u32 as HRESULT;

/// Unable to set required authentication level for update request
pub const COMADMIN_E_AUTHENTICATIONLEVEL: HRESULT = 0x80110413u32 as HRESULT;

/// The identity or password set on the application is not valid
pub const COMADMIN_E_USERPASSWDNOTVALID: HRESULT = 0x80110414u32 as HRESULT;

/// Application file CLSIDs or IIDs do not match corresponding DLLs
pub const COMADMIN_E_CLSIDORIIDMISMATCH: HRESULT = 0x80110418u32 as HRESULT;

/// Interface information is either missing or changed
pub const COMADMIN_E_REMOTEINTERFACE: HRESULT = 0x80110419u32 as HRESULT;

/// DllRegisterServer failed on component install
pub const COMADMIN_E_DLLREGISTERSERVER: HRESULT = 0x8011041Au32 as HRESULT;

/// No server file share available
pub const COMADMIN_E_NOSERVERSHARE: HRESULT = 0x8011041Bu32 as HRESULT;

/// DLL could not be loaded
pub const COMADMIN_E_DLLLOADFAILED: HRESULT = 0x8011041Du32 as HRESULT;

/// The registered TypeLib ID is not valid
pub const COMADMIN_E_BADREGISTRYLIBID: HRESULT = 0x8011041Eu32 as HRESULT;

/// Application install directory not found
pub const COMADMIN_E_APPDIRNOTFOUND: HRESULT = 0x8011041Fu32 as HRESULT;

/// Errors occurred while in the component registrar
pub const COMADMIN_E_REGISTRARFAILED: HRESULT = 0x80110423u32 as HRESULT;

/// The file does not exist
pub const COMADMIN_E_COMPFILE_DOESNOTEXIST: HRESULT = 0x80110424u32 as HRESULT;

/// The DLL could not be loaded
pub const COMADMIN_E_COMPFILE_LOADDLLFAIL: HRESULT = 0x80110425u32 as HRESULT;

/// GetClassObject failed in the DLL
pub const COMADMIN_E_COMPFILE_GETCLASSOBJ: HRESULT = 0x80110426u32 as HRESULT;

/// The DLL does not support the components listed in the TypeLib
pub const COMADMIN_E_COMPFILE_CLASSNOTAVAIL: HRESULT = 0x80110427u32 as HRESULT;

/// The TypeLib could not be loaded
pub const COMADMIN_E_COMPFILE_BADTLB: HRESULT = 0x80110428u32 as HRESULT;

/// The file does not contain components or component information
pub const COMADMIN_E_COMPFILE_NOTINSTALLABLE: HRESULT = 0x80110429u32 as HRESULT;

/// Changes to this object and its sub-objects have been disabled
pub const COMADMIN_E_NOTCHANGEABLE: HRESULT = 0x8011042Au32 as HRESULT;

/// The delete function has been disabled for this object
pub const COMADMIN_E_NOTDELETEABLE: HRESULT = 0x8011042Bu32 as HRESULT;

/// The server catalog version is not supported
pub const COMADMIN_E_SESSION: HRESULT = 0x8011042Cu32 as HRESULT;

/// The component move was disallowed, because the source or destination application is either a system application or currently locked against changes
pub const COMADMIN_E_COMP_MOVE_LOCKED: HRESULT = 0x8011042Du32 as HRESULT;

/// The component move failed because the destination application no longer exists
pub const COMADMIN_E_COMP_MOVE_BAD_DEST: HRESULT = 0x8011042Eu32 as HRESULT;

/// The system was unable to register the TypeLib
pub const COMADMIN_E_REGISTERTLB: HRESULT = 0x80110430u32 as HRESULT;

/// This operation cannot be performed on the system application
pub const COMADMIN_E_SYSTEMAPP: HRESULT = 0x80110433u32 as HRESULT;

/// The component registrar referenced in this file is not available
pub const COMADMIN_E_COMPFILE_NOREGISTRAR: HRESULT = 0x80110434u32 as HRESULT;

/// A component in the same DLL is already installed
pub const COMADMIN_E_COREQCOMPINSTALLED: HRESULT = 0x80110435u32 as HRESULT;

/// The service is not installed
pub const COMADMIN_E_SERVICENOTINSTALLED: HRESULT = 0x80110436u32 as HRESULT;

/// One or more property settings are either invalid or in conflict with each other
pub const COMADMIN_E_PROPERTYSAVEFAILED: HRESULT = 0x80110437u32 as HRESULT;

/// The object you are attempting to add or rename already exists
pub const COMADMIN_E_OBJECTEXISTS: HRESULT = 0x80110438u32 as HRESULT;

/// The component already exists
pub const COMADMIN_E_COMPONENTEXISTS: HRESULT = 0x80110439u32 as HRESULT;

/// The registration file is corrupt
pub const COMADMIN_E_REGFILE_CORRUPT: HRESULT = 0x8011043Bu32 as HRESULT;

/// The property value is too large
pub const COMADMIN_E_PROPERTY_OVERFLOW: HRESULT = 0x8011043Cu32 as HRESULT;

/// Object was not found in registry
pub const COMADMIN_E_NOTINREGISTRY: HRESULT = 0x8011043Eu32 as HRESULT;

/// This object is not poolable
pub const COMADMIN_E_OBJECTNOTPOOLABLE: HRESULT = 0x8011043Fu32 as HRESULT;

/// A CLSID with the same GUID as the new application ID is already installed on this machine
pub const COMADMIN_E_APPLID_MATCHES_CLSID: HRESULT = 0x80110446u32 as HRESULT;

/// A role assigned to a component, interface, or method did not exist in the application
pub const COMADMIN_E_ROLE_DOES_NOT_EXIST: HRESULT = 0x80110447u32 as HRESULT;

/// You must have components in an application in order to start the application
pub const COMADMIN_E_START_APP_NEEDS_COMPONENTS: HRESULT = 0x80110448u32 as HRESULT;

/// This operation is not enabled on this platform
pub const COMADMIN_E_REQUIRES_DIFFERENT_PLATFORM: HRESULT = 0x80110449u32 as HRESULT;

/// Application Proxy is not exportable
pub const COMADMIN_E_CAN_NOT_EXPORT_APP_PROXY: HRESULT = 0x8011044Au32 as HRESULT;

/// Failed to start application because it is either a library application or an application proxy
pub const COMADMIN_E_CAN_NOT_START_APP: HRESULT = 0x8011044Bu32 as HRESULT;

/// System application is not exportable
pub const COMADMIN_E_CAN_NOT_EXPORT_SYS_APP: HRESULT = 0x8011044Cu32 as HRESULT;

/// Cannot subscribe to this component (the component may have been imported)
pub const COMADMIN_E_CANT_SUBSCRIBE_TO_COMPONENT: HRESULT = 0x8011044Du32 as HRESULT;

/// An event class cannot also be a subscriber component
pub const COMADMIN_E_EVENTCLASS_CANT_BE_SUBSCRIBER: HRESULT = 0x8011044Eu32 as HRESULT;

/// Library applications and application proxies are incompatible
pub const COMADMIN_E_LIB_APP_PROXY_INCOMPATIBLE: HRESULT = 0x8011044Fu32 as HRESULT;

/// This function is valid for the base partition only
pub const COMADMIN_E_BASE_PARTITION_ONLY: HRESULT = 0x80110450u32 as HRESULT;

/// You cannot start an application that has been disabled
pub const COMADMIN_E_START_APP_DISABLED: HRESULT = 0x80110451u32 as HRESULT;

/// The specified partition name is already in use on this computer
pub const COMADMIN_E_CAT_DUPLICATE_PARTITION_NAME: HRESULT = 0x80110457u32 as HRESULT;

/// The specified partition name is invalid. Check that the name contains at least one visible character
pub const COMADMIN_E_CAT_INVALID_PARTITION_NAME: HRESULT = 0x80110458u32 as HRESULT;

/// The partition cannot be deleted because it is the default partition for one or more users
pub const COMADMIN_E_CAT_PARTITION_IN_USE: HRESULT = 0x80110459u32 as HRESULT;

/// The partition cannot be exported, because one or more components in the partition have the same file name
pub const COMADMIN_E_FILE_PARTITION_DUPLICATE_FILES: HRESULT = 0x8011045Au32 as HRESULT;

/// Applications that contain one or more imported components cannot be installed into a non-base partition
pub const COMADMIN_E_CAT_IMPORTED_COMPONENTS_NOT_ALLOWED: HRESULT = 0x8011045Bu32 as HRESULT;

/// The application name is not unique and cannot be resolved to an application id
pub const COMADMIN_E_AMBIGUOUS_APPLICATION_NAME: HRESULT = 0x8011045Cu32 as HRESULT;

/// The partition name is not unique and cannot be resolved to a partition id
pub const COMADMIN_E_AMBIGUOUS_PARTITION_NAME: HRESULT = 0x8011045Du32 as HRESULT;

/// The COM+ registry database has not been initialized
pub const COMADMIN_E_REGDB_NOTINITIALIZED: HRESULT = 0x80110472u32 as HRESULT;

/// The COM+ registry database is not open
pub const COMADMIN_E_REGDB_NOTOPEN: HRESULT = 0x80110473u32 as HRESULT;

/// The COM+ registry database detected a system error
pub const COMADMIN_E_REGDB_SYSTEMERR: HRESULT = 0x80110474u32 as HRESULT;

/// The COM+ registry database is already running
pub const COMADMIN_E_REGDB_ALREADYRUNNING: HRESULT = 0x80110475u32 as HRESULT;

/// This version of the COM+ registry database cannot be migrated
pub const COMADMIN_E_MIG_VERSIONNOTSUPPORTED: HRESULT = 0x80110480u32 as HRESULT;

/// The schema version to be migrated could not be found in the COM+ registry database
pub const COMADMIN_E_MIG_SCHEMANOTFOUND: HRESULT = 0x80110481u32 as HRESULT;

/// There was a type mismatch between binaries
pub const COMADMIN_E_CAT_BITNESSMISMATCH: HRESULT = 0x80110482u32 as HRESULT;

/// A binary of unknown or invalid type was provided
pub const COMADMIN_E_CAT_UNACCEPTABLEBITNESS: HRESULT = 0x80110483u32 as HRESULT;

/// There was a type mismatch between a binary and an application
pub const COMADMIN_E_CAT_WRONGAPPBITNESS: HRESULT = 0x80110484u32 as HRESULT;

/// The application cannot be paused or resumed
pub const COMADMIN_E_CAT_PAUSE_RESUME_NOT_SUPPORTED: HRESULT = 0x80110485u32 as HRESULT;

/// The COM+ Catalog Server threw an exception during execution
pub const COMADMIN_E_CAT_SERVERFAULT: HRESULT = 0x80110486u32 as HRESULT;

/// Only COM+ Applications marked "queued" can be invoked using the "queue" moniker
pub const COMQC_E_APPLICATION_NOT_QUEUED: HRESULT = 0x80110600u32 as HRESULT;

/// At least one interface must be marked "queued" in order to create a queued component instance with the "queue" moniker
pub const COMQC_E_NO_QUEUEABLE_INTERFACES: HRESULT = 0x80110601u32 as HRESULT;

/// MSMQ is required for the requested operation and is not installed
pub const COMQC_E_QUEUING_SERVICE_NOT_AVAILABLE: HRESULT = 0x80110602u32 as HRESULT;

/// Unable to marshal an interface that does not support IPersistStream
pub const COMQC_E_NO_IPERSISTSTREAM: HRESULT = 0x80110603u32 as HRESULT;

/// The message is improperly formatted or was damaged in transit
pub const COMQC_E_BAD_MESSAGE: HRESULT = 0x80110604u32 as HRESULT;

/// An unauthenticated message was received by an application that accepts only authenticated messages
pub const COMQC_E_UNAUTHENTICATED: HRESULT = 0x80110605u32 as HRESULT;

/// The message was requeued or moved by a user not in the "QC Trusted User" role
pub const COMQC_E_UNTRUSTED_ENQUEUER: HRESULT = 0x80110606u32 as HRESULT;

/// Cannot create a duplicate resource of type Distributed Transaction Coordinator
pub const MSDTC_E_DUPLICATE_RESOURCE: HRESULT = 0x80110701u32 as HRESULT;

/// One of the objects being inserted or updated does not belong to a valid parent collection
pub const COMADMIN_E_OBJECT_PARENT_MISSING: HRESULT = 0x80110808u32 as HRESULT;

/// One of the specified objects cannot be found
pub const COMADMIN_E_OBJECT_DOES_NOT_EXIST: HRESULT = 0x80110809u32 as HRESULT;

/// The specified application is not currently running
pub const COMADMIN_E_APP_NOT_RUNNING: HRESULT = 0x8011080Au32 as HRESULT;

/// The partition(s) specified are not valid.
pub const COMADMIN_E_INVALID_PARTITION: HRESULT = 0x8011080Bu32 as HRESULT;

/// COM+ applications that run as NT service may not be pooled or recycled
pub const COMADMIN_E_SVCAPP_NOT_POOLABLE_OR_RECYCLABLE: HRESULT = 0x8011080Du32 as HRESULT;

/// One or more users are already assigned to a local partition set.
pub const COMADMIN_E_USER_IN_SET: HRESULT = 0x8011080Eu32 as HRESULT;

/// Library applications may not be recycled.
pub const COMADMIN_E_CANTRECYCLELIBRARYAPPS: HRESULT = 0x8011080Fu32 as HRESULT;

/// Applications running as NT services may not be recycled.
pub const COMADMIN_E_CANTRECYCLESERVICEAPPS: HRESULT = 0x80110811u32 as HRESULT;

/// The process has already been recycled.
pub const COMADMIN_E_PROCESSALREADYRECYCLED: HRESULT = 0x80110812u32 as HRESULT;

/// A paused process may not be recycled.
pub const COMADMIN_E_PAUSEDPROCESSMAYNOTBERECYCLED: HRESULT = 0x80110813u32 as HRESULT;

/// Library applications may not be NT services.
pub const COMADMIN_E_CANTMAKEINPROCSERVICE: HRESULT = 0x80110814u32 as HRESULT;

/// The ProgID provided to the copy operation is invalid. The ProgID is in use by another registered CLSID.
pub const COMADMIN_E_PROGIDINUSEBYCLSID: HRESULT = 0x80110815u32 as HRESULT;

/// The partition specified as default is not a member of the partition set.
pub const COMADMIN_E_DEFAULT_PARTITION_NOT_IN_SET: HRESULT = 0x80110816u32 as HRESULT;

/// A recycled process may not be paused.
pub const COMADMIN_E_RECYCLEDPROCESSMAYNOTBEPAUSED: HRESULT = 0x80110817u32 as HRESULT;

/// Access to the specified partition is denied.
pub const COMADMIN_E_PARTITION_ACCESSDENIED: HRESULT = 0x80110818u32 as HRESULT;

/// Only Application Files (*.MSI files) can be installed into partitions.
pub const COMADMIN_E_PARTITION_MSI_ONLY: HRESULT = 0x80110819u32 as HRESULT;

/// Applications containing one or more legacy components may not be exported to 1.0 format.
pub const COMADMIN_E_LEGACYCOMPS_NOT_ALLOWED_IN_1_0_FORMAT: HRESULT = 0x8011081Au32 as HRESULT;

/// Legacy components may not exist in non-base partitions.
pub const COMADMIN_E_LEGACYCOMPS_NOT_ALLOWED_IN_NONBASE_PARTITIONS: HRESULT =
    0x8011081Bu32 as HRESULT;

/// A component cannot be moved (or copied) from the System Application, an application proxy or a non-changeable application
pub const COMADMIN_E_COMP_MOVE_SOURCE: HRESULT = 0x8011081Cu32 as HRESULT;

/// A component cannot be moved (or copied) to the System Application, an application proxy or a non-changeable application
pub const COMADMIN_E_COMP_MOVE_DEST: HRESULT = 0x8011081Du32 as HRESULT;

/// A private component cannot be moved (or copied) to a library application or to the base partition
pub const COMADMIN_E_COMP_MOVE_PRIVATE: HRESULT = 0x8011081Eu32 as HRESULT;

/// The Base Application Partition exists in all partition sets and cannot be removed.
pub const COMADMIN_E_BASEPARTITION_REQUIRED_IN_SET: HRESULT = 0x8011081Fu32 as HRESULT;

/// Alas, Event Class components cannot be aliased.
pub const COMADMIN_E_CANNOT_ALIAS_EVENTCLASS: HRESULT = 0x80110820u32 as HRESULT;

/// Access is denied because the component is private.
pub const COMADMIN_E_PRIVATE_ACCESSDENIED: HRESULT = 0x80110821u32 as HRESULT;

/// The specified SAFER level is invalid.
pub const COMADMIN_E_SAFERINVALID: HRESULT = 0x80110822u32 as HRESULT;

/// The specified user cannot write to the system registry
pub const COMADMIN_E_REGISTRY_ACCESSDENIED: HRESULT = 0x80110823u32 as HRESULT;

/// COM+ partitions are currently disabled.
pub const COMADMIN_E_PARTITIONS_DISABLED: HRESULT = 0x80110824u32 as HRESULT;

/// Invalid message from the Mobile Device Management (MDM) server.
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: HRESULT = 0x80180001u32 as HRESULT;

/// The Mobile Device Management (MDM) server failed to authenticate the user. Try again or contact your system administrator.
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: HRESULT = 0x80180002u32 as HRESULT;

/// The user is not authorized to enroll to Mobile Device Management (MDM). Try again or contact your system administrator.
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: HRESULT = 0x80180003u32 as HRESULT;

/// The user has no permission for the certificate template or the certificate authority is unreachable. Try again or contact your system administrator.
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: HRESULT = 0x80180004u32 as HRESULT;

/// The Mobile Device Management (MDM) server encountered an error. Try again or contact your system administrator.
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: HRESULT = 0x80180005u32 as HRESULT;

/// There was an unhandled exception on the Mobile Device Management (MDM) server. Try again or contact your system administrator.
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: HRESULT = 0x80180006u32 as HRESULT;

/// The Mobile Device Management (MDM) server was not able to validate your account. Try again or contact your system administrator.
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: HRESULT = 0x80180007u32 as HRESULT;

/// Unknown Mobile Device Management (MDM) server error. Try again or contact your system administrator.
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: HRESULT = 0x80180008u32 as HRESULT;

/// Another enrollment operation is currently in progress.
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: HRESULT = 0x80180009u32 as HRESULT;

/// The device is already enrolled.
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: HRESULT = 0x8018000Au32 as HRESULT;

/// There was a Mobile Device Management (MDM) certificate error. The security certificate date is invalid. Try again or contact your system administrator.
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: HRESULT = 0x8018000Du32 as HRESULT;

/// A password is required to authenticate your account or device, but was not supplied. Please try again.
pub const MENROLL_E_PASSWORD_NEEDED: HRESULT = 0x8018000Eu32 as HRESULT;

/// There was a problem authenticating your account or device. Try again or contact your system administrator.
pub const MENROLL_E_WAB_ERROR: HRESULT = 0x8018000Fu32 as HRESULT;

/// There was a network error communicating with the Mobile Device Management (MDM) server. Please try again.
pub const MENROLL_E_CONNECTIVITY: HRESULT = 0x80180010u32 as HRESULT;

/// Enrollment was suspended.
pub const MENROLL_S_ENROLLMENT_SUSPENDED: HRESULT = 0x00180011;

/// There was a Mobile Device Management (MDM) certificate error. Try again or contact your system administrator.
pub const MENROLL_E_INVALIDSSLCERT: HRESULT = 0x80180012u32 as HRESULT;

/// The account has too many devices enrolled to Mobile Device Management (MDM). Delete or unenroll old devices to fix this error.
pub const MENROLL_E_DEVICECAPREACHED: HRESULT = 0x80180013u32 as HRESULT;

/// The Mobile Device Management (MDM) server doesn't support this platform or version, consider upgrading your device.
pub const MENROLL_E_DEVICENOTSUPPORTED: HRESULT = 0x80180014u32 as HRESULT;

/// Mobile Device Management (MDM) is generally not supported for this device.
pub const MENROLL_E_NOT_SUPPORTED: HRESULT = 0x80180015u32 as HRESULT;

/// The device is attempting to renew the Mobile Device Management (MDM) certificate, but the server rejected the request. Check renew schedule on the device.
pub const MENROLL_E_NOTELIGIBLETORENEW: HRESULT = 0x80180016u32 as HRESULT;

/// The Mobile Device Management (MDM) server states your account is in maintenance, try again later.
pub const MENROLL_E_INMAINTENANCE: HRESULT = 0x80180017u32 as HRESULT;

/// There was an error with your Mobile Device Management (MDM) user license. Contact your system administrator.
pub const MENROLL_E_USER_LICENSE: HRESULT = 0x80180018u32 as HRESULT;

/// The Mobile Device Management (MDM) server rejected the enrollment data. The server may not be configured correctly.
pub const MENROLL_E_ENROLLMENTDATAINVALID: HRESULT = 0x80180019u32 as HRESULT;

/// The server requested HTTP rather than HTTPS but it was not accepted.
pub const MENROLL_E_INSECUREREDIRECT: HRESULT = 0x8018001Au32 as HRESULT;

/// An invalid operation was attempted, such as trying to enroll the same device twice or unenroll an unknown device.
pub const MENROLL_E_PLATFORM_WRONG_STATE: HRESULT = 0x8018001Bu32 as HRESULT;

/// The version of Windows installed on the client does not support this enrollment type.
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: HRESULT = 0x8018001Cu32 as HRESULT;

/// An error occurred on the client.
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: HRESULT = 0x8018001Du32 as HRESULT;

/// Provisioning failed in the certificate store CSP.
pub const MENROLL_E_PROV_CSP_CERTSTORE: HRESULT = 0x8018001Eu32 as HRESULT;

/// Provisioning failed in a W7/DMAcc CSP.
pub const MENROLL_E_PROV_CSP_W7: HRESULT = 0x8018001Fu32 as HRESULT;

/// Provisioning failed in the DM client CSP.
pub const MENROLL_E_PROV_CSP_DMCLIENT: HRESULT = 0x80180020u32 as HRESULT;

/// Provisioning failed in the Passport for Work CSP.
pub const MENROLL_E_PROV_CSP_PFW: HRESULT = 0x80180021u32 as HRESULT;

/// Provisioning failed in an unspecified CSP.
pub const MENROLL_E_PROV_CSP_MISC: HRESULT = 0x80180022u32 as HRESULT;

/// Provisioning failed, but a specific CSP is not indicated.
pub const MENROLL_E_PROV_UNKNOWN: HRESULT = 0x80180023u32 as HRESULT;

/// When attempting to bind the public cert/private key, the public cert was not found either: when attempting to bind the public cert/private key, or when looking into provisioning payload.
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: HRESULT = 0x80180024u32 as HRESULT;

/// Provisioning failed in the EnterpriseAppManagement CSP.
pub const MENROLL_E_PROV_CSP_APPMGMT: HRESULT = 0x80180025u32 as HRESULT;

/// Mobile Device Management (MDM) was blocked, possibly by Group Policy or the local management agent.
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: HRESULT = 0x80180026u32 as HRESULT;

/// Failed to create the private key.
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: HRESULT = 0x80180027u32 as HRESULT;

/// Certificate Authentication was requested, but failed to find a certificate to use.
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: HRESULT = 0x80180028u32 as HRESULT;

/// The server responded with HTTP 200, but the message was empty.
pub const MENROLL_E_EMPTY_MESSAGE: HRESULT = 0x80180029u32 as HRESULT;

/// The user canceled the operation.
pub const MENROLL_E_USER_CANCELLED: HRESULT = 0x80180030u32 as HRESULT;

/// Mobile Device Management (MDM) is not configured.
pub const MENROLL_E_MDM_NOT_CONFIGURED: HRESULT = 0x80180031u32 as HRESULT;

/// The server responded with a custom error string, see DeviceManagement-Enterprise-Diagnostics for details.
pub const MENROLL_E_CUSTOMSERVERERROR: HRESULT = 0x80180032u32 as HRESULT;

/// Debugger was attached.
pub const WER_S_REPORT_DEBUG: HRESULT = 0x001B0000;

/// Report was uploaded.
pub const WER_S_REPORT_UPLOADED: HRESULT = 0x001B0001;

/// Report was queued.
pub const WER_S_REPORT_QUEUED: HRESULT = 0x001B0002;

/// Reporting was disabled.
pub const WER_S_DISABLED: HRESULT = 0x001B0003;

/// Reporting was temporarily suspended.
pub const WER_S_SUSPENDED_UPLOAD: HRESULT = 0x001B0004;

/// Report was not queued to queuing being disabled.
pub const WER_S_DISABLED_QUEUE: HRESULT = 0x001B0005;

/// Report was uploaded, but not archived due to archiving being disabled.
pub const WER_S_DISABLED_ARCHIVE: HRESULT = 0x001B0006;

/// Reporting was successfully spun off as an asynchronous operation.
pub const WER_S_REPORT_ASYNC: HRESULT = 0x001B0007;

/// The assertion was handled.
pub const WER_S_IGNORE_ASSERT_INSTANCE: HRESULT = 0x001B0008;

/// The assertion was handled and added to a permanent ignore list.
pub const WER_S_IGNORE_ALL_ASSERTS: HRESULT = 0x001B0009;

/// The assertion was resumed as unhandled.
pub const WER_S_ASSERT_CONTINUE: HRESULT = 0x001B000A;

/// Report was throttled.
pub const WER_S_THROTTLED: HRESULT = 0x001B000B;

/// Report was uploaded with cab.
pub const WER_S_REPORT_UPLOADED_CAB: HRESULT = 0x001B000C;

/// Crash reporting failed.
pub const WER_E_CRASH_FAILURE: HRESULT = 0x801B8000u32 as HRESULT;

/// Report aborted due to user cancellation.
pub const WER_E_CANCELED: HRESULT = 0x801B8001u32 as HRESULT;

/// Report aborted due to network failure.
pub const WER_E_NETWORK_FAILURE: HRESULT = 0x801B8002u32 as HRESULT;

/// Report not initialized.
pub const WER_E_NOT_INITIALIZED: HRESULT = 0x801B8003u32 as HRESULT;

/// Reporting is already in progress for the specified process.
pub const WER_E_ALREADY_REPORTING: HRESULT = 0x801B8004u32 as HRESULT;

/// Dump not generated due to a throttle.
pub const WER_E_DUMP_THROTTLED: HRESULT = 0x801B8005u32 as HRESULT;

/// Operation failed due to insufficient user consent.
pub const WER_E_INSUFFICIENT_CONSENT: HRESULT = 0x801B8006u32 as HRESULT;

/// Report aborted due to performance criteria.
pub const WER_E_TOO_HEAVY: HRESULT = 0x801B8007u32 as HRESULT;

/// The IO was completed by a filter.
pub const ERROR_FLT_IO_COMPLETE: HRESULT = 0x001F0001;

/// A handler was not defined by the filter for this operation.
pub const ERROR_FLT_NO_HANDLER_DEFINED: HRESULT = 0x801F0001u32 as HRESULT;

/// A context is already defined for this object.
pub const ERROR_FLT_CONTEXT_ALREADY_DEFINED: HRESULT = 0x801F0002u32 as HRESULT;

/// Asynchronous requests are not valid for this operation.
pub const ERROR_FLT_INVALID_ASYNCHRONOUS_REQUEST: HRESULT = 0x801F0003u32 as HRESULT;

/// Disallow the Fast IO path for this operation.
pub const ERROR_FLT_DISALLOW_FAST_IO: HRESULT = 0x801F0004u32 as HRESULT;

/// An invalid name request was made. The name requested cannot be retrieved at this time.
pub const ERROR_FLT_INVALID_NAME_REQUEST: HRESULT = 0x801F0005u32 as HRESULT;

/// Posting this operation to a worker thread for further processing is not safe at this time because it could lead to a system deadlock.
pub const ERROR_FLT_NOT_SAFE_TO_POST_OPERATION: HRESULT = 0x801F0006u32 as HRESULT;

/// The Filter Manager was not initialized when a filter tried to register. Make sure that the Filter Manager is getting loaded as a driver.
pub const ERROR_FLT_NOT_INITIALIZED: HRESULT = 0x801F0007u32 as HRESULT;

/// The filter is not ready for attachment to volumes because it has not finished initializing (FltStartFiltering has not been called).
pub const ERROR_FLT_FILTER_NOT_READY: HRESULT = 0x801F0008u32 as HRESULT;

/// The filter must cleanup any operation specific context at this time because it is being removed from the system before the operation is completed by the lower drivers.
pub const ERROR_FLT_POST_OPERATION_CLEANUP: HRESULT = 0x801F0009u32 as HRESULT;

/// The Filter Manager had an internal error from which it cannot recover, therefore the operation has been failed. This is usually the result of a filter returning an invalid value from a pre-operation callback.
pub const ERROR_FLT_INTERNAL_ERROR: HRESULT = 0x801F000Au32 as HRESULT;

/// The object specified for this action is in the process of being deleted, therefore the action requested cannot be completed at this time.
pub const ERROR_FLT_DELETING_OBJECT: HRESULT = 0x801F000Bu32 as HRESULT;

/// Non-paged pool must be used for this type of context.
pub const ERROR_FLT_MUST_BE_NONPAGED_POOL: HRESULT = 0x801F000Cu32 as HRESULT;

/// A duplicate handler definition has been provided for an operation.
pub const ERROR_FLT_DUPLICATE_ENTRY: HRESULT = 0x801F000Du32 as HRESULT;

/// The callback data queue has been disabled.
pub const ERROR_FLT_CBDQ_DISABLED: HRESULT = 0x801F000Eu32 as HRESULT;

/// Do not attach the filter to the volume at this time.
pub const ERROR_FLT_DO_NOT_ATTACH: HRESULT = 0x801F000Fu32 as HRESULT;

/// Do not detach the filter from the volume at this time.
pub const ERROR_FLT_DO_NOT_DETACH: HRESULT = 0x801F0010u32 as HRESULT;

/// An instance already exists at this altitude on the volume specified.
pub const ERROR_FLT_INSTANCE_ALTITUDE_COLLISION: HRESULT = 0x801F0011u32 as HRESULT;

/// An instance already exists with this name on the volume specified.
pub const ERROR_FLT_INSTANCE_NAME_COLLISION: HRESULT = 0x801F0012u32 as HRESULT;

/// The system could not find the filter specified.
pub const ERROR_FLT_FILTER_NOT_FOUND: HRESULT = 0x801F0013u32 as HRESULT;

/// The system could not find the volume specified.
pub const ERROR_FLT_VOLUME_NOT_FOUND: HRESULT = 0x801F0014u32 as HRESULT;

/// The system could not find the instance specified.
pub const ERROR_FLT_INSTANCE_NOT_FOUND: HRESULT = 0x801F0015u32 as HRESULT;

/// No registered context allocation definition was found for the given request.
pub const ERROR_FLT_CONTEXT_ALLOCATION_NOT_FOUND: HRESULT = 0x801F0016u32 as HRESULT;

/// An invalid parameter was specified during context registration.
pub const ERROR_FLT_INVALID_CONTEXT_REGISTRATION: HRESULT = 0x801F0017u32 as HRESULT;

/// The name requested was not found in Filter Manager's name cache and could not be retrieved from the file system.
pub const ERROR_FLT_NAME_CACHE_MISS: HRESULT = 0x801F0018u32 as HRESULT;

/// The requested device object does not exist for the given volume.
pub const ERROR_FLT_NO_DEVICE_OBJECT: HRESULT = 0x801F0019u32 as HRESULT;

/// The specified volume is already mounted.
pub const ERROR_FLT_VOLUME_ALREADY_MOUNTED: HRESULT = 0x801F001Au32 as HRESULT;

/// The specified Transaction Context is already enlisted in a transaction
pub const ERROR_FLT_ALREADY_ENLISTED: HRESULT = 0x801F001Bu32 as HRESULT;

/// The specified context is already attached to another object
pub const ERROR_FLT_CONTEXT_ALREADY_LINKED: HRESULT = 0x801F001Cu32 as HRESULT;

/// No waiter is present for the filter's reply to this message.
pub const ERROR_FLT_NO_WAITER_FOR_REPLY: HRESULT = 0x801F0020u32 as HRESULT;

/// The filesystem database resource is in use. Registration cannot complete at this time.
pub const ERROR_FLT_REGISTRATION_BUSY: HRESULT = 0x801F0023u32 as HRESULT;

/// The filter is not allowed to attach because it has not declared compability with WCOS.
pub const ERROR_FLT_WCOS_NOT_SUPPORTED: HRESULT = 0x801F0024u32 as HRESULT;

///  The %hs display driver has stopped working normally. Save your work and reboot the system to restore full display functionality.
///
/// The next time you reboot the machine a dialog will be displayed giving you a chance to report this failure to Microsoft.
pub const ERROR_HUNG_DISPLAY_DRIVER_THREAD: HRESULT = 0x80260001u32 as HRESULT;

/// The operation could not be completed because desktop composition is disabled.
pub const DWM_E_COMPOSITIONDISABLED: HRESULT = 0x80263001u32 as HRESULT;

/// The operation is not supported while running in a remote session.
pub const DWM_E_REMOTING_NOT_SUPPORTED: HRESULT = 0x80263002u32 as HRESULT;

/// The DWM was unable to provide a redirection surface to complete the DirectX present.
pub const DWM_E_NO_REDIRECTION_SURFACE_AVAILABLE: HRESULT = 0x80263003u32 as HRESULT;

/// The window specified is not currently using queued presents.
pub const DWM_E_NOT_QUEUING_PRESENTS: HRESULT = 0x80263004u32 as HRESULT;

/// DWM can not find the adapter specified by the LUID.
pub const DWM_E_ADAPTER_NOT_FOUND: HRESULT = 0x80263005u32 as HRESULT;

/// GDI redirection surface of the top level window was returned.
pub const DWM_S_GDI_REDIRECTION_SURFACE: HRESULT = 0x00263005;

/// Redirection surface can not be created.  The size of the surface is larger than what is supported on this machine.
pub const DWM_E_TEXTURE_TOO_LARGE: HRESULT = 0x80263007u32 as HRESULT;

/// GDI redirection surface is either on a different adapter or in system memory. Perform blt via GDI.
pub const DWM_S_GDI_REDIRECTION_SURFACE_BLT_VIA_GDI: HRESULT = 0x00263008;

/// Monitor descriptor could not be obtained.
pub const ERROR_MONITOR_NO_DESCRIPTOR: HRESULT = 0x00261001;

/// Format of the obtained monitor descriptor is not supported by this release.
pub const ERROR_MONITOR_UNKNOWN_DESCRIPTOR_FORMAT: HRESULT = 0x00261002;

/// Checksum of the obtained monitor descriptor is invalid.
pub const ERROR_MONITOR_INVALID_DESCRIPTOR_CHECKSUM: HRESULT = 0xC0261003u32 as HRESULT;

/// Monitor descriptor contains an invalid standard timing block.
pub const ERROR_MONITOR_INVALID_STANDARD_TIMING_BLOCK: HRESULT = 0xC0261004u32 as HRESULT;

/// WMI data block registration failed for one of the MSMonitorClass WMI subclasses.
pub const ERROR_MONITOR_WMI_DATABLOCK_REGISTRATION_FAILED: HRESULT = 0xC0261005u32 as HRESULT;

/// Provided monitor descriptor block is either corrupted or does not contain monitor's detailed serial number.
pub const ERROR_MONITOR_INVALID_SERIAL_NUMBER_MONDSC_BLOCK: HRESULT = 0xC0261006u32 as HRESULT;

/// Provided monitor descriptor block is either corrupted or does not contain monitor's user friendly name.
pub const ERROR_MONITOR_INVALID_USER_FRIENDLY_MONDSC_BLOCK: HRESULT = 0xC0261007u32 as HRESULT;

/// There is no monitor descriptor data at the specified (offset, size) region.
pub const ERROR_MONITOR_NO_MORE_DESCRIPTOR_DATA: HRESULT = 0xC0261008u32 as HRESULT;

/// Monitor descriptor contains an invalid detailed timing block.
pub const ERROR_MONITOR_INVALID_DETAILED_TIMING_BLOCK: HRESULT = 0xC0261009u32 as HRESULT;

/// Monitor descriptor contains invalid manufacture date.
pub const ERROR_MONITOR_INVALID_MANUFACTURE_DATE: HRESULT = 0xC026100Au32 as HRESULT;

/// Exclusive mode ownership is needed to create unmanaged primary allocation.
pub const ERROR_GRAPHICS_NOT_EXCLUSIVE_MODE_OWNER: HRESULT = 0xC0262000u32 as HRESULT;

/// The driver needs more DMA buffer space in order to complete the requested operation.
pub const ERROR_GRAPHICS_INSUFFICIENT_DMA_BUFFER: HRESULT = 0xC0262001u32 as HRESULT;

/// Specified display adapter handle is invalid.
pub const ERROR_GRAPHICS_INVALID_DISPLAY_ADAPTER: HRESULT = 0xC0262002u32 as HRESULT;

/// Specified display adapter and all of its state has been reset.
pub const ERROR_GRAPHICS_ADAPTER_WAS_RESET: HRESULT = 0xC0262003u32 as HRESULT;

/// The driver stack doesn't match the expected driver model.
pub const ERROR_GRAPHICS_INVALID_DRIVER_MODEL: HRESULT = 0xC0262004u32 as HRESULT;

/// Present happened but ended up into the changed desktop mode
pub const ERROR_GRAPHICS_PRESENT_MODE_CHANGED: HRESULT = 0xC0262005u32 as HRESULT;

/// Nothing to present due to desktop occlusion
pub const ERROR_GRAPHICS_PRESENT_OCCLUDED: HRESULT = 0xC0262006u32 as HRESULT;

/// Not able to present due to denial of desktop access
pub const ERROR_GRAPHICS_PRESENT_DENIED: HRESULT = 0xC0262007u32 as HRESULT;

/// Not able to present with color conversion
pub const ERROR_GRAPHICS_CANNOTCOLORCONVERT: HRESULT = 0xC0262008u32 as HRESULT;

/// The kernel driver detected a version mismatch between it and the user mode driver.
pub const ERROR_GRAPHICS_DRIVER_MISMATCH: HRESULT = 0xC0262009u32 as HRESULT;

/// Specified buffer is not big enough to contain entire requested dataset. Partial data populated up to the size of the buffer. Caller needs to provide buffer of size as specified in the partially populated buffer's content (interface specific).
pub const ERROR_GRAPHICS_PARTIAL_DATA_POPULATED: HRESULT = 0x4026200A;

/// Present redirection is disabled (desktop windowing management subsystem is off).
pub const ERROR_GRAPHICS_PRESENT_REDIRECTION_DISABLED: HRESULT = 0xC026200Bu32 as HRESULT;

/// Previous exclusive VidPn source owner has released its ownership
pub const ERROR_GRAPHICS_PRESENT_UNOCCLUDED: HRESULT = 0xC026200Cu32 as HRESULT;

/// Window DC is not available for presentation
pub const ERROR_GRAPHICS_WINDOWDC_NOT_AVAILABLE: HRESULT = 0xC026200Du32 as HRESULT;

/// Windowless present is disabled (desktop windowing management subsystem is off).
pub const ERROR_GRAPHICS_WINDOWLESS_PRESENT_DISABLED: HRESULT = 0xC026200Eu32 as HRESULT;

/// Window handle is invalid
pub const ERROR_GRAPHICS_PRESENT_INVALID_WINDOW: HRESULT = 0xC026200Fu32 as HRESULT;

/// No buffer is bound to composition surface
pub const ERROR_GRAPHICS_PRESENT_BUFFER_NOT_BOUND: HRESULT = 0xC0262010u32 as HRESULT;

/// Vail state has been changed
pub const ERROR_GRAPHICS_VAIL_STATE_CHANGED: HRESULT = 0xC0262011u32 as HRESULT;

/// Notifying indirect display UMDF class driver to abandon current swapchain.
pub const ERROR_GRAPHICS_INDIRECT_DISPLAY_ABANDON_SWAPCHAIN: HRESULT = 0xC0262012u32 as HRESULT;

/// Notifying indirect display UMDF class driver that indirect display device has been stopped.
pub const ERROR_GRAPHICS_INDIRECT_DISPLAY_DEVICE_STOPPED: HRESULT = 0xC0262013u32 as HRESULT;

/// Failed to send Create Vail Super Wet Ink message.
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_CREATE_SUPERWETINK_MESSAGE: HRESULT =
    0xC0262014u32 as HRESULT;

/// Failed to send Destroy Vail Super Wet Ink message.
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_DESTROY_SUPERWETINK_MESSAGE: HRESULT =
    0xC0262015u32 as HRESULT;

/// Failed to send Window Dpi message.
pub const ERROR_GRAPHICS_VAIL_FAILED_TO_SEND_COMPOSITION_WINDOW_DPI_MESSAGE: HRESULT =
    0xC0262016u32 as HRESULT;

/// Link configuration for the display timing is still in progress.
pub const ERROR_GRAPHICS_LINK_CONFIGURATION_IN_PROGRESS: HRESULT = 0xC0262017u32 as HRESULT;

/// The allocation for the MPO has been unpinned.
pub const ERROR_GRAPHICS_MPO_ALLOCATION_UNPINNED: HRESULT = 0xC0262018u32 as HRESULT;

/// Not enough video memory available to complete the operation.
pub const ERROR_GRAPHICS_NO_VIDEO_MEMORY: HRESULT = 0xC0262100u32 as HRESULT;

/// Couldn't probe and lock the underlying memory of an allocation.
pub const ERROR_GRAPHICS_CANT_LOCK_MEMORY: HRESULT = 0xC0262101u32 as HRESULT;

/// The allocation is currently busy.
pub const ERROR_GRAPHICS_ALLOCATION_BUSY: HRESULT = 0xC0262102u32 as HRESULT;

/// An object being referenced has reach the maximum reference count already and can't be reference further.
pub const ERROR_GRAPHICS_TOO_MANY_REFERENCES: HRESULT = 0xC0262103u32 as HRESULT;

/// A problem couldn't be solved due to some currently existing condition. The problem should be tried again later.
pub const ERROR_GRAPHICS_TRY_AGAIN_LATER: HRESULT = 0xC0262104u32 as HRESULT;

/// A problem couldn't be solved due to some currently existing condition. The problem should be tried again immediately.
pub const ERROR_GRAPHICS_TRY_AGAIN_NOW: HRESULT = 0xC0262105u32 as HRESULT;

/// The allocation is invalid.
pub const ERROR_GRAPHICS_ALLOCATION_INVALID: HRESULT = 0xC0262106u32 as HRESULT;

/// No more unswizzling aperture are currently available.
pub const ERROR_GRAPHICS_UNSWIZZLING_APERTURE_UNAVAILABLE: HRESULT = 0xC0262107u32 as HRESULT;

/// The current allocation can't be unswizzled by an aperture.
pub const ERROR_GRAPHICS_UNSWIZZLING_APERTURE_UNSUPPORTED: HRESULT = 0xC0262108u32 as HRESULT;

/// The request failed because a pinned allocation can't be evicted.
pub const ERROR_GRAPHICS_CANT_EVICT_PINNED_ALLOCATION: HRESULT = 0xC0262109u32 as HRESULT;

/// The allocation can't be used from its current segment location for the specified operation.
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_USAGE: HRESULT = 0xC0262110u32 as HRESULT;

/// A locked allocation can't be used in the current command buffer.
pub const ERROR_GRAPHICS_CANT_RENDER_LOCKED_ALLOCATION: HRESULT = 0xC0262111u32 as HRESULT;

/// The allocation being referenced has been closed permanently.
pub const ERROR_GRAPHICS_ALLOCATION_CLOSED: HRESULT = 0xC0262112u32 as HRESULT;

/// An invalid allocation instance is being referenced.
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_INSTANCE: HRESULT = 0xC0262113u32 as HRESULT;

/// An invalid allocation handle is being referenced.
pub const ERROR_GRAPHICS_INVALID_ALLOCATION_HANDLE: HRESULT = 0xC0262114u32 as HRESULT;

/// The allocation being referenced doesn't belong to the current device.
pub const ERROR_GRAPHICS_WRONG_ALLOCATION_DEVICE: HRESULT = 0xC0262115u32 as HRESULT;

/// The specified allocation lost its content.
pub const ERROR_GRAPHICS_ALLOCATION_CONTENT_LOST: HRESULT = 0xC0262116u32 as HRESULT;

/// GPU exception is detected on the given device. The device is not able to be scheduled.
pub const ERROR_GRAPHICS_GPU_EXCEPTION_ON_DEVICE: HRESULT = 0xC0262200u32 as HRESULT;

/// Skip preparation of allocations referenced by the DMA buffer.
pub const ERROR_GRAPHICS_SKIP_ALLOCATION_PREPARATION: HRESULT = 0x40262201;

/// Specified VidPN topology is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDPN_TOPOLOGY: HRESULT = 0xC0262300u32 as HRESULT;

/// Specified VidPN topology is valid but is not supported by this model of the display adapter.
pub const ERROR_GRAPHICS_VIDPN_TOPOLOGY_NOT_SUPPORTED: HRESULT = 0xC0262301u32 as HRESULT;

/// Specified VidPN topology is valid but is not supported by the display adapter at this time, due to current allocation of its resources.
pub const ERROR_GRAPHICS_VIDPN_TOPOLOGY_CURRENTLY_NOT_SUPPORTED: HRESULT = 0xC0262302u32 as HRESULT;

/// Specified VidPN handle is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDPN: HRESULT = 0xC0262303u32 as HRESULT;

/// Specified video present source is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE: HRESULT = 0xC0262304u32 as HRESULT;

/// Specified video present target is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET: HRESULT = 0xC0262305u32 as HRESULT;

/// Specified VidPN modality is not supported (e.g. at least two of the pinned modes are not cofunctional).
pub const ERROR_GRAPHICS_VIDPN_MODALITY_NOT_SUPPORTED: HRESULT = 0xC0262306u32 as HRESULT;

/// No mode is pinned on the specified VidPN source/target.
pub const ERROR_GRAPHICS_MODE_NOT_PINNED: HRESULT = 0x00262307;

/// Specified VidPN source mode set is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDPN_SOURCEMODESET: HRESULT = 0xC0262308u32 as HRESULT;

/// Specified VidPN target mode set is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDPN_TARGETMODESET: HRESULT = 0xC0262309u32 as HRESULT;

/// Specified video signal frequency is invalid.
pub const ERROR_GRAPHICS_INVALID_FREQUENCY: HRESULT = 0xC026230Au32 as HRESULT;

/// Specified video signal active region is invalid.
pub const ERROR_GRAPHICS_INVALID_ACTIVE_REGION: HRESULT = 0xC026230Bu32 as HRESULT;

/// Specified video signal total region is invalid.
pub const ERROR_GRAPHICS_INVALID_TOTAL_REGION: HRESULT = 0xC026230Cu32 as HRESULT;

/// Specified video present source mode is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_SOURCE_MODE: HRESULT = 0xC0262310u32 as HRESULT;

/// Specified video present target mode is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDEO_PRESENT_TARGET_MODE: HRESULT = 0xC0262311u32 as HRESULT;

/// Pinned mode must remain in the set on VidPN's cofunctional modality enumeration.
pub const ERROR_GRAPHICS_PINNED_MODE_MUST_REMAIN_IN_SET: HRESULT = 0xC0262312u32 as HRESULT;

/// Specified video present path is already in VidPN's topology.
pub const ERROR_GRAPHICS_PATH_ALREADY_IN_TOPOLOGY: HRESULT = 0xC0262313u32 as HRESULT;

/// Specified mode is already in the mode set.
pub const ERROR_GRAPHICS_MODE_ALREADY_IN_MODESET: HRESULT = 0xC0262314u32 as HRESULT;

/// Specified video present source set is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDEOPRESENTSOURCESET: HRESULT = 0xC0262315u32 as HRESULT;

/// Specified video present target set is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDEOPRESENTTARGETSET: HRESULT = 0xC0262316u32 as HRESULT;

/// Specified video present source is already in the video present source set.
pub const ERROR_GRAPHICS_SOURCE_ALREADY_IN_SET: HRESULT = 0xC0262317u32 as HRESULT;

/// Specified video present target is already in the video present target set.
pub const ERROR_GRAPHICS_TARGET_ALREADY_IN_SET: HRESULT = 0xC0262318u32 as HRESULT;

/// Specified VidPN present path is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDPN_PRESENT_PATH: HRESULT = 0xC0262319u32 as HRESULT;

/// Miniport has no recommendation for augmentation of the specified VidPN's topology.
pub const ERROR_GRAPHICS_NO_RECOMMENDED_VIDPN_TOPOLOGY: HRESULT = 0xC026231Au32 as HRESULT;

/// Specified monitor frequency range set is invalid.
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGESET: HRESULT = 0xC026231Bu32 as HRESULT;

/// Specified monitor frequency range is invalid.
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE: HRESULT = 0xC026231Cu32 as HRESULT;

/// Specified frequency range is not in the specified monitor frequency range set.
pub const ERROR_GRAPHICS_FREQUENCYRANGE_NOT_IN_SET: HRESULT = 0xC026231Du32 as HRESULT;

/// Specified mode set does not specify preference for one of its modes.
pub const ERROR_GRAPHICS_NO_PREFERRED_MODE: HRESULT = 0x0026231E;

/// Specified frequency range is already in the specified monitor frequency range set.
pub const ERROR_GRAPHICS_FREQUENCYRANGE_ALREADY_IN_SET: HRESULT = 0xC026231Fu32 as HRESULT;

/// Specified mode set is stale. Please reacquire the new mode set.
pub const ERROR_GRAPHICS_STALE_MODESET: HRESULT = 0xC0262320u32 as HRESULT;

/// Specified monitor source mode set is invalid.
pub const ERROR_GRAPHICS_INVALID_MONITOR_SOURCEMODESET: HRESULT = 0xC0262321u32 as HRESULT;

/// Specified monitor source mode is invalid.
pub const ERROR_GRAPHICS_INVALID_MONITOR_SOURCE_MODE: HRESULT = 0xC0262322u32 as HRESULT;

/// Miniport does not have any recommendation regarding the request to provide a functional VidPN given the current display adapter configuration.
pub const ERROR_GRAPHICS_NO_RECOMMENDED_FUNCTIONAL_VIDPN: HRESULT = 0xC0262323u32 as HRESULT;

/// ID of the specified mode is already used by another mode in the set.
pub const ERROR_GRAPHICS_MODE_ID_MUST_BE_UNIQUE: HRESULT = 0xC0262324u32 as HRESULT;

/// System failed to determine a mode that is supported by both the display adapter and the monitor connected to it.
pub const ERROR_GRAPHICS_EMPTY_ADAPTER_MONITOR_MODE_SUPPORT_INTERSECTION: HRESULT =
    0xC0262325u32 as HRESULT;

/// Number of video present targets must be greater than or equal to the number of video present sources.
pub const ERROR_GRAPHICS_VIDEO_PRESENT_TARGETS_LESS_THAN_SOURCES: HRESULT =
    0xC0262326u32 as HRESULT;

/// Specified present path is not in VidPN's topology.
pub const ERROR_GRAPHICS_PATH_NOT_IN_TOPOLOGY: HRESULT = 0xC0262327u32 as HRESULT;

/// Display adapter must have at least one video present source.
pub const ERROR_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_SOURCE: HRESULT = 0xC0262328u32 as HRESULT;

/// Display adapter must have at least one video present target.
pub const ERROR_GRAPHICS_ADAPTER_MUST_HAVE_AT_LEAST_ONE_TARGET: HRESULT = 0xC0262329u32 as HRESULT;

/// Specified monitor descriptor set is invalid.
pub const ERROR_GRAPHICS_INVALID_MONITORDESCRIPTORSET: HRESULT = 0xC026232Au32 as HRESULT;

/// Specified monitor descriptor is invalid.
pub const ERROR_GRAPHICS_INVALID_MONITORDESCRIPTOR: HRESULT = 0xC026232Bu32 as HRESULT;

/// Specified descriptor is not in the specified monitor descriptor set.
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_NOT_IN_SET: HRESULT = 0xC026232Cu32 as HRESULT;

/// Specified descriptor is already in the specified monitor descriptor set.
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_ALREADY_IN_SET: HRESULT = 0xC026232Du32 as HRESULT;

/// ID of the specified monitor descriptor is already used by another descriptor in the set.
pub const ERROR_GRAPHICS_MONITORDESCRIPTOR_ID_MUST_BE_UNIQUE: HRESULT = 0xC026232Eu32 as HRESULT;

/// Specified video present target subset type is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDPN_TARGET_SUBSET_TYPE: HRESULT = 0xC026232Fu32 as HRESULT;

/// Two or more of the specified resources are not related to each other, as defined by the interface semantics.
pub const ERROR_GRAPHICS_RESOURCES_NOT_RELATED: HRESULT = 0xC0262330u32 as HRESULT;

/// ID of the specified video present source is already used by another source in the set.
pub const ERROR_GRAPHICS_SOURCE_ID_MUST_BE_UNIQUE: HRESULT = 0xC0262331u32 as HRESULT;

/// ID of the specified video present target is already used by another target in the set.
pub const ERROR_GRAPHICS_TARGET_ID_MUST_BE_UNIQUE: HRESULT = 0xC0262332u32 as HRESULT;

/// Specified VidPN source cannot be used because there is no available VidPN target to connect it to.
pub const ERROR_GRAPHICS_NO_AVAILABLE_VIDPN_TARGET: HRESULT = 0xC0262333u32 as HRESULT;

/// Newly arrived monitor could not be associated with a display adapter.
pub const ERROR_GRAPHICS_MONITOR_COULD_NOT_BE_ASSOCIATED_WITH_ADAPTER: HRESULT =
    0xC0262334u32 as HRESULT;

/// Display adapter in question does not have an associated VidPN manager.
pub const ERROR_GRAPHICS_NO_VIDPNMGR: HRESULT = 0xC0262335u32 as HRESULT;

/// VidPN manager of the display adapter in question does not have an active VidPN.
pub const ERROR_GRAPHICS_NO_ACTIVE_VIDPN: HRESULT = 0xC0262336u32 as HRESULT;

/// Specified VidPN topology is stale. Please reacquire the new topology.
pub const ERROR_GRAPHICS_STALE_VIDPN_TOPOLOGY: HRESULT = 0xC0262337u32 as HRESULT;

/// There is no monitor connected on the specified video present target.
pub const ERROR_GRAPHICS_MONITOR_NOT_CONNECTED: HRESULT = 0xC0262338u32 as HRESULT;

/// Specified source is not part of the specified VidPN's topology.
pub const ERROR_GRAPHICS_SOURCE_NOT_IN_TOPOLOGY: HRESULT = 0xC0262339u32 as HRESULT;

/// Specified primary surface size is invalid.
pub const ERROR_GRAPHICS_INVALID_PRIMARYSURFACE_SIZE: HRESULT = 0xC026233Au32 as HRESULT;

/// Specified visible region size is invalid.
pub const ERROR_GRAPHICS_INVALID_VISIBLEREGION_SIZE: HRESULT = 0xC026233Bu32 as HRESULT;

/// Specified stride is invalid.
pub const ERROR_GRAPHICS_INVALID_STRIDE: HRESULT = 0xC026233Cu32 as HRESULT;

/// Specified pixel format is invalid.
pub const ERROR_GRAPHICS_INVALID_PIXELFORMAT: HRESULT = 0xC026233Du32 as HRESULT;

/// Specified color basis is invalid.
pub const ERROR_GRAPHICS_INVALID_COLORBASIS: HRESULT = 0xC026233Eu32 as HRESULT;

/// Specified pixel value access mode is invalid.
pub const ERROR_GRAPHICS_INVALID_PIXELVALUEACCESSMODE: HRESULT = 0xC026233Fu32 as HRESULT;

/// Specified target is not part of the specified VidPN's topology.
pub const ERROR_GRAPHICS_TARGET_NOT_IN_TOPOLOGY: HRESULT = 0xC0262340u32 as HRESULT;

/// Failed to acquire display mode management interface.
pub const ERROR_GRAPHICS_NO_DISPLAY_MODE_MANAGEMENT_SUPPORT: HRESULT = 0xC0262341u32 as HRESULT;

/// Specified VidPN source is already owned by a DMM client and cannot be used until that client releases it.
pub const ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: HRESULT = 0xC0262342u32 as HRESULT;

/// Specified VidPN is active and cannot be accessed.
pub const ERROR_GRAPHICS_CANT_ACCESS_ACTIVE_VIDPN: HRESULT = 0xC0262343u32 as HRESULT;

/// Specified VidPN present path importance ordinal is invalid.
pub const ERROR_GRAPHICS_INVALID_PATH_IMPORTANCE_ORDINAL: HRESULT = 0xC0262344u32 as HRESULT;

/// Specified VidPN present path content geometry transformation is invalid.
pub const ERROR_GRAPHICS_INVALID_PATH_CONTENT_GEOMETRY_TRANSFORMATION: HRESULT =
    0xC0262345u32 as HRESULT;

/// Specified content geometry transformation is not supported on the respective VidPN present path.
pub const ERROR_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_SUPPORTED: HRESULT =
    0xC0262346u32 as HRESULT;

/// Specified gamma ramp is invalid.
pub const ERROR_GRAPHICS_INVALID_GAMMA_RAMP: HRESULT = 0xC0262347u32 as HRESULT;

/// Specified gamma ramp is not supported on the respective VidPN present path.
pub const ERROR_GRAPHICS_GAMMA_RAMP_NOT_SUPPORTED: HRESULT = 0xC0262348u32 as HRESULT;

/// Multi-sampling is not supported on the respective VidPN present path.
pub const ERROR_GRAPHICS_MULTISAMPLING_NOT_SUPPORTED: HRESULT = 0xC0262349u32 as HRESULT;

/// Specified mode is not in the specified mode set.
pub const ERROR_GRAPHICS_MODE_NOT_IN_MODESET: HRESULT = 0xC026234Au32 as HRESULT;

/// Specified data set (e.g. mode set, frequency range set, descriptor set, topology, etc.) is empty.
pub const ERROR_GRAPHICS_DATASET_IS_EMPTY: HRESULT = 0x0026234B;

/// Specified data set (e.g. mode set, frequency range set, descriptor set, topology, etc.) does not contain any more elements.
pub const ERROR_GRAPHICS_NO_MORE_ELEMENTS_IN_DATASET: HRESULT = 0x0026234C;

/// Specified VidPN topology recommendation reason is invalid.
pub const ERROR_GRAPHICS_INVALID_VIDPN_TOPOLOGY_RECOMMENDATION_REASON: HRESULT =
    0xC026234Du32 as HRESULT;

/// Specified VidPN present path content type is invalid.
pub const ERROR_GRAPHICS_INVALID_PATH_CONTENT_TYPE: HRESULT = 0xC026234Eu32 as HRESULT;

/// Specified VidPN present path copy protection type is invalid.
pub const ERROR_GRAPHICS_INVALID_COPYPROTECTION_TYPE: HRESULT = 0xC026234Fu32 as HRESULT;

/// No more than one unassigned mode set can exist at any given time for a given VidPN source/target.
pub const ERROR_GRAPHICS_UNASSIGNED_MODESET_ALREADY_EXISTS: HRESULT = 0xC0262350u32 as HRESULT;

/// Specified content transformation is not pinned on the specified VidPN present path.
pub const ERROR_GRAPHICS_PATH_CONTENT_GEOMETRY_TRANSFORMATION_NOT_PINNED: HRESULT = 0x00262351;

/// Specified scanline ordering type is invalid.
pub const ERROR_GRAPHICS_INVALID_SCANLINE_ORDERING: HRESULT = 0xC0262352u32 as HRESULT;

/// Topology changes are not allowed for the specified VidPN.
pub const ERROR_GRAPHICS_TOPOLOGY_CHANGES_NOT_ALLOWED: HRESULT = 0xC0262353u32 as HRESULT;

/// All available importance ordinals are already used in specified topology.
pub const ERROR_GRAPHICS_NO_AVAILABLE_IMPORTANCE_ORDINALS: HRESULT = 0xC0262354u32 as HRESULT;

/// Specified primary surface has a different private format attribute than the current primary surface
pub const ERROR_GRAPHICS_INCOMPATIBLE_PRIVATE_FORMAT: HRESULT = 0xC0262355u32 as HRESULT;

/// Specified mode pruning algorithm is invalid
pub const ERROR_GRAPHICS_INVALID_MODE_PRUNING_ALGORITHM: HRESULT = 0xC0262356u32 as HRESULT;

/// Specified monitor capability origin is invalid.
pub const ERROR_GRAPHICS_INVALID_MONITOR_CAPABILITY_ORIGIN: HRESULT = 0xC0262357u32 as HRESULT;

/// Specified monitor frequency range constraint is invalid.
pub const ERROR_GRAPHICS_INVALID_MONITOR_FREQUENCYRANGE_CONSTRAINT: HRESULT =
    0xC0262358u32 as HRESULT;

/// Maximum supported number of present paths has been reached.
pub const ERROR_GRAPHICS_MAX_NUM_PATHS_REACHED: HRESULT = 0xC0262359u32 as HRESULT;

/// Miniport requested that augmentation be cancelled for the specified source of the specified VidPN's topology.
pub const ERROR_GRAPHICS_CANCEL_VIDPN_TOPOLOGY_AUGMENTATION: HRESULT = 0xC026235Au32 as HRESULT;

/// Specified client type was not recognized.
pub const ERROR_GRAPHICS_INVALID_CLIENT_TYPE: HRESULT = 0xC026235Bu32 as HRESULT;

/// Client VidPN is not set on this adapter (e.g. no user mode initiated mode changes took place on this adapter yet).
pub const ERROR_GRAPHICS_CLIENTVIDPN_NOT_SET: HRESULT = 0xC026235Cu32 as HRESULT;

/// Specified display adapter child device already has an external device connected to it.
pub const ERROR_GRAPHICS_SPECIFIED_CHILD_ALREADY_CONNECTED: HRESULT = 0xC0262400u32 as HRESULT;

/// Specified display adapter child device does not support descriptor exposure.
pub const ERROR_GRAPHICS_CHILD_DESCRIPTOR_NOT_SUPPORTED: HRESULT = 0xC0262401u32 as HRESULT;

/// Child device presence was not reliably detected.
pub const ERROR_GRAPHICS_UNKNOWN_CHILD_STATUS: HRESULT = 0x4026242F;

/// The display adapter is not linked to any other adapters.
pub const ERROR_GRAPHICS_NOT_A_LINKED_ADAPTER: HRESULT = 0xC0262430u32 as HRESULT;

/// Lead adapter in a linked configuration was not enumerated yet.
pub const ERROR_GRAPHICS_LEADLINK_NOT_ENUMERATED: HRESULT = 0xC0262431u32 as HRESULT;

/// Some chain adapters in a linked configuration were not enumerated yet.
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_ENUMERATED: HRESULT = 0xC0262432u32 as HRESULT;

/// The chain of linked adapters is not ready to start because of an unknown failure.
pub const ERROR_GRAPHICS_ADAPTER_CHAIN_NOT_READY: HRESULT = 0xC0262433u32 as HRESULT;

/// An attempt was made to start a lead link display adapter when the chain links were not started yet.
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_STARTED: HRESULT = 0xC0262434u32 as HRESULT;

/// An attempt was made to power up a lead link display adapter when the chain links were powered down.
pub const ERROR_GRAPHICS_CHAINLINKS_NOT_POWERED_ON: HRESULT = 0xC0262435u32 as HRESULT;

/// The adapter link was found to be in an inconsistent state. Not all adapters are in an expected PNP/Power state.
pub const ERROR_GRAPHICS_INCONSISTENT_DEVICE_LINK_STATE: HRESULT = 0xC0262436u32 as HRESULT;

/// Starting the leadlink adapter has been deferred temporarily.
pub const ERROR_GRAPHICS_LEADLINK_START_DEFERRED: HRESULT = 0x40262437;

/// The driver trying to start is not the same as the driver for the POSTed display adapter.
pub const ERROR_GRAPHICS_NOT_POST_DEVICE_DRIVER: HRESULT = 0xC0262438u32 as HRESULT;

/// The display adapter is being polled for children too frequently at the same polling level.
pub const ERROR_GRAPHICS_POLLING_TOO_FREQUENTLY: HRESULT = 0x40262439;

/// Starting the adapter has been deferred temporarily.
pub const ERROR_GRAPHICS_START_DEFERRED: HRESULT = 0x4026243A;

/// An operation is being attempted that requires the display adapter to be in a quiescent state.
pub const ERROR_GRAPHICS_ADAPTER_ACCESS_NOT_EXCLUDED: HRESULT = 0xC026243Bu32 as HRESULT;

/// We can depend on the child device presence returned by the driver.
pub const ERROR_GRAPHICS_DEPENDABLE_CHILD_STATUS: HRESULT = 0x4026243C;

/// The driver does not support OPM.
pub const ERROR_GRAPHICS_OPM_NOT_SUPPORTED: HRESULT = 0xC0262500u32 as HRESULT;

/// The driver does not support COPP.
pub const ERROR_GRAPHICS_COPP_NOT_SUPPORTED: HRESULT = 0xC0262501u32 as HRESULT;

/// The driver does not support UAB.
pub const ERROR_GRAPHICS_UAB_NOT_SUPPORTED: HRESULT = 0xC0262502u32 as HRESULT;

/// The specified encrypted parameters are invalid.
pub const ERROR_GRAPHICS_OPM_INVALID_ENCRYPTED_PARAMETERS: HRESULT = 0xC0262503u32 as HRESULT;

/// The GDI display device passed to this function does not have any active video outputs.
pub const ERROR_GRAPHICS_OPM_NO_VIDEO_OUTPUTS_EXIST: HRESULT = 0xC0262505u32 as HRESULT;

/// An internal error caused this operation to fail.
pub const ERROR_GRAPHICS_OPM_INTERNAL_ERROR: HRESULT = 0xC026250Bu32 as HRESULT;

/// The function failed because the caller passed in an invalid OPM user mode handle.
pub const ERROR_GRAPHICS_OPM_INVALID_HANDLE: HRESULT = 0xC026250Cu32 as HRESULT;

/// A certificate could not be returned because the certificate buffer passed to the function was too small.
pub const ERROR_GRAPHICS_PVP_INVALID_CERTIFICATE_LENGTH: HRESULT = 0xC026250Eu32 as HRESULT;

/// A video output could not be created because the frame buffer is in spanning mode.
pub const ERROR_GRAPHICS_OPM_SPANNING_MODE_ENABLED: HRESULT = 0xC026250Fu32 as HRESULT;

/// A video output could not be created because the frame buffer is in theater mode.
pub const ERROR_GRAPHICS_OPM_THEATER_MODE_ENABLED: HRESULT = 0xC0262510u32 as HRESULT;

/// The function failed because the display adapter's Hardware Functionality Scan failed to validate the graphics hardware.
pub const ERROR_GRAPHICS_PVP_HFS_FAILED: HRESULT = 0xC0262511u32 as HRESULT;

/// The HDCP System Renewability Message passed to this function did not comply with section 5 of the HDCP 1.1 specification.
pub const ERROR_GRAPHICS_OPM_INVALID_SRM: HRESULT = 0xC0262512u32 as HRESULT;

/// The video output cannot enable the High-bandwidth Digital Content Protection (HDCP) System because it does not support HDCP.
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_HDCP: HRESULT = 0xC0262513u32 as HRESULT;

/// The video output cannot enable Analogue Copy Protection (ACP) because it does not support ACP.
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_ACP: HRESULT = 0xC0262514u32 as HRESULT;

/// The video output cannot enable the Content Generation Management System Analogue (CGMS-A) protection technology because it does not support CGMS-A.
pub const ERROR_GRAPHICS_OPM_OUTPUT_DOES_NOT_SUPPORT_CGMSA: HRESULT = 0xC0262515u32 as HRESULT;

/// The IOPMVideoOutput::GetInformation method cannot return the version of the SRM being used because the application never successfully passed an SRM to the video output.
pub const ERROR_GRAPHICS_OPM_HDCP_SRM_NEVER_SET: HRESULT = 0xC0262516u32 as HRESULT;

/// The IOPMVideoOutput::Configure method cannot enable the specified output protection technology because the output's screen resolution is too high.
pub const ERROR_GRAPHICS_OPM_RESOLUTION_TOO_HIGH: HRESULT = 0xC0262517u32 as HRESULT;

/// The IOPMVideoOutput::Configure method cannot enable HDCP because the display adapter's HDCP hardware is already being used by other physical outputs.
pub const ERROR_GRAPHICS_OPM_ALL_HDCP_HARDWARE_ALREADY_IN_USE: HRESULT = 0xC0262518u32 as HRESULT;

/// The operating system asynchronously destroyed this OPM video output because the operating system's state changed. This error typically occurs because the monitor PDO associated with this video output was removed, the monitor PDO associated with this video output was stopped, the video output's session became a non-console session or the video output's desktop became an inactive desktop.
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_NO_LONGER_EXISTS: HRESULT = 0xC026251Au32 as HRESULT;

/// The method failed because the session is changing its type. No IOPMVideoOutput methods can be called when a session is changing its type. There are currently three types of sessions: console, disconnected and remote.
pub const ERROR_GRAPHICS_OPM_SESSION_TYPE_CHANGE_IN_PROGRESS: HRESULT = 0xC026251Bu32 as HRESULT;

/// Either the IOPMVideoOutput::COPPCompatibleGetInformation, IOPMVideoOutput::GetInformation, or IOPMVideoOutput::Configure method failed. This error is returned when the caller tries to use a COPP specific command while the video output has OPM semantics only.
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_DOES_NOT_HAVE_COPP_SEMANTICS: HRESULT =
    0xC026251Cu32 as HRESULT;

/// The IOPMVideoOutput::GetInformation and IOPMVideoOutput::COPPCompatibleGetInformation methods return this error if the passed in sequence number is not the expected sequence number or the passed in OMAC value is invalid.
pub const ERROR_GRAPHICS_OPM_INVALID_INFORMATION_REQUEST: HRESULT = 0xC026251Du32 as HRESULT;

/// The method failed because an unexpected error occurred inside of a display driver.
pub const ERROR_GRAPHICS_OPM_DRIVER_INTERNAL_ERROR: HRESULT = 0xC026251Eu32 as HRESULT;

/// Either the IOPMVideoOutput::COPPCompatibleGetInformation, IOPMVideoOutput::GetInformation, or IOPMVideoOutput::Configure method failed. This error is returned when the caller tries to use an OPM specific command while the video output has COPP semantics only.
pub const ERROR_GRAPHICS_OPM_VIDEO_OUTPUT_DOES_NOT_HAVE_OPM_SEMANTICS: HRESULT =
    0xC026251Fu32 as HRESULT;

/// The IOPMVideoOutput::COPPCompatibleGetInformation or IOPMVideoOutput::Configure method failed because the display driver does not support the OPM_GET_ACP_AND_CGMSA_SIGNALING and OPM_SET_ACP_AND_CGMSA_SIGNALING GUIDs.
pub const ERROR_GRAPHICS_OPM_SIGNALING_NOT_SUPPORTED: HRESULT = 0xC0262520u32 as HRESULT;

/// The IOPMVideoOutput::Configure function returns this error code if the passed in sequence number is not the expected sequence number or the passed in OMAC value is invalid.
pub const ERROR_GRAPHICS_OPM_INVALID_CONFIGURATION_REQUEST: HRESULT = 0xC0262521u32 as HRESULT;

/// The monitor connected to the specified video output does not have an I2C bus.
pub const ERROR_GRAPHICS_I2C_NOT_SUPPORTED: HRESULT = 0xC0262580u32 as HRESULT;

/// No device on the I2C bus has the specified address.
pub const ERROR_GRAPHICS_I2C_DEVICE_DOES_NOT_EXIST: HRESULT = 0xC0262581u32 as HRESULT;

/// An error occurred while transmitting data to the device on the I2C bus.
pub const ERROR_GRAPHICS_I2C_ERROR_TRANSMITTING_DATA: HRESULT = 0xC0262582u32 as HRESULT;

/// An error occurred while receiving data from the device on the I2C bus.
pub const ERROR_GRAPHICS_I2C_ERROR_RECEIVING_DATA: HRESULT = 0xC0262583u32 as HRESULT;

/// The monitor does not support the specified VCP code.
pub const ERROR_GRAPHICS_DDCCI_VCP_NOT_SUPPORTED: HRESULT = 0xC0262584u32 as HRESULT;

/// The data received from the monitor is invalid.
pub const ERROR_GRAPHICS_DDCCI_INVALID_DATA: HRESULT = 0xC0262585u32 as HRESULT;

/// The function failed because a monitor returned an invalid Timing Status byte when the operating system used the DDC/CI Get Timing Report & Timing Message command to get a timing report from a monitor.
pub const ERROR_GRAPHICS_DDCCI_MONITOR_RETURNED_INVALID_TIMING_STATUS_BYTE: HRESULT =
    0xC0262586u32 as HRESULT;

/// The monitor returned a DDC/CI capabilities string which did not comply with the ACCESS.bus 3.0, DDC/CI 1.1, or MCCS 2 Revision 1 specification.
pub const ERROR_GRAPHICS_MCA_INVALID_CAPABILITIES_STRING: HRESULT = 0xC0262587u32 as HRESULT;

/// An internal Monitor Configuration API error occurred.
pub const ERROR_GRAPHICS_MCA_INTERNAL_ERROR: HRESULT = 0xC0262588u32 as HRESULT;

/// An operation failed because a DDC/CI message had an invalid value in its command field.
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_COMMAND: HRESULT = 0xC0262589u32 as HRESULT;

/// An error occurred because the field length of a DDC/CI message contained an invalid value.
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_LENGTH: HRESULT = 0xC026258Au32 as HRESULT;

/// An error occurred because the checksum field in a DDC/CI message did not match the message's computed checksum value. This error implies that the data was corrupted while it was being transmitted from a monitor to a computer.
pub const ERROR_GRAPHICS_DDCCI_INVALID_MESSAGE_CHECKSUM: HRESULT = 0xC026258Bu32 as HRESULT;

/// This function failed because an invalid monitor handle was passed to it.
pub const ERROR_GRAPHICS_INVALID_PHYSICAL_MONITOR_HANDLE: HRESULT = 0xC026258Cu32 as HRESULT;

/// The operating system asynchronously destroyed the monitor which corresponds to this handle because the operating system's state changed. This error typically occurs because the monitor PDO associated with this handle was removed, the monitor PDO associated with this handle was stopped, or a display mode change occurred. A display mode change occurs when windows sends a WM_DISPLAYCHANGE windows message to applications.
pub const ERROR_GRAPHICS_MONITOR_NO_LONGER_EXISTS: HRESULT = 0xC026258Du32 as HRESULT;

/// A continuous VCP code's current value is greater than its maximum value. This error code indicates that a monitor returned an invalid value.
pub const ERROR_GRAPHICS_DDCCI_CURRENT_CURRENT_VALUE_GREATER_THAN_MAXIMUM_VALUE: HRESULT =
    0xC02625D8u32 as HRESULT;

/// The monitor's VCP Version (0xDF) VCP code returned an invalid version value.
pub const ERROR_GRAPHICS_MCA_INVALID_VCP_VERSION: HRESULT = 0xC02625D9u32 as HRESULT;

/// The monitor does not comply with the MCCS specification it claims to support.
pub const ERROR_GRAPHICS_MCA_MONITOR_VIOLATES_MCCS_SPECIFICATION: HRESULT =
    0xC02625DAu32 as HRESULT;

/// The MCCS version in a monitor's mccs_ver capability does not match the MCCS version the monitor reports when the VCP Version (0xDF) VCP code is used.
pub const ERROR_GRAPHICS_MCA_MCCS_VERSION_MISMATCH: HRESULT = 0xC02625DBu32 as HRESULT;

/// The Monitor Configuration API only works with monitors which support the MCCS 1.0 specification, MCCS 2.0 specification or the MCCS 2.0 Revision 1 specification.
pub const ERROR_GRAPHICS_MCA_UNSUPPORTED_MCCS_VERSION: HRESULT = 0xC02625DCu32 as HRESULT;

/// The monitor returned an invalid monitor technology type. CRT, Plasma and LCD (TFT) are examples of monitor technology types. This error implies that the monitor violated the MCCS 2.0 or MCCS 2.0 Revision 1 specification.
pub const ERROR_GRAPHICS_MCA_INVALID_TECHNOLOGY_TYPE_RETURNED: HRESULT = 0xC02625DEu32 as HRESULT;

/// SetMonitorColorTemperature()'s caller passed a color temperature to it which the current monitor did not support. This error implies that the monitor violated the MCCS 2.0 or MCCS 2.0 Revision 1 specification.
pub const ERROR_GRAPHICS_MCA_UNSUPPORTED_COLOR_TEMPERATURE: HRESULT = 0xC02625DFu32 as HRESULT;

/// This function can only be used if a program is running in the local console session. It cannot be used if the program is running on a remote desktop session or on a terminal server session.
pub const ERROR_GRAPHICS_ONLY_CONSOLE_SESSION_SUPPORTED: HRESULT = 0xC02625E0u32 as HRESULT;

/// This function cannot find an actual GDI display device which corresponds to the specified GDI display device name.
pub const ERROR_GRAPHICS_NO_DISPLAY_DEVICE_CORRESPONDS_TO_NAME: HRESULT = 0xC02625E1u32 as HRESULT;

/// The function failed because the specified GDI display device was not attached to the Windows desktop.
pub const ERROR_GRAPHICS_DISPLAY_DEVICE_NOT_ATTACHED_TO_DESKTOP: HRESULT = 0xC02625E2u32 as HRESULT;

/// This function does not support GDI mirroring display devices because GDI mirroring display devices do not have any physical monitors associated with them.
pub const ERROR_GRAPHICS_MIRRORING_DEVICES_NOT_SUPPORTED: HRESULT = 0xC02625E3u32 as HRESULT;

/// The function failed because an invalid pointer parameter was passed to it. A pointer parameter is invalid if it is NULL, points to an invalid address, points to a kernel mode address, or is not correctly aligned.
pub const ERROR_GRAPHICS_INVALID_POINTER: HRESULT = 0xC02625E4u32 as HRESULT;

/// The function failed because the specified GDI device did not have any monitors associated with it.
pub const ERROR_GRAPHICS_NO_MONITORS_CORRESPOND_TO_DISPLAY_DEVICE: HRESULT =
    0xC02625E5u32 as HRESULT;

/// An array passed to the function cannot hold all of the data that the function must copy into the array.
pub const ERROR_GRAPHICS_PARAMETER_ARRAY_TOO_SMALL: HRESULT = 0xC02625E6u32 as HRESULT;

/// An internal error caused an operation to fail.
pub const ERROR_GRAPHICS_INTERNAL_ERROR: HRESULT = 0xC02625E7u32 as HRESULT;

/// The function failed because the current session is changing its type. This function cannot be called when the current session is changing its type. There are currently three types of sessions: console, disconnected and remote.
pub const ERROR_GRAPHICS_SESSION_TYPE_CHANGE_IN_PROGRESS: HRESULT = 0xC02605E8u32 as HRESULT;

/// The NAP SoH packet is invalid.
pub const NAP_E_INVALID_PACKET: HRESULT = 0x80270001u32 as HRESULT;

/// An SoH was missing from the NAP packet.
pub const NAP_E_MISSING_SOH: HRESULT = 0x80270002u32 as HRESULT;

/// The entity ID conflicts with an already registered id.
pub const NAP_E_CONFLICTING_ID: HRESULT = 0x80270003u32 as HRESULT;

/// No cached SoH is present.
pub const NAP_E_NO_CACHED_SOH: HRESULT = 0x80270004u32 as HRESULT;

/// The entity is still bound to the NAP system.
pub const NAP_E_STILL_BOUND: HRESULT = 0x80270005u32 as HRESULT;

/// The entity is not registered with the NAP system.
pub const NAP_E_NOT_REGISTERED: HRESULT = 0x80270006u32 as HRESULT;

/// The entity is not initialized with the NAP system.
pub const NAP_E_NOT_INITIALIZED: HRESULT = 0x80270007u32 as HRESULT;

/// The correlation id in the SoH-Request and SoH-Response do not match up.
pub const NAP_E_MISMATCHED_ID: HRESULT = 0x80270008u32 as HRESULT;

/// Completion was indicated on a request that is not currently pending.
pub const NAP_E_NOT_PENDING: HRESULT = 0x80270009u32 as HRESULT;

/// The NAP component's id was not found.
pub const NAP_E_ID_NOT_FOUND: HRESULT = 0x8027000Au32 as HRESULT;

/// The maximum size of the connection is too small for an SoH packet.
pub const NAP_E_MAXSIZE_TOO_SMALL: HRESULT = 0x8027000Bu32 as HRESULT;

/// The NapAgent service is not running.
pub const NAP_E_SERVICE_NOT_RUNNING: HRESULT = 0x8027000Cu32 as HRESULT;

/// A certificate is already present in the cert store.
pub const NAP_S_CERT_ALREADY_PRESENT: HRESULT = 0x0027000D;

/// The entity is disabled with the NapAgent service.
pub const NAP_E_ENTITY_DISABLED: HRESULT = 0x8027000Eu32 as HRESULT;

/// Group Policy is not configured.
pub const NAP_E_NETSH_GROUPPOLICY_ERROR: HRESULT = 0x8027000Fu32 as HRESULT;

/// Too many simultaneous calls.
pub const NAP_E_TOO_MANY_CALLS: HRESULT = 0x80270010u32 as HRESULT;

/// SHV configuration already existed.
pub const NAP_E_SHV_CONFIG_EXISTED: HRESULT = 0x80270011u32 as HRESULT;

/// SHV configuration is not found.
pub const NAP_E_SHV_CONFIG_NOT_FOUND: HRESULT = 0x80270012u32 as HRESULT;

/// SHV timed out on the request.
pub const NAP_E_SHV_TIMEOUT: HRESULT = 0x80270013u32 as HRESULT;

/// This is an error mask to convert TPM hardware errors to win errors.
pub const TPM_E_ERROR_MASK: HRESULT = 0x80280000u32 as HRESULT;

/// TPM 1.2: Authentication failed.
pub const TPM_E_AUTHFAIL: HRESULT = 0x80280001u32 as HRESULT;

/// TPM 1.2: The index to a PCR, DIR or other register is incorrect.
pub const TPM_E_BADINDEX: HRESULT = 0x80280002u32 as HRESULT;

/// TPM 1.2: One or more parameter is bad.
pub const TPM_E_BAD_PARAMETER: HRESULT = 0x80280003u32 as HRESULT;

/// TPM 1.2: An operation completed successfully but the auditing of that operation failed.
pub const TPM_E_AUDITFAILURE: HRESULT = 0x80280004u32 as HRESULT;

/// TPM 1.2: The clear disable flag is set and all clear operations now require physical access.
pub const TPM_E_CLEAR_DISABLED: HRESULT = 0x80280005u32 as HRESULT;

/// TPM 1.2: Activate the Trusted Platform Module (TPM).
pub const TPM_E_DEACTIVATED: HRESULT = 0x80280006u32 as HRESULT;

/// TPM 1.2: Enable the Trusted Platform Module (TPM).
pub const TPM_E_DISABLED: HRESULT = 0x80280007u32 as HRESULT;

/// TPM 1.2: The target command has been disabled.
pub const TPM_E_DISABLED_CMD: HRESULT = 0x80280008u32 as HRESULT;

/// TPM 1.2: The operation failed.
pub const TPM_E_FAIL: HRESULT = 0x80280009u32 as HRESULT;

/// TPM 1.2: The ordinal was unknown or inconsistent.
pub const TPM_E_BAD_ORDINAL: HRESULT = 0x8028000Au32 as HRESULT;

/// TPM 1.2: The ability to install an owner is disabled.
pub const TPM_E_INSTALL_DISABLED: HRESULT = 0x8028000Bu32 as HRESULT;

/// TPM 1.2: The key handle cannot be interpreted.
pub const TPM_E_INVALID_KEYHANDLE: HRESULT = 0x8028000Cu32 as HRESULT;

/// TPM 1.2: The key handle points to an invalid key.
pub const TPM_E_KEYNOTFOUND: HRESULT = 0x8028000Du32 as HRESULT;

/// TPM 1.2: Unacceptable encryption scheme.
pub const TPM_E_INAPPROPRIATE_ENC: HRESULT = 0x8028000Eu32 as HRESULT;

/// TPM 1.2: Migration authorization failed.
pub const TPM_E_MIGRATEFAIL: HRESULT = 0x8028000Fu32 as HRESULT;

/// TPM 1.2: PCR information could not be interpreted.
pub const TPM_E_INVALID_PCR_INFO: HRESULT = 0x80280010u32 as HRESULT;

/// TPM 1.2: No room to load key.
pub const TPM_E_NOSPACE: HRESULT = 0x80280011u32 as HRESULT;

/// TPM 1.2: There is no Storage Root Key (SRK) set.
pub const TPM_E_NOSRK: HRESULT = 0x80280012u32 as HRESULT;

/// TPM 1.2: An encrypted blob is invalid or was not created by this TPM.
pub const TPM_E_NOTSEALED_BLOB: HRESULT = 0x80280013u32 as HRESULT;

/// TPM 1.2: The Trusted Platform Module (TPM) already has an owner.
pub const TPM_E_OWNER_SET: HRESULT = 0x80280014u32 as HRESULT;

/// TPM 1.2: The TPM has insufficient internal resources to perform the requested action.
pub const TPM_E_RESOURCES: HRESULT = 0x80280015u32 as HRESULT;

/// TPM 1.2: A random string was too short.
pub const TPM_E_SHORTRANDOM: HRESULT = 0x80280016u32 as HRESULT;

/// TPM 1.2: The TPM does not have the space to perform the operation.
pub const TPM_E_SIZE: HRESULT = 0x80280017u32 as HRESULT;

/// TPM 1.2: The named PCR value does not match the current PCR value.
pub const TPM_E_WRONGPCRVAL: HRESULT = 0x80280018u32 as HRESULT;

/// TPM 1.2: The paramSize argument to the command has the incorrect value .
pub const TPM_E_BAD_PARAM_SIZE: HRESULT = 0x80280019u32 as HRESULT;

/// TPM 1.2: There is no existing SHA-1 thread.
pub const TPM_E_SHA_THREAD: HRESULT = 0x8028001Au32 as HRESULT;

/// TPM 1.2: The calculation is unable to proceed because the existing SHA-1 thread has already encountered an error.
pub const TPM_E_SHA_ERROR: HRESULT = 0x8028001Bu32 as HRESULT;

/// TPM 1.2: The TPM hardware device reported a failure during its internal self test. Try restarting the computer to resolve the problem. If the problem continues, check for the latest BIOS or firmware update for your TPM hardware. Consult the computer manufacturer's documentation for instructions.
pub const TPM_E_FAILEDSELFTEST: HRESULT = 0x8028001Cu32 as HRESULT;

/// TPM 1.2: The authorization for the second key in a 2 key function failed authorization.
pub const TPM_E_AUTH2FAIL: HRESULT = 0x8028001Du32 as HRESULT;

/// TPM 1.2: The tag value sent to for a command is invalid.
pub const TPM_E_BADTAG: HRESULT = 0x8028001Eu32 as HRESULT;

/// TPM 1.2: An IO error occurred transmitting information to the TPM.
pub const TPM_E_IOERROR: HRESULT = 0x8028001Fu32 as HRESULT;

/// TPM 1.2: The encryption process had a problem.
pub const TPM_E_ENCRYPT_ERROR: HRESULT = 0x80280020u32 as HRESULT;

/// TPM 1.2: The decryption process did not complete.
pub const TPM_E_DECRYPT_ERROR: HRESULT = 0x80280021u32 as HRESULT;

/// TPM 1.2: An invalid handle was used.
pub const TPM_E_INVALID_AUTHHANDLE: HRESULT = 0x80280022u32 as HRESULT;

/// TPM 1.2: The TPM does not have an Endorsement Key (EK) installed.
pub const TPM_E_NO_ENDORSEMENT: HRESULT = 0x80280023u32 as HRESULT;

/// TPM 1.2: The usage of a key is not allowed.
pub const TPM_E_INVALID_KEYUSAGE: HRESULT = 0x80280024u32 as HRESULT;

/// TPM 1.2: The submitted entity type is not allowed.
pub const TPM_E_WRONG_ENTITYTYPE: HRESULT = 0x80280025u32 as HRESULT;

/// TPM 1.2: The command was received in the wrong sequence relative to TPM_Init and a subsequent TPM_Startup.
pub const TPM_E_INVALID_POSTINIT: HRESULT = 0x80280026u32 as HRESULT;

/// TPM 1.2: Signed data cannot include additional DER information.
pub const TPM_E_INAPPROPRIATE_SIG: HRESULT = 0x80280027u32 as HRESULT;

/// TPM 1.2: The key properties in TPM_KEY_PARMs are not supported by this TPM.
pub const TPM_E_BAD_KEY_PROPERTY: HRESULT = 0x80280028u32 as HRESULT;

/// TPM 1.2: The migration properties of this key are incorrect.
pub const TPM_E_BAD_MIGRATION: HRESULT = 0x80280029u32 as HRESULT;

/// TPM 1.2: The signature or encryption scheme for this key is incorrect or not permitted in this situation.
pub const TPM_E_BAD_SCHEME: HRESULT = 0x8028002Au32 as HRESULT;

/// TPM 1.2: The size of the data (or blob) parameter is bad or inconsistent with the referenced key.
pub const TPM_E_BAD_DATASIZE: HRESULT = 0x8028002Bu32 as HRESULT;

/// TPM 1.2: A mode parameter is bad, such as capArea or subCapArea for TPM_GetCapability, phsicalPresence parameter for TPM_PhysicalPresence, or migrationType for TPM_CreateMigrationBlob.
pub const TPM_E_BAD_MODE: HRESULT = 0x8028002Cu32 as HRESULT;

/// TPM 1.2: Either the physicalPresence or physicalPresenceLock bits have the wrong value.
pub const TPM_E_BAD_PRESENCE: HRESULT = 0x8028002Du32 as HRESULT;

/// TPM 1.2: The TPM cannot perform this version of the capability.
pub const TPM_E_BAD_VERSION: HRESULT = 0x8028002Eu32 as HRESULT;

/// TPM 1.2: The TPM does not allow for wrapped transport sessions.
pub const TPM_E_NO_WRAP_TRANSPORT: HRESULT = 0x8028002Fu32 as HRESULT;

/// TPM 1.2: TPM audit construction failed and the underlying command was returning a failure code also.
pub const TPM_E_AUDITFAIL_UNSUCCESSFUL: HRESULT = 0x80280030u32 as HRESULT;

/// TPM 1.2: TPM audit construction failed and the underlying command was returning success.
pub const TPM_E_AUDITFAIL_SUCCESSFUL: HRESULT = 0x80280031u32 as HRESULT;

/// TPM 1.2: Attempt to reset a PCR register that does not have the resettable attribute.
pub const TPM_E_NOTRESETABLE: HRESULT = 0x80280032u32 as HRESULT;

/// TPM 1.2: Attempt to reset a PCR register that requires locality and locality modifier not part of command transport.
pub const TPM_E_NOTLOCAL: HRESULT = 0x80280033u32 as HRESULT;

/// TPM 1.2: Make identity blob not properly typed.
pub const TPM_E_BAD_TYPE: HRESULT = 0x80280034u32 as HRESULT;

/// TPM 1.2: When saving context identified resource type does not match actual resource.
pub const TPM_E_INVALID_RESOURCE: HRESULT = 0x80280035u32 as HRESULT;

/// TPM 1.2: The TPM is attempting to execute a command that is not allowed when in FIPS mode.
pub const TPM_E_NOTFIPS: HRESULT = 0x80280036u32 as HRESULT;

/// TPM 1.2: The command is attempting to use an invalid family ID.
pub const TPM_E_INVALID_FAMILY: HRESULT = 0x80280037u32 as HRESULT;

/// TPM 1.2: The permission to manipulate the NV storage is not available.
pub const TPM_E_NO_NV_PERMISSION: HRESULT = 0x80280038u32 as HRESULT;

/// TPM 1.2: The operation requires a signed command.
pub const TPM_E_REQUIRES_SIGN: HRESULT = 0x80280039u32 as HRESULT;

/// TPM 1.2: Wrong operation to load an NV key.
pub const TPM_E_KEY_NOTSUPPORTED: HRESULT = 0x8028003Au32 as HRESULT;

/// TPM 1.2: NV_LoadKey blob requires both owner and blob authorization.
pub const TPM_E_AUTH_CONFLICT: HRESULT = 0x8028003Bu32 as HRESULT;

/// TPM 1.2: The NV area is locked and not writable.
pub const TPM_E_AREA_LOCKED: HRESULT = 0x8028003Cu32 as HRESULT;

/// TPM 1.2: The locality is incorrect for the attempted operation.
pub const TPM_E_BAD_LOCALITY: HRESULT = 0x8028003Du32 as HRESULT;

/// TPM 1.2: The NV area is read only and can't be written to.
pub const TPM_E_READ_ONLY: HRESULT = 0x8028003Eu32 as HRESULT;

/// TPM 1.2: There is no protection on the write to the NV area.
pub const TPM_E_PER_NOWRITE: HRESULT = 0x8028003Fu32 as HRESULT;

/// TPM 1.2: The family count value does not match.
pub const TPM_E_FAMILYCOUNT: HRESULT = 0x80280040u32 as HRESULT;

/// TPM 1.2: The NV area has already been written to.
pub const TPM_E_WRITE_LOCKED: HRESULT = 0x80280041u32 as HRESULT;

/// TPM 1.2: The NV area attributes conflict.
pub const TPM_E_BAD_ATTRIBUTES: HRESULT = 0x80280042u32 as HRESULT;

/// TPM 1.2: The structure tag and version are invalid or inconsistent.
pub const TPM_E_INVALID_STRUCTURE: HRESULT = 0x80280043u32 as HRESULT;

/// TPM 1.2: The key is under control of the TPM Owner and can only be evicted by the TPM Owner.
pub const TPM_E_KEY_OWNER_CONTROL: HRESULT = 0x80280044u32 as HRESULT;

/// TPM 1.2: The counter handle is incorrect.
pub const TPM_E_BAD_COUNTER: HRESULT = 0x80280045u32 as HRESULT;

/// TPM 1.2: The write is not a complete write of the area.
pub const TPM_E_NOT_FULLWRITE: HRESULT = 0x80280046u32 as HRESULT;

/// TPM 1.2: The gap between saved context counts is too large.
pub const TPM_E_CONTEXT_GAP: HRESULT = 0x80280047u32 as HRESULT;

/// TPM 1.2: The maximum number of NV writes without an owner has been exceeded.
pub const TPM_E_MAXNVWRITES: HRESULT = 0x80280048u32 as HRESULT;

/// TPM 1.2: No operator AuthData value is set.
pub const TPM_E_NOOPERATOR: HRESULT = 0x80280049u32 as HRESULT;

/// TPM 1.2: The resource pointed to by context is not loaded.
pub const TPM_E_RESOURCEMISSING: HRESULT = 0x8028004Au32 as HRESULT;

/// TPM 1.2: The delegate administration is locked.
pub const TPM_E_DELEGATE_LOCK: HRESULT = 0x8028004Bu32 as HRESULT;

/// TPM 1.2: Attempt to manage a family other then the delegated family.
pub const TPM_E_DELEGATE_FAMILY: HRESULT = 0x8028004Cu32 as HRESULT;

/// TPM 1.2: Delegation table management not enabled.
pub const TPM_E_DELEGATE_ADMIN: HRESULT = 0x8028004Du32 as HRESULT;

/// TPM 1.2: There was a command executed outside of an exclusive transport session.
pub const TPM_E_TRANSPORT_NOTEXCLUSIVE: HRESULT = 0x8028004Eu32 as HRESULT;

/// TPM 1.2: Attempt to context save a owner evict controlled key.
pub const TPM_E_OWNER_CONTROL: HRESULT = 0x8028004Fu32 as HRESULT;

/// TPM 1.2: The DAA command has no resources available to execute the command.
pub const TPM_E_DAA_RESOURCES: HRESULT = 0x80280050u32 as HRESULT;

/// TPM 1.2: The consistency check on DAA parameter inputData0 has failed.
pub const TPM_E_DAA_INPUT_DATA0: HRESULT = 0x80280051u32 as HRESULT;

/// TPM 1.2: The consistency check on DAA parameter inputData1 has failed.
pub const TPM_E_DAA_INPUT_DATA1: HRESULT = 0x80280052u32 as HRESULT;

/// TPM 1.2: The consistency check on DAA_issuerSettings has failed.
pub const TPM_E_DAA_ISSUER_SETTINGS: HRESULT = 0x80280053u32 as HRESULT;

/// TPM 1.2: The consistency check on DAA_tpmSpecific has failed.
pub const TPM_E_DAA_TPM_SETTINGS: HRESULT = 0x80280054u32 as HRESULT;

/// TPM 1.2: The atomic process indicated by the submitted DAA command is not the expected process.
pub const TPM_E_DAA_STAGE: HRESULT = 0x80280055u32 as HRESULT;

/// TPM 1.2: The issuer's validity check has detected an inconsistency.
pub const TPM_E_DAA_ISSUER_VALIDITY: HRESULT = 0x80280056u32 as HRESULT;

/// TPM 1.2: The consistency check on w has failed.
pub const TPM_E_DAA_WRONG_W: HRESULT = 0x80280057u32 as HRESULT;

/// TPM 1.2: The handle is incorrect.
pub const TPM_E_BAD_HANDLE: HRESULT = 0x80280058u32 as HRESULT;

/// TPM 1.2: Delegation is not correct.
pub const TPM_E_BAD_DELEGATE: HRESULT = 0x80280059u32 as HRESULT;

/// TPM 1.2: The context blob is invalid.
pub const TPM_E_BADCONTEXT: HRESULT = 0x8028005Au32 as HRESULT;

/// TPM 1.2: Too many contexts held by the TPM.
pub const TPM_E_TOOMANYCONTEXTS: HRESULT = 0x8028005Bu32 as HRESULT;

/// TPM 1.2: Migration authority signature validation failure.
pub const TPM_E_MA_TICKET_SIGNATURE: HRESULT = 0x8028005Cu32 as HRESULT;

/// TPM 1.2: Migration destination not authenticated.
pub const TPM_E_MA_DESTINATION: HRESULT = 0x8028005Du32 as HRESULT;

/// TPM 1.2: Migration source incorrect.
pub const TPM_E_MA_SOURCE: HRESULT = 0x8028005Eu32 as HRESULT;

/// TPM 1.2: Incorrect migration authority.
pub const TPM_E_MA_AUTHORITY: HRESULT = 0x8028005Fu32 as HRESULT;

/// TPM 1.2: Attempt to revoke the EK and the EK is not revocable.
pub const TPM_E_PERMANENTEK: HRESULT = 0x80280061u32 as HRESULT;

/// TPM 1.2: Bad signature of CMK ticket.
pub const TPM_E_BAD_SIGNATURE: HRESULT = 0x80280062u32 as HRESULT;

/// TPM 1.2: There is no room in the context list for additional contexts.
pub const TPM_E_NOCONTEXTSPACE: HRESULT = 0x80280063u32 as HRESULT;

/// TPM 2.0: Asymmetric algorithm not supported or not correct.
pub const TPM_20_E_ASYMMETRIC: HRESULT = 0x80280081u32 as HRESULT;

/// TPM 2.0: Inconsistent attributes.
pub const TPM_20_E_ATTRIBUTES: HRESULT = 0x80280082u32 as HRESULT;

/// TPM 2.0: Hash algorithm not supported or not appropriate.
pub const TPM_20_E_HASH: HRESULT = 0x80280083u32 as HRESULT;

/// TPM 2.0: Value is out of range or is not correct for the context.
pub const TPM_20_E_VALUE: HRESULT = 0x80280084u32 as HRESULT;

/// TPM 2.0: Hierarchy is not enabled or is not correct for the use.
pub const TPM_20_E_HIERARCHY: HRESULT = 0x80280085u32 as HRESULT;

/// TPM 2.0: Key size is not supported.
pub const TPM_20_E_KEY_SIZE: HRESULT = 0x80280087u32 as HRESULT;

/// TPM 2.0: Mask generation function not supported.
pub const TPM_20_E_MGF: HRESULT = 0x80280088u32 as HRESULT;

/// TPM 2.0: Mode of operation not supported.
pub const TPM_20_E_MODE: HRESULT = 0x80280089u32 as HRESULT;

/// TPM 2.0: The type of the value is not appropriate for the use.
pub const TPM_20_E_TYPE: HRESULT = 0x8028008Au32 as HRESULT;

/// TPM 2.0: The Handle is not correct for the use.
pub const TPM_20_E_HANDLE: HRESULT = 0x8028008Bu32 as HRESULT;

/// TPM 2.0: Unsupported key derivation function or function not appropriate for use.
pub const TPM_20_E_KDF: HRESULT = 0x8028008Cu32 as HRESULT;

/// TPM 2.0: Value was out of allowed range.
pub const TPM_20_E_RANGE: HRESULT = 0x8028008Du32 as HRESULT;

/// TPM 2.0: The authorization HMAC check failed and DA counter incremented.
pub const TPM_20_E_AUTH_FAIL: HRESULT = 0x8028008Eu32 as HRESULT;

/// TPM 2.0: Invalid nonce size.
pub const TPM_20_E_NONCE: HRESULT = 0x8028008Fu32 as HRESULT;

/// TPM 2.0: Authorization requires assertion of PP.
pub const TPM_20_E_PP: HRESULT = 0x80280090u32 as HRESULT;

/// TPM 2.0: Unsupported or incompatible scheme.
pub const TPM_20_E_SCHEME: HRESULT = 0x80280092u32 as HRESULT;

/// TPM 2.0: Structure is wrong size.
pub const TPM_20_E_SIZE: HRESULT = 0x80280095u32 as HRESULT;

/// TPM 2.0: Unsupported symmetric algorithm or key size, or not appropriate for instance.
pub const TPM_20_E_SYMMETRIC: HRESULT = 0x80280096u32 as HRESULT;

/// TPM 2.0: Incorrect structure tag.
pub const TPM_20_E_TAG: HRESULT = 0x80280097u32 as HRESULT;

/// TPM 2.0: Union selector is incorrect.
pub const TPM_20_E_SELECTOR: HRESULT = 0x80280098u32 as HRESULT;

/// TPM 2.0: The TPM was unable to unmarshal a value because there were not enough octets in the input buffer.
pub const TPM_20_E_INSUFFICIENT: HRESULT = 0x8028009Au32 as HRESULT;

/// TPM 2.0: The signature is not valid.
pub const TPM_20_E_SIGNATURE: HRESULT = 0x8028009Bu32 as HRESULT;

/// TPM 2.0: Key fields are not compatible with the selected use.
pub const TPM_20_E_KEY: HRESULT = 0x8028009Cu32 as HRESULT;

/// TPM 2.0: A policy check failed.
pub const TPM_20_E_POLICY_FAIL: HRESULT = 0x8028009Du32 as HRESULT;

/// TPM 2.0: Integrity check failed.
pub const TPM_20_E_INTEGRITY: HRESULT = 0x8028009Fu32 as HRESULT;

/// TPM 2.0: Invalid ticket.
pub const TPM_20_E_TICKET: HRESULT = 0x802800A0u32 as HRESULT;

/// TPM 2.0: Reserved bits not set to zero as required.
pub const TPM_20_E_RESERVED_BITS: HRESULT = 0x802800A1u32 as HRESULT;

/// TPM 2.0: Authorization failure without DA implications.
pub const TPM_20_E_BAD_AUTH: HRESULT = 0x802800A2u32 as HRESULT;

/// TPM 2.0: The policy has expired.
pub const TPM_20_E_EXPIRED: HRESULT = 0x802800A3u32 as HRESULT;

/// TPM 2.0: The command code in the policy is not the command code of the command or the command code in a policy command references a command that is not implemented.
pub const TPM_20_E_POLICY_CC: HRESULT = 0x802800A4u32 as HRESULT;

/// TPM 2.0: Public and sensitive portions of an object are not cryptographically bound.
pub const TPM_20_E_BINDING: HRESULT = 0x802800A5u32 as HRESULT;

/// TPM 2.0: Curve not supported.
pub const TPM_20_E_CURVE: HRESULT = 0x802800A6u32 as HRESULT;

/// TPM 2.0: Point is not on the required curve.
pub const TPM_20_E_ECC_POINT: HRESULT = 0x802800A7u32 as HRESULT;

/// TPM 2.0: TPM not initialized.
pub const TPM_20_E_INITIALIZE: HRESULT = 0x80280100u32 as HRESULT;

/// TPM 2.0: Commands not being accepted because of a TPM failure.
pub const TPM_20_E_FAILURE: HRESULT = 0x80280101u32 as HRESULT;

/// TPM 2.0: Improper use of a sequence handle.
pub const TPM_20_E_SEQUENCE: HRESULT = 0x80280103u32 as HRESULT;

/// TPM 2.0: TPM_RC_PRIVATE error.
pub const TPM_20_E_PRIVATE: HRESULT = 0x8028010Bu32 as HRESULT;

/// TPM 2.0: TPM_RC_HMAC.
pub const TPM_20_E_HMAC: HRESULT = 0x80280119u32 as HRESULT;

/// TPM 2.0: TPM_RC_DISABLED.
pub const TPM_20_E_DISABLED: HRESULT = 0x80280120u32 as HRESULT;

/// TPM 2.0: Command failed because audit sequence required exclusivity.
pub const TPM_20_E_EXCLUSIVE: HRESULT = 0x80280121u32 as HRESULT;

/// TPM 2.0: Unsupported ECC curve.
pub const TPM_20_E_ECC_CURVE: HRESULT = 0x80280123u32 as HRESULT;

/// TPM 2.0: Authorization handle is not correct for command.
pub const TPM_20_E_AUTH_TYPE: HRESULT = 0x80280124u32 as HRESULT;

/// TPM 2.0: Command requires an authorization session for handle and is not present.
pub const TPM_20_E_AUTH_MISSING: HRESULT = 0x80280125u32 as HRESULT;

/// TPM 2.0: Policy failure in Math Operation or an invalid authPolicy value.
pub const TPM_20_E_POLICY: HRESULT = 0x80280126u32 as HRESULT;

/// TPM 2.0: PCR check fail.
pub const TPM_20_E_PCR: HRESULT = 0x80280127u32 as HRESULT;

/// TPM 2.0: PCR have changed since checked.
pub const TPM_20_E_PCR_CHANGED: HRESULT = 0x80280128u32 as HRESULT;

/// TPM 2.0: The TPM is not in the right mode for upgrade.
pub const TPM_20_E_UPGRADE: HRESULT = 0x8028012Du32 as HRESULT;

/// TPM 2.0: Context ID counter is at maximum.
pub const TPM_20_E_TOO_MANY_CONTEXTS: HRESULT = 0x8028012Eu32 as HRESULT;

/// TPM 2.0: authValue or authPolicy is not available for selected entity.
pub const TPM_20_E_AUTH_UNAVAILABLE: HRESULT = 0x8028012Fu32 as HRESULT;

/// TPM 2.0: A _TPM_Init and Startup(CLEAR) is required before the TPM can resume operation.
pub const TPM_20_E_REBOOT: HRESULT = 0x80280130u32 as HRESULT;

/// TPM 2.0: The protection algorithms (hash and symmetric) are not reasonably balanced. The digest size of the hash must be larger than the key size of the symmetric algorithm.
pub const TPM_20_E_UNBALANCED: HRESULT = 0x80280131u32 as HRESULT;

/// TPM 2.0: The TPM command's commandSize value is inconsistent with contents of the command buffer; either the size is not the same as the bytes loaded by the hardware interface layer or the value is not large enough to hold a command header.
pub const TPM_20_E_COMMAND_SIZE: HRESULT = 0x80280142u32 as HRESULT;

/// TPM 2.0: Command code not supported.
pub const TPM_20_E_COMMAND_CODE: HRESULT = 0x80280143u32 as HRESULT;

/// TPM 2.0: The value of authorizationSize is out of range or the number of octets in the authorization Area is greater than required.
pub const TPM_20_E_AUTHSIZE: HRESULT = 0x80280144u32 as HRESULT;

/// TPM 2.0: Use of an authorization session with a context command or another command that cannot have an authorization session.
pub const TPM_20_E_AUTH_CONTEXT: HRESULT = 0x80280145u32 as HRESULT;

/// TPM 2.0: NV offset+size is out of range.
pub const TPM_20_E_NV_RANGE: HRESULT = 0x80280146u32 as HRESULT;

/// TPM 2.0: Requested allocation size is larger than allowed.
pub const TPM_20_E_NV_SIZE: HRESULT = 0x80280147u32 as HRESULT;

/// TPM 2.0: NV access locked.
pub const TPM_20_E_NV_LOCKED: HRESULT = 0x80280148u32 as HRESULT;

/// TPM 2.0: NV access authorization fails in command actions
pub const TPM_20_E_NV_AUTHORIZATION: HRESULT = 0x80280149u32 as HRESULT;

/// TPM 2.0: An NV index is used before being initialized or the state saved by TPM2_Shutdown(STATE) could not be restored.
pub const TPM_20_E_NV_UNINITIALIZED: HRESULT = 0x8028014Au32 as HRESULT;

/// TPM 2.0: Insufficient space for NV allocation.
pub const TPM_20_E_NV_SPACE: HRESULT = 0x8028014Bu32 as HRESULT;

/// TPM 2.0: NV index or persistent object already defined.
pub const TPM_20_E_NV_DEFINED: HRESULT = 0x8028014Cu32 as HRESULT;

/// TPM 2.0: Context in TPM2_ContextLoad() is not valid.
pub const TPM_20_E_BAD_CONTEXT: HRESULT = 0x80280150u32 as HRESULT;

/// TPM 2.0: chHash value already set or not correct for use.
pub const TPM_20_E_CPHASH: HRESULT = 0x80280151u32 as HRESULT;

/// TPM 2.0: Handle for parent is not a valid parent.
pub const TPM_20_E_PARENT: HRESULT = 0x80280152u32 as HRESULT;

/// TPM 2.0: Some function needs testing.
pub const TPM_20_E_NEEDS_TEST: HRESULT = 0x80280153u32 as HRESULT;

/// TPM 2.0: returned when an internal function cannot process a request due to an unspecified problem. This code is usually related to invalid parameters that are not properly filtered by the input unmarshaling code.
pub const TPM_20_E_NO_RESULT: HRESULT = 0x80280154u32 as HRESULT;

/// TPM 2.0: The sensitive area did not unmarshal correctly after decryption - this code is used in lieu of the other unmarshaling errors so that an attacker cannot determine where the unmarshaling error occurred.
pub const TPM_20_E_SENSITIVE: HRESULT = 0x80280155u32 as HRESULT;

/// The command was blocked.
pub const TPM_E_COMMAND_BLOCKED: HRESULT = 0x80280400u32 as HRESULT;

/// The specified handle was not found.
pub const TPM_E_INVALID_HANDLE: HRESULT = 0x80280401u32 as HRESULT;

/// The TPM returned a duplicate handle and the command needs to be resubmitted.
pub const TPM_E_DUPLICATE_VHANDLE: HRESULT = 0x80280402u32 as HRESULT;

/// The command within the transport was blocked.
pub const TPM_E_EMBEDDED_COMMAND_BLOCKED: HRESULT = 0x80280403u32 as HRESULT;

/// The command within the transport is not supported.
pub const TPM_E_EMBEDDED_COMMAND_UNSUPPORTED: HRESULT = 0x80280404u32 as HRESULT;

/// The TPM is too busy to respond to the command immediately, but the command could be resubmitted at a later time.
pub const TPM_E_RETRY: HRESULT = 0x80280800u32 as HRESULT;

/// SelfTestFull has not been run.
pub const TPM_E_NEEDS_SELFTEST: HRESULT = 0x80280801u32 as HRESULT;

/// The TPM is currently executing a full selftest.
pub const TPM_E_DOING_SELFTEST: HRESULT = 0x80280802u32 as HRESULT;

/// The TPM is defending against dictionary attacks and is in a time-out period.
pub const TPM_E_DEFEND_LOCK_RUNNING: HRESULT = 0x80280803u32 as HRESULT;

/// TPM 2.0: Gap for context ID is too large.
pub const TPM_20_E_CONTEXT_GAP: HRESULT = 0x80280901u32 as HRESULT;

/// TPM 2.0: Out of memory for object contexts.
pub const TPM_20_E_OBJECT_MEMORY: HRESULT = 0x80280902u32 as HRESULT;

/// TPM 2.0: Out of memory for session contexts.
pub const TPM_20_E_SESSION_MEMORY: HRESULT = 0x80280903u32 as HRESULT;

/// TPM 2.0: Out of shared object/session memory or need space for internal operations.
pub const TPM_20_E_MEMORY: HRESULT = 0x80280904u32 as HRESULT;

/// TPM 2.0: Out of session handles - a session must be flushed before a nes session may be created.
pub const TPM_20_E_SESSION_HANDLES: HRESULT = 0x80280905u32 as HRESULT;

/// TPM 2.0: Out of object handles - the handle space for objects is depleted and a reboot is required.
pub const TPM_20_E_OBJECT_HANDLES: HRESULT = 0x80280906u32 as HRESULT;

/// TPM 2.0: Bad locality.
pub const TPM_20_E_LOCALITY: HRESULT = 0x80280907u32 as HRESULT;

/// TPM 2.0: The TPM has suspended operation on the command; forward progress was made and the command may be retried.
pub const TPM_20_E_YIELDED: HRESULT = 0x80280908u32 as HRESULT;

/// TPM 2.0: The command was canceled.
pub const TPM_20_E_CANCELED: HRESULT = 0x80280909u32 as HRESULT;

/// TPM 2.0: TPM is performing self-tests.
pub const TPM_20_E_TESTING: HRESULT = 0x8028090Au32 as HRESULT;

/// TPM 2.0: The TPM is rate-limiting accesses to prevent wearout of NV
pub const TPM_20_E_NV_RATE: HRESULT = 0x80280920u32 as HRESULT;

/// TPM 2.0: Authorization for objects subject to DA protection are not allowed at this time because the TPM is in DA lockout mode.
pub const TPM_20_E_LOCKOUT: HRESULT = 0x80280921u32 as HRESULT;

/// TPM 2.0: The TPM was not able to start the command.
pub const TPM_20_E_RETRY: HRESULT = 0x80280922u32 as HRESULT;

/// TPM 2.0: the command may require writing of NV and NV is not current accessible.
pub const TPM_20_E_NV_UNAVAILABLE: HRESULT = 0x80280923u32 as HRESULT;

/// An internal error has occurred within the Trusted Platform Module support program.
pub const TBS_E_INTERNAL_ERROR: HRESULT = 0x80284001u32 as HRESULT;

/// One or more input parameters is bad.
pub const TBS_E_BAD_PARAMETER: HRESULT = 0x80284002u32 as HRESULT;

/// A specified output pointer is bad.
pub const TBS_E_INVALID_OUTPUT_POINTER: HRESULT = 0x80284003u32 as HRESULT;

/// The specified context handle does not refer to a valid context.
pub const TBS_E_INVALID_CONTEXT: HRESULT = 0x80284004u32 as HRESULT;

/// A specified output buffer is too small.
pub const TBS_E_INSUFFICIENT_BUFFER: HRESULT = 0x80284005u32 as HRESULT;

/// An error occurred while communicating with the TPM.
pub const TBS_E_IOERROR: HRESULT = 0x80284006u32 as HRESULT;

/// One or more context parameters is invalid.
pub const TBS_E_INVALID_CONTEXT_PARAM: HRESULT = 0x80284007u32 as HRESULT;

/// The TBS service is not running and could not be started.
pub const TBS_E_SERVICE_NOT_RUNNING: HRESULT = 0x80284008u32 as HRESULT;

/// A new context could not be created because there are too many open contexts.
pub const TBS_E_TOO_MANY_TBS_CONTEXTS: HRESULT = 0x80284009u32 as HRESULT;

/// A new virtual resource could not be created because there are too many open virtual resources.
pub const TBS_E_TOO_MANY_RESOURCES: HRESULT = 0x8028400Au32 as HRESULT;

/// The TBS service has been started but is not yet running.
pub const TBS_E_SERVICE_START_PENDING: HRESULT = 0x8028400Bu32 as HRESULT;

/// The physical presence interface is not supported.
pub const TBS_E_PPI_NOT_SUPPORTED: HRESULT = 0x8028400Cu32 as HRESULT;

/// The command was canceled.
pub const TBS_E_COMMAND_CANCELED: HRESULT = 0x8028400Du32 as HRESULT;

/// The input or output buffer is too large.
pub const TBS_E_BUFFER_TOO_LARGE: HRESULT = 0x8028400Eu32 as HRESULT;

/// A compatible Trusted Platform Module (TPM) Security Device cannot be found on this computer.
pub const TBS_E_TPM_NOT_FOUND: HRESULT = 0x8028400Fu32 as HRESULT;

/// The TBS service has been disabled.
pub const TBS_E_SERVICE_DISABLED: HRESULT = 0x80284010u32 as HRESULT;

/// No TCG event log is available.
pub const TBS_E_NO_EVENT_LOG: HRESULT = 0x80284011u32 as HRESULT;

/// The caller does not have the appropriate rights to perform the requested operation.
pub const TBS_E_ACCESS_DENIED: HRESULT = 0x80284012u32 as HRESULT;

/// The TPM provisioning action is not allowed by the specified flags.  For provisioning to be successful, one of several actions may be required.  The TPM management console (tpm.msc) action to make the TPM Ready may help.  For further information, see the documentation for the Win32_Tpm WMI method 'Provision'.  (The actions that may be required include importing the TPM Owner Authorization value into the system, calling the Win32_Tpm WMI method for provisioning the TPM and specifying TRUE for either 'ForceClear_Allowed' or 'PhysicalPresencePrompts_Allowed' (as indicated by the value returned in the Additional Information), or enabling the TPM in the system BIOS.)
pub const TBS_E_PROVISIONING_NOT_ALLOWED: HRESULT = 0x80284013u32 as HRESULT;

/// The Physical Presence Interface of this firmware does not support the requested method.
pub const TBS_E_PPI_FUNCTION_UNSUPPORTED: HRESULT = 0x80284014u32 as HRESULT;

/// The requested TPM OwnerAuth value was not found.
pub const TBS_E_OWNERAUTH_NOT_FOUND: HRESULT = 0x80284015u32 as HRESULT;

/// The TPM provisioning did not complete.  For more information on completing the provisioning, call the Win32_Tpm WMI method for provisioning the TPM ('Provision') and check the returned Information.
pub const TBS_E_PROVISIONING_INCOMPLETE: HRESULT = 0x80284016u32 as HRESULT;

/// The command buffer is not in the correct state.
pub const TPMAPI_E_INVALID_STATE: HRESULT = 0x80290100u32 as HRESULT;

/// The command buffer does not contain enough data to satisfy the request.
pub const TPMAPI_E_NOT_ENOUGH_DATA: HRESULT = 0x80290101u32 as HRESULT;

/// The command buffer cannot contain any more data.
pub const TPMAPI_E_TOO_MUCH_DATA: HRESULT = 0x80290102u32 as HRESULT;

/// One or more output parameters was NULL or invalid.
pub const TPMAPI_E_INVALID_OUTPUT_POINTER: HRESULT = 0x80290103u32 as HRESULT;

/// One or more input parameters is invalid.
pub const TPMAPI_E_INVALID_PARAMETER: HRESULT = 0x80290104u32 as HRESULT;

/// Not enough memory was available to satisfy the request.
pub const TPMAPI_E_OUT_OF_MEMORY: HRESULT = 0x80290105u32 as HRESULT;

/// The specified buffer was too small.
pub const TPMAPI_E_BUFFER_TOO_SMALL: HRESULT = 0x80290106u32 as HRESULT;

/// An internal error was detected.
pub const TPMAPI_E_INTERNAL_ERROR: HRESULT = 0x80290107u32 as HRESULT;

/// The caller does not have the appropriate rights to perform the requested operation.
pub const TPMAPI_E_ACCESS_DENIED: HRESULT = 0x80290108u32 as HRESULT;

/// The specified authorization information was invalid.
pub const TPMAPI_E_AUTHORIZATION_FAILED: HRESULT = 0x80290109u32 as HRESULT;

/// The specified context handle was not valid.
pub const TPMAPI_E_INVALID_CONTEXT_HANDLE: HRESULT = 0x8029010Au32 as HRESULT;

/// An error occurred while communicating with the TBS.
pub const TPMAPI_E_TBS_COMMUNICATION_ERROR: HRESULT = 0x8029010Bu32 as HRESULT;

/// The TPM returned an unexpected result.
pub const TPMAPI_E_TPM_COMMAND_ERROR: HRESULT = 0x8029010Cu32 as HRESULT;

/// The message was too large for the encoding scheme.
pub const TPMAPI_E_MESSAGE_TOO_LARGE: HRESULT = 0x8029010Du32 as HRESULT;

/// The encoding in the blob was not recognized.
pub const TPMAPI_E_INVALID_ENCODING: HRESULT = 0x8029010Eu32 as HRESULT;

/// The key size is not valid.
pub const TPMAPI_E_INVALID_KEY_SIZE: HRESULT = 0x8029010Fu32 as HRESULT;

/// The encryption operation failed.
pub const TPMAPI_E_ENCRYPTION_FAILED: HRESULT = 0x80290110u32 as HRESULT;

/// The key parameters structure was not valid
pub const TPMAPI_E_INVALID_KEY_PARAMS: HRESULT = 0x80290111u32 as HRESULT;

/// The requested supplied data does not appear to be a valid migration authorization blob.
pub const TPMAPI_E_INVALID_MIGRATION_AUTHORIZATION_BLOB: HRESULT = 0x80290112u32 as HRESULT;

/// The specified PCR index was invalid
pub const TPMAPI_E_INVALID_PCR_INDEX: HRESULT = 0x80290113u32 as HRESULT;

/// The data given does not appear to be a valid delegate blob.
pub const TPMAPI_E_INVALID_DELEGATE_BLOB: HRESULT = 0x80290114u32 as HRESULT;

/// One or more of the specified context parameters was not valid.
pub const TPMAPI_E_INVALID_CONTEXT_PARAMS: HRESULT = 0x80290115u32 as HRESULT;

/// The data given does not appear to be a valid key blob
pub const TPMAPI_E_INVALID_KEY_BLOB: HRESULT = 0x80290116u32 as HRESULT;

/// The specified PCR data was invalid.
pub const TPMAPI_E_INVALID_PCR_DATA: HRESULT = 0x80290117u32 as HRESULT;

/// The format of the owner auth data was invalid.
pub const TPMAPI_E_INVALID_OWNER_AUTH: HRESULT = 0x80290118u32 as HRESULT;

/// The random number generated did not pass FIPS RNG check.
pub const TPMAPI_E_FIPS_RNG_CHECK_FAILED: HRESULT = 0x80290119u32 as HRESULT;

/// The TCG Event Log does not contain any data.
pub const TPMAPI_E_EMPTY_TCG_LOG: HRESULT = 0x8029011Au32 as HRESULT;

/// An entry in the TCG Event Log was invalid.
pub const TPMAPI_E_INVALID_TCG_LOG_ENTRY: HRESULT = 0x8029011Bu32 as HRESULT;

/// A TCG Separator was not found.
pub const TPMAPI_E_TCG_SEPARATOR_ABSENT: HRESULT = 0x8029011Cu32 as HRESULT;

/// A digest value in a TCG Log entry did not match hashed data.
pub const TPMAPI_E_TCG_INVALID_DIGEST_ENTRY: HRESULT = 0x8029011Du32 as HRESULT;

/// The requested operation was blocked by current TPM policy. Please contact your system administrator for assistance.
pub const TPMAPI_E_POLICY_DENIES_OPERATION: HRESULT = 0x8029011Eu32 as HRESULT;

/// The Windows TPM NV Bits index is not defined.
pub const TPMAPI_E_NV_BITS_NOT_DEFINED: HRESULT = 0x8029011Fu32 as HRESULT;

/// The Windows TPM NV Bits index is not ready for use.
pub const TPMAPI_E_NV_BITS_NOT_READY: HRESULT = 0x80290120u32 as HRESULT;

/// The TPM key that was used to seal the data is no longer available.
pub const TPMAPI_E_SEALING_KEY_NOT_AVAILABLE: HRESULT = 0x80290121u32 as HRESULT;

/// An authorization chain could not be found that authorizes the PolicyAuthorize unseal.
pub const TPMAPI_E_NO_AUTHORIZATION_CHAIN_FOUND: HRESULT = 0x80290122u32 as HRESULT;

/// The SVN counter to which the authorization was bound is not available.
pub const TPMAPI_E_SVN_COUNTER_NOT_AVAILABLE: HRESULT = 0x80290123u32 as HRESULT;

/// The TPM Storage hierarchy (Owner) auth value is required to be NULL for this operation.
pub const TPMAPI_E_OWNER_AUTH_NOT_NULL: HRESULT = 0x80290124u32 as HRESULT;

/// The TPM Endorsement hierarchy auth value is required to be NULL for this operation.
pub const TPMAPI_E_ENDORSEMENT_AUTH_NOT_NULL: HRESULT = 0x80290125u32 as HRESULT;

/// The authorization to perform this operation has been revoked.
pub const TPMAPI_E_AUTHORIZATION_REVOKED: HRESULT = 0x80290126u32 as HRESULT;

/// The authorization public key is malformed.
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_KEY: HRESULT = 0x80290127u32 as HRESULT;

/// The authorization public key is not supported.
pub const TPMAPI_E_AUTHORIZING_KEY_NOT_SUPPORTED: HRESULT = 0x80290128u32 as HRESULT;

/// The authorization signature is invalid.
pub const TPMAPI_E_INVALID_AUTHORIZATION_SIGNATURE: HRESULT = 0x80290129u32 as HRESULT;

/// The authorization policy is malformed.
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_POLICY: HRESULT = 0x8029012Au32 as HRESULT;

/// The authorization data is malformed.
pub const TPMAPI_E_MALFORMED_AUTHORIZATION_OTHER: HRESULT = 0x8029012Bu32 as HRESULT;

/// The key used to unseal this data has changed since sealing the data. This may be the result of a TPM clear.
pub const TPMAPI_E_SEALING_KEY_CHANGED: HRESULT = 0x8029012Cu32 as HRESULT;

/// The TPM version is invalid.
pub const TPMAPI_E_INVALID_TPM_VERSION: HRESULT = 0x8029012Du32 as HRESULT;

/// The policy authorization blob type is invalid.
pub const TPMAPI_E_INVALID_POLICYAUTH_BLOB_TYPE: HRESULT = 0x8029012Eu32 as HRESULT;

/// The specified buffer was too small.
pub const TBSIMP_E_BUFFER_TOO_SMALL: HRESULT = 0x80290200u32 as HRESULT;

/// The context could not be cleaned up.
pub const TBSIMP_E_CLEANUP_FAILED: HRESULT = 0x80290201u32 as HRESULT;

/// The specified context handle is invalid.
pub const TBSIMP_E_INVALID_CONTEXT_HANDLE: HRESULT = 0x80290202u32 as HRESULT;

/// An invalid context parameter was specified.
pub const TBSIMP_E_INVALID_CONTEXT_PARAM: HRESULT = 0x80290203u32 as HRESULT;

/// An error occurred while communicating with the TPM
pub const TBSIMP_E_TPM_ERROR: HRESULT = 0x80290204u32 as HRESULT;

/// No entry with the specified key was found.
pub const TBSIMP_E_HASH_BAD_KEY: HRESULT = 0x80290205u32 as HRESULT;

/// The specified virtual handle matches a virtual handle already in use.
pub const TBSIMP_E_DUPLICATE_VHANDLE: HRESULT = 0x80290206u32 as HRESULT;

/// The pointer to the returned handle location was NULL or invalid
pub const TBSIMP_E_INVALID_OUTPUT_POINTER: HRESULT = 0x80290207u32 as HRESULT;

/// One or more parameters is invalid
pub const TBSIMP_E_INVALID_PARAMETER: HRESULT = 0x80290208u32 as HRESULT;

/// The RPC subsystem could not be initialized.
pub const TBSIMP_E_RPC_INIT_FAILED: HRESULT = 0x80290209u32 as HRESULT;

/// The TBS scheduler is not running.
pub const TBSIMP_E_SCHEDULER_NOT_RUNNING: HRESULT = 0x8029020Au32 as HRESULT;

/// The command was canceled.
pub const TBSIMP_E_COMMAND_CANCELED: HRESULT = 0x8029020Bu32 as HRESULT;

/// There was not enough memory to fulfill the request
pub const TBSIMP_E_OUT_OF_MEMORY: HRESULT = 0x8029020Cu32 as HRESULT;

/// The specified list is empty, or the iteration has reached the end of the list.
pub const TBSIMP_E_LIST_NO_MORE_ITEMS: HRESULT = 0x8029020Du32 as HRESULT;

/// The specified item was not found in the list.
pub const TBSIMP_E_LIST_NOT_FOUND: HRESULT = 0x8029020Eu32 as HRESULT;

/// The TPM does not have enough space to load the requested resource.
pub const TBSIMP_E_NOT_ENOUGH_SPACE: HRESULT = 0x8029020Fu32 as HRESULT;

/// There are too many TPM contexts in use.
pub const TBSIMP_E_NOT_ENOUGH_TPM_CONTEXTS: HRESULT = 0x80290210u32 as HRESULT;

/// The TPM command failed.
pub const TBSIMP_E_COMMAND_FAILED: HRESULT = 0x80290211u32 as HRESULT;

/// The TBS does not recognize the specified ordinal.
pub const TBSIMP_E_UNKNOWN_ORDINAL: HRESULT = 0x80290212u32 as HRESULT;

/// The requested resource is no longer available.
pub const TBSIMP_E_RESOURCE_EXPIRED: HRESULT = 0x80290213u32 as HRESULT;

/// The resource type did not match.
pub const TBSIMP_E_INVALID_RESOURCE: HRESULT = 0x80290214u32 as HRESULT;

/// No resources can be unloaded.
pub const TBSIMP_E_NOTHING_TO_UNLOAD: HRESULT = 0x80290215u32 as HRESULT;

/// No new entries can be added to the hash table.
pub const TBSIMP_E_HASH_TABLE_FULL: HRESULT = 0x80290216u32 as HRESULT;

/// A new TBS context could not be created because there are too many open contexts.
pub const TBSIMP_E_TOO_MANY_TBS_CONTEXTS: HRESULT = 0x80290217u32 as HRESULT;

/// A new virtual resource could not be created because there are too many open virtual resources.
pub const TBSIMP_E_TOO_MANY_RESOURCES: HRESULT = 0x80290218u32 as HRESULT;

/// The physical presence interface is not supported.
pub const TBSIMP_E_PPI_NOT_SUPPORTED: HRESULT = 0x80290219u32 as HRESULT;

/// TBS is not compatible with the version of TPM found on the system.
pub const TBSIMP_E_TPM_INCOMPATIBLE: HRESULT = 0x8029021Au32 as HRESULT;

/// No TCG event log is available.
pub const TBSIMP_E_NO_EVENT_LOG: HRESULT = 0x8029021Bu32 as HRESULT;

/// A general error was detected when attempting to acquire the BIOS's response to a Physical Presence command.
pub const TPM_E_PPI_ACPI_FAILURE: HRESULT = 0x80290300u32 as HRESULT;

/// The user failed to confirm the TPM operation request.
pub const TPM_E_PPI_USER_ABORT: HRESULT = 0x80290301u32 as HRESULT;

/// The BIOS failure prevented the successful execution of the requested TPM operation (e.g. invalid TPM operation request, BIOS communication error with the TPM).
pub const TPM_E_PPI_BIOS_FAILURE: HRESULT = 0x80290302u32 as HRESULT;

/// The BIOS does not support the physical presence interface.
pub const TPM_E_PPI_NOT_SUPPORTED: HRESULT = 0x80290303u32 as HRESULT;

/// The Physical Presence command was blocked by current BIOS settings. The system owner may be able to reconfigure the BIOS settings to allow the command.
pub const TPM_E_PPI_BLOCKED_IN_BIOS: HRESULT = 0x80290304u32 as HRESULT;

/// This is an error mask to convert Platform Crypto Provider errors to win errors.
pub const TPM_E_PCP_ERROR_MASK: HRESULT = 0x80290400u32 as HRESULT;

/// The Platform Crypto Device is currently not ready. It needs to be fully provisioned to be operational.
pub const TPM_E_PCP_DEVICE_NOT_READY: HRESULT = 0x80290401u32 as HRESULT;

/// The handle provided to the Platform Crypto Provider is invalid.
pub const TPM_E_PCP_INVALID_HANDLE: HRESULT = 0x80290402u32 as HRESULT;

/// A parameter provided to the Platform Crypto Provider is invalid.
pub const TPM_E_PCP_INVALID_PARAMETER: HRESULT = 0x80290403u32 as HRESULT;

/// A provided flag to the Platform Crypto Provider is not supported.
pub const TPM_E_PCP_FLAG_NOT_SUPPORTED: HRESULT = 0x80290404u32 as HRESULT;

/// The requested operation is not supported by this Platform Crypto Provider.
pub const TPM_E_PCP_NOT_SUPPORTED: HRESULT = 0x80290405u32 as HRESULT;

/// The buffer is too small to contain all data. No information has been written to the buffer.
pub const TPM_E_PCP_BUFFER_TOO_SMALL: HRESULT = 0x80290406u32 as HRESULT;

/// An unexpected internal error has occurred in the Platform Crypto Provider.
pub const TPM_E_PCP_INTERNAL_ERROR: HRESULT = 0x80290407u32 as HRESULT;

/// The authorization to use a provider object has failed.
pub const TPM_E_PCP_AUTHENTICATION_FAILED: HRESULT = 0x80290408u32 as HRESULT;

/// The Platform Crypto Device has ignored the authorization for the provider object, to mitigate against a dictionary attack.
pub const TPM_E_PCP_AUTHENTICATION_IGNORED: HRESULT = 0x80290409u32 as HRESULT;

/// The referenced policy was not found.
pub const TPM_E_PCP_POLICY_NOT_FOUND: HRESULT = 0x8029040Au32 as HRESULT;

/// The referenced profile was not found.
pub const TPM_E_PCP_PROFILE_NOT_FOUND: HRESULT = 0x8029040Bu32 as HRESULT;

/// The validation was not succesful.
pub const TPM_E_PCP_VALIDATION_FAILED: HRESULT = 0x8029040Cu32 as HRESULT;

/// An attempt was made to import or load a key under an incorrect storage parent.
pub const TPM_E_PCP_WRONG_PARENT: HRESULT = 0x8029040Eu32 as HRESULT;

/// The TPM key is not loaded.
pub const TPM_E_KEY_NOT_LOADED: HRESULT = 0x8029040Fu32 as HRESULT;

/// The TPM key certification has not been generated.
pub const TPM_E_NO_KEY_CERTIFICATION: HRESULT = 0x80290410u32 as HRESULT;

/// The TPM key is not yet finalized.
pub const TPM_E_KEY_NOT_FINALIZED: HRESULT = 0x80290411u32 as HRESULT;

/// The TPM attestation challenge is not set.
pub const TPM_E_ATTESTATION_CHALLENGE_NOT_SET: HRESULT = 0x80290412u32 as HRESULT;

/// The TPM PCR info is not available.
pub const TPM_E_NOT_PCR_BOUND: HRESULT = 0x80290413u32 as HRESULT;

/// The TPM key is already finalized.
pub const TPM_E_KEY_ALREADY_FINALIZED: HRESULT = 0x80290414u32 as HRESULT;

/// The TPM key usage policy is not supported.
pub const TPM_E_KEY_USAGE_POLICY_NOT_SUPPORTED: HRESULT = 0x80290415u32 as HRESULT;

/// The TPM key usage policy is invalid.
pub const TPM_E_KEY_USAGE_POLICY_INVALID: HRESULT = 0x80290416u32 as HRESULT;

/// There was a problem with the software key being imported into the TPM.
pub const TPM_E_SOFT_KEY_ERROR: HRESULT = 0x80290417u32 as HRESULT;

/// The TPM key is not authenticated.
pub const TPM_E_KEY_NOT_AUTHENTICATED: HRESULT = 0x80290418u32 as HRESULT;

/// The TPM key is not an AIK.
pub const TPM_E_PCP_KEY_NOT_AIK: HRESULT = 0x80290419u32 as HRESULT;

/// The TPM key is not a signing key.
pub const TPM_E_KEY_NOT_SIGNING_KEY: HRESULT = 0x8029041Au32 as HRESULT;

/// The TPM is locked out.
pub const TPM_E_LOCKED_OUT: HRESULT = 0x8029041Bu32 as HRESULT;

/// The claim type requested is not supported.
pub const TPM_E_CLAIM_TYPE_NOT_SUPPORTED: HRESULT = 0x8029041Cu32 as HRESULT;

/// TPM version is not supported.
pub const TPM_E_VERSION_NOT_SUPPORTED: HRESULT = 0x8029041Du32 as HRESULT;

/// The buffer lengths do not match.
pub const TPM_E_BUFFER_LENGTH_MISMATCH: HRESULT = 0x8029041Eu32 as HRESULT;

/// The RSA key creation is blocked on this TPM due to known security vulnerabilities.
pub const TPM_E_PCP_IFX_RSA_KEY_CREATION_BLOCKED: HRESULT = 0x8029041Fu32 as HRESULT;

/// A ticket required to use a key was not provided.
pub const TPM_E_PCP_TICKET_MISSING: HRESULT = 0x80290420u32 as HRESULT;

/// This key has a raw policy so the KSP can't authenticate against it.
pub const TPM_E_PCP_RAW_POLICY_NOT_SUPPORTED: HRESULT = 0x80290421u32 as HRESULT;

/// The TPM key's handle was unexpectedly invalidated due to a hardware or firmware issue.
pub const TPM_E_PCP_KEY_HANDLE_INVALIDATED: HRESULT = 0x80290422u32 as HRESULT;

/// The requested salt size for signing with RSAPSS does not match what the TPM uses.
pub const TPM_E_PCP_UNSUPPORTED_PSS_SALT: HRESULT = 0x40290423;

/// Validation of the platform claim failed.
pub const TPM_E_PCP_PLATFORM_CLAIM_MAY_BE_OUTDATED: HRESULT = 0x40290424;

/// The requested platform claim is for a previous boot.
pub const TPM_E_PCP_PLATFORM_CLAIM_OUTDATED: HRESULT = 0x40290425;

/// The platform claim is for a previous boot, and cannot be created without reboot.
pub const TPM_E_PCP_PLATFORM_CLAIM_REBOOT: HRESULT = 0x40290426;

/// TPM related network operations are blocked as Zero Exhaust mode is enabled on client.
pub const TPM_E_ZERO_EXHAUST_ENABLED: HRESULT = 0x80290500u32 as HRESULT;

/// TPM provisioning did not run to completion.
pub const TPM_E_PROVISIONING_INCOMPLETE: HRESULT = 0x80290600u32 as HRESULT;

/// An invalid owner authorization value was specified.
pub const TPM_E_INVALID_OWNER_AUTH: HRESULT = 0x80290601u32 as HRESULT;

/// TPM command returned too much data.
pub const TPM_E_TOO_MUCH_DATA: HRESULT = 0x80290602u32 as HRESULT;

/// Unable to fetch EK Certificate if TPM-generated EPS (after a TPM2_ChangeEPS).
pub const TPM_E_TPM_GENERATED_EPS: HRESULT = 0x80290603u32 as HRESULT;

/// Data Collector Set was not found.
pub const PLA_E_DCS_NOT_FOUND: HRESULT = 0x80300002u32 as HRESULT;

/// The Data Collector Set or one of its dependencies is already in use.
pub const PLA_E_DCS_IN_USE: HRESULT = 0x803000AAu32 as HRESULT;

/// Unable to start Data Collector Set because there are too many folders.
pub const PLA_E_TOO_MANY_FOLDERS: HRESULT = 0x80300045u32 as HRESULT;

/// Not enough free disk space to start Data Collector Set.
pub const PLA_E_NO_MIN_DISK: HRESULT = 0x80300070u32 as HRESULT;

/// Data Collector Set already exists.
pub const PLA_E_DCS_ALREADY_EXISTS: HRESULT = 0x803000B7u32 as HRESULT;

/// Property value will be ignored.
pub const PLA_S_PROPERTY_IGNORED: HRESULT = 0x00300100;

/// Property value conflict.
pub const PLA_E_PROPERTY_CONFLICT: HRESULT = 0x80300101u32 as HRESULT;

/// The current configuration for this Data Collector Set requires that it contain exactly one Data Collector.
pub const PLA_E_DCS_SINGLETON_REQUIRED: HRESULT = 0x80300102u32 as HRESULT;

/// A user account is required in order to commit the current Data Collector Set properties.
pub const PLA_E_CREDENTIALS_REQUIRED: HRESULT = 0x80300103u32 as HRESULT;

/// Data Collector Set is not running.
pub const PLA_E_DCS_NOT_RUNNING: HRESULT = 0x80300104u32 as HRESULT;

/// A conflict was detected in the list of include/exclude APIs. Do not specify the same API in both the include list and the exclude list.
pub const PLA_E_CONFLICT_INCL_EXCL_API: HRESULT = 0x80300105u32 as HRESULT;

/// The executable path you have specified refers to a network share or UNC path.
pub const PLA_E_NETWORK_EXE_NOT_VALID: HRESULT = 0x80300106u32 as HRESULT;

/// The executable path you have specified is already configured for API tracing.
pub const PLA_E_EXE_ALREADY_CONFIGURED: HRESULT = 0x80300107u32 as HRESULT;

/// The executable path you have specified does not exist. Verify that the specified path is correct.
pub const PLA_E_EXE_PATH_NOT_VALID: HRESULT = 0x80300108u32 as HRESULT;

/// Data Collector already exists.
pub const PLA_E_DC_ALREADY_EXISTS: HRESULT = 0x80300109u32 as HRESULT;

/// The wait for the Data Collector Set start notification has timed out.
pub const PLA_E_DCS_START_WAIT_TIMEOUT: HRESULT = 0x8030010Au32 as HRESULT;

/// The wait for the Data Collector to start has timed out.
pub const PLA_E_DC_START_WAIT_TIMEOUT: HRESULT = 0x8030010Bu32 as HRESULT;

/// The wait for the report generation tool to finish has timed out.
pub const PLA_E_REPORT_WAIT_TIMEOUT: HRESULT = 0x8030010Cu32 as HRESULT;

/// Duplicate items are not allowed.
pub const PLA_E_NO_DUPLICATES: HRESULT = 0x8030010Du32 as HRESULT;

/// When specifying the executable that you want to trace, you must specify a full path to the executable and not just a filename.
pub const PLA_E_EXE_FULL_PATH_REQUIRED: HRESULT = 0x8030010Eu32 as HRESULT;

/// The session name provided is invalid.
pub const PLA_E_INVALID_SESSION_NAME: HRESULT = 0x8030010Fu32 as HRESULT;

/// The Event Log channel Microsoft-Windows-Diagnosis-PLA/Operational must be enabled to perform this operation.
pub const PLA_E_PLA_CHANNEL_NOT_ENABLED: HRESULT = 0x80300110u32 as HRESULT;

/// The Event Log channel Microsoft-Windows-TaskScheduler must be enabled to perform this operation.
pub const PLA_E_TASKSCHED_CHANNEL_NOT_ENABLED: HRESULT = 0x80300111u32 as HRESULT;

/// The execution of the Rules Manager failed.
pub const PLA_E_RULES_MANAGER_FAILED: HRESULT = 0x80300112u32 as HRESULT;

/// An error occurred while attempting to compress or extract the data.
pub const PLA_E_CABAPI_FAILURE: HRESULT = 0x80300113u32 as HRESULT;

/// This drive is locked by BitLocker Drive Encryption. You must unlock this drive from Control Panel.
pub const FVE_E_LOCKED_VOLUME: HRESULT = 0x80310000u32 as HRESULT;

/// This drive is not encrypted.
pub const FVE_E_NOT_ENCRYPTED: HRESULT = 0x80310001u32 as HRESULT;

/// The BIOS did not correctly communicate with the Trusted Platform Module (TPM). Contact the computer manufacturer for BIOS upgrade instructions.
pub const FVE_E_NO_TPM_BIOS: HRESULT = 0x80310002u32 as HRESULT;

/// The BIOS did not correctly communicate with the master boot record (MBR). Contact the computer manufacturer for BIOS upgrade instructions.
pub const FVE_E_NO_MBR_METRIC: HRESULT = 0x80310003u32 as HRESULT;

/// A required TPM measurement is missing. If there is a bootable CD or DVD in your computer, remove it, restart the computer, and turn on BitLocker again. If the problem persists, ensure the master boot record is up to date.
pub const FVE_E_NO_BOOTSECTOR_METRIC: HRESULT = 0x80310004u32 as HRESULT;

/// The boot sector of this drive is not compatible with BitLocker Drive Encryption. Use the Bootrec.exe tool in the Windows Recovery Environment to update or repair the boot manager (BOOTMGR).
pub const FVE_E_NO_BOOTMGR_METRIC: HRESULT = 0x80310005u32 as HRESULT;

/// The boot manager of this operating system is not compatible with BitLocker Drive Encryption. Use the Bootrec.exe tool in the Windows Recovery Environment to update or repair the boot manager (BOOTMGR).
pub const FVE_E_WRONG_BOOTMGR: HRESULT = 0x80310006u32 as HRESULT;

/// At least one secure key protector is required for this operation to be performed.
pub const FVE_E_SECURE_KEY_REQUIRED: HRESULT = 0x80310007u32 as HRESULT;

/// BitLocker Drive Encryption is not enabled on this drive. Turn on BitLocker.
pub const FVE_E_NOT_ACTIVATED: HRESULT = 0x80310008u32 as HRESULT;

/// BitLocker Drive Encryption cannot perform the requested action. This condition may occur when two requests are issued at the same time. Wait a few moments and then try the action again.
pub const FVE_E_ACTION_NOT_ALLOWED: HRESULT = 0x80310009u32 as HRESULT;

/// The Active Directory Domain Services forest does not contain the required attributes and classes to host BitLocker Drive Encryption or Trusted Platform Module information. Contact your domain administrator to verify that any required BitLocker Active Directory schema extensions have been installed.
pub const FVE_E_AD_SCHEMA_NOT_INSTALLED: HRESULT = 0x8031000Au32 as HRESULT;

/// The type of the data obtained from Active Directory was not expected. The BitLocker recovery information may be missing or corrupted.
pub const FVE_E_AD_INVALID_DATATYPE: HRESULT = 0x8031000Bu32 as HRESULT;

/// The size of the data obtained from Active Directory was not expected. The BitLocker recovery information may be missing or corrupted.
pub const FVE_E_AD_INVALID_DATASIZE: HRESULT = 0x8031000Cu32 as HRESULT;

/// The attribute read from Active Directory does not contain any values. The BitLocker recovery information may be missing or corrupted.
pub const FVE_E_AD_NO_VALUES: HRESULT = 0x8031000Du32 as HRESULT;

/// The attribute was not set. Verify that you are logged on with a domain account that has the ability to write information to Active Directory objects.
pub const FVE_E_AD_ATTR_NOT_SET: HRESULT = 0x8031000Eu32 as HRESULT;

/// The specified attribute cannot be found in Active Directory Domain Services. Contact your domain administrator to verify that any required BitLocker Active Directory schema extensions have been installed.
pub const FVE_E_AD_GUID_NOT_FOUND: HRESULT = 0x8031000Fu32 as HRESULT;

/// The BitLocker metadata for the encrypted drive is not valid. You can attempt to repair the drive to restore access.
pub const FVE_E_BAD_INFORMATION: HRESULT = 0x80310010u32 as HRESULT;

/// The drive cannot be encrypted because it does not have enough free space. Delete any unnecessary data on the drive to create additional free space and then try again.
pub const FVE_E_TOO_SMALL: HRESULT = 0x80310011u32 as HRESULT;

/// The drive cannot be encrypted because it contains system boot information. Create a separate partition for use as the system drive that contains the boot information and a second partition for use as the operating system drive and then encrypt the operating system drive.
pub const FVE_E_SYSTEM_VOLUME: HRESULT = 0x80310012u32 as HRESULT;

/// The drive cannot be encrypted because the file system is not supported.
pub const FVE_E_FAILED_WRONG_FS: HRESULT = 0x80310013u32 as HRESULT;

/// The file system size is larger than the partition size in the partition table. This drive may be corrupt or may have been tampered with. To use it with BitLocker, you must reformat the partition.
pub const FVE_E_BAD_PARTITION_SIZE: HRESULT = 0x80310014u32 as HRESULT;

/// This drive cannot be encrypted.
pub const FVE_E_NOT_SUPPORTED: HRESULT = 0x80310015u32 as HRESULT;

/// The data is not valid.
pub const FVE_E_BAD_DATA: HRESULT = 0x80310016u32 as HRESULT;

/// The data drive specified is not set to automatically unlock on the current computer and cannot be unlocked automatically.
pub const FVE_E_VOLUME_NOT_BOUND: HRESULT = 0x80310017u32 as HRESULT;

/// You must initialize the Trusted Platform Module (TPM) before you can use BitLocker Drive Encryption.
pub const FVE_E_TPM_NOT_OWNED: HRESULT = 0x80310018u32 as HRESULT;

/// The operation attempted cannot be performed on an operating system drive.
pub const FVE_E_NOT_DATA_VOLUME: HRESULT = 0x80310019u32 as HRESULT;

/// The buffer supplied to a function was insufficient to contain the returned data. Increase the buffer size before running the function again.
pub const FVE_E_AD_INSUFFICIENT_BUFFER: HRESULT = 0x8031001Au32 as HRESULT;

/// A read operation failed while converting the drive. The drive was not converted. Please re-enable BitLocker.
pub const FVE_E_CONV_READ: HRESULT = 0x8031001Bu32 as HRESULT;

/// A write operation failed while converting the drive. The drive was not converted. Please re-enable BitLocker.
pub const FVE_E_CONV_WRITE: HRESULT = 0x8031001Cu32 as HRESULT;

/// One or more BitLocker key protectors are required. You cannot delete the last key on this drive.
pub const FVE_E_KEY_REQUIRED: HRESULT = 0x8031001Du32 as HRESULT;

/// Cluster configurations are not supported by BitLocker Drive Encryption.
pub const FVE_E_CLUSTERING_NOT_SUPPORTED: HRESULT = 0x8031001Eu32 as HRESULT;

/// The drive specified is already configured to be automatically unlocked on the current computer.
pub const FVE_E_VOLUME_BOUND_ALREADY: HRESULT = 0x8031001Fu32 as HRESULT;

/// The operating system drive is not protected by BitLocker Drive Encryption.
pub const FVE_E_OS_NOT_PROTECTED: HRESULT = 0x80310020u32 as HRESULT;

/// BitLocker Drive Encryption has been suspended on this drive. All BitLocker key protectors configured for this drive are effectively disabled, and the drive will be automatically unlocked using an unencrypted (clear) key.
pub const FVE_E_PROTECTION_DISABLED: HRESULT = 0x80310021u32 as HRESULT;

/// The drive you are attempting to lock does not have any key protectors available for encryption because BitLocker protection is currently suspended. Re-enable BitLocker to lock this drive.
pub const FVE_E_RECOVERY_KEY_REQUIRED: HRESULT = 0x80310022u32 as HRESULT;

/// BitLocker cannot use the Trusted Platform Module (TPM) to protect a data drive. TPM protection can only be used with the operating system drive.
pub const FVE_E_FOREIGN_VOLUME: HRESULT = 0x80310023u32 as HRESULT;

/// The BitLocker metadata for the encrypted drive cannot be updated because it was locked for updating by another process. Please try this process again.
pub const FVE_E_OVERLAPPED_UPDATE: HRESULT = 0x80310024u32 as HRESULT;

/// The authorization data for the storage root key (SRK) of the Trusted Platform Module (TPM) is not zero and is therefore incompatible with BitLocker. Please initialize the TPM before attempting to use it with BitLocker.
pub const FVE_E_TPM_SRK_AUTH_NOT_ZERO: HRESULT = 0x80310025u32 as HRESULT;

/// The drive encryption algorithm cannot be used on this sector size.
pub const FVE_E_FAILED_SECTOR_SIZE: HRESULT = 0x80310026u32 as HRESULT;

/// The drive cannot be unlocked with the key provided. Confirm that you have provided the correct key and try again.
pub const FVE_E_FAILED_AUTHENTICATION: HRESULT = 0x80310027u32 as HRESULT;

/// The drive specified is not the operating system drive.
pub const FVE_E_NOT_OS_VOLUME: HRESULT = 0x80310028u32 as HRESULT;

/// BitLocker Drive Encryption cannot be turned off on the operating system drive until the auto unlock feature has been disabled for the fixed data drives and removable data drives associated with this computer.
pub const FVE_E_AUTOUNLOCK_ENABLED: HRESULT = 0x80310029u32 as HRESULT;

/// The system partition boot sector does not perform Trusted Platform Module (TPM) measurements. Use the Bootrec.exe tool in the Windows Recovery Environment to update or repair the boot sector.
pub const FVE_E_WRONG_BOOTSECTOR: HRESULT = 0x8031002Au32 as HRESULT;

/// BitLocker Drive Encryption operating system drives must be formatted with the NTFS file system in order to be encrypted. Convert the drive to NTFS, and then turn on BitLocker.
pub const FVE_E_WRONG_SYSTEM_FS: HRESULT = 0x8031002Bu32 as HRESULT;

/// Group Policy settings require that a recovery password be specified before encrypting the drive.
pub const FVE_E_POLICY_PASSWORD_REQUIRED: HRESULT = 0x8031002Cu32 as HRESULT;

/// The drive encryption algorithm and key cannot be set on a previously encrypted drive. To encrypt this drive with BitLocker Drive Encryption, remove the previous encryption and then turn on BitLocker.
pub const FVE_E_CANNOT_SET_FVEK_ENCRYPTED: HRESULT = 0x8031002Du32 as HRESULT;

/// BitLocker Drive Encryption cannot encrypt the specified drive because an encryption key is not available. Add a key protector to encrypt this drive.
pub const FVE_E_CANNOT_ENCRYPT_NO_KEY: HRESULT = 0x8031002Eu32 as HRESULT;

/// BitLocker Drive Encryption detected bootable media (CD or DVD) in the computer. Remove the media and restart the computer before configuring BitLocker.
pub const FVE_E_BOOTABLE_CDDVD: HRESULT = 0x80310030u32 as HRESULT;

/// This key protector cannot be added. Only one key protector of this type is allowed for this drive.
pub const FVE_E_PROTECTOR_EXISTS: HRESULT = 0x80310031u32 as HRESULT;

/// The recovery password file was not found because a relative path was specified. Recovery passwords must be saved to a fully qualified path. Environment variables configured on the computer can be used in the path.
pub const FVE_E_RELATIVE_PATH: HRESULT = 0x80310032u32 as HRESULT;

/// The specified key protector was not found on the drive. Try another key protector.
pub const FVE_E_PROTECTOR_NOT_FOUND: HRESULT = 0x80310033u32 as HRESULT;

/// The recovery key provided is corrupt and cannot be used to access the drive. An alternative recovery method, such as recovery password, a data recovery agent, or a backup version of the recovery key must be used to recover access to the drive.
pub const FVE_E_INVALID_KEY_FORMAT: HRESULT = 0x80310034u32 as HRESULT;

/// The format of the recovery password provided is invalid. BitLocker recovery passwords are 48 digits. Verify that the recovery password is in the correct format and then try again.
pub const FVE_E_INVALID_PASSWORD_FORMAT: HRESULT = 0x80310035u32 as HRESULT;

/// The random number generator check test failed.
pub const FVE_E_FIPS_RNG_CHECK_FAILED: HRESULT = 0x80310036u32 as HRESULT;

/// The Group Policy setting requiring FIPS compliance prevents a local recovery password from being generated or used by BitLocker Drive Encryption. When operating in FIPS-compliant mode, BitLocker recovery options can be either a recovery key stored on a USB drive or recovery through a data recovery agent.
pub const FVE_E_FIPS_PREVENTS_RECOVERY_PASSWORD: HRESULT = 0x80310037u32 as HRESULT;

/// The Group Policy setting requiring FIPS compliance prevents the recovery password from being saved to Active Directory. When operating in FIPS-compliant mode, BitLocker recovery options can be either a recovery key stored on a USB drive or recovery through a data recovery agent. Check your Group Policy settings configuration.
pub const FVE_E_FIPS_PREVENTS_EXTERNAL_KEY_EXPORT: HRESULT = 0x80310038u32 as HRESULT;

/// The drive must be fully decrypted to complete this operation.
pub const FVE_E_NOT_DECRYPTED: HRESULT = 0x80310039u32 as HRESULT;

/// The key protector specified cannot be used for this operation.
pub const FVE_E_INVALID_PROTECTOR_TYPE: HRESULT = 0x8031003Au32 as HRESULT;

/// No key protectors exist on the drive to perform the hardware test.
pub const FVE_E_NO_PROTECTORS_TO_TEST: HRESULT = 0x8031003Bu32 as HRESULT;

/// The BitLocker startup key or recovery password cannot be found on the USB device. Verify that you have the correct USB device, that the USB device is plugged into the computer on an active USB port, restart the computer, and then try again. If the problem persists, contact the computer manufacturer for BIOS upgrade instructions.
pub const FVE_E_KEYFILE_NOT_FOUND: HRESULT = 0x8031003Cu32 as HRESULT;

/// The BitLocker startup key or recovery password file provided is corrupt or invalid. Verify that you have the correct startup key or recovery password file and try again.
pub const FVE_E_KEYFILE_INVALID: HRESULT = 0x8031003Du32 as HRESULT;

/// The BitLocker encryption key cannot be obtained from the startup key or recovery password. Verify that you have the correct startup key or recovery password and try again.
pub const FVE_E_KEYFILE_NO_VMK: HRESULT = 0x8031003Eu32 as HRESULT;

/// The Trusted Platform Module (TPM) is disabled. The TPM must be enabled, initialized, and have valid ownership before it can be used with BitLocker Drive Encryption.
pub const FVE_E_TPM_DISABLED: HRESULT = 0x8031003Fu32 as HRESULT;

/// The BitLocker configuration of the specified drive cannot be managed because this computer is currently operating in Safe Mode. While in Safe Mode, BitLocker Drive Encryption can only be used for recovery purposes.
pub const FVE_E_NOT_ALLOWED_IN_SAFE_MODE: HRESULT = 0x80310040u32 as HRESULT;

/// The Trusted Platform Module (TPM) was unable to unlock the drive. Either the system boot information changed after choosing BitLocker settings or the PIN did not match. If the problem persists after several tries, there may be a hardware or firmware problem.
pub const FVE_E_TPM_INVALID_PCR: HRESULT = 0x80310041u32 as HRESULT;

/// The BitLocker encryption key cannot be obtained from the Trusted Platform Module (TPM).
pub const FVE_E_TPM_NO_VMK: HRESULT = 0x80310042u32 as HRESULT;

/// The BitLocker encryption key cannot be obtained from the Trusted Platform Module (TPM) and PIN.
pub const FVE_E_PIN_INVALID: HRESULT = 0x80310043u32 as HRESULT;

/// A boot application has changed since BitLocker Drive Encryption was enabled.
pub const FVE_E_AUTH_INVALID_APPLICATION: HRESULT = 0x80310044u32 as HRESULT;

/// The Boot Configuration Data (BCD) settings have changed since BitLocker Drive Encryption was enabled.
pub const FVE_E_AUTH_INVALID_CONFIG: HRESULT = 0x80310045u32 as HRESULT;

/// The Group Policy setting requiring FIPS compliance prohibits the use of unencrypted keys, which prevents BitLocker from being suspended on this drive. Please contact your domain administrator for more information.
pub const FVE_E_FIPS_DISABLE_PROTECTION_NOT_ALLOWED: HRESULT = 0x80310046u32 as HRESULT;

/// This drive cannot be encrypted by BitLocker Drive Encryption because the file system does not extend to the end of the drive. Repartition this drive and then try again.
pub const FVE_E_FS_NOT_EXTENDED: HRESULT = 0x80310047u32 as HRESULT;

/// BitLocker Drive Encryption cannot be enabled on the operating system drive. Contact the computer manufacturer for BIOS upgrade instructions.
pub const FVE_E_FIRMWARE_TYPE_NOT_SUPPORTED: HRESULT = 0x80310048u32 as HRESULT;

/// This version of Windows does not include BitLocker Drive Encryption. To use BitLocker Drive Encryption, please upgrade the operating system.
pub const FVE_E_NO_LICENSE: HRESULT = 0x80310049u32 as HRESULT;

/// BitLocker Drive Encryption cannot be used because critical BitLocker system files are missing or corrupted. Use Windows Startup Repair to restore these files to your computer.
pub const FVE_E_NOT_ON_STACK: HRESULT = 0x8031004Au32 as HRESULT;

/// The drive cannot be locked when the drive is in use.
pub const FVE_E_FS_MOUNTED: HRESULT = 0x8031004Bu32 as HRESULT;

/// The access token associated with the current thread is not an impersonated token.
pub const FVE_E_TOKEN_NOT_IMPERSONATED: HRESULT = 0x8031004Cu32 as HRESULT;

/// The BitLocker encryption key cannot be obtained. Verify that the Trusted Platform Module (TPM) is enabled and ownership has been taken. If this computer does not have a TPM, verify that the USB drive is inserted and available.
pub const FVE_E_DRY_RUN_FAILED: HRESULT = 0x8031004Du32 as HRESULT;

/// You must restart your computer before continuing with BitLocker Drive Encryption.
pub const FVE_E_REBOOT_REQUIRED: HRESULT = 0x8031004Eu32 as HRESULT;

/// Drive encryption cannot occur while boot debugging is enabled. Use the bcdedit command-line tool to turn off boot debugging.
pub const FVE_E_DEBUGGER_ENABLED: HRESULT = 0x8031004Fu32 as HRESULT;

/// No action was taken as BitLocker Drive Encryption is in raw access mode.
pub const FVE_E_RAW_ACCESS: HRESULT = 0x80310050u32 as HRESULT;

/// BitLocker Drive Encryption cannot enter raw access mode for this drive because the drive is currently in use.
pub const FVE_E_RAW_BLOCKED: HRESULT = 0x80310051u32 as HRESULT;

/// The path specified in the Boot Configuration Data (BCD) for a BitLocker Drive Encryption integrity-protected application is incorrect. Please verify and correct your BCD settings and try again.
pub const FVE_E_BCD_APPLICATIONS_PATH_INCORRECT: HRESULT = 0x80310052u32 as HRESULT;

/// BitLocker Drive Encryption can only be used for limited provisioning or recovery purposes when the computer is running in pre-installation or recovery environments.
pub const FVE_E_NOT_ALLOWED_IN_VERSION: HRESULT = 0x80310053u32 as HRESULT;

/// The auto-unlock master key was not available from the operating system drive.
pub const FVE_E_NO_AUTOUNLOCK_MASTER_KEY: HRESULT = 0x80310054u32 as HRESULT;

/// The system firmware failed to enable clearing of system memory when the computer was restarted.
pub const FVE_E_MOR_FAILED: HRESULT = 0x80310055u32 as HRESULT;

/// The hidden drive cannot be encrypted.
pub const FVE_E_HIDDEN_VOLUME: HRESULT = 0x80310056u32 as HRESULT;

/// BitLocker encryption keys were ignored because the drive was in a transient state.
pub const FVE_E_TRANSIENT_STATE: HRESULT = 0x80310057u32 as HRESULT;

/// Public key based protectors are not allowed on this drive.
pub const FVE_E_PUBKEY_NOT_ALLOWED: HRESULT = 0x80310058u32 as HRESULT;

/// BitLocker Drive Encryption is already performing an operation on this drive. Please complete all operations before continuing.
pub const FVE_E_VOLUME_HANDLE_OPEN: HRESULT = 0x80310059u32 as HRESULT;

/// This version of Windows does not support this feature of BitLocker Drive Encryption. To use this feature, upgrade the operating system.
pub const FVE_E_NO_FEATURE_LICENSE: HRESULT = 0x8031005Au32 as HRESULT;

/// The Group Policy settings for BitLocker startup options are in conflict and cannot be applied. Contact your system administrator for more information.
pub const FVE_E_INVALID_STARTUP_OPTIONS: HRESULT = 0x8031005Bu32 as HRESULT;

/// Group Policy settings do not permit the creation of a recovery password.
pub const FVE_E_POLICY_RECOVERY_PASSWORD_NOT_ALLOWED: HRESULT = 0x8031005Cu32 as HRESULT;

/// Group Policy settings require the creation of a recovery password.
pub const FVE_E_POLICY_RECOVERY_PASSWORD_REQUIRED: HRESULT = 0x8031005Du32 as HRESULT;

/// Group Policy settings do not permit the creation of a recovery key.
pub const FVE_E_POLICY_RECOVERY_KEY_NOT_ALLOWED: HRESULT = 0x8031005Eu32 as HRESULT;

/// Group Policy settings require the creation of a recovery key.
pub const FVE_E_POLICY_RECOVERY_KEY_REQUIRED: HRESULT = 0x8031005Fu32 as HRESULT;

/// Group Policy settings do not permit the use of a PIN at startup. Please choose a different BitLocker startup option.
pub const FVE_E_POLICY_STARTUP_PIN_NOT_ALLOWED: HRESULT = 0x80310060u32 as HRESULT;

/// Group Policy settings require the use of a PIN at startup. Please choose this BitLocker startup option.
pub const FVE_E_POLICY_STARTUP_PIN_REQUIRED: HRESULT = 0x80310061u32 as HRESULT;

/// Group Policy settings do not permit the use of a startup key. Please choose a different BitLocker startup option.
pub const FVE_E_POLICY_STARTUP_KEY_NOT_ALLOWED: HRESULT = 0x80310062u32 as HRESULT;

/// Group Policy settings require the use of a startup key. Please choose this BitLocker startup option.
pub const FVE_E_POLICY_STARTUP_KEY_REQUIRED: HRESULT = 0x80310063u32 as HRESULT;

/// Group Policy settings do not permit the use of a startup key and PIN. Please choose a different BitLocker startup option.
pub const FVE_E_POLICY_STARTUP_PIN_KEY_NOT_ALLOWED: HRESULT = 0x80310064u32 as HRESULT;

/// Group Policy settings require the use of a startup key and PIN. Please choose this BitLocker startup option.
pub const FVE_E_POLICY_STARTUP_PIN_KEY_REQUIRED: HRESULT = 0x80310065u32 as HRESULT;

/// Group policy does not permit the use of TPM-only at startup. Please choose a different BitLocker startup option.
pub const FVE_E_POLICY_STARTUP_TPM_NOT_ALLOWED: HRESULT = 0x80310066u32 as HRESULT;

/// Group Policy settings require the use of TPM-only at startup. Please choose this BitLocker startup option.
pub const FVE_E_POLICY_STARTUP_TPM_REQUIRED: HRESULT = 0x80310067u32 as HRESULT;

/// The PIN provided does not meet minimum or maximum length requirements.
pub const FVE_E_POLICY_INVALID_PIN_LENGTH: HRESULT = 0x80310068u32 as HRESULT;

/// The key protector is not supported by the version of BitLocker Drive Encryption currently on the drive. Upgrade the drive to add the key protector.
pub const FVE_E_KEY_PROTECTOR_NOT_SUPPORTED: HRESULT = 0x80310069u32 as HRESULT;

/// Group Policy settings do not permit the creation of a password.
pub const FVE_E_POLICY_PASSPHRASE_NOT_ALLOWED: HRESULT = 0x8031006Au32 as HRESULT;

/// Group Policy settings require the creation of a password.
pub const FVE_E_POLICY_PASSPHRASE_REQUIRED: HRESULT = 0x8031006Bu32 as HRESULT;

/// The Group Policy setting requiring FIPS compliance prevents passwords from being generated or used. Please contact your system administrator for more information.
pub const FVE_E_FIPS_PREVENTS_PASSPHRASE: HRESULT = 0x8031006Cu32 as HRESULT;

/// A password cannot be added to the operating system drive.
pub const FVE_E_OS_VOLUME_PASSPHRASE_NOT_ALLOWED: HRESULT = 0x8031006Du32 as HRESULT;

/// The BitLocker object identifier (OID) on the drive appears to be invalid or corrupt. Use manage-BDE to reset the OID on this drive.
pub const FVE_E_INVALID_BITLOCKER_OID: HRESULT = 0x8031006Eu32 as HRESULT;

/// The drive is too small to be protected using BitLocker Drive Encryption.
pub const FVE_E_VOLUME_TOO_SMALL: HRESULT = 0x8031006Fu32 as HRESULT;

/// The selected discovery drive type is incompatible with the file system on the drive. BitLocker To Go discovery drives must be created on FAT formatted drives.
pub const FVE_E_DV_NOT_SUPPORTED_ON_FS: HRESULT = 0x80310070u32 as HRESULT;

/// The selected discovery drive type is not allowed by the computer's Group Policy settings. Verify that Group Policy settings allow the creation of discovery drives for use with BitLocker To Go.
pub const FVE_E_DV_NOT_ALLOWED_BY_GP: HRESULT = 0x80310071u32 as HRESULT;

/// Group Policy settings do not permit user certificates such as smart cards to be used with BitLocker Drive Encryption.
pub const FVE_E_POLICY_USER_CERTIFICATE_NOT_ALLOWED: HRESULT = 0x80310072u32 as HRESULT;

/// Group Policy settings require that you have a valid user certificate, such as a smart card, to be used with BitLocker Drive Encryption.
pub const FVE_E_POLICY_USER_CERTIFICATE_REQUIRED: HRESULT = 0x80310073u32 as HRESULT;

/// Group Policy settings requires that you use a smart card-based key protector with BitLocker Drive Encryption.
pub const FVE_E_POLICY_USER_CERT_MUST_BE_HW: HRESULT = 0x80310074u32 as HRESULT;

/// Group Policy settings do not permit BitLocker-protected fixed data drives to be automatically unlocked.
pub const FVE_E_POLICY_USER_CONFIGURE_FDV_AUTOUNLOCK_NOT_ALLOWED: HRESULT =
    0x80310075u32 as HRESULT;

/// Group Policy settings do not permit BitLocker-protected removable data drives to be automatically unlocked.
pub const FVE_E_POLICY_USER_CONFIGURE_RDV_AUTOUNLOCK_NOT_ALLOWED: HRESULT =
    0x80310076u32 as HRESULT;

/// Group Policy settings do not permit you to configure BitLocker Drive Encryption on removable data drives.
pub const FVE_E_POLICY_USER_CONFIGURE_RDV_NOT_ALLOWED: HRESULT = 0x80310077u32 as HRESULT;

/// Group Policy settings do not permit you to turn on BitLocker Drive Encryption on removable data drives. Please contact your system administrator if you need to turn on BitLocker.
pub const FVE_E_POLICY_USER_ENABLE_RDV_NOT_ALLOWED: HRESULT = 0x80310078u32 as HRESULT;

/// Group Policy settings do not permit turning off BitLocker Drive Encryption on removable data drives. Please contact your system administrator if you need to turn off BitLocker.
pub const FVE_E_POLICY_USER_DISABLE_RDV_NOT_ALLOWED: HRESULT = 0x80310079u32 as HRESULT;

/// Your password does not meet minimum password length requirements. By default, passwords must be at least 8 characters in length. Check with your system administrator for the password length requirement in your organization.
pub const FVE_E_POLICY_INVALID_PASSPHRASE_LENGTH: HRESULT = 0x80310080u32 as HRESULT;

/// Your password does not meet the complexity requirements set by your system administrator. Try adding upper and lowercase characters, numbers, and symbols.
pub const FVE_E_POLICY_PASSPHRASE_TOO_SIMPLE: HRESULT = 0x80310081u32 as HRESULT;

/// This drive cannot be encrypted because it is reserved for Windows System Recovery Options.
pub const FVE_E_RECOVERY_PARTITION: HRESULT = 0x80310082u32 as HRESULT;

/// BitLocker Drive Encryption cannot be applied to this drive because of conflicting Group Policy settings. BitLocker cannot be configured to automatically unlock fixed data drives when user recovery options are disabled. If you want BitLocker-protected fixed data drives to be automatically unlocked after key validation has occurred, please ask your system administrator to resolve the settings conflict before enabling BitLocker.
pub const FVE_E_POLICY_CONFLICT_FDV_RK_OFF_AUK_ON: HRESULT = 0x80310083u32 as HRESULT;

/// BitLocker Drive Encryption cannot be applied to this drive because of conflicting Group Policy settings. BitLocker cannot be configured to automatically unlock removable data drives when user recovery option are disabled. If you want BitLocker-protected removable data drives to be automatically unlocked after key validation has occurred, please ask your system administrator to resolve the settings conflict before enabling BitLocker.
pub const FVE_E_POLICY_CONFLICT_RDV_RK_OFF_AUK_ON: HRESULT = 0x80310084u32 as HRESULT;

/// The Enhanced Key Usage (EKU) attribute of the specified certificate does not permit it to be used for BitLocker Drive Encryption. BitLocker does not require that a certificate have an EKU attribute, but if one is configured it must be set to an object identifier (OID) that matches the OID configured for BitLocker.
pub const FVE_E_NON_BITLOCKER_OID: HRESULT = 0x80310085u32 as HRESULT;

/// BitLocker Drive Encryption cannot be applied to this drive as currently configured because of Group Policy settings. The certificate you provided for drive encryption is self-signed. Current Group Policy settings do not permit the use of self-signed certificates. Obtain a new certificate from your certification authority before attempting to enable BitLocker.
pub const FVE_E_POLICY_PROHIBITS_SELFSIGNED: HRESULT = 0x80310086u32 as HRESULT;

/// BitLocker Encryption cannot be applied to this drive because of conflicting Group Policy settings. When write access to drives not protected by BitLocker is denied, the use of a USB startup key cannot be required. Please have your system administrator resolve these policy conflicts before attempting to enable BitLocker.
pub const FVE_E_POLICY_CONFLICT_RO_AND_STARTUP_KEY_REQUIRED: HRESULT = 0x80310087u32 as HRESULT;

/// BitLocker Drive Encryption failed to recover from an abruptly terminated conversion. This could be due to either all conversion logs being corrupted or the media being write-protected.
pub const FVE_E_CONV_RECOVERY_FAILED: HRESULT = 0x80310088u32 as HRESULT;

/// The requested virtualization size is too big.
pub const FVE_E_VIRTUALIZED_SPACE_TOO_BIG: HRESULT = 0x80310089u32 as HRESULT;

/// BitLocker Drive Encryption cannot be applied to this drive because there are conflicting Group Policy settings for recovery options on operating system drives. Storing recovery information to Active Directory Domain Services cannot be required when the generation of recovery passwords is not permitted. Please have your system administrator resolve these policy conflicts before attempting to enable BitLocker.
pub const FVE_E_POLICY_CONFLICT_OSV_RP_OFF_ADB_ON: HRESULT = 0x80310090u32 as HRESULT;

/// BitLocker Drive Encryption cannot be applied to this drive because there are conflicting Group Policy settings for recovery options on fixed data drives. Storing recovery information to Active Directory Domain Services cannot be required when the generation of recovery passwords is not permitted. Please have your system administrator resolve these policy conflicts before attempting to enable BitLocker.
pub const FVE_E_POLICY_CONFLICT_FDV_RP_OFF_ADB_ON: HRESULT = 0x80310091u32 as HRESULT;

/// BitLocker Drive Encryption cannot be applied to this drive because there are conflicting Group Policy settings for recovery options on removable data drives. Storing recovery information to Active Directory Domain Services cannot be required when the generation of recovery passwords is not permitted. Please have your system administrator resolve these policy conflicts before attempting to enable BitLocker.
pub const FVE_E_POLICY_CONFLICT_RDV_RP_OFF_ADB_ON: HRESULT = 0x80310092u32 as HRESULT;

/// The Key Usage (KU) attribute of the specified certificate does not permit it to be used for BitLocker Drive Encryption. BitLocker does not require that a certificate have a KU attribute, but if one is configured it must be set to either Key Encipherment or Key Agreement.
pub const FVE_E_NON_BITLOCKER_KU: HRESULT = 0x80310093u32 as HRESULT;

/// The private key associated with the specified certificate cannot be authorized. The private key authorization was either not provided or the provided authorization was invalid.
pub const FVE_E_PRIVATEKEY_AUTH_FAILED: HRESULT = 0x80310094u32 as HRESULT;

/// Removal of the data recovery agent certificate must be done using the Certificates snap-in.
pub const FVE_E_REMOVAL_OF_DRA_FAILED: HRESULT = 0x80310095u32 as HRESULT;

/// This drive was encrypted using the version of BitLocker Drive Encryption included with Windows Vista and Windows Server 2008 which does not support organizational identifiers. To specify organizational identifiers for this drive upgrade the drive encryption to the latest version using the "manage-bde -upgrade" command.
pub const FVE_E_OPERATION_NOT_SUPPORTED_ON_VISTA_VOLUME: HRESULT = 0x80310096u32 as HRESULT;

/// The drive cannot be locked because it is automatically unlocked on this computer.  Remove the automatic unlock protector to lock this drive.
pub const FVE_E_CANT_LOCK_AUTOUNLOCK_ENABLED_VOLUME: HRESULT = 0x80310097u32 as HRESULT;

/// The default BitLocker Key Derivation Function SP800-56A for ECC smart cards is not supported by your smart card. The Group Policy setting requiring FIPS-compliance prevents BitLocker from using any other key derivation function for encryption. You have to use a FIPS compliant smart card in FIPS restricted environments.
pub const FVE_E_FIPS_HASH_KDF_NOT_ALLOWED: HRESULT = 0x80310098u32 as HRESULT;

/// The BitLocker encryption key could not be obtained from the Trusted Platform Module (TPM) and enhanced PIN. Try using a PIN containing only numerals.
pub const FVE_E_ENH_PIN_INVALID: HRESULT = 0x80310099u32 as HRESULT;

/// The requested TPM PIN contains invalid characters.
pub const FVE_E_INVALID_PIN_CHARS: HRESULT = 0x8031009Au32 as HRESULT;

/// The management information stored on the drive contained an unknown type. If you are using an old version of Windows, try accessing the drive from the latest version.
pub const FVE_E_INVALID_DATUM_TYPE: HRESULT = 0x8031009Bu32 as HRESULT;

/// The feature is only supported on EFI systems.
pub const FVE_E_EFI_ONLY: HRESULT = 0x8031009Cu32 as HRESULT;

/// More than one Network Key Protector certificate has been found on the system.
pub const FVE_E_MULTIPLE_NKP_CERTS: HRESULT = 0x8031009Du32 as HRESULT;

/// Removal of the Network Key Protector certificate must be done using the Certificates snap-in.
pub const FVE_E_REMOVAL_OF_NKP_FAILED: HRESULT = 0x8031009Eu32 as HRESULT;

/// An invalid certificate has been found in the Network Key Protector certificate store.
pub const FVE_E_INVALID_NKP_CERT: HRESULT = 0x8031009Fu32 as HRESULT;

/// This drive isn't protected with a PIN.
pub const FVE_E_NO_EXISTING_PIN: HRESULT = 0x803100A0u32 as HRESULT;

/// Please enter the correct current PIN.
pub const FVE_E_PROTECTOR_CHANGE_PIN_MISMATCH: HRESULT = 0x803100A1u32 as HRESULT;

/// You must be logged on with an administrator account to change the PIN. Click the link to reset the PIN as an administrator.
pub const FVE_E_PIN_PROTECTOR_CHANGE_BY_STD_USER_DISALLOWED: HRESULT = 0x803100A2u32 as HRESULT;

/// BitLocker has disabled PIN changes after too many failed requests. Click the link to reset the PIN as an administrator.
pub const FVE_E_PROTECTOR_CHANGE_MAX_PIN_CHANGE_ATTEMPTS_REACHED: HRESULT =
    0x803100A3u32 as HRESULT;

/// Your system administrator requires that passwords contain only printable ASCII characters. This includes unaccented letters (A-Z, a-z), numbers (0-9), space, arithmetic signs, common punctuation, separators, and the following symbols: # $ & @ ^ _ ~ .
pub const FVE_E_POLICY_PASSPHRASE_REQUIRES_ASCII: HRESULT = 0x803100A4u32 as HRESULT;

/// BitLocker Drive Encryption only supports Used Space Only encryption on thin provisioned storage.
pub const FVE_E_FULL_ENCRYPTION_NOT_ALLOWED_ON_TP_STORAGE: HRESULT = 0x803100A5u32 as HRESULT;

/// BitLocker Drive Encryption does not support wiping free space on thin provisioned storage.
pub const FVE_E_WIPE_NOT_ALLOWED_ON_TP_STORAGE: HRESULT = 0x803100A6u32 as HRESULT;

/// The required authentication key length is not supported by the drive.
pub const FVE_E_KEY_LENGTH_NOT_SUPPORTED_BY_EDRIVE: HRESULT = 0x803100A7u32 as HRESULT;

/// This drive isn't protected with a password.
pub const FVE_E_NO_EXISTING_PASSPHRASE: HRESULT = 0x803100A8u32 as HRESULT;

/// Please enter the correct current password.
pub const FVE_E_PROTECTOR_CHANGE_PASSPHRASE_MISMATCH: HRESULT = 0x803100A9u32 as HRESULT;

/// The password cannot exceed 256 characters.
pub const FVE_E_PASSPHRASE_TOO_LONG: HRESULT = 0x803100AAu32 as HRESULT;

/// A password key protector cannot be added because a TPM protector exists on the drive.
pub const FVE_E_NO_PASSPHRASE_WITH_TPM: HRESULT = 0x803100ABu32 as HRESULT;

/// A TPM key protector cannot be added because a password protector exists on the drive.
pub const FVE_E_NO_TPM_WITH_PASSPHRASE: HRESULT = 0x803100ACu32 as HRESULT;

/// This command can only be performed from the coordinator node for the specified CSV volume.
pub const FVE_E_NOT_ALLOWED_ON_CSV_STACK: HRESULT = 0x803100ADu32 as HRESULT;

/// This command cannot be performed on a volume when it is part of a cluster.
pub const FVE_E_NOT_ALLOWED_ON_CLUSTER: HRESULT = 0x803100AEu32 as HRESULT;

/// BitLocker did not revert to using BitLocker software encryption due to group policy configuration.
pub const FVE_E_EDRIVE_NO_FAILOVER_TO_SW: HRESULT = 0x803100AFu32 as HRESULT;

/// The drive cannot be managed by BitLocker because the drive's hardware encryption feature is already in use.
pub const FVE_E_EDRIVE_BAND_IN_USE: HRESULT = 0x803100B0u32 as HRESULT;

/// Group Policy settings do not allow the use of hardware-based encryption.
pub const FVE_E_EDRIVE_DISALLOWED_BY_GP: HRESULT = 0x803100B1u32 as HRESULT;

/// The drive specified does not support hardware-based encryption.
pub const FVE_E_EDRIVE_INCOMPATIBLE_VOLUME: HRESULT = 0x803100B2u32 as HRESULT;

/// BitLocker cannot be upgraded during disk encryption or decryption.
pub const FVE_E_NOT_ALLOWED_TO_UPGRADE_WHILE_CONVERTING: HRESULT = 0x803100B3u32 as HRESULT;

/// Discovery Volumes are not supported for volumes using hardware encryption.
pub const FVE_E_EDRIVE_DV_NOT_SUPPORTED: HRESULT = 0x803100B4u32 as HRESULT;

/// No pre-boot keyboard detected. The user may not be able to provide required input to unlock the volume.
pub const FVE_E_NO_PREBOOT_KEYBOARD_DETECTED: HRESULT = 0x803100B5u32 as HRESULT;

/// No pre-boot keyboard or Windows Recovery Environment detected. The user may not be able to provide required input to unlock the volume.
pub const FVE_E_NO_PREBOOT_KEYBOARD_OR_WINRE_DETECTED: HRESULT = 0x803100B6u32 as HRESULT;

/// Group Policy settings require the creation of a startup PIN, but a pre-boot keyboard is not available on this device. The user may not be able to provide required input to unlock the volume.
pub const FVE_E_POLICY_REQUIRES_STARTUP_PIN_ON_TOUCH_DEVICE: HRESULT = 0x803100B7u32 as HRESULT;

/// Group Policy settings require the creation of a recovery password, but neither a pre-boot keyboard nor Windows Recovery Environment is available on this device. The user may not be able to provide required input to unlock the volume.
pub const FVE_E_POLICY_REQUIRES_RECOVERY_PASSWORD_ON_TOUCH_DEVICE: HRESULT =
    0x803100B8u32 as HRESULT;

/// Wipe of free space is not currently taking place.
pub const FVE_E_WIPE_CANCEL_NOT_APPLICABLE: HRESULT = 0x803100B9u32 as HRESULT;

/// BitLocker cannot use Secure Boot for platform integrity because Secure Boot has been disabled.
pub const FVE_E_SECUREBOOT_DISABLED: HRESULT = 0x803100BAu32 as HRESULT;

/// BitLocker cannot use Secure Boot for platform integrity because the Secure Boot configuration does not meet the requirements for BitLocker.
pub const FVE_E_SECUREBOOT_CONFIGURATION_INVALID: HRESULT = 0x803100BBu32 as HRESULT;

/// Your computer doesn't support BitLocker hardware-based encryption. Check with your computer manufacturer for firmware updates.
pub const FVE_E_EDRIVE_DRY_RUN_FAILED: HRESULT = 0x803100BCu32 as HRESULT;

/// BitLocker cannot be enabled on the volume because it contains a Volume Shadow Copy. Remove all Volume Shadow Copies before encrypting the volume.
pub const FVE_E_SHADOW_COPY_PRESENT: HRESULT = 0x803100BDu32 as HRESULT;

/// BitLocker Drive Encryption cannot be applied to this drive because the Group Policy setting for Enhanced Boot Configuration Data contains invalid data. Please have your system administrator resolve this invalid configuration before attempting to enable BitLocker.
pub const FVE_E_POLICY_INVALID_ENHANCED_BCD_SETTINGS: HRESULT = 0x803100BEu32 as HRESULT;

/// This PC's firmware is not capable of supporting hardware encryption.
pub const FVE_E_EDRIVE_INCOMPATIBLE_FIRMWARE: HRESULT = 0x803100BFu32 as HRESULT;

/// BitLocker has disabled password changes after too many failed requests. Click the link to reset the password as an administrator.
pub const FVE_E_PROTECTOR_CHANGE_MAX_PASSPHRASE_CHANGE_ATTEMPTS_REACHED: HRESULT =
    0x803100C0u32 as HRESULT;

/// You must be logged on with an administrator account to change the password. Click the link to reset the password as an administrator.
pub const FVE_E_PASSPHRASE_PROTECTOR_CHANGE_BY_STD_USER_DISALLOWED: HRESULT =
    0x803100C1u32 as HRESULT;

/// BitLocker cannot save the recovery password because the specified Microsoft account is Suspended.
pub const FVE_E_LIVEID_ACCOUNT_SUSPENDED: HRESULT = 0x803100C2u32 as HRESULT;

/// BitLocker cannot save the recovery password because the specified Microsoft account is Blocked.
pub const FVE_E_LIVEID_ACCOUNT_BLOCKED: HRESULT = 0x803100C3u32 as HRESULT;

/// This PC is not provisioned to support device encryption. Please enable BitLocker on all volumes to comply with device encryption policy.
pub const FVE_E_NOT_PROVISIONED_ON_ALL_VOLUMES: HRESULT = 0x803100C4u32 as HRESULT;

/// This PC cannot support device encryption because unencrypted fixed data volumes are present.
pub const FVE_E_DE_FIXED_DATA_NOT_SUPPORTED: HRESULT = 0x803100C5u32 as HRESULT;

/// This PC does not meet the hardware requirements to support device encryption.
pub const FVE_E_DE_HARDWARE_NOT_COMPLIANT: HRESULT = 0x803100C6u32 as HRESULT;

/// This PC cannot support device encryption because WinRE is not properly configured.
pub const FVE_E_DE_WINRE_NOT_CONFIGURED: HRESULT = 0x803100C7u32 as HRESULT;

/// Protection is enabled on the volume but has been suspended. This is likely to have happened due to an update being applied to your system. Please try again after a reboot.
pub const FVE_E_DE_PROTECTION_SUSPENDED: HRESULT = 0x803100C8u32 as HRESULT;

/// This PC is not provisioned to support device encryption.
pub const FVE_E_DE_OS_VOLUME_NOT_PROTECTED: HRESULT = 0x803100C9u32 as HRESULT;

/// Device Lock has been triggered due to too many incorrect password attempts.
pub const FVE_E_DE_DEVICE_LOCKEDOUT: HRESULT = 0x803100CAu32 as HRESULT;

/// Protection has not been enabled on the volume. Enabling protection requires a connected account. If you already have a connected account and are seeing this error, please refer to the event log for more information.
pub const FVE_E_DE_PROTECTION_NOT_YET_ENABLED: HRESULT = 0x803100CBu32 as HRESULT;

/// Your PIN can only contain numbers from 0 to 9.
pub const FVE_E_INVALID_PIN_CHARS_DETAILED: HRESULT = 0x803100CCu32 as HRESULT;

/// BitLocker cannot use hardware replay protection because no counter is available on your PC.
pub const FVE_E_DEVICE_LOCKOUT_COUNTER_UNAVAILABLE: HRESULT = 0x803100CDu32 as HRESULT;

/// Device Lockout state validation failed due to counter mismatch.
pub const FVE_E_DEVICELOCKOUT_COUNTER_MISMATCH: HRESULT = 0x803100CEu32 as HRESULT;

/// The input buffer is too large.
pub const FVE_E_BUFFER_TOO_LARGE: HRESULT = 0x803100CFu32 as HRESULT;

/// The target of an invocation does not support requested capability.
pub const FVE_E_NO_SUCH_CAPABILITY_ON_TARGET: HRESULT = 0x803100D0u32 as HRESULT;

/// Device encryption is currently blocked by this PC's configuration.
pub const FVE_E_DE_PREVENTED_FOR_OS: HRESULT = 0x803100D1u32 as HRESULT;

/// This drive has been opted out of device encryption.
pub const FVE_E_DE_VOLUME_OPTED_OUT: HRESULT = 0x803100D2u32 as HRESULT;

/// Device encryption isn't available for this drive.
pub const FVE_E_DE_VOLUME_NOT_SUPPORTED: HRESULT = 0x803100D3u32 as HRESULT;

/// The encrypt on write mode for BitLocker is not supported in this version of Windows. You can turn on BitLocker without using the encrypt on write mode.
pub const FVE_E_EOW_NOT_SUPPORTED_IN_VERSION: HRESULT = 0x803100D4u32 as HRESULT;

/// Group policy prevents you from backing up your recovery password to Active Directory for this drive type. For more info, contact your system administrator.
pub const FVE_E_ADBACKUP_NOT_ENABLED: HRESULT = 0x803100D5u32 as HRESULT;

/// Device encryption can't be turned off while this drive is being encrypted. Please try again later.
pub const FVE_E_VOLUME_EXTEND_PREVENTS_EOW_DECRYPT: HRESULT = 0x803100D6u32 as HRESULT;

/// This action isn't supported because this drive isn't automatically managed with device encryption.
pub const FVE_E_NOT_DE_VOLUME: HRESULT = 0x803100D7u32 as HRESULT;

/// BitLocker can't be suspended on this drive until the next restart.
pub const FVE_E_PROTECTION_CANNOT_BE_DISABLED: HRESULT = 0x803100D8u32 as HRESULT;

/// BitLocker Drive Encryption policy does not allow KSR operation with protected OS volume.
pub const FVE_E_OSV_KSR_NOT_ALLOWED: HRESULT = 0x803100D9u32 as HRESULT;

/// BitLocker recovery password rotation cannot be performed because backup policy for BitLocker recovery information is not set to required for the OS drive.
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_OS_DRIVE: HRESULT = 0x803100DAu32 as HRESULT;

/// BitLocker recovery password rotation cannot be performed because backup policy for BitLocker recovery information is not set to required for fixed data drives.
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_FIXED_DRIVE: HRESULT = 0x803100DBu32 as HRESULT;

/// BitLocker recovery password rotation cannot be performed because backup policy for BitLocker recovery information is not set to required for removable data drives
pub const FVE_E_AD_BACKUP_REQUIRED_POLICY_NOT_SET_REMOVABLE_DRIVE: HRESULT =
    0x803100DCu32 as HRESULT;

/// BitLocker recovery password rotation not supported.
pub const FVE_E_KEY_ROTATION_NOT_SUPPORTED: HRESULT = 0x803100DDu32 as HRESULT;

/// A server issued BitLocker recovery password rotation was denied because requests must be 15 minutes apart.
pub const FVE_E_EXECUTE_REQUEST_SENT_TOO_SOON: HRESULT = 0x803100DEu32 as HRESULT;

/// BitLocker recovery password key rotation policy is not enabled.
pub const FVE_E_KEY_ROTATION_NOT_ENABLED: HRESULT = 0x803100DFu32 as HRESULT;

/// BitLocker recovery password key rotation could not be performed because the device is neither Azure AD joined nor Hybrid Azure AD joined.
pub const FVE_E_DEVICE_NOT_JOINED: HRESULT = 0x803100E0u32 as HRESULT;

/// BitLocker recovery key backup endpoint is busy and cannot perform requested operation. Please retry after sometime.
pub const FVE_E_AAD_ENDPOINT_BUSY: HRESULT = 0x803100E1u32 as HRESULT;

/// An invalid certificate has been found in the Network Boot Protector certificate store.
pub const FVE_E_INVALID_NBP_CERT: HRESULT = 0x803100E2u32 as HRESULT;

/// BitLocker can't enable encryption on this hardware encrypting drive volume because the drive bands couldn't be enumerated.
pub const FVE_E_EDRIVE_BAND_ENUMERATION_FAILED: HRESULT = 0x803100E3u32 as HRESULT;

/// This removable data drive has been excluded from device encryption by BitLocker Drive Encryption policy.
pub const FVE_E_POLICY_ON_RDV_EXCLUSION_LIST: HRESULT = 0x803100E4u32 as HRESULT;

/// Adding BitLocker predicted TPM based protector is not supported.
pub const FVE_E_PREDICTED_TPM_PROTECTOR_NOT_SUPPORTED: HRESULT = 0x803100E5u32 as HRESULT;

/// Registeration for TPM callback is not supported.
pub const FVE_E_SETUP_TPM_CALLBACK_NOT_SUPPORTED: HRESULT = 0x803100E6u32 as HRESULT;

/// Creating new TPM context is not supported.
pub const FVE_E_TPM_CONTEXT_SETUP_NOT_SUPPORTED: HRESULT = 0x803100E7u32 as HRESULT;

/// The Secure Boot update was not applied due to a known incompatibility with the current BitLocker configuration.
pub const FVE_E_UPDATE_INVALID_CONFIG: HRESULT = 0x803100E8u32 as HRESULT;

/// The AAD request has failed and it has been advised to backoff to prevent throttling. Device will retry soon.
pub const FVE_E_AAD_SERVER_FAIL_RETRY_AFTER: HRESULT = 0x803100E9u32 as HRESULT;

/// The AAD request has failed due to server issues and it has been advised to backoff to prevent throttling. Device will retry soon.
pub const FVE_E_AAD_SERVER_FAIL_BACKOFF: HRESULT = 0x803100EAu32 as HRESULT;

/// This operation cannot be completed because dataset is full.
pub const FVE_E_DATASET_FULL: HRESULT = 0x803100EBu32 as HRESULT;

/// This operation cannot be completed because BitLocker Drive Encryption metadata area is full. Consider removing unnecessary key protectors for this drive.
pub const FVE_E_METADATA_FULL: HRESULT = 0x803100ECu32 as HRESULT;

/// The callout does not exist.
pub const FWP_E_CALLOUT_NOT_FOUND: HRESULT = 0x80320001u32 as HRESULT;

/// The filter condition does not exist.
pub const FWP_E_CONDITION_NOT_FOUND: HRESULT = 0x80320002u32 as HRESULT;

/// The filter does not exist.
pub const FWP_E_FILTER_NOT_FOUND: HRESULT = 0x80320003u32 as HRESULT;

/// The layer does not exist.
pub const FWP_E_LAYER_NOT_FOUND: HRESULT = 0x80320004u32 as HRESULT;

/// The provider does not exist.
pub const FWP_E_PROVIDER_NOT_FOUND: HRESULT = 0x80320005u32 as HRESULT;

/// The provider context does not exist.
pub const FWP_E_PROVIDER_CONTEXT_NOT_FOUND: HRESULT = 0x80320006u32 as HRESULT;

/// The sublayer does not exist.
pub const FWP_E_SUBLAYER_NOT_FOUND: HRESULT = 0x80320007u32 as HRESULT;

/// The object does not exist.
pub const FWP_E_NOT_FOUND: HRESULT = 0x80320008u32 as HRESULT;

/// An object with that GUID or LUID already exists.
pub const FWP_E_ALREADY_EXISTS: HRESULT = 0x80320009u32 as HRESULT;

/// The object is referenced by other objects so cannot be deleted.
pub const FWP_E_IN_USE: HRESULT = 0x8032000Au32 as HRESULT;

/// The call is not allowed from within a dynamic session.
pub const FWP_E_DYNAMIC_SESSION_IN_PROGRESS: HRESULT = 0x8032000Bu32 as HRESULT;

/// The call was made from the wrong session so cannot be completed.
pub const FWP_E_WRONG_SESSION: HRESULT = 0x8032000Cu32 as HRESULT;

/// The call must be made from within an explicit transaction.
pub const FWP_E_NO_TXN_IN_PROGRESS: HRESULT = 0x8032000Du32 as HRESULT;

/// The call is not allowed from within an explicit transaction.
pub const FWP_E_TXN_IN_PROGRESS: HRESULT = 0x8032000Eu32 as HRESULT;

/// The explicit transaction has been forcibly cancelled.
pub const FWP_E_TXN_ABORTED: HRESULT = 0x8032000Fu32 as HRESULT;

/// The session has been cancelled.
pub const FWP_E_SESSION_ABORTED: HRESULT = 0x80320010u32 as HRESULT;

/// The call is not allowed from within a read-only transaction.
pub const FWP_E_INCOMPATIBLE_TXN: HRESULT = 0x80320011u32 as HRESULT;

/// The call timed out while waiting to acquire the transaction lock.
pub const FWP_E_TIMEOUT: HRESULT = 0x80320012u32 as HRESULT;

/// Collection of network diagnostic events is disabled.
pub const FWP_E_NET_EVENTS_DISABLED: HRESULT = 0x80320013u32 as HRESULT;

/// The operation is not supported by the specified layer.
pub const FWP_E_INCOMPATIBLE_LAYER: HRESULT = 0x80320014u32 as HRESULT;

/// The call is allowed for kernel-mode callers only.
pub const FWP_E_KM_CLIENTS_ONLY: HRESULT = 0x80320015u32 as HRESULT;

/// The call tried to associate two objects with incompatible lifetimes.
pub const FWP_E_LIFETIME_MISMATCH: HRESULT = 0x80320016u32 as HRESULT;

/// The object is built in so cannot be deleted.
pub const FWP_E_BUILTIN_OBJECT: HRESULT = 0x80320017u32 as HRESULT;

/// The maximum number of callouts has been reached.
pub const FWP_E_TOO_MANY_CALLOUTS: HRESULT = 0x80320018u32 as HRESULT;

/// A notification could not be delivered because a message queue is at its maximum capacity.
pub const FWP_E_NOTIFICATION_DROPPED: HRESULT = 0x80320019u32 as HRESULT;

/// The traffic parameters do not match those for the security association context.
pub const FWP_E_TRAFFIC_MISMATCH: HRESULT = 0x8032001Au32 as HRESULT;

/// The call is not allowed for the current security association state.
pub const FWP_E_INCOMPATIBLE_SA_STATE: HRESULT = 0x8032001Bu32 as HRESULT;

/// A required pointer is null.
pub const FWP_E_NULL_POINTER: HRESULT = 0x8032001Cu32 as HRESULT;

/// An enumerator is not valid.
pub const FWP_E_INVALID_ENUMERATOR: HRESULT = 0x8032001Du32 as HRESULT;

/// The flags field contains an invalid value.
pub const FWP_E_INVALID_FLAGS: HRESULT = 0x8032001Eu32 as HRESULT;

/// A network mask is not valid.
pub const FWP_E_INVALID_NET_MASK: HRESULT = 0x8032001Fu32 as HRESULT;

/// An FWP_RANGE is not valid.
pub const FWP_E_INVALID_RANGE: HRESULT = 0x80320020u32 as HRESULT;

/// The time interval is not valid.
pub const FWP_E_INVALID_INTERVAL: HRESULT = 0x80320021u32 as HRESULT;

/// An array that must contain at least one element is zero length.
pub const FWP_E_ZERO_LENGTH_ARRAY: HRESULT = 0x80320022u32 as HRESULT;

/// The displayData.name field cannot be null.
pub const FWP_E_NULL_DISPLAY_NAME: HRESULT = 0x80320023u32 as HRESULT;

/// The action type is not one of the allowed action types for a filter.
pub const FWP_E_INVALID_ACTION_TYPE: HRESULT = 0x80320024u32 as HRESULT;

/// The filter weight is not valid.
pub const FWP_E_INVALID_WEIGHT: HRESULT = 0x80320025u32 as HRESULT;

/// A filter condition contains a match type that is not compatible with the operands.
pub const FWP_E_MATCH_TYPE_MISMATCH: HRESULT = 0x80320026u32 as HRESULT;

/// An FWP_VALUE or FWPM_CONDITION_VALUE is of the wrong type.
pub const FWP_E_TYPE_MISMATCH: HRESULT = 0x80320027u32 as HRESULT;

/// An integer value is outside the allowed range.
pub const FWP_E_OUT_OF_BOUNDS: HRESULT = 0x80320028u32 as HRESULT;

/// A reserved field is non-zero.
pub const FWP_E_RESERVED: HRESULT = 0x80320029u32 as HRESULT;

/// A filter cannot contain multiple conditions operating on a single field.
pub const FWP_E_DUPLICATE_CONDITION: HRESULT = 0x8032002Au32 as HRESULT;

/// A policy cannot contain the same keying module more than once.
pub const FWP_E_DUPLICATE_KEYMOD: HRESULT = 0x8032002Bu32 as HRESULT;

/// The action type is not compatible with the layer.
pub const FWP_E_ACTION_INCOMPATIBLE_WITH_LAYER: HRESULT = 0x8032002Cu32 as HRESULT;

/// The action type is not compatible with the sublayer.
pub const FWP_E_ACTION_INCOMPATIBLE_WITH_SUBLAYER: HRESULT = 0x8032002Du32 as HRESULT;

/// The raw context or the provider context is not compatible with the layer.
pub const FWP_E_CONTEXT_INCOMPATIBLE_WITH_LAYER: HRESULT = 0x8032002Eu32 as HRESULT;

/// The raw context or the provider context is not compatible with the callout.
pub const FWP_E_CONTEXT_INCOMPATIBLE_WITH_CALLOUT: HRESULT = 0x8032002Fu32 as HRESULT;

/// The authentication method is not compatible with the policy type.
pub const FWP_E_INCOMPATIBLE_AUTH_METHOD: HRESULT = 0x80320030u32 as HRESULT;

/// The Diffie-Hellman group is not compatible with the policy type.
pub const FWP_E_INCOMPATIBLE_DH_GROUP: HRESULT = 0x80320031u32 as HRESULT;

/// An IKE policy cannot contain an Extended Mode policy.
pub const FWP_E_EM_NOT_SUPPORTED: HRESULT = 0x80320032u32 as HRESULT;

/// The enumeration template or subscription will never match any objects.
pub const FWP_E_NEVER_MATCH: HRESULT = 0x80320033u32 as HRESULT;

/// The provider context is of the wrong type.
pub const FWP_E_PROVIDER_CONTEXT_MISMATCH: HRESULT = 0x80320034u32 as HRESULT;

/// The parameter is incorrect.
pub const FWP_E_INVALID_PARAMETER: HRESULT = 0x80320035u32 as HRESULT;

/// The maximum number of sublayers has been reached.
pub const FWP_E_TOO_MANY_SUBLAYERS: HRESULT = 0x80320036u32 as HRESULT;

/// The notification function for a callout returned an error.
pub const FWP_E_CALLOUT_NOTIFICATION_FAILED: HRESULT = 0x80320037u32 as HRESULT;

/// The IPsec authentication transform is not valid.
pub const FWP_E_INVALID_AUTH_TRANSFORM: HRESULT = 0x80320038u32 as HRESULT;

/// The IPsec cipher transform is not valid.
pub const FWP_E_INVALID_CIPHER_TRANSFORM: HRESULT = 0x80320039u32 as HRESULT;

/// The IPsec cipher transform is not compatible with the policy.
pub const FWP_E_INCOMPATIBLE_CIPHER_TRANSFORM: HRESULT = 0x8032003Au32 as HRESULT;

/// The combination of IPsec transform types is not valid.
pub const FWP_E_INVALID_TRANSFORM_COMBINATION: HRESULT = 0x8032003Bu32 as HRESULT;

/// A policy cannot contain the same auth method more than once.
pub const FWP_E_DUPLICATE_AUTH_METHOD: HRESULT = 0x8032003Cu32 as HRESULT;

/// A tunnel endpoint configuration is invalid.
pub const FWP_E_INVALID_TUNNEL_ENDPOINT: HRESULT = 0x8032003Du32 as HRESULT;

/// The WFP MAC Layers are not ready.
pub const FWP_E_L2_DRIVER_NOT_READY: HRESULT = 0x8032003Eu32 as HRESULT;

/// A key manager capable of key dictation is already registered
pub const FWP_E_KEY_DICTATOR_ALREADY_REGISTERED: HRESULT = 0x8032003Fu32 as HRESULT;

/// A key manager dictated invalid keys
pub const FWP_E_KEY_DICTATION_INVALID_KEYING_MATERIAL: HRESULT = 0x80320040u32 as HRESULT;

/// The BFE IPsec Connection Tracking is disabled.
pub const FWP_E_CONNECTIONS_DISABLED: HRESULT = 0x80320041u32 as HRESULT;

/// The DNS name is invalid.
pub const FWP_E_INVALID_DNS_NAME: HRESULT = 0x80320042u32 as HRESULT;

/// The engine option is still enabled due to other configuration settings.
pub const FWP_E_STILL_ON: HRESULT = 0x80320043u32 as HRESULT;

/// The IKEEXT service is not running.  This service only runs when there is IPsec policy applied to the machine.
pub const FWP_E_IKEEXT_NOT_RUNNING: HRESULT = 0x80320044u32 as HRESULT;

/// The packet should be dropped, no ICMP should be sent.
pub const FWP_E_DROP_NOICMP: HRESULT = 0x80320104u32 as HRESULT;

/// The function call is completing asynchronously.
pub const WS_S_ASYNC: HRESULT = 0x003D0000;

/// There are no more messages available on the channel.
pub const WS_S_END: HRESULT = 0x003D0001;

/// The input data was not in the expected format or did not have the expected value.
pub const WS_E_INVALID_FORMAT: HRESULT = 0x803D0000u32 as HRESULT;

/// The operation could not be completed because the object is in a faulted state due to a previous error.
pub const WS_E_OBJECT_FAULTED: HRESULT = 0x803D0001u32 as HRESULT;

/// The operation could not be completed because it would lead to numeric overflow.
pub const WS_E_NUMERIC_OVERFLOW: HRESULT = 0x803D0002u32 as HRESULT;

/// The operation is not allowed due to the current state of the object.
pub const WS_E_INVALID_OPERATION: HRESULT = 0x803D0003u32 as HRESULT;

/// The operation was aborted.
pub const WS_E_OPERATION_ABORTED: HRESULT = 0x803D0004u32 as HRESULT;

/// Access was denied by the remote endpoint.
pub const WS_E_ENDPOINT_ACCESS_DENIED: HRESULT = 0x803D0005u32 as HRESULT;

/// The operation did not complete within the time allotted.
pub const WS_E_OPERATION_TIMED_OUT: HRESULT = 0x803D0006u32 as HRESULT;

/// The operation was abandoned.
pub const WS_E_OPERATION_ABANDONED: HRESULT = 0x803D0007u32 as HRESULT;

/// A quota was exceeded.
pub const WS_E_QUOTA_EXCEEDED: HRESULT = 0x803D0008u32 as HRESULT;

/// The information was not available in the specified language.
pub const WS_E_NO_TRANSLATION_AVAILABLE: HRESULT = 0x803D0009u32 as HRESULT;

/// Security verification was not successful for the received data.
pub const WS_E_SECURITY_VERIFICATION_FAILURE: HRESULT = 0x803D000Au32 as HRESULT;

/// The address is already being used.
pub const WS_E_ADDRESS_IN_USE: HRESULT = 0x803D000Bu32 as HRESULT;

/// The address is not valid for this context.
pub const WS_E_ADDRESS_NOT_AVAILABLE: HRESULT = 0x803D000Cu32 as HRESULT;

/// The remote endpoint does not exist or could not be located.
pub const WS_E_ENDPOINT_NOT_FOUND: HRESULT = 0x803D000Du32 as HRESULT;

/// The remote endpoint is not currently in service at this location.
pub const WS_E_ENDPOINT_NOT_AVAILABLE: HRESULT = 0x803D000Eu32 as HRESULT;

/// The remote endpoint could not process the request.
pub const WS_E_ENDPOINT_FAILURE: HRESULT = 0x803D000Fu32 as HRESULT;

/// The remote endpoint was not reachable.
pub const WS_E_ENDPOINT_UNREACHABLE: HRESULT = 0x803D0010u32 as HRESULT;

/// The operation was not supported by the remote endpoint.
pub const WS_E_ENDPOINT_ACTION_NOT_SUPPORTED: HRESULT = 0x803D0011u32 as HRESULT;

/// The remote endpoint is unable to process the request due to being overloaded.
pub const WS_E_ENDPOINT_TOO_BUSY: HRESULT = 0x803D0012u32 as HRESULT;

/// A message containing a fault was received from the remote endpoint.
pub const WS_E_ENDPOINT_FAULT_RECEIVED: HRESULT = 0x803D0013u32 as HRESULT;

/// The connection with the remote endpoint was terminated.
pub const WS_E_ENDPOINT_DISCONNECTED: HRESULT = 0x803D0014u32 as HRESULT;

/// The HTTP proxy server could not process the request.
pub const WS_E_PROXY_FAILURE: HRESULT = 0x803D0015u32 as HRESULT;

/// Access was denied by the HTTP proxy server.
pub const WS_E_PROXY_ACCESS_DENIED: HRESULT = 0x803D0016u32 as HRESULT;

/// The requested feature is not available on this platform.
pub const WS_E_NOT_SUPPORTED: HRESULT = 0x803D0017u32 as HRESULT;

/// The HTTP proxy server requires HTTP authentication scheme 'basic'.
pub const WS_E_PROXY_REQUIRES_BASIC_AUTH: HRESULT = 0x803D0018u32 as HRESULT;

/// The HTTP proxy server requires HTTP authentication scheme 'digest'.
pub const WS_E_PROXY_REQUIRES_DIGEST_AUTH: HRESULT = 0x803D0019u32 as HRESULT;

/// The HTTP proxy server requires HTTP authentication scheme 'NTLM'.
pub const WS_E_PROXY_REQUIRES_NTLM_AUTH: HRESULT = 0x803D001Au32 as HRESULT;

/// The HTTP proxy server requires HTTP authentication scheme 'negotiate'.
pub const WS_E_PROXY_REQUIRES_NEGOTIATE_AUTH: HRESULT = 0x803D001Bu32 as HRESULT;

/// The remote endpoint requires HTTP authentication scheme 'basic'.
pub const WS_E_SERVER_REQUIRES_BASIC_AUTH: HRESULT = 0x803D001Cu32 as HRESULT;

/// The remote endpoint requires HTTP authentication scheme 'digest'.
pub const WS_E_SERVER_REQUIRES_DIGEST_AUTH: HRESULT = 0x803D001Du32 as HRESULT;

/// The remote endpoint requires HTTP authentication scheme 'NTLM'.
pub const WS_E_SERVER_REQUIRES_NTLM_AUTH: HRESULT = 0x803D001Eu32 as HRESULT;

/// The remote endpoint requires HTTP authentication scheme 'negotiate'.
pub const WS_E_SERVER_REQUIRES_NEGOTIATE_AUTH: HRESULT = 0x803D001Fu32 as HRESULT;

/// The endpoint address URL is invalid.
pub const WS_E_INVALID_ENDPOINT_URL: HRESULT = 0x803D0020u32 as HRESULT;

/// Unrecognized error occurred in the Windows Web Services framework.
pub const WS_E_OTHER: HRESULT = 0x803D0021u32 as HRESULT;

/// A security token was rejected by the server because it has expired.
pub const WS_E_SECURITY_TOKEN_EXPIRED: HRESULT = 0x803D0022u32 as HRESULT;

/// A security operation failed in the Windows Web Services framework.
pub const WS_E_SECURITY_SYSTEM_FAILURE: HRESULT = 0x803D0023u32 as HRESULT;

/// The binding to the network interface is being closed.
pub const ERROR_NDIS_INTERFACE_CLOSING: HRESULT = 0x80340002u32 as HRESULT;

/// An invalid version was specified.
pub const ERROR_NDIS_BAD_VERSION: HRESULT = 0x80340004u32 as HRESULT;

/// An invalid characteristics table was used.
pub const ERROR_NDIS_BAD_CHARACTERISTICS: HRESULT = 0x80340005u32 as HRESULT;

/// Failed to find the network interface or network interface is not ready.
pub const ERROR_NDIS_ADAPTER_NOT_FOUND: HRESULT = 0x80340006u32 as HRESULT;

/// Failed to open the network interface.
pub const ERROR_NDIS_OPEN_FAILED: HRESULT = 0x80340007u32 as HRESULT;

/// Network interface has encountered an internal unrecoverable failure.
pub const ERROR_NDIS_DEVICE_FAILED: HRESULT = 0x80340008u32 as HRESULT;

/// The multicast list on the network interface is full.
pub const ERROR_NDIS_MULTICAST_FULL: HRESULT = 0x80340009u32 as HRESULT;

/// An attempt was made to add a duplicate multicast address to the list.
pub const ERROR_NDIS_MULTICAST_EXISTS: HRESULT = 0x8034000Au32 as HRESULT;

/// At attempt was made to remove a multicast address that was never added.
pub const ERROR_NDIS_MULTICAST_NOT_FOUND: HRESULT = 0x8034000Bu32 as HRESULT;

/// Netowork interface aborted the request.
pub const ERROR_NDIS_REQUEST_ABORTED: HRESULT = 0x8034000Cu32 as HRESULT;

/// Network interface can not process the request because it is being reset.
pub const ERROR_NDIS_RESET_IN_PROGRESS: HRESULT = 0x8034000Du32 as HRESULT;

/// Netword interface does not support this request.
pub const ERROR_NDIS_NOT_SUPPORTED: HRESULT = 0x803400BBu32 as HRESULT;

/// An attempt was made to send an invalid packet on a network interface.
pub const ERROR_NDIS_INVALID_PACKET: HRESULT = 0x8034000Fu32 as HRESULT;

/// Network interface is not ready to complete this operation.
pub const ERROR_NDIS_ADAPTER_NOT_READY: HRESULT = 0x80340011u32 as HRESULT;

/// The length of the buffer submitted for this operation is not valid.
pub const ERROR_NDIS_INVALID_LENGTH: HRESULT = 0x80340014u32 as HRESULT;

/// The data used for this operation is not valid.
pub const ERROR_NDIS_INVALID_DATA: HRESULT = 0x80340015u32 as HRESULT;

/// The length of buffer submitted for this operation is too small.
pub const ERROR_NDIS_BUFFER_TOO_SHORT: HRESULT = 0x80340016u32 as HRESULT;

/// Network interface does not support this OID (Object Identifier)
pub const ERROR_NDIS_INVALID_OID: HRESULT = 0x80340017u32 as HRESULT;

/// The network interface has been removed.
pub const ERROR_NDIS_ADAPTER_REMOVED: HRESULT = 0x80340018u32 as HRESULT;

/// Network interface does not support this media type.
pub const ERROR_NDIS_UNSUPPORTED_MEDIA: HRESULT = 0x80340019u32 as HRESULT;

/// An attempt was made to remove a token ring group address that is in use by other components.
pub const ERROR_NDIS_GROUP_ADDRESS_IN_USE: HRESULT = 0x8034001Au32 as HRESULT;

/// An attempt was made to map a file that can not be found.
pub const ERROR_NDIS_FILE_NOT_FOUND: HRESULT = 0x8034001Bu32 as HRESULT;

/// An error occurred while NDIS tried to map the file.
pub const ERROR_NDIS_ERROR_READING_FILE: HRESULT = 0x8034001Cu32 as HRESULT;

/// An attempt was made to map a file that is alreay mapped.
pub const ERROR_NDIS_ALREADY_MAPPED: HRESULT = 0x8034001Du32 as HRESULT;

/// An attempt to allocate a hardware resource failed because the resource is used by another component.
pub const ERROR_NDIS_RESOURCE_CONFLICT: HRESULT = 0x8034001Eu32 as HRESULT;

/// The I/O operation failed because network media is disconnected or wireless access point is out of range.
pub const ERROR_NDIS_MEDIA_DISCONNECTED: HRESULT = 0x8034001Fu32 as HRESULT;

/// The network address used in the request is invalid.
pub const ERROR_NDIS_INVALID_ADDRESS: HRESULT = 0x80340022u32 as HRESULT;

/// The specified request is not a valid operation for the target device.
pub const ERROR_NDIS_INVALID_DEVICE_REQUEST: HRESULT = 0x80340010u32 as HRESULT;

/// The offload operation on the network interface has been paused.
pub const ERROR_NDIS_PAUSED: HRESULT = 0x8034002Au32 as HRESULT;

/// Network interface was not found.
pub const ERROR_NDIS_INTERFACE_NOT_FOUND: HRESULT = 0x8034002Bu32 as HRESULT;

/// The revision number specified in the structure is not supported.
pub const ERROR_NDIS_UNSUPPORTED_REVISION: HRESULT = 0x8034002Cu32 as HRESULT;

/// The specified port does not exist on this network interface.
pub const ERROR_NDIS_INVALID_PORT: HRESULT = 0x8034002Du32 as HRESULT;

/// The current state of the specified port on this network interface does not support the requested operation.
pub const ERROR_NDIS_INVALID_PORT_STATE: HRESULT = 0x8034002Eu32 as HRESULT;

/// The miniport adapter is in low power state.
pub const ERROR_NDIS_LOW_POWER_STATE: HRESULT = 0x8034002Fu32 as HRESULT;

/// This operation requires the miniport adapter to be reinitialized.
pub const ERROR_NDIS_REINIT_REQUIRED: HRESULT = 0x80340030u32 as HRESULT;

/// There are not enough queues to complete the operation.
pub const ERROR_NDIS_NO_QUEUES: HRESULT = 0x80340031u32 as HRESULT;

/// The wireless local area network interface is in auto configuration mode and doesn't support the requested parameter change operation.
pub const ERROR_NDIS_DOT11_AUTO_CONFIG_ENABLED: HRESULT = 0x80342000u32 as HRESULT;

/// The wireless local area network interface is busy and can not perform the requested operation.
pub const ERROR_NDIS_DOT11_MEDIA_IN_USE: HRESULT = 0x80342001u32 as HRESULT;

/// The wireless local area network interface is powered down and doesn't support the requested operation.
pub const ERROR_NDIS_DOT11_POWER_STATE_INVALID: HRESULT = 0x80342002u32 as HRESULT;

/// The list of wake on LAN patterns is full.
pub const ERROR_NDIS_PM_WOL_PATTERN_LIST_FULL: HRESULT = 0x80342003u32 as HRESULT;

/// The list of low power protocol offloads is full.
pub const ERROR_NDIS_PM_PROTOCOL_OFFLOAD_LIST_FULL: HRESULT = 0x80342004u32 as HRESULT;

/// The wireless local area network interface cannot start an AP on the specified channel right now.
pub const ERROR_NDIS_DOT11_AP_CHANNEL_CURRENTLY_NOT_AVAILABLE: HRESULT = 0x80342005u32 as HRESULT;

/// The wireless local area network interface cannot start an AP on the specified band right now.
pub const ERROR_NDIS_DOT11_AP_BAND_CURRENTLY_NOT_AVAILABLE: HRESULT = 0x80342006u32 as HRESULT;

/// The wireless local area network interface cannot start an AP on this channel due to regulatory reasons.
pub const ERROR_NDIS_DOT11_AP_CHANNEL_NOT_ALLOWED: HRESULT = 0x80342007u32 as HRESULT;

/// The wireless local area network interface cannot start an AP on this band due to regulatory reasons.
pub const ERROR_NDIS_DOT11_AP_BAND_NOT_ALLOWED: HRESULT = 0x80342008u32 as HRESULT;

/// The request will be completed later by NDIS status indication.
pub const ERROR_NDIS_INDICATION_REQUIRED: HRESULT = 0x00340001;

/// The TCP connection is not offloadable because of a local policy setting.
pub const ERROR_NDIS_OFFLOAD_POLICY: HRESULT = 0xC034100Fu32 as HRESULT;

/// The TCP connection is not offloadable by the Chimney Offload target.
pub const ERROR_NDIS_OFFLOAD_CONNECTION_REJECTED: HRESULT = 0xC0341012u32 as HRESULT;

/// The IP Path object is not in an offloadable state.
pub const ERROR_NDIS_OFFLOAD_PATH_REJECTED: HRESULT = 0xC0341013u32 as HRESULT;

/// The hypervisor does not support the operation because the specified hypercall code is not supported.
pub const ERROR_HV_INVALID_HYPERCALL_CODE: HRESULT = 0xC0350002u32 as HRESULT;

/// The hypervisor does not support the operation because the encoding for the hypercall input register is not supported.
pub const ERROR_HV_INVALID_HYPERCALL_INPUT: HRESULT = 0xC0350003u32 as HRESULT;

/// The hypervisor could not perform the operation because a parameter has an invalid alignment.
pub const ERROR_HV_INVALID_ALIGNMENT: HRESULT = 0xC0350004u32 as HRESULT;

/// The hypervisor could not perform the operation because an invalid parameter was specified.
pub const ERROR_HV_INVALID_PARAMETER: HRESULT = 0xC0350005u32 as HRESULT;

/// Access to the specified object was denied.
pub const ERROR_HV_ACCESS_DENIED: HRESULT = 0xC0350006u32 as HRESULT;

/// The hypervisor could not perform the operation because the partition is entering or in an invalid state.
pub const ERROR_HV_INVALID_PARTITION_STATE: HRESULT = 0xC0350007u32 as HRESULT;

/// The operation is not allowed in the current state.
pub const ERROR_HV_OPERATION_DENIED: HRESULT = 0xC0350008u32 as HRESULT;

/// The hypervisor does not recognize the specified partition property.
pub const ERROR_HV_UNKNOWN_PROPERTY: HRESULT = 0xC0350009u32 as HRESULT;

/// The specified value of a partition property is out of range or violates an invariant.
pub const ERROR_HV_PROPERTY_VALUE_OUT_OF_RANGE: HRESULT = 0xC035000Au32 as HRESULT;

/// There is not enough memory in the hypervisor pool to complete the operation.
pub const ERROR_HV_INSUFFICIENT_MEMORY: HRESULT = 0xC035000Bu32 as HRESULT;

/// The maximum partition depth has been exceeded for the partition hierarchy.
pub const ERROR_HV_PARTITION_TOO_DEEP: HRESULT = 0xC035000Cu32 as HRESULT;

/// A partition with the specified partition Id does not exist.
pub const ERROR_HV_INVALID_PARTITION_ID: HRESULT = 0xC035000Du32 as HRESULT;

/// The hypervisor could not perform the operation because the specified VP index is invalid.
pub const ERROR_HV_INVALID_VP_INDEX: HRESULT = 0xC035000Eu32 as HRESULT;

/// The hypervisor could not perform the operation because the specified port identifier is invalid.
pub const ERROR_HV_INVALID_PORT_ID: HRESULT = 0xC0350011u32 as HRESULT;

/// The hypervisor could not perform the operation because the specified connection identifier is invalid.
pub const ERROR_HV_INVALID_CONNECTION_ID: HRESULT = 0xC0350012u32 as HRESULT;

/// Not enough buffers were supplied to send a message.
pub const ERROR_HV_INSUFFICIENT_BUFFERS: HRESULT = 0xC0350013u32 as HRESULT;

/// The previous virtual interrupt has not been acknowledged.
pub const ERROR_HV_NOT_ACKNOWLEDGED: HRESULT = 0xC0350014u32 as HRESULT;

/// A virtual processor is not in the correct state for the indicated operation.
pub const ERROR_HV_INVALID_VP_STATE: HRESULT = 0xC0350015u32 as HRESULT;

/// The previous virtual interrupt has already been acknowledged.
pub const ERROR_HV_ACKNOWLEDGED: HRESULT = 0xC0350016u32 as HRESULT;

/// The indicated partition is not in a valid state for saving or restoring.
pub const ERROR_HV_INVALID_SAVE_RESTORE_STATE: HRESULT = 0xC0350017u32 as HRESULT;

/// The hypervisor could not complete the operation because a required feature of the synthetic interrupt controller (SynIC) was disabled.
pub const ERROR_HV_INVALID_SYNIC_STATE: HRESULT = 0xC0350018u32 as HRESULT;

/// The hypervisor could not perform the operation because the object or value was either already in use or being used for a purpose that would not permit completing the operation.
pub const ERROR_HV_OBJECT_IN_USE: HRESULT = 0xC0350019u32 as HRESULT;

/// The proximity domain information is invalid.
pub const ERROR_HV_INVALID_PROXIMITY_DOMAIN_INFO: HRESULT = 0xC035001Au32 as HRESULT;

/// An attempt to retrieve debugging data failed because none was available.
pub const ERROR_HV_NO_DATA: HRESULT = 0xC035001Bu32 as HRESULT;

/// The physical connection being used for debugging has not recorded any receive activity since the last operation.
pub const ERROR_HV_INACTIVE: HRESULT = 0xC035001Cu32 as HRESULT;

/// There are not enough resources to complete the operation.
pub const ERROR_HV_NO_RESOURCES: HRESULT = 0xC035001Du32 as HRESULT;

/// A hypervisor feature is not available to the user.
pub const ERROR_HV_FEATURE_UNAVAILABLE: HRESULT = 0xC035001Eu32 as HRESULT;

/// The specified buffer was too small to contain all of the requested data.
pub const ERROR_HV_INSUFFICIENT_BUFFER: HRESULT = 0xC0350033u32 as HRESULT;

/// The maximum number of domains supported by the platform I/O remapping hardware is currently in use. No domains are available to assign this device to this partition.
pub const ERROR_HV_INSUFFICIENT_DEVICE_DOMAINS: HRESULT = 0xC0350038u32 as HRESULT;

/// Validation of CPUID data of the processor failed.
pub const ERROR_HV_CPUID_FEATURE_VALIDATION: HRESULT = 0xC035003Cu32 as HRESULT;

/// Validation of XSAVE CPUID data of the processor failed.
pub const ERROR_HV_CPUID_XSAVE_FEATURE_VALIDATION: HRESULT = 0xC035003Du32 as HRESULT;

/// Processor did not respond within the timeout period.
pub const ERROR_HV_PROCESSOR_STARTUP_TIMEOUT: HRESULT = 0xC035003Eu32 as HRESULT;

/// SMX has been enabled in the BIOS.
pub const ERROR_HV_SMX_ENABLED: HRESULT = 0xC035003Fu32 as HRESULT;

/// The hypervisor could not perform the operation because the specified LP index is invalid.
pub const ERROR_HV_INVALID_LP_INDEX: HRESULT = 0xC0350041u32 as HRESULT;

/// The supplied register value is invalid.
pub const ERROR_HV_INVALID_REGISTER_VALUE: HRESULT = 0xC0350050u32 as HRESULT;

/// The supplied virtual trust level is not in the correct state to perform the requested operation.
pub const ERROR_HV_INVALID_VTL_STATE: HRESULT = 0xC0350051u32 as HRESULT;

/// No execute feature (NX) is not present or not enabled in the BIOS.
pub const ERROR_HV_NX_NOT_DETECTED: HRESULT = 0xC0350055u32 as HRESULT;

/// The supplied device ID is invalid.
pub const ERROR_HV_INVALID_DEVICE_ID: HRESULT = 0xC0350057u32 as HRESULT;

/// The operation is not allowed in the current device state.
pub const ERROR_HV_INVALID_DEVICE_STATE: HRESULT = 0xC0350058u32 as HRESULT;

/// The device had pending page requests which were discarded.
pub const ERROR_HV_PENDING_PAGE_REQUESTS: HRESULT = 0x00350059;

/// The supplied page request specifies a memory access that the guest does not have permissions to perform.
pub const ERROR_HV_PAGE_REQUEST_INVALID: HRESULT = 0xC0350060u32 as HRESULT;

/// A CPU group with the specified CPU group Id does not exist.
pub const ERROR_HV_INVALID_CPU_GROUP_ID: HRESULT = 0xC035006Fu32 as HRESULT;

/// The hypervisor could not perform the operation because the CPU group is entering or in an invalid state.
pub const ERROR_HV_INVALID_CPU_GROUP_STATE: HRESULT = 0xC0350070u32 as HRESULT;

/// The requested operation failed.
pub const ERROR_HV_OPERATION_FAILED: HRESULT = 0xC0350071u32 as HRESULT;

/// The hypervisor could not perform the operation because it is not allowed with nested virtualization active.
pub const ERROR_HV_NOT_ALLOWED_WITH_NESTED_VIRT_ACTIVE: HRESULT = 0xC0350072u32 as HRESULT;

/// There is not enough memory in the root partition's pool to complete the operation.
pub const ERROR_HV_INSUFFICIENT_ROOT_MEMORY: HRESULT = 0xC0350073u32 as HRESULT;

/// The provided event log buffer was already marked as freed.
pub const ERROR_HV_EVENT_BUFFER_ALREADY_FREED: HRESULT = 0xC0350074u32 as HRESULT;

/// There is not enough contiguous memory in the partition's pool to complete the operation.
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_MEMORY: HRESULT = 0xC0350075u32 as HRESULT;

/// The device is not in a device domain.
pub const ERROR_HV_DEVICE_NOT_IN_DOMAIN: HRESULT = 0xC0350076u32 as HRESULT;

/// The requested operation would result in a nested vm-exit.
pub const ERROR_HV_NESTED_VM_EXIT: HRESULT = 0xC0350077u32 as HRESULT;

/// The requested access to the model specific register failed.
pub const ERROR_HV_MSR_ACCESS_FAILED: HRESULT = 0xC0350080u32 as HRESULT;

/// There is not enough memory in the hypervisor pool to complete the mirroring operation.
pub const ERROR_HV_INSUFFICIENT_MEMORY_MIRRORING: HRESULT = 0xC0350081u32 as HRESULT;

/// There is not enough contiguous memory in the hypervisor pool to complete the mirroring operation.
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_MEMORY_MIRRORING: HRESULT = 0xC0350082u32 as HRESULT;

/// There is not enough contiguous memory in the root partition's pool to complete the operation.
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_ROOT_MEMORY: HRESULT = 0xC0350083u32 as HRESULT;

/// There is not enough memory in the root partition's pool to complete the mirroring operation.
pub const ERROR_HV_INSUFFICIENT_ROOT_MEMORY_MIRRORING: HRESULT = 0xC0350084u32 as HRESULT;

/// There is not enough contiguous memory in the root partition's pool to complete the mirroring operation.
pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_ROOT_MEMORY_MIRRORING: HRESULT =
    0xC0350085u32 as HRESULT;

/// No hypervisor is present on this system.
pub const ERROR_HV_NOT_PRESENT: HRESULT = 0xC0351000u32 as HRESULT;

/// The handler for the virtualization infrastructure driver is already registered. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_DUPLICATE_HANDLER: HRESULT = 0xC0370001u32 as HRESULT;

/// The number of registered handlers for the virtualization infrastructure driver exceeded the maximum. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_TOO_MANY_HANDLERS: HRESULT = 0xC0370002u32 as HRESULT;

/// The message queue for the virtualization infrastructure driver is full and cannot accept new messages. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_QUEUE_FULL: HRESULT = 0xC0370003u32 as HRESULT;

/// No handler exists to handle the message for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_HANDLER_NOT_PRESENT: HRESULT = 0xC0370004u32 as HRESULT;

/// The name of the partition or message queue for the virtualization infrastructure driver is invalid. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_INVALID_OBJECT_NAME: HRESULT = 0xC0370005u32 as HRESULT;

/// The partition name of the virtualization infrastructure driver exceeds the maximum.
pub const ERROR_VID_PARTITION_NAME_TOO_LONG: HRESULT = 0xC0370006u32 as HRESULT;

/// The message queue name of the virtualization infrastructure driver exceeds the maximum.
pub const ERROR_VID_MESSAGE_QUEUE_NAME_TOO_LONG: HRESULT = 0xC0370007u32 as HRESULT;

/// Cannot create the partition for the virtualization infrastructure driver because another partition with the same name already exists.
pub const ERROR_VID_PARTITION_ALREADY_EXISTS: HRESULT = 0xC0370008u32 as HRESULT;

/// The virtualization infrastructure driver has encountered an error. The requested partition does not exist. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_PARTITION_DOES_NOT_EXIST: HRESULT = 0xC0370009u32 as HRESULT;

/// The virtualization infrastructure driver has encountered an error. Could not find the requested partition. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_PARTITION_NAME_NOT_FOUND: HRESULT = 0xC037000Au32 as HRESULT;

/// A message queue with the same name already exists for the virtualization infrastructure driver.
pub const ERROR_VID_MESSAGE_QUEUE_ALREADY_EXISTS: HRESULT = 0xC037000Bu32 as HRESULT;

/// The memory block page for the virtualization infrastructure driver cannot be mapped because the page map limit has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT: HRESULT = 0xC037000Cu32 as HRESULT;

/// The memory block for the virtualization infrastructure driver is still being used and cannot be destroyed.
pub const ERROR_VID_MB_STILL_REFERENCED: HRESULT = 0xC037000Du32 as HRESULT;

/// Cannot unlock the page array for the guest operating system memory address because it does not match a previous lock request. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_CHILD_GPA_PAGE_SET_CORRUPTED: HRESULT = 0xC037000Eu32 as HRESULT;

/// The non-uniform memory access (NUMA) node settings do not match the system NUMA topology. In order to start the virtual machine, you will need to modify the NUMA configuration.
pub const ERROR_VID_INVALID_NUMA_SETTINGS: HRESULT = 0xC037000Fu32 as HRESULT;

/// The non-uniform memory access (NUMA) node index does not match a valid index in the system NUMA topology.
pub const ERROR_VID_INVALID_NUMA_NODE_INDEX: HRESULT = 0xC0370010u32 as HRESULT;

/// The memory block for the virtualization infrastructure driver is already associated with a message queue.
pub const ERROR_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED: HRESULT = 0xC0370011u32 as HRESULT;

/// The handle is not a valid memory block handle for the virtualization infrastructure driver.
pub const ERROR_VID_INVALID_MEMORY_BLOCK_HANDLE: HRESULT = 0xC0370012u32 as HRESULT;

/// The request exceeded the memory block page limit for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_PAGE_RANGE_OVERFLOW: HRESULT = 0xC0370013u32 as HRESULT;

/// The handle is not a valid message queue handle for the virtualization infrastructure driver.
pub const ERROR_VID_INVALID_MESSAGE_QUEUE_HANDLE: HRESULT = 0xC0370014u32 as HRESULT;

/// The handle is not a valid page range handle for the virtualization infrastructure driver.
pub const ERROR_VID_INVALID_GPA_RANGE_HANDLE: HRESULT = 0xC0370015u32 as HRESULT;

/// Cannot install client notifications because no message queue for the virtualization infrastructure driver is associated with the memory block.
pub const ERROR_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE: HRESULT = 0xC0370016u32 as HRESULT;

/// The request to lock or map a memory block page failed because the virtualization infrastructure driver memory block limit has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED: HRESULT = 0xC0370017u32 as HRESULT;

/// The handle is not a valid parent partition mapping handle for the virtualization infrastructure driver.
pub const ERROR_VID_INVALID_PPM_HANDLE: HRESULT = 0xC0370018u32 as HRESULT;

/// Notifications cannot be created on the memory block because it is use.
pub const ERROR_VID_MBPS_ARE_LOCKED: HRESULT = 0xC0370019u32 as HRESULT;

/// The message queue for the virtualization infrastructure driver has been closed. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_MESSAGE_QUEUE_CLOSED: HRESULT = 0xC037001Au32 as HRESULT;

/// Cannot add a virtual processor to the partition because the maximum has been reached.
pub const ERROR_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED: HRESULT = 0xC037001Bu32 as HRESULT;

/// Cannot stop the virtual processor immediately because of a pending intercept.
pub const ERROR_VID_STOP_PENDING: HRESULT = 0xC037001Cu32 as HRESULT;

/// Invalid state for the virtual processor. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_INVALID_PROCESSOR_STATE: HRESULT = 0xC037001Du32 as HRESULT;

/// The maximum number of kernel mode clients for the virtualization infrastructure driver has been reached. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT: HRESULT = 0xC037001Eu32 as HRESULT;

/// This kernel mode interface for the virtualization infrastructure driver has already been initialized. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_KM_INTERFACE_ALREADY_INITIALIZED: HRESULT = 0xC037001Fu32 as HRESULT;

/// Cannot set or reset the memory block property more than once for the virtualization infrastructure driver. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_MB_PROPERTY_ALREADY_SET_RESET: HRESULT = 0xC0370020u32 as HRESULT;

/// The memory mapped I/O for this page range no longer exists. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_MMIO_RANGE_DESTROYED: HRESULT = 0xC0370021u32 as HRESULT;

/// The lock or unlock request uses an invalid guest operating system memory address. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_INVALID_CHILD_GPA_PAGE_SET: HRESULT = 0xC0370022u32 as HRESULT;

/// Cannot destroy or reuse the reserve page set for the virtualization infrastructure driver because it is in use. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_RESERVE_PAGE_SET_IS_BEING_USED: HRESULT = 0xC0370023u32 as HRESULT;

/// The reserve page set for the virtualization infrastructure driver is too small to use in the lock request. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_RESERVE_PAGE_SET_TOO_SMALL: HRESULT = 0xC0370024u32 as HRESULT;

/// Cannot lock or map the memory block page for the virtualization infrastructure driver because it has already been locked using a reserve page set page. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE: HRESULT = 0xC0370025u32 as HRESULT;

/// Cannot create the memory block for the virtualization infrastructure driver because the requested number of pages exceeded the limit. Restarting the virtual machine may fix the problem. If the problem persists, try restarting the physical computer.
pub const ERROR_VID_MBP_COUNT_EXCEEDED_LIMIT: HRESULT = 0xC0370026u32 as HRESULT;

/// Cannot restore this virtual machine because the saved state data cannot be read. Delete the saved state data and then try to start the virtual machine.
pub const ERROR_VID_SAVED_STATE_CORRUPT: HRESULT = 0xC0370027u32 as HRESULT;

/// Cannot restore this virtual machine because an item read from the saved state data is not recognized. Delete the saved state data and then try to start the virtual machine.
pub const ERROR_VID_SAVED_STATE_UNRECOGNIZED_ITEM: HRESULT = 0xC0370028u32 as HRESULT;

/// Cannot restore this virtual machine to the saved state because of hypervisor incompatibility. Delete the saved state data and then try to start the virtual machine.
pub const ERROR_VID_SAVED_STATE_INCOMPATIBLE: HRESULT = 0xC0370029u32 as HRESULT;

/// The specified VTL does not have the permission to access the resource.
pub const ERROR_VID_VTL_ACCESS_DENIED: HRESULT = 0xC037002Au32 as HRESULT;

/// Failed to allocate backing memory due to insufficient memory resources.
pub const ERROR_VID_INSUFFICIENT_RESOURCES_RESERVE: HRESULT = 0xC037002Bu32 as HRESULT;

/// Failed to allocate memory for the physical buffer used to back certain internal structures.
pub const ERROR_VID_INSUFFICIENT_RESOURCES_PHYSICAL_BUFFER: HRESULT = 0xC037002Cu32 as HRESULT;

/// Failed to allocate memory to be deposited in the hypervisor.
pub const ERROR_VID_INSUFFICIENT_RESOURCES_HV_DEPOSIT: HRESULT = 0xC037002Du32 as HRESULT;

/// Memory type not supported for requested operation.
pub const ERROR_VID_MEMORY_TYPE_NOT_SUPPORTED: HRESULT = 0xC037002Eu32 as HRESULT;

/// Failed to withdraw memory.
pub const ERROR_VID_INSUFFICIENT_RESOURCES_WITHDRAW: HRESULT = 0xC037002Fu32 as HRESULT;

/// The process has already been set.
pub const ERROR_VID_PROCESS_ALREADY_SET: HRESULT = 0xC0370030u32 as HRESULT;

/// The virtual machine or container exited unexpectedly while starting.
pub const ERROR_VMCOMPUTE_TERMINATED_DURING_START: HRESULT = 0xC0370100u32 as HRESULT;

/// The container operating system does not match the host operating system.
pub const ERROR_VMCOMPUTE_IMAGE_MISMATCH: HRESULT = 0xC0370101u32 as HRESULT;

/// The virtual machine could not be started because a required feature is not installed.
pub const ERROR_VMCOMPUTE_HYPERV_NOT_INSTALLED: HRESULT = 0xC0370102u32 as HRESULT;

/// The call to start an asynchronous operation succeeded and the operation is performed in the background.
pub const ERROR_VMCOMPUTE_OPERATION_PENDING: HRESULT = 0xC0370103u32 as HRESULT;

/// The supported number of notification callbacks has been exceeded.
pub const ERROR_VMCOMPUTE_TOO_MANY_NOTIFICATIONS: HRESULT = 0xC0370104u32 as HRESULT;

/// The requested virtual machine or container operation is not valid in the current state.
pub const ERROR_VMCOMPUTE_INVALID_STATE: HRESULT = 0xC0370105u32 as HRESULT;

/// The virtual machine or container exited unexpectedly.
pub const ERROR_VMCOMPUTE_UNEXPECTED_EXIT: HRESULT = 0xC0370106u32 as HRESULT;

/// The virtual machine or container was forcefully exited.
pub const ERROR_VMCOMPUTE_TERMINATED: HRESULT = 0xC0370107u32 as HRESULT;

/// A connection could not be established with the container or virtual machine.
pub const ERROR_VMCOMPUTE_CONNECT_FAILED: HRESULT = 0xC0370108u32 as HRESULT;

/// The operation timed out because a response was not received from the virtual machine or container.
pub const ERROR_VMCOMPUTE_TIMEOUT: HRESULT = 0xC0370109u32 as HRESULT;

/// The connection with the virtual machine or container was closed.
pub const ERROR_VMCOMPUTE_CONNECTION_CLOSED: HRESULT = 0xC037010Au32 as HRESULT;

/// An unknown internal message was received by the virtual machine or container.
pub const ERROR_VMCOMPUTE_UNKNOWN_MESSAGE: HRESULT = 0xC037010Bu32 as HRESULT;

/// The virtual machine or container does not support an available version of the communication protocol with the host.
pub const ERROR_VMCOMPUTE_UNSUPPORTED_PROTOCOL_VERSION: HRESULT = 0xC037010Cu32 as HRESULT;

/// The virtual machine or container JSON document is invalid.
pub const ERROR_VMCOMPUTE_INVALID_JSON: HRESULT = 0xC037010Du32 as HRESULT;

/// A virtual machine or container with the specified identifier does not exist.
pub const ERROR_VMCOMPUTE_SYSTEM_NOT_FOUND: HRESULT = 0xC037010Eu32 as HRESULT;

/// A virtual machine or container with the specified identifier already exists.
pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_EXISTS: HRESULT = 0xC037010Fu32 as HRESULT;

/// The virtual machine or container with the specified identifier is not running.
pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_STOPPED: HRESULT = 0xC0370110u32 as HRESULT;

/// A communication protocol error has occurred between the virtual machine or container and the host.
pub const ERROR_VMCOMPUTE_PROTOCOL_ERROR: HRESULT = 0xC0370111u32 as HRESULT;

/// The container image contains a layer with an unrecognized format.
pub const ERROR_VMCOMPUTE_INVALID_LAYER: HRESULT = 0xC0370112u32 as HRESULT;

/// To use this container image, you must join the Windows Insider Program. Please see <https://go.microsoft.com/fwlink/?linkid=850659> for more information.
pub const ERROR_VMCOMPUTE_WINDOWS_INSIDER_REQUIRED: HRESULT = 0xC0370113u32 as HRESULT;

/// The virtual machine or container exited unexpectedly while starting.
pub const HCS_E_TERMINATED_DURING_START: HRESULT = 0x80370100u32 as HRESULT;

/// The container operating system does not match the host operating system.
pub const HCS_E_IMAGE_MISMATCH: HRESULT = 0x80370101u32 as HRESULT;

/// The virtual machine could not be started because a required feature is not installed.
pub const HCS_E_HYPERV_NOT_INSTALLED: HRESULT = 0x80370102u32 as HRESULT;

/// The requested virtual machine or container operation is not valid in the current state.
pub const HCS_E_INVALID_STATE: HRESULT = 0x80370105u32 as HRESULT;

/// The virtual machine or container exited unexpectedly.
pub const HCS_E_UNEXPECTED_EXIT: HRESULT = 0x80370106u32 as HRESULT;

/// The virtual machine or container was forcefully exited.
pub const HCS_E_TERMINATED: HRESULT = 0x80370107u32 as HRESULT;

/// A connection could not be established with the container or virtual machine.
pub const HCS_E_CONNECT_FAILED: HRESULT = 0x80370108u32 as HRESULT;

/// The operation timed out because a response was not received from the virtual machine or container.
pub const HCS_E_CONNECTION_TIMEOUT: HRESULT = 0x80370109u32 as HRESULT;

/// The connection with the virtual machine or container was closed.
pub const HCS_E_CONNECTION_CLOSED: HRESULT = 0x8037010Au32 as HRESULT;

/// An unknown internal message was received by the virtual machine or container.
pub const HCS_E_UNKNOWN_MESSAGE: HRESULT = 0x8037010Bu32 as HRESULT;

/// The virtual machine or container does not support an available version of the communication protocol with the host.
pub const HCS_E_UNSUPPORTED_PROTOCOL_VERSION: HRESULT = 0x8037010Cu32 as HRESULT;

/// The virtual machine or container JSON document is invalid.
pub const HCS_E_INVALID_JSON: HRESULT = 0x8037010Du32 as HRESULT;

/// A virtual machine or container with the specified identifier does not exist.
pub const HCS_E_SYSTEM_NOT_FOUND: HRESULT = 0x8037010Eu32 as HRESULT;

/// A virtual machine or container with the specified identifier already exists.
pub const HCS_E_SYSTEM_ALREADY_EXISTS: HRESULT = 0x8037010Fu32 as HRESULT;

/// The virtual machine or container with the specified identifier is not running.
pub const HCS_E_SYSTEM_ALREADY_STOPPED: HRESULT = 0x80370110u32 as HRESULT;

/// A communication protocol error has occurred between the virtual machine or container and the host.
pub const HCS_E_PROTOCOL_ERROR: HRESULT = 0x80370111u32 as HRESULT;

/// The container image contains a layer with an unrecognized format.
pub const HCS_E_INVALID_LAYER: HRESULT = 0x80370112u32 as HRESULT;

/// To use this container image, you must join the Windows Insider Program. Please see <https://go.microsoft.com/fwlink/?linkid=850659> for more information.
pub const HCS_E_WINDOWS_INSIDER_REQUIRED: HRESULT = 0x80370113u32 as HRESULT;

/// The operation could not be started because a required feature is not installed.
pub const HCS_E_SERVICE_NOT_AVAILABLE: HRESULT = 0x80370114u32 as HRESULT;

/// The operation has not started.
pub const HCS_E_OPERATION_NOT_STARTED: HRESULT = 0x80370115u32 as HRESULT;

/// The operation is already running.
pub const HCS_E_OPERATION_ALREADY_STARTED: HRESULT = 0x80370116u32 as HRESULT;

/// The operation is still running.
pub const HCS_E_OPERATION_PENDING: HRESULT = 0x80370117u32 as HRESULT;

/// The operation did not complete in time.
pub const HCS_E_OPERATION_TIMEOUT: HRESULT = 0x80370118u32 as HRESULT;

/// An event callback has already been registered on this handle.
pub const HCS_E_OPERATION_SYSTEM_CALLBACK_ALREADY_SET: HRESULT = 0x80370119u32 as HRESULT;

/// Not enough memory available to return the result of the operation.
pub const HCS_E_OPERATION_RESULT_ALLOCATION_FAILED: HRESULT = 0x8037011Au32 as HRESULT;

/// Insufficient privileges. Only administrators or users that are members of the Hyper-V Administrators user group are permitted to access virtual machines or containers. To add yourself to the Hyper-V Administrators user group, please see <https://aka.ms/hcsadmin> for more information.
pub const HCS_E_ACCESS_DENIED: HRESULT = 0x8037011Bu32 as HRESULT;

/// The virtual machine or container reported a critical error and was stopped or restarted.
pub const HCS_E_GUEST_CRITICAL_ERROR: HRESULT = 0x8037011Cu32 as HRESULT;

/// The process information is not available.
pub const HCS_E_PROCESS_INFO_NOT_AVAILABLE: HRESULT = 0x8037011Du32 as HRESULT;

/// The host compute system service has disconnected unexpectedly.
pub const HCS_E_SERVICE_DISCONNECT: HRESULT = 0x8037011Eu32 as HRESULT;

/// The process has already exited.
pub const HCS_E_PROCESS_ALREADY_STOPPED: HRESULT = 0x8037011Fu32 as HRESULT;

/// The virtual machine or container is not configured to perform the operation.
pub const HCS_E_SYSTEM_NOT_CONFIGURED_FOR_OPERATION: HRESULT = 0x80370120u32 as HRESULT;

/// The operation has already been cancelled.
pub const HCS_E_OPERATION_ALREADY_CANCELLED: HRESULT = 0x80370121u32 as HRESULT;

/// A virtual switch with the given name was not found.
pub const ERROR_VNET_VIRTUAL_SWITCH_NAME_NOT_FOUND: HRESULT = 0xC0370200u32 as HRESULT;

/// A virtual machine is running with its memory allocated across multiple NUMA nodes. This does not indicate a problem unless the performance of your virtual machine is unusually slow. If you are experiencing performance problems, you may need to modify the NUMA configuration.
pub const ERROR_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED: HRESULT = 0x80370001u32 as HRESULT;

/// The specified capability does not exist.
pub const WHV_E_UNKNOWN_CAPABILITY: HRESULT = 0x80370300u32 as HRESULT;

/// The specified buffer is too small for the requested data.
pub const WHV_E_INSUFFICIENT_BUFFER: HRESULT = 0x80370301u32 as HRESULT;

/// The specified property does not exist.
pub const WHV_E_UNKNOWN_PROPERTY: HRESULT = 0x80370302u32 as HRESULT;

/// The configuration of the hypervisor on this system is not supported.
pub const WHV_E_UNSUPPORTED_HYPERVISOR_CONFIG: HRESULT = 0x80370303u32 as HRESULT;

/// The configuration of the partition is not valid.
pub const WHV_E_INVALID_PARTITION_CONFIG: HRESULT = 0x80370304u32 as HRESULT;

/// The specified GPA range was not found.
pub const WHV_E_GPA_RANGE_NOT_FOUND: HRESULT = 0x80370305u32 as HRESULT;

/// A virtual processor with the specified index already exists.
pub const WHV_E_VP_ALREADY_EXISTS: HRESULT = 0x80370306u32 as HRESULT;

/// A virtual processor with the specified index does not exist.
pub const WHV_E_VP_DOES_NOT_EXIST: HRESULT = 0x80370307u32 as HRESULT;

/// The virtual processor is not in the correct state to perform the requested operation.
pub const WHV_E_INVALID_VP_STATE: HRESULT = 0x80370308u32 as HRESULT;

/// A virtual processor register with the specified name does not exist.
pub const WHV_E_INVALID_VP_REGISTER_NAME: HRESULT = 0x80370309u32 as HRESULT;

/// The Windows Hypervisor Platform is not supported due to a processor limitation.
pub const WHV_E_UNSUPPORTED_PROCESSOR_CONFIG: HRESULT = 0x80370310u32 as HRESULT;

/// Cannot restore this virtual machine because a file read from the vSMB saved state data could not be found. Delete the saved state data and then try to start the virtual machine.
pub const ERROR_VSMB_SAVED_STATE_FILE_NOT_FOUND: HRESULT = 0xC0370400u32 as HRESULT;

/// Cannot restore this virtual machine because the vSMB saved state data cannot be read. Delete the saved state data and then try to start the virtual machine.
pub const ERROR_VSMB_SAVED_STATE_CORRUPT: HRESULT = 0xC0370401u32 as HRESULT;

/// Partition state blob not found. Make sure the virtual machine is saved for this content to be included in the saved state file(s).
pub const VM_SAVED_STATE_DUMP_E_PARTITION_STATE_NOT_FOUND: HRESULT = 0xC0370500u32 as HRESULT;

/// Guest memory not found. Make sure the virtual machine is saved for this content to be included in the saved state file(s).
pub const VM_SAVED_STATE_DUMP_E_GUEST_MEMORY_NOT_FOUND: HRESULT = 0xC0370501u32 as HRESULT;

/// No virtual processor information found in the saved partition blob. Make sure the virtual machine is saved successfully for this content to be included in the partition state.
pub const VM_SAVED_STATE_DUMP_E_NO_VP_FOUND_IN_PARTITION_STATE: HRESULT = 0xC0370502u32 as HRESULT;

/// A virtual processor has been detected to have nested virtualization enabled. Nested Virtualization is not supported yet by VmSavedStateDumpProvider.
pub const VM_SAVED_STATE_DUMP_E_NESTED_VIRTUALIZATION_NOT_SUPPORTED: HRESULT =
    0xC0370503u32 as HRESULT;

/// The Windows kernel image address could not be found in the virtual machine saved state.
pub const VM_SAVED_STATE_DUMP_E_WINDOWS_KERNEL_IMAGE_NOT_FOUND: HRESULT = 0xC0370504u32 as HRESULT;

/// The given virtual address is not mapped to a physical address.
pub const VM_SAVED_STATE_DUMP_E_VA_NOT_MAPPED: HRESULT = 0xC0370505u32 as HRESULT;

/// The virtual processor is not in the correct state for the operation.
pub const VM_SAVED_STATE_DUMP_E_INVALID_VP_STATE: HRESULT = 0xC0370506u32 as HRESULT;

/// The active virtual trust level is not enabled on the specified virtual processor.
pub const VM_SAVED_STATE_DUMP_E_VP_VTL_NOT_ENABLED: HRESULT = 0xC0370509u32 as HRESULT;

/// The attempted DM / resize operation exceeds the supported size.
pub const ERROR_DM_OPERATION_LIMIT_EXCEEDED: HRESULT = 0xC0370600u32 as HRESULT;

/// The regeneration operation was not able to copy all data from the active plexes due to bad sectors.
pub const ERROR_VOLMGR_INCOMPLETE_REGENERATION: HRESULT = 0x80380001u32 as HRESULT;

/// One or more disks were not fully migrated to the target pack. They may or may not require reimport after fixing the hardware problems.
pub const ERROR_VOLMGR_INCOMPLETE_DISK_MIGRATION: HRESULT = 0x80380002u32 as HRESULT;

/// The configuration database is full.
pub const ERROR_VOLMGR_DATABASE_FULL: HRESULT = 0xC0380001u32 as HRESULT;

/// The configuration data on the disk is corrupted.
pub const ERROR_VOLMGR_DISK_CONFIGURATION_CORRUPTED: HRESULT = 0xC0380002u32 as HRESULT;

/// The configuration on the disk is not insync with the in-memory configuration.
pub const ERROR_VOLMGR_DISK_CONFIGURATION_NOT_IN_SYNC: HRESULT = 0xC0380003u32 as HRESULT;

/// A majority of disks failed to be updated with the new configuration.
pub const ERROR_VOLMGR_PACK_CONFIG_UPDATE_FAILED: HRESULT = 0xC0380004u32 as HRESULT;

/// The disk contains non-simple volumes.
pub const ERROR_VOLMGR_DISK_CONTAINS_NON_SIMPLE_VOLUME: HRESULT = 0xC0380005u32 as HRESULT;

/// The same disk was specified more than once in the migration list.
pub const ERROR_VOLMGR_DISK_DUPLICATE: HRESULT = 0xC0380006u32 as HRESULT;

/// The disk is already dynamic.
pub const ERROR_VOLMGR_DISK_DYNAMIC: HRESULT = 0xC0380007u32 as HRESULT;

/// The specified disk id is invalid. There are no disks with the specified disk id.
pub const ERROR_VOLMGR_DISK_ID_INVALID: HRESULT = 0xC0380008u32 as HRESULT;

/// The specified disk is an invalid disk. Operation cannot complete on an invalid disk.
pub const ERROR_VOLMGR_DISK_INVALID: HRESULT = 0xC0380009u32 as HRESULT;

/// The specified disk(s) cannot be removed since it is the last remaining voter.
pub const ERROR_VOLMGR_DISK_LAST_VOTER: HRESULT = 0xC038000Au32 as HRESULT;

/// The specified disk has an invalid disk layout.
pub const ERROR_VOLMGR_DISK_LAYOUT_INVALID: HRESULT = 0xC038000Bu32 as HRESULT;

/// The disk layout contains non-basic partitions which appear after basic partitions. This is an invalid disk layout.
pub const ERROR_VOLMGR_DISK_LAYOUT_NON_BASIC_BETWEEN_BASIC_PARTITIONS: HRESULT =
    0xC038000Cu32 as HRESULT;

/// The disk layout contains partitions which are not cylinder aligned.
pub const ERROR_VOLMGR_DISK_LAYOUT_NOT_CYLINDER_ALIGNED: HRESULT = 0xC038000Du32 as HRESULT;

/// The disk layout contains partitions which are smaller than the minimum size.
pub const ERROR_VOLMGR_DISK_LAYOUT_PARTITIONS_TOO_SMALL: HRESULT = 0xC038000Eu32 as HRESULT;

/// The disk layout contains primary partitions in between logical drives. This is an invalid disk layout.
pub const ERROR_VOLMGR_DISK_LAYOUT_PRIMARY_BETWEEN_LOGICAL_PARTITIONS: HRESULT =
    0xC038000Fu32 as HRESULT;

/// The disk layout contains more than the maximum number of supported partitions.
pub const ERROR_VOLMGR_DISK_LAYOUT_TOO_MANY_PARTITIONS: HRESULT = 0xC0380010u32 as HRESULT;

/// The specified disk is missing. The operation cannot complete on a missing disk.
pub const ERROR_VOLMGR_DISK_MISSING: HRESULT = 0xC0380011u32 as HRESULT;

/// The specified disk is not empty.
pub const ERROR_VOLMGR_DISK_NOT_EMPTY: HRESULT = 0xC0380012u32 as HRESULT;

/// There is not enough usable space for this operation.
pub const ERROR_VOLMGR_DISK_NOT_ENOUGH_SPACE: HRESULT = 0xC0380013u32 as HRESULT;

/// The force revectoring of bad sectors failed.
pub const ERROR_VOLMGR_DISK_REVECTORING_FAILED: HRESULT = 0xC0380014u32 as HRESULT;

/// The specified disk has an invalid sector size.
pub const ERROR_VOLMGR_DISK_SECTOR_SIZE_INVALID: HRESULT = 0xC0380015u32 as HRESULT;

/// The specified disk set contains volumes which exist on disks outside of the set.
pub const ERROR_VOLMGR_DISK_SET_NOT_CONTAINED: HRESULT = 0xC0380016u32 as HRESULT;

/// A disk in the volume layout provides extents to more than one member of a plex.
pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_MEMBERS: HRESULT = 0xC0380017u32 as HRESULT;

/// A disk in the volume layout provides extents to more than one plex.
pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_PLEXES: HRESULT = 0xC0380018u32 as HRESULT;

/// Dynamic disks are not supported on this system.
pub const ERROR_VOLMGR_DYNAMIC_DISK_NOT_SUPPORTED: HRESULT = 0xC0380019u32 as HRESULT;

/// The specified extent is already used by other volumes.
pub const ERROR_VOLMGR_EXTENT_ALREADY_USED: HRESULT = 0xC038001Au32 as HRESULT;

/// The specified volume is retained and can only be extended into a contiguous extent. The specified extent to grow the volume is not contiguous with the specified volume.
pub const ERROR_VOLMGR_EXTENT_NOT_CONTIGUOUS: HRESULT = 0xC038001Bu32 as HRESULT;

/// The specified volume extent is not within the public region of the disk.
pub const ERROR_VOLMGR_EXTENT_NOT_IN_PUBLIC_REGION: HRESULT = 0xC038001Cu32 as HRESULT;

/// The specified volume extent is not sector aligned.
pub const ERROR_VOLMGR_EXTENT_NOT_SECTOR_ALIGNED: HRESULT = 0xC038001Du32 as HRESULT;

/// The specified partition overlaps an EBR (the first track of an extended partition on an MBR disk).
pub const ERROR_VOLMGR_EXTENT_OVERLAPS_EBR_PARTITION: HRESULT = 0xC038001Eu32 as HRESULT;

/// The specified extent lengths cannot be used to construct a volume with specified length.
pub const ERROR_VOLMGR_EXTENT_VOLUME_LENGTHS_DO_NOT_MATCH: HRESULT = 0xC038001Fu32 as HRESULT;

/// The system does not support fault tolerant volumes.
pub const ERROR_VOLMGR_FAULT_TOLERANT_NOT_SUPPORTED: HRESULT = 0xC0380020u32 as HRESULT;

/// The specified interleave length is invalid.
pub const ERROR_VOLMGR_INTERLEAVE_LENGTH_INVALID: HRESULT = 0xC0380021u32 as HRESULT;

/// There is already a maximum number of registered users.
pub const ERROR_VOLMGR_MAXIMUM_REGISTERED_USERS: HRESULT = 0xC0380022u32 as HRESULT;

/// The specified member is already in-sync with the other active members. It does not need to be regenerated.
pub const ERROR_VOLMGR_MEMBER_IN_SYNC: HRESULT = 0xC0380023u32 as HRESULT;

/// The same member index was specified more than once.
pub const ERROR_VOLMGR_MEMBER_INDEX_DUPLICATE: HRESULT = 0xC0380024u32 as HRESULT;

/// The specified member index is greater or equal than the number of members in the volume plex.
pub const ERROR_VOLMGR_MEMBER_INDEX_INVALID: HRESULT = 0xC0380025u32 as HRESULT;

/// The specified member is missing. It cannot be regenerated.
pub const ERROR_VOLMGR_MEMBER_MISSING: HRESULT = 0xC0380026u32 as HRESULT;

/// The specified member is not detached. Cannot replace a member which is not detached.
pub const ERROR_VOLMGR_MEMBER_NOT_DETACHED: HRESULT = 0xC0380027u32 as HRESULT;

/// The specified member is already regenerating.
pub const ERROR_VOLMGR_MEMBER_REGENERATING: HRESULT = 0xC0380028u32 as HRESULT;

/// All disks belonging to the pack failed.
pub const ERROR_VOLMGR_ALL_DISKS_FAILED: HRESULT = 0xC0380029u32 as HRESULT;

/// There are currently no registered users for notifications. The task number is irrelevant unless there are registered users.
pub const ERROR_VOLMGR_NO_REGISTERED_USERS: HRESULT = 0xC038002Au32 as HRESULT;

/// The specified notification user does not exist. Failed to unregister user for notifications.
pub const ERROR_VOLMGR_NO_SUCH_USER: HRESULT = 0xC038002Bu32 as HRESULT;

/// The notifications have been reset. Notifications for the current user are invalid. Unregister and re-register for notifications.
pub const ERROR_VOLMGR_NOTIFICATION_RESET: HRESULT = 0xC038002Cu32 as HRESULT;

/// The specified number of members is invalid.
pub const ERROR_VOLMGR_NUMBER_OF_MEMBERS_INVALID: HRESULT = 0xC038002Du32 as HRESULT;

/// The specified number of plexes is invalid.
pub const ERROR_VOLMGR_NUMBER_OF_PLEXES_INVALID: HRESULT = 0xC038002Eu32 as HRESULT;

/// The specified source and target packs are identical.
pub const ERROR_VOLMGR_PACK_DUPLICATE: HRESULT = 0xC038002Fu32 as HRESULT;

/// The specified pack id is invalid. There are no packs with the specified pack id.
pub const ERROR_VOLMGR_PACK_ID_INVALID: HRESULT = 0xC0380030u32 as HRESULT;

/// The specified pack is the invalid pack. The operation cannot complete with the invalid pack.
pub const ERROR_VOLMGR_PACK_INVALID: HRESULT = 0xC0380031u32 as HRESULT;

/// The specified pack name is invalid.
pub const ERROR_VOLMGR_PACK_NAME_INVALID: HRESULT = 0xC0380032u32 as HRESULT;

/// The specified pack is offline.
pub const ERROR_VOLMGR_PACK_OFFLINE: HRESULT = 0xC0380033u32 as HRESULT;

/// The specified pack already has a quorum of healthy disks.
pub const ERROR_VOLMGR_PACK_HAS_QUORUM: HRESULT = 0xC0380034u32 as HRESULT;

/// The pack does not have a quorum of healthy disks.
pub const ERROR_VOLMGR_PACK_WITHOUT_QUORUM: HRESULT = 0xC0380035u32 as HRESULT;

/// The specified disk has an unsupported partition style. Only MBR and GPT partition styles are supported.
pub const ERROR_VOLMGR_PARTITION_STYLE_INVALID: HRESULT = 0xC0380036u32 as HRESULT;

/// Failed to update the disk's partition layout.
pub const ERROR_VOLMGR_PARTITION_UPDATE_FAILED: HRESULT = 0xC0380037u32 as HRESULT;

/// The specified plex is already in-sync with the other active plexes. It does not need to be regenerated.
pub const ERROR_VOLMGR_PLEX_IN_SYNC: HRESULT = 0xC0380038u32 as HRESULT;

/// The same plex index was specified more than once.
pub const ERROR_VOLMGR_PLEX_INDEX_DUPLICATE: HRESULT = 0xC0380039u32 as HRESULT;

/// The specified plex index is greater or equal than the number of plexes in the volume.
pub const ERROR_VOLMGR_PLEX_INDEX_INVALID: HRESULT = 0xC038003Au32 as HRESULT;

/// The specified plex is the last active plex in the volume. The plex cannot be removed or else the volume will go offline.
pub const ERROR_VOLMGR_PLEX_LAST_ACTIVE: HRESULT = 0xC038003Bu32 as HRESULT;

/// The specified plex is missing.
pub const ERROR_VOLMGR_PLEX_MISSING: HRESULT = 0xC038003Cu32 as HRESULT;

/// The specified plex is currently regenerating.
pub const ERROR_VOLMGR_PLEX_REGENERATING: HRESULT = 0xC038003Du32 as HRESULT;

/// The specified plex type is invalid.
pub const ERROR_VOLMGR_PLEX_TYPE_INVALID: HRESULT = 0xC038003Eu32 as HRESULT;

/// The operation is only supported on RAID-5 plexes.
pub const ERROR_VOLMGR_PLEX_NOT_RAID5: HRESULT = 0xC038003Fu32 as HRESULT;

/// The operation is only supported on simple plexes.
pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE: HRESULT = 0xC0380040u32 as HRESULT;

/// The Size fields in the VM_VOLUME_LAYOUT input structure are incorrectly set.
pub const ERROR_VOLMGR_STRUCTURE_SIZE_INVALID: HRESULT = 0xC0380041u32 as HRESULT;

/// There is already a pending request for notifications. Wait for the existing request to return before requesting for more notifications.
pub const ERROR_VOLMGR_TOO_MANY_NOTIFICATION_REQUESTS: HRESULT = 0xC0380042u32 as HRESULT;

/// There is currently a transaction in process.
pub const ERROR_VOLMGR_TRANSACTION_IN_PROGRESS: HRESULT = 0xC0380043u32 as HRESULT;

/// An unexpected layout change occurred outside of the volume manager.
pub const ERROR_VOLMGR_UNEXPECTED_DISK_LAYOUT_CHANGE: HRESULT = 0xC0380044u32 as HRESULT;

/// The specified volume contains a missing disk.
pub const ERROR_VOLMGR_VOLUME_CONTAINS_MISSING_DISK: HRESULT = 0xC0380045u32 as HRESULT;

/// The specified volume id is invalid. There are no volumes with the specified volume id.
pub const ERROR_VOLMGR_VOLUME_ID_INVALID: HRESULT = 0xC0380046u32 as HRESULT;

/// The specified volume length is invalid.
pub const ERROR_VOLMGR_VOLUME_LENGTH_INVALID: HRESULT = 0xC0380047u32 as HRESULT;

/// The specified size for the volume is not a multiple of the sector size.
pub const ERROR_VOLMGR_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: HRESULT = 0xC0380048u32 as HRESULT;

/// The operation is only supported on mirrored volumes.
pub const ERROR_VOLMGR_VOLUME_NOT_MIRRORED: HRESULT = 0xC0380049u32 as HRESULT;

/// The specified volume does not have a retain partition.
pub const ERROR_VOLMGR_VOLUME_NOT_RETAINED: HRESULT = 0xC038004Au32 as HRESULT;

/// The specified volume is offline.
pub const ERROR_VOLMGR_VOLUME_OFFLINE: HRESULT = 0xC038004Bu32 as HRESULT;

/// The specified volume already has a retain partition.
pub const ERROR_VOLMGR_VOLUME_RETAINED: HRESULT = 0xC038004Cu32 as HRESULT;

/// The specified number of extents is invalid.
pub const ERROR_VOLMGR_NUMBER_OF_EXTENTS_INVALID: HRESULT = 0xC038004Du32 as HRESULT;

/// All disks participating to the volume must have the same sector size.
pub const ERROR_VOLMGR_DIFFERENT_SECTOR_SIZE: HRESULT = 0xC038004Eu32 as HRESULT;

/// The boot disk experienced failures.
pub const ERROR_VOLMGR_BAD_BOOT_DISK: HRESULT = 0xC038004Fu32 as HRESULT;

/// The configuration of the pack is offline.
pub const ERROR_VOLMGR_PACK_CONFIG_OFFLINE: HRESULT = 0xC0380050u32 as HRESULT;

/// The configuration of the pack is online.
pub const ERROR_VOLMGR_PACK_CONFIG_ONLINE: HRESULT = 0xC0380051u32 as HRESULT;

/// The specified pack is not the primary pack.
pub const ERROR_VOLMGR_NOT_PRIMARY_PACK: HRESULT = 0xC0380052u32 as HRESULT;

/// All disks failed to be updated with the new content of the log.
pub const ERROR_VOLMGR_PACK_LOG_UPDATE_FAILED: HRESULT = 0xC0380053u32 as HRESULT;

/// The specified number of disks in a plex is invalid.
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_PLEX_INVALID: HRESULT = 0xC0380054u32 as HRESULT;

/// The specified number of disks in a plex member is invalid.
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_MEMBER_INVALID: HRESULT = 0xC0380055u32 as HRESULT;

/// The operation is not supported on mirrored volumes.
pub const ERROR_VOLMGR_VOLUME_MIRRORED: HRESULT = 0xC0380056u32 as HRESULT;

/// The operation is only supported on simple and spanned plexes.
pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE_SPANNED: HRESULT = 0xC0380057u32 as HRESULT;

/// The pack has no valid log copies.
pub const ERROR_VOLMGR_NO_VALID_LOG_COPIES: HRESULT = 0xC0380058u32 as HRESULT;

/// A primary pack is already present.
pub const ERROR_VOLMGR_PRIMARY_PACK_PRESENT: HRESULT = 0xC0380059u32 as HRESULT;

/// The specified number of disks is invalid.
pub const ERROR_VOLMGR_NUMBER_OF_DISKS_INVALID: HRESULT = 0xC038005Au32 as HRESULT;

/// The system does not support mirrored volumes.
pub const ERROR_VOLMGR_MIRROR_NOT_SUPPORTED: HRESULT = 0xC038005Bu32 as HRESULT;

/// The system does not support RAID-5 volumes.
pub const ERROR_VOLMGR_RAID5_NOT_SUPPORTED: HRESULT = 0xC038005Cu32 as HRESULT;

/// Some BCD entries were not imported correctly from the BCD store.
pub const ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED: HRESULT = 0x80390001u32 as HRESULT;

/// Entries enumerated have exceeded the allowed threshold.
pub const ERROR_BCD_TOO_MANY_ELEMENTS: HRESULT = 0xC0390002u32 as HRESULT;

/// Some BCD entries were not synchronized correctly with the firmware.
pub const ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED: HRESULT = 0x80390003u32 as HRESULT;

/// The virtual hard disk is corrupted. The virtual hard disk drive footer is missing.
pub const ERROR_VHD_DRIVE_FOOTER_MISSING: HRESULT = 0xC03A0001u32 as HRESULT;

/// The virtual hard disk is corrupted. The virtual hard disk drive footer checksum does not match the on-disk checksum.
pub const ERROR_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH: HRESULT = 0xC03A0002u32 as HRESULT;

/// The virtual hard disk is corrupted. The virtual hard disk drive footer in the virtual hard disk is corrupted.
pub const ERROR_VHD_DRIVE_FOOTER_CORRUPT: HRESULT = 0xC03A0003u32 as HRESULT;

/// The system does not recognize the file format of this virtual hard disk.
pub const ERROR_VHD_FORMAT_UNKNOWN: HRESULT = 0xC03A0004u32 as HRESULT;

/// The version does not support this version of the file format.
pub const ERROR_VHD_FORMAT_UNSUPPORTED_VERSION: HRESULT = 0xC03A0005u32 as HRESULT;

/// The virtual hard disk is corrupted. The sparse header checksum does not match the on-disk checksum.
pub const ERROR_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH: HRESULT = 0xC03A0006u32 as HRESULT;

/// The system does not support this version of the virtual hard disk.This version of the sparse header is not supported.
pub const ERROR_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION: HRESULT = 0xC03A0007u32 as HRESULT;

/// The virtual hard disk is corrupted. The sparse header in the virtual hard disk is corrupt.
pub const ERROR_VHD_SPARSE_HEADER_CORRUPT: HRESULT = 0xC03A0008u32 as HRESULT;

/// Failed to write to the virtual hard disk failed because the system failed to allocate a new block in the virtual hard disk.
pub const ERROR_VHD_BLOCK_ALLOCATION_FAILURE: HRESULT = 0xC03A0009u32 as HRESULT;

/// The virtual hard disk is corrupted. The block allocation table in the virtual hard disk is corrupt.
pub const ERROR_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT: HRESULT = 0xC03A000Au32 as HRESULT;

/// The system does not support this version of the virtual hard disk. The block size is invalid.
pub const ERROR_VHD_INVALID_BLOCK_SIZE: HRESULT = 0xC03A000Bu32 as HRESULT;

/// The virtual hard disk is corrupted. The block bitmap does not match with the block data present in the virtual hard disk.
pub const ERROR_VHD_BITMAP_MISMATCH: HRESULT = 0xC03A000Cu32 as HRESULT;

/// The chain of virtual hard disks is broken. The system cannot locate the parent virtual hard disk for the differencing disk.
pub const ERROR_VHD_PARENT_VHD_NOT_FOUND: HRESULT = 0xC03A000Du32 as HRESULT;

/// The chain of virtual hard disks is corrupted. There is a mismatch in the identifiers of the parent virtual hard disk and differencing disk.
pub const ERROR_VHD_CHILD_PARENT_ID_MISMATCH: HRESULT = 0xC03A000Eu32 as HRESULT;

/// The chain of virtual hard disks is corrupted. The time stamp of the parent virtual hard disk does not match the time stamp of the differencing disk.
pub const ERROR_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH: HRESULT = 0xC03A000Fu32 as HRESULT;

/// Failed to read the metadata of the virtual hard disk.
pub const ERROR_VHD_METADATA_READ_FAILURE: HRESULT = 0xC03A0010u32 as HRESULT;

/// Failed to write to the metadata of the virtual hard disk.
pub const ERROR_VHD_METADATA_WRITE_FAILURE: HRESULT = 0xC03A0011u32 as HRESULT;

/// The size of the virtual hard disk is not valid.
pub const ERROR_VHD_INVALID_SIZE: HRESULT = 0xC03A0012u32 as HRESULT;

/// The file size of this virtual hard disk is not valid.
pub const ERROR_VHD_INVALID_FILE_SIZE: HRESULT = 0xC03A0013u32 as HRESULT;

/// A virtual disk support provider for the specified file was not found.
pub const ERROR_VIRTDISK_PROVIDER_NOT_FOUND: HRESULT = 0xC03A0014u32 as HRESULT;

/// The specified disk is not a virtual disk.
pub const ERROR_VIRTDISK_NOT_VIRTUAL_DISK: HRESULT = 0xC03A0015u32 as HRESULT;

/// The chain of virtual hard disks is inaccessible. The process has not been granted access rights to the parent virtual hard disk for the differencing disk.
pub const ERROR_VHD_PARENT_VHD_ACCESS_DENIED: HRESULT = 0xC03A0016u32 as HRESULT;

/// The chain of virtual hard disks is corrupted. There is a mismatch in the virtual sizes of the parent virtual hard disk and differencing disk.
pub const ERROR_VHD_CHILD_PARENT_SIZE_MISMATCH: HRESULT = 0xC03A0017u32 as HRESULT;

/// The chain of virtual hard disks is corrupted. A differencing disk is indicated in its own parent chain.
pub const ERROR_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED: HRESULT = 0xC03A0018u32 as HRESULT;

/// The chain of virtual hard disks is inaccessible. There was an error opening a virtual hard disk further up the chain.
pub const ERROR_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT: HRESULT = 0xC03A0019u32 as HRESULT;

/// The requested operation could not be completed due to a virtual disk system limitation.  Virtual hard disk files must be uncompressed and unencrypted and must not be sparse.
pub const ERROR_VIRTUAL_DISK_LIMITATION: HRESULT = 0xC03A001Au32 as HRESULT;

/// The requested operation cannot be performed on a virtual disk of this type.
pub const ERROR_VHD_INVALID_TYPE: HRESULT = 0xC03A001Bu32 as HRESULT;

/// The requested operation cannot be performed on the virtual disk in its current state.
pub const ERROR_VHD_INVALID_STATE: HRESULT = 0xC03A001Cu32 as HRESULT;

/// The sector size of the physical disk on which the virtual disk resides is not supported.
pub const ERROR_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE: HRESULT = 0xC03A001Du32 as HRESULT;

/// The disk is already owned by a different owner.
pub const ERROR_VIRTDISK_DISK_ALREADY_OWNED: HRESULT = 0xC03A001Eu32 as HRESULT;

/// The disk must be offline or read-only.
pub const ERROR_VIRTDISK_DISK_ONLINE_AND_WRITABLE: HRESULT = 0xC03A001Fu32 as HRESULT;

/// Change Tracking is not initialized for this virtual disk.
pub const ERROR_CTLOG_TRACKING_NOT_INITIALIZED: HRESULT = 0xC03A0020u32 as HRESULT;

/// Size of change tracking file exceeded the maximum size limit.
pub const ERROR_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE: HRESULT = 0xC03A0021u32 as HRESULT;

/// VHD file is changed due to compaction, expansion, or offline updates.
pub const ERROR_CTLOG_VHD_CHANGED_OFFLINE: HRESULT = 0xC03A0022u32 as HRESULT;

/// Change Tracking for the virtual disk is not in a valid state to perform this request.  Change tracking could be discontinued or already in the requested state.
pub const ERROR_CTLOG_INVALID_TRACKING_STATE: HRESULT = 0xC03A0023u32 as HRESULT;

/// Change Tracking file for the virtual disk is not in a valid state.
pub const ERROR_CTLOG_INCONSISTENT_TRACKING_FILE: HRESULT = 0xC03A0024u32 as HRESULT;

/// The requested resize operation could not be completed because it might truncate user data residing on the virtual disk.
pub const ERROR_VHD_RESIZE_WOULD_TRUNCATE_DATA: HRESULT = 0xC03A0025u32 as HRESULT;

///  The requested operation could not be completed because the virtual disk's minimum safe size could not be determined.
///
/// This may be due to a missing or corrupt partition table.
pub const ERROR_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE: HRESULT = 0xC03A0026u32 as HRESULT;

/// The requested operation could not be completed because the virtual disk's size cannot be safely reduced further.
pub const ERROR_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE: HRESULT = 0xC03A0027u32 as HRESULT;

/// There is not enough space in the virtual disk file for the provided metadata item.
pub const ERROR_VHD_METADATA_FULL: HRESULT = 0xC03A0028u32 as HRESULT;

/// The specified change tracking identifier is not valid.
pub const ERROR_VHD_INVALID_CHANGE_TRACKING_ID: HRESULT = 0xC03A0029u32 as HRESULT;

/// Change tracking is disabled for the specified virtual hard disk, so no change tracking information is available.
pub const ERROR_VHD_CHANGE_TRACKING_DISABLED: HRESULT = 0xC03A002Au32 as HRESULT;

/// There is no change tracking data available associated with the specified change tracking identifier.
pub const ERROR_VHD_MISSING_CHANGE_TRACKING_INFORMATION: HRESULT = 0xC03A0030u32 as HRESULT;

/// The specified VHD has an unexpected virtual disk identifier.
pub const ERROR_VHD_UNEXPECTED_ID: HRESULT = 0xC03A0034u32 as HRESULT;

/// The virtualization storage subsystem has generated an error.
pub const ERROR_QUERY_STORAGE_ERROR: HRESULT = 0x803A0001u32 as HRESULT;

/// The network was not found.
pub const HCN_E_NETWORK_NOT_FOUND: HRESULT = 0x803B0001u32 as HRESULT;

/// The endpoint was not found.
pub const HCN_E_ENDPOINT_NOT_FOUND: HRESULT = 0x803B0002u32 as HRESULT;

/// The network's underlying layer was not found.
pub const HCN_E_LAYER_NOT_FOUND: HRESULT = 0x803B0003u32 as HRESULT;

/// The virtual switch was not found.
pub const HCN_E_SWITCH_NOT_FOUND: HRESULT = 0x803B0004u32 as HRESULT;

/// The network does not have a subnet for this endpoint.
pub const HCN_E_SUBNET_NOT_FOUND: HRESULT = 0x803B0005u32 as HRESULT;

/// An adapter was not found.
pub const HCN_E_ADAPTER_NOT_FOUND: HRESULT = 0x803B0006u32 as HRESULT;

/// The switch-port was not found.
pub const HCN_E_PORT_NOT_FOUND: HRESULT = 0x803B0007u32 as HRESULT;

/// An expected policy was not found.
pub const HCN_E_POLICY_NOT_FOUND: HRESULT = 0x803B0008u32 as HRESULT;

/// A required VFP port setting was not found.
pub const HCN_E_VFP_PORTSETTING_NOT_FOUND: HRESULT = 0x803B0009u32 as HRESULT;

/// The provided network configuration is invalid or missing parameters.
pub const HCN_E_INVALID_NETWORK: HRESULT = 0x803B000Au32 as HRESULT;

/// Invalid network type.
pub const HCN_E_INVALID_NETWORK_TYPE: HRESULT = 0x803B000Bu32 as HRESULT;

/// The provided endpoint configuration is invalid or missing parameters.
pub const HCN_E_INVALID_ENDPOINT: HRESULT = 0x803B000Cu32 as HRESULT;

/// The provided policy configuration is invalid or missing parameters.
pub const HCN_E_INVALID_POLICY: HRESULT = 0x803B000Du32 as HRESULT;

/// Invalid policy type.
pub const HCN_E_INVALID_POLICY_TYPE: HRESULT = 0x803B000Eu32 as HRESULT;

/// This requested operation is invalid for a remote endpoint.
pub const HCN_E_INVALID_REMOTE_ENDPOINT_OPERATION: HRESULT = 0x803B000Fu32 as HRESULT;

/// A network with this name already exists.
pub const HCN_E_NETWORK_ALREADY_EXISTS: HRESULT = 0x803B0010u32 as HRESULT;

/// A network with this name already exists.
pub const HCN_E_LAYER_ALREADY_EXISTS: HRESULT = 0x803B0011u32 as HRESULT;

/// Policy information already exists on this object.
pub const HCN_E_POLICY_ALREADY_EXISTS: HRESULT = 0x803B0012u32 as HRESULT;

/// The specified port already exists.
pub const HCN_E_PORT_ALREADY_EXISTS: HRESULT = 0x803B0013u32 as HRESULT;

/// This endpoint is already attached to the switch.
pub const HCN_E_ENDPOINT_ALREADY_ATTACHED: HRESULT = 0x803B0014u32 as HRESULT;

/// The specified request is unsupported.
pub const HCN_E_REQUEST_UNSUPPORTED: HRESULT = 0x803B0015u32 as HRESULT;

/// Port mapping is not supported on the given network.
pub const HCN_E_MAPPING_NOT_SUPPORTED: HRESULT = 0x803B0016u32 as HRESULT;

/// There was an operation attempted on a degraded object.
pub const HCN_E_DEGRADED_OPERATION: HRESULT = 0x803B0017u32 as HRESULT;

/// Cannot modify a switch shared by multiple networks.
pub const HCN_E_SHARED_SWITCH_MODIFICATION: HRESULT = 0x803B0018u32 as HRESULT;

/// Failed to interpret a parameter as a GUID.
pub const HCN_E_GUID_CONVERSION_FAILURE: HRESULT = 0x803B0019u32 as HRESULT;

/// Failed to process registry key.
pub const HCN_E_REGKEY_FAILURE: HRESULT = 0x803B001Au32 as HRESULT;

/// Invalid JSON document string.
pub const HCN_E_INVALID_JSON: HRESULT = 0x803B001Bu32 as HRESULT;

/// The reference is invalid in the JSON document.
pub const HCN_E_INVALID_JSON_REFERENCE: HRESULT = 0x803B001Cu32 as HRESULT;

/// Endpoint sharing is disabled.
pub const HCN_E_ENDPOINT_SHARING_DISABLED: HRESULT = 0x803B001Du32 as HRESULT;

/// IP address is either invalid or not part of any configured subnet(s).
pub const HCN_E_INVALID_IP: HRESULT = 0x803B001Eu32 as HRESULT;

/// The specified switch extension does not exist on this switch.
pub const HCN_E_SWITCH_EXTENSION_NOT_FOUND: HRESULT = 0x803B001Fu32 as HRESULT;

/// Operation cannot be performed while service is stopping.
pub const HCN_E_MANAGER_STOPPED: HRESULT = 0x803B0020u32 as HRESULT;

/// Operation cannot be performed while service module not found.
pub const GCN_E_MODULE_NOT_FOUND: HRESULT = 0x803B0021u32 as HRESULT;

/// Request Handlers not present to handle the JSON request.
pub const GCN_E_NO_REQUEST_HANDLERS: HRESULT = 0x803B0022u32 as HRESULT;

/// The specified request is unsupported.
pub const GCN_E_REQUEST_UNSUPPORTED: HRESULT = 0x803B0023u32 as HRESULT;

/// Add runtime keys to container failed.
pub const GCN_E_RUNTIMEKEYS_FAILED: HRESULT = 0x803B0024u32 as HRESULT;

/// Timeout while waiting for network adapter with the given instance id
pub const GCN_E_NETADAPTER_TIMEOUT: HRESULT = 0x803B0025u32 as HRESULT;

/// Network adapter not found for the given instance id
pub const GCN_E_NETADAPTER_NOT_FOUND: HRESULT = 0x803B0026u32 as HRESULT;

/// Network compartment not found for the given  id
pub const GCN_E_NETCOMPARTMENT_NOT_FOUND: HRESULT = 0x803B0027u32 as HRESULT;

/// Network interface not found for the given  id
pub const GCN_E_NETINTERFACE_NOT_FOUND: HRESULT = 0x803B0028u32 as HRESULT;

/// Default Namespace already exists
pub const GCN_E_DEFAULTNAMESPACE_EXISTS: HRESULT = 0x803B0029u32 as HRESULT;

/// Internet Connection Sharing service (SharedAccess) is disabled and cannot be started
pub const HCN_E_ICS_DISABLED: HRESULT = 0x803B002Au32 as HRESULT;

/// This requested operation is invalid as endpoint is already part of a network namespace.
pub const HCN_E_ENDPOINT_NAMESPACE_ALREADY_EXISTS: HRESULT = 0x803B002Bu32 as HRESULT;

/// The specified entity cannot be removed while it still has references.
pub const HCN_E_ENTITY_HAS_REFERENCES: HRESULT = 0x803B002Cu32 as HRESULT;

/// The internal port must exist and cannot be zero.
pub const HCN_E_INVALID_INTERNAL_PORT: HRESULT = 0x803B002Du32 as HRESULT;

/// The requested operation for attach namespace failed.
pub const HCN_E_NAMESPACE_ATTACH_FAILED: HRESULT = 0x803B002Eu32 as HRESULT;

/// An address provided is invalid or reserved.
pub const HCN_E_ADDR_INVALID_OR_RESERVED: HRESULT = 0x803B002Fu32 as HRESULT;

/// The prefix provided is invalid.
pub const HCN_E_INVALID_PREFIX: HRESULT = 0x803B0030u32 as HRESULT;

/// A call was performed against an object that was torn down.
pub const HCN_E_OBJECT_USED_AFTER_UNLOAD: HRESULT = 0x803B0031u32 as HRESULT;

/// The provided subnet configuration is invalid or missing parameters.
pub const HCN_E_INVALID_SUBNET: HRESULT = 0x803B0032u32 as HRESULT;

/// The provided IP subnet configuration is invalid or missing parameters.
pub const HCN_E_INVALID_IP_SUBNET: HRESULT = 0x803B0033u32 as HRESULT;

/// The endpoint must be attached to complete the operation.
pub const HCN_E_ENDPOINT_NOT_ATTACHED: HRESULT = 0x803B0034u32 as HRESULT;

/// The endpoint must be local to complete the operation.
pub const HCN_E_ENDPOINT_NOT_LOCAL: HRESULT = 0x803B0035u32 as HRESULT;

/// Cannot apply more than one InterfaceParameters policy.
pub const HCN_INTERFACEPARAMETERS_ALREADY_APPLIED: HRESULT = 0x803B0036u32 as HRESULT;

/// A network of this type can not be created because VFP is not available.
pub const HCN_E_VFP_NOT_ALLOWED: HRESULT = 0x803B0037u32 as HRESULT;

/// The operation was cancelled.
pub const SDIAG_E_CANCELLED: HRESULT = 0x803C0100u32 as HRESULT;

/// An error occurred when running a PowerShell script.
pub const SDIAG_E_SCRIPT: HRESULT = 0x803C0101u32 as HRESULT;

/// An error occurred when interacting with PowerShell runtime.
pub const SDIAG_E_POWERSHELL: HRESULT = 0x803C0102u32 as HRESULT;

/// An error occurred in the Scripted Diagnostic Managed Host.
pub const SDIAG_E_MANAGEDHOST: HRESULT = 0x803C0103u32 as HRESULT;

/// The troubleshooting pack does not contain a required verifier to complete the verification.
pub const SDIAG_E_NOVERIFIER: HRESULT = 0x803C0104u32 as HRESULT;

/// The troubleshooting pack cannot be executed on this system.
pub const SDIAG_S_CANNOTRUN: HRESULT = 0x003C0105;

/// Scripted diagnostics is disabled by group policy.
pub const SDIAG_E_DISABLED: HRESULT = 0x803C0106u32 as HRESULT;

/// Trust validation of the troubleshooting pack failed.
pub const SDIAG_E_TRUST: HRESULT = 0x803C0107u32 as HRESULT;

/// The troubleshooting pack cannot be executed on this system.
pub const SDIAG_E_CANNOTRUN: HRESULT = 0x803C0108u32 as HRESULT;

/// This version of the troubleshooting pack is not supported.
pub const SDIAG_E_VERSION: HRESULT = 0x803C0109u32 as HRESULT;

/// A required resource cannot be loaded.
pub const SDIAG_E_RESOURCE: HRESULT = 0x803C010Au32 as HRESULT;

/// The troubleshooting pack reported information for a root cause without adding the root cause.
pub const SDIAG_E_ROOTCAUSE: HRESULT = 0x803C010Bu32 as HRESULT;

/// The notification channel has already been closed.
pub const WPN_E_CHANNEL_CLOSED: HRESULT = 0x803E0100u32 as HRESULT;

/// The notification channel request did not complete successfully.
pub const WPN_E_CHANNEL_REQUEST_NOT_COMPLETE: HRESULT = 0x803E0101u32 as HRESULT;

/// The application identifier provided is invalid.
pub const WPN_E_INVALID_APP: HRESULT = 0x803E0102u32 as HRESULT;

/// A notification channel request for the provided application identifier is in progress.
pub const WPN_E_OUTSTANDING_CHANNEL_REQUEST: HRESULT = 0x803E0103u32 as HRESULT;

/// The channel identifier is already tied to another application endpoint.
pub const WPN_E_DUPLICATE_CHANNEL: HRESULT = 0x803E0104u32 as HRESULT;

/// The notification platform is unavailable.
pub const WPN_E_PLATFORM_UNAVAILABLE: HRESULT = 0x803E0105u32 as HRESULT;

/// The notification has already been posted.
pub const WPN_E_NOTIFICATION_POSTED: HRESULT = 0x803E0106u32 as HRESULT;

/// The notification has already been hidden.
pub const WPN_E_NOTIFICATION_HIDDEN: HRESULT = 0x803E0107u32 as HRESULT;

/// The notification cannot be hidden until it has been shown.
pub const WPN_E_NOTIFICATION_NOT_POSTED: HRESULT = 0x803E0108u32 as HRESULT;

/// Cloud notifications have been turned off.
pub const WPN_E_CLOUD_DISABLED: HRESULT = 0x803E0109u32 as HRESULT;

/// The application does not have the cloud notification capability.
pub const WPN_E_CLOUD_INCAPABLE: HRESULT = 0x803E0110u32 as HRESULT;

/// The notification platform is unable to retrieve the authentication credentials required to connect to the cloud notification service.
pub const WPN_E_CLOUD_AUTH_UNAVAILABLE: HRESULT = 0x803E011Au32 as HRESULT;

/// The notification platform is unable to connect to the cloud notification service.
pub const WPN_E_CLOUD_SERVICE_UNAVAILABLE: HRESULT = 0x803E011Bu32 as HRESULT;

/// The notification platform is unable to initialize a callback for lock screen updates.
pub const WPN_E_FAILED_LOCK_SCREEN_UPDATE_INTIALIZATION: HRESULT = 0x803E011Cu32 as HRESULT;

/// Settings prevent the notification from being delivered.
pub const WPN_E_NOTIFICATION_DISABLED: HRESULT = 0x803E0111u32 as HRESULT;

/// Application capabilities prevent the notification from being delivered.
pub const WPN_E_NOTIFICATION_INCAPABLE: HRESULT = 0x803E0112u32 as HRESULT;

/// The application does not have the internet access capability.
pub const WPN_E_INTERNET_INCAPABLE: HRESULT = 0x803E0113u32 as HRESULT;

/// Settings prevent the notification type from being delivered.
pub const WPN_E_NOTIFICATION_TYPE_DISABLED: HRESULT = 0x803E0114u32 as HRESULT;

/// The size of the notification content is too large.
pub const WPN_E_NOTIFICATION_SIZE: HRESULT = 0x803E0115u32 as HRESULT;

/// The size of the notification tag is too large.
pub const WPN_E_TAG_SIZE: HRESULT = 0x803E0116u32 as HRESULT;

/// The notification platform doesn't have appropriate privilege on resources.
pub const WPN_E_ACCESS_DENIED: HRESULT = 0x803E0117u32 as HRESULT;

/// The notification platform found application is already registered.
pub const WPN_E_DUPLICATE_REGISTRATION: HRESULT = 0x803E0118u32 as HRESULT;

/// The application background task does not have the push notification capability.
pub const WPN_E_PUSH_NOTIFICATION_INCAPABLE: HRESULT = 0x803E0119u32 as HRESULT;

/// The size of the developer id for scheduled notification is too large.
pub const WPN_E_DEV_ID_SIZE: HRESULT = 0x803E0120u32 as HRESULT;

/// The notification tag is not alphanumeric.
pub const WPN_E_TAG_ALPHANUMERIC: HRESULT = 0x803E012Au32 as HRESULT;

/// The notification platform has received invalid HTTP status code other than 2xx for polling.
pub const WPN_E_INVALID_HTTP_STATUS_CODE: HRESULT = 0x803E012Bu32 as HRESULT;

/// The notification platform has run out of presentation layer sessions.
pub const WPN_E_OUT_OF_SESSION: HRESULT = 0x803E0200u32 as HRESULT;

/// The notification platform rejects image download request due to system in power save mode.
pub const WPN_E_POWER_SAVE: HRESULT = 0x803E0201u32 as HRESULT;

/// The notification platform doesn't have the requested image in its cache.
pub const WPN_E_IMAGE_NOT_FOUND_IN_CACHE: HRESULT = 0x803E0202u32 as HRESULT;

/// The notification platform cannot complete all of requested image.
pub const WPN_E_ALL_URL_NOT_COMPLETED: HRESULT = 0x803E0203u32 as HRESULT;

/// A cloud image downloaded from the notification platform is invalid.
pub const WPN_E_INVALID_CLOUD_IMAGE: HRESULT = 0x803E0204u32 as HRESULT;

/// Notification Id provided as filter is matched with what the notification platform maintains.
pub const WPN_E_NOTIFICATION_ID_MATCHED: HRESULT = 0x803E0205u32 as HRESULT;

/// Notification callback interface is already registered.
pub const WPN_E_CALLBACK_ALREADY_REGISTERED: HRESULT = 0x803E0206u32 as HRESULT;

/// Toast Notification was dropped without being displayed to the user.
pub const WPN_E_TOAST_NOTIFICATION_DROPPED: HRESULT = 0x803E0207u32 as HRESULT;

/// The notification platform does not have the proper privileges to complete the request.
pub const WPN_E_STORAGE_LOCKED: HRESULT = 0x803E0208u32 as HRESULT;

/// The size of the notification group is too large.
pub const WPN_E_GROUP_SIZE: HRESULT = 0x803E0209u32 as HRESULT;

/// The notification group is not alphanumeric.
pub const WPN_E_GROUP_ALPHANUMERIC: HRESULT = 0x803E020Au32 as HRESULT;

/// Cloud notifications have been disabled for the application due to a policy setting.
pub const WPN_E_CLOUD_DISABLED_FOR_APP: HRESULT = 0x803E020Bu32 as HRESULT;

/// Context is not activated.
pub const E_MBN_CONTEXT_NOT_ACTIVATED: HRESULT = 0x80548201u32 as HRESULT;

/// Bad SIM is inserted.
pub const E_MBN_BAD_SIM: HRESULT = 0x80548202u32 as HRESULT;

/// Requested data class is not available.
pub const E_MBN_DATA_CLASS_NOT_AVAILABLE: HRESULT = 0x80548203u32 as HRESULT;

/// Access point name (APN) or Access string is incorrect.
pub const E_MBN_INVALID_ACCESS_STRING: HRESULT = 0x80548204u32 as HRESULT;

/// Max activated contexts have reached.
pub const E_MBN_MAX_ACTIVATED_CONTEXTS: HRESULT = 0x80548205u32 as HRESULT;

/// Device is in packet detach state.
pub const E_MBN_PACKET_SVC_DETACHED: HRESULT = 0x80548206u32 as HRESULT;

/// Provider is not visible.
pub const E_MBN_PROVIDER_NOT_VISIBLE: HRESULT = 0x80548207u32 as HRESULT;

/// Radio is powered off.
pub const E_MBN_RADIO_POWER_OFF: HRESULT = 0x80548208u32 as HRESULT;

/// MBN subscription is not activated.
pub const E_MBN_SERVICE_NOT_ACTIVATED: HRESULT = 0x80548209u32 as HRESULT;

/// SIM is not inserted.
pub const E_MBN_SIM_NOT_INSERTED: HRESULT = 0x8054820Au32 as HRESULT;

/// Voice call in progress.
pub const E_MBN_VOICE_CALL_IN_PROGRESS: HRESULT = 0x8054820Bu32 as HRESULT;

/// Visible provider cache is invalid.
pub const E_MBN_INVALID_CACHE: HRESULT = 0x8054820Cu32 as HRESULT;

/// Device is not registered.
pub const E_MBN_NOT_REGISTERED: HRESULT = 0x8054820Du32 as HRESULT;

/// Providers not found.
pub const E_MBN_PROVIDERS_NOT_FOUND: HRESULT = 0x8054820Eu32 as HRESULT;

/// Pin is not supported.
pub const E_MBN_PIN_NOT_SUPPORTED: HRESULT = 0x8054820Fu32 as HRESULT;

/// Pin is required.
pub const E_MBN_PIN_REQUIRED: HRESULT = 0x80548210u32 as HRESULT;

/// PIN is disabled.
pub const E_MBN_PIN_DISABLED: HRESULT = 0x80548211u32 as HRESULT;

/// Generic Failure.
pub const E_MBN_FAILURE: HRESULT = 0x80548212u32 as HRESULT;

/// Profile is invalid.
pub const E_MBN_INVALID_PROFILE: HRESULT = 0x80548218u32 as HRESULT;

/// Default profile exist.
pub const E_MBN_DEFAULT_PROFILE_EXIST: HRESULT = 0x80548219u32 as HRESULT;

/// SMS encoding is not supported.
pub const E_MBN_SMS_ENCODING_NOT_SUPPORTED: HRESULT = 0x80548220u32 as HRESULT;

/// SMS filter is not supported.
pub const E_MBN_SMS_FILTER_NOT_SUPPORTED: HRESULT = 0x80548221u32 as HRESULT;

/// Invalid SMS memory index is used.
pub const E_MBN_SMS_INVALID_MEMORY_INDEX: HRESULT = 0x80548222u32 as HRESULT;

/// SMS language is not supported.
pub const E_MBN_SMS_LANG_NOT_SUPPORTED: HRESULT = 0x80548223u32 as HRESULT;

/// SMS memory failure occurred.
pub const E_MBN_SMS_MEMORY_FAILURE: HRESULT = 0x80548224u32 as HRESULT;

/// SMS network timeout happened.
pub const E_MBN_SMS_NETWORK_TIMEOUT: HRESULT = 0x80548225u32 as HRESULT;

/// Unknown SMSC address is used.
pub const E_MBN_SMS_UNKNOWN_SMSC_ADDRESS: HRESULT = 0x80548226u32 as HRESULT;

/// SMS format is not supported.
pub const E_MBN_SMS_FORMAT_NOT_SUPPORTED: HRESULT = 0x80548227u32 as HRESULT;

/// SMS operation is not allowed.
pub const E_MBN_SMS_OPERATION_NOT_ALLOWED: HRESULT = 0x80548228u32 as HRESULT;

/// Device SMS memory is full.
pub const E_MBN_SMS_MEMORY_FULL: HRESULT = 0x80548229u32 as HRESULT;

/// The IPv6 protocol is not installed.
pub const PEER_E_IPV6_NOT_INSTALLED: HRESULT = 0x80630001u32 as HRESULT;

/// The component has not been initialized.
pub const PEER_E_NOT_INITIALIZED: HRESULT = 0x80630002u32 as HRESULT;

/// The required service cannot be started.
pub const PEER_E_CANNOT_START_SERVICE: HRESULT = 0x80630003u32 as HRESULT;

/// The P2P protocol is not licensed to run on this OS.
pub const PEER_E_NOT_LICENSED: HRESULT = 0x80630004u32 as HRESULT;

/// The graph handle is invalid.
pub const PEER_E_INVALID_GRAPH: HRESULT = 0x80630010u32 as HRESULT;

/// The graph database name has changed.
pub const PEER_E_DBNAME_CHANGED: HRESULT = 0x80630011u32 as HRESULT;

/// A graph with the same ID already exists.
pub const PEER_E_DUPLICATE_GRAPH: HRESULT = 0x80630012u32 as HRESULT;

/// The graph is not ready.
pub const PEER_E_GRAPH_NOT_READY: HRESULT = 0x80630013u32 as HRESULT;

/// The graph is shutting down.
pub const PEER_E_GRAPH_SHUTTING_DOWN: HRESULT = 0x80630014u32 as HRESULT;

/// The graph is still in use.
pub const PEER_E_GRAPH_IN_USE: HRESULT = 0x80630015u32 as HRESULT;

/// The graph database is corrupt.
pub const PEER_E_INVALID_DATABASE: HRESULT = 0x80630016u32 as HRESULT;

/// Too many attributes have been used.
pub const PEER_E_TOO_MANY_ATTRIBUTES: HRESULT = 0x80630017u32 as HRESULT;

/// The connection can not be found.
pub const PEER_E_CONNECTION_NOT_FOUND: HRESULT = 0x80630103u32 as HRESULT;

/// The peer attempted to connect to itself.
pub const PEER_E_CONNECT_SELF: HRESULT = 0x80630106u32 as HRESULT;

/// The peer is already listening for connections.
pub const PEER_E_ALREADY_LISTENING: HRESULT = 0x80630107u32 as HRESULT;

/// The node was not found.
pub const PEER_E_NODE_NOT_FOUND: HRESULT = 0x80630108u32 as HRESULT;

/// The Connection attempt failed.
pub const PEER_E_CONNECTION_FAILED: HRESULT = 0x80630109u32 as HRESULT;

/// The peer connection could not be authenticated.
pub const PEER_E_CONNECTION_NOT_AUTHENTICATED: HRESULT = 0x8063010Au32 as HRESULT;

/// The connection was refused.
pub const PEER_E_CONNECTION_REFUSED: HRESULT = 0x8063010Bu32 as HRESULT;

/// The peer name classifier is too long.
pub const PEER_E_CLASSIFIER_TOO_LONG: HRESULT = 0x80630201u32 as HRESULT;

/// The maximum number of identities have been created.
pub const PEER_E_TOO_MANY_IDENTITIES: HRESULT = 0x80630202u32 as HRESULT;

/// Unable to access a key.
pub const PEER_E_NO_KEY_ACCESS: HRESULT = 0x80630203u32 as HRESULT;

/// The group already exists.
pub const PEER_E_GROUPS_EXIST: HRESULT = 0x80630204u32 as HRESULT;

/// The requested record could not be found.
pub const PEER_E_RECORD_NOT_FOUND: HRESULT = 0x80630301u32 as HRESULT;

/// Access to the database was denied.
pub const PEER_E_DATABASE_ACCESSDENIED: HRESULT = 0x80630302u32 as HRESULT;

/// The Database could not be initialized.
pub const PEER_E_DBINITIALIZATION_FAILED: HRESULT = 0x80630303u32 as HRESULT;

/// The record is too big.
pub const PEER_E_MAX_RECORD_SIZE_EXCEEDED: HRESULT = 0x80630304u32 as HRESULT;

/// The database already exists.
pub const PEER_E_DATABASE_ALREADY_PRESENT: HRESULT = 0x80630305u32 as HRESULT;

/// The database could not be found.
pub const PEER_E_DATABASE_NOT_PRESENT: HRESULT = 0x80630306u32 as HRESULT;

/// The identity could not be found.
pub const PEER_E_IDENTITY_NOT_FOUND: HRESULT = 0x80630401u32 as HRESULT;

/// The event handle could not be found.
pub const PEER_E_EVENT_HANDLE_NOT_FOUND: HRESULT = 0x80630501u32 as HRESULT;

/// Invalid search.
pub const PEER_E_INVALID_SEARCH: HRESULT = 0x80630601u32 as HRESULT;

/// The search attributes are invalid.
pub const PEER_E_INVALID_ATTRIBUTES: HRESULT = 0x80630602u32 as HRESULT;

/// The invitation is not trusted.
pub const PEER_E_INVITATION_NOT_TRUSTED: HRESULT = 0x80630701u32 as HRESULT;

/// The certchain is too long.
pub const PEER_E_CHAIN_TOO_LONG: HRESULT = 0x80630703u32 as HRESULT;

/// The time period is invalid.
pub const PEER_E_INVALID_TIME_PERIOD: HRESULT = 0x80630705u32 as HRESULT;

/// A circular cert chain was detected.
pub const PEER_E_CIRCULAR_CHAIN_DETECTED: HRESULT = 0x80630706u32 as HRESULT;

/// The certstore is corrupted.
pub const PEER_E_CERT_STORE_CORRUPTED: HRESULT = 0x80630801u32 as HRESULT;

/// The specified PNRP cloud does not exist.
pub const PEER_E_NO_CLOUD: HRESULT = 0x80631001u32 as HRESULT;

/// The cloud name is ambiguous.
pub const PEER_E_CLOUD_NAME_AMBIGUOUS: HRESULT = 0x80631005u32 as HRESULT;

/// The record is invalid.
pub const PEER_E_INVALID_RECORD: HRESULT = 0x80632010u32 as HRESULT;

/// Not authorized.
pub const PEER_E_NOT_AUTHORIZED: HRESULT = 0x80632020u32 as HRESULT;

/// The password does not meet policy requirements.
pub const PEER_E_PASSWORD_DOES_NOT_MEET_POLICY: HRESULT = 0x80632021u32 as HRESULT;

/// The record validation has been deferred.
pub const PEER_E_DEFERRED_VALIDATION: HRESULT = 0x80632030u32 as HRESULT;

/// The group properties are invalid.
pub const PEER_E_INVALID_GROUP_PROPERTIES: HRESULT = 0x80632040u32 as HRESULT;

/// The peername is invalid.
pub const PEER_E_INVALID_PEER_NAME: HRESULT = 0x80632050u32 as HRESULT;

/// The classifier is invalid.
pub const PEER_E_INVALID_CLASSIFIER: HRESULT = 0x80632060u32 as HRESULT;

/// The friendly name is invalid.
pub const PEER_E_INVALID_FRIENDLY_NAME: HRESULT = 0x80632070u32 as HRESULT;

/// Invalid role property.
pub const PEER_E_INVALID_ROLE_PROPERTY: HRESULT = 0x80632071u32 as HRESULT;

/// Invalid classifier property.
pub const PEER_E_INVALID_CLASSIFIER_PROPERTY: HRESULT = 0x80632072u32 as HRESULT;

/// Invalid record expiration.
pub const PEER_E_INVALID_RECORD_EXPIRATION: HRESULT = 0x80632080u32 as HRESULT;

/// Invalid credential info.
pub const PEER_E_INVALID_CREDENTIAL_INFO: HRESULT = 0x80632081u32 as HRESULT;

/// Invalid credential.
pub const PEER_E_INVALID_CREDENTIAL: HRESULT = 0x80632082u32 as HRESULT;

/// Invalid record size.
pub const PEER_E_INVALID_RECORD_SIZE: HRESULT = 0x80632083u32 as HRESULT;

/// Unsupported version.
pub const PEER_E_UNSUPPORTED_VERSION: HRESULT = 0x80632090u32 as HRESULT;

/// The group is not ready.
pub const PEER_E_GROUP_NOT_READY: HRESULT = 0x80632091u32 as HRESULT;

/// The group is still in use.
pub const PEER_E_GROUP_IN_USE: HRESULT = 0x80632092u32 as HRESULT;

/// The group is invalid.
pub const PEER_E_INVALID_GROUP: HRESULT = 0x80632093u32 as HRESULT;

/// No members were found.
pub const PEER_E_NO_MEMBERS_FOUND: HRESULT = 0x80632094u32 as HRESULT;

/// There are no member connections.
pub const PEER_E_NO_MEMBER_CONNECTIONS: HRESULT = 0x80632095u32 as HRESULT;

/// Unable to listen.
pub const PEER_E_UNABLE_TO_LISTEN: HRESULT = 0x80632096u32 as HRESULT;

/// The identity does not exist.
pub const PEER_E_IDENTITY_DELETED: HRESULT = 0x806320A0u32 as HRESULT;

/// The service is not available.
pub const PEER_E_SERVICE_NOT_AVAILABLE: HRESULT = 0x806320A1u32 as HRESULT;

/// THe contact could not be found.
pub const PEER_E_CONTACT_NOT_FOUND: HRESULT = 0x80636001u32 as HRESULT;

/// The graph data was created.
pub const PEER_S_GRAPH_DATA_CREATED: HRESULT = 0x00630001;

/// There is not more event data.
pub const PEER_S_NO_EVENT_DATA: HRESULT = 0x00630002;

/// The graph is already connect.
pub const PEER_S_ALREADY_CONNECTED: HRESULT = 0x00632000;

/// The subscription already exists.
pub const PEER_S_SUBSCRIPTION_EXISTS: HRESULT = 0x00636000;

/// No connectivity.
pub const PEER_S_NO_CONNECTIVITY: HRESULT = 0x00630005;

/// Already a member.
pub const PEER_S_ALREADY_A_MEMBER: HRESULT = 0x00630006;

/// The peername could not be converted to a DNS pnrp name.
pub const PEER_E_CANNOT_CONVERT_PEER_NAME: HRESULT = 0x80634001u32 as HRESULT;

/// Invalid peer host name.
pub const PEER_E_INVALID_PEER_HOST_NAME: HRESULT = 0x80634002u32 as HRESULT;

/// No more data could be found.
pub const PEER_E_NO_MORE: HRESULT = 0x80634003u32 as HRESULT;

/// The existing peer name is already registered.
pub const PEER_E_PNRP_DUPLICATE_PEER_NAME: HRESULT = 0x80634005u32 as HRESULT;

/// The app invite request was cancelled by the user.
pub const PEER_E_INVITE_CANCELLED: HRESULT = 0x80637000u32 as HRESULT;

/// No response of the invite was received.
pub const PEER_E_INVITE_RESPONSE_NOT_AVAILABLE: HRESULT = 0x80637001u32 as HRESULT;

/// User is not signed into serverless presence.
pub const PEER_E_NOT_SIGNED_IN: HRESULT = 0x80637003u32 as HRESULT;

/// The user declined the privacy policy prompt.
pub const PEER_E_PRIVACY_DECLINED: HRESULT = 0x80637004u32 as HRESULT;

/// A timeout occurred.
pub const PEER_E_TIMEOUT: HRESULT = 0x80637005u32 as HRESULT;

/// The address is invalid.
pub const PEER_E_INVALID_ADDRESS: HRESULT = 0x80637007u32 as HRESULT;

/// A required firewall exception is disabled.
pub const PEER_E_FW_EXCEPTION_DISABLED: HRESULT = 0x80637008u32 as HRESULT;

/// The service is blocked by a firewall policy.
pub const PEER_E_FW_BLOCKED_BY_POLICY: HRESULT = 0x80637009u32 as HRESULT;

/// Firewall exceptions are disabled.
pub const PEER_E_FW_BLOCKED_BY_SHIELDS_UP: HRESULT = 0x8063700Au32 as HRESULT;

/// The user declined to enable the firewall exceptions.
pub const PEER_E_FW_DECLINED: HRESULT = 0x8063700Bu32 as HRESULT;

/// The object could not be created.
pub const UI_E_CREATE_FAILED: HRESULT = 0x802A0001u32 as HRESULT;

/// Shutdown was already called on this object or the object that owns it.
pub const UI_E_SHUTDOWN_CALLED: HRESULT = 0x802A0002u32 as HRESULT;

/// This method cannot be called during this type of callback.
pub const UI_E_ILLEGAL_REENTRANCY: HRESULT = 0x802A0003u32 as HRESULT;

/// This object has been sealed, so this change is no longer allowed.
pub const UI_E_OBJECT_SEALED: HRESULT = 0x802A0004u32 as HRESULT;

/// The requested value was never set.
pub const UI_E_VALUE_NOT_SET: HRESULT = 0x802A0005u32 as HRESULT;

/// The requested value cannot be determined.
pub const UI_E_VALUE_NOT_DETERMINED: HRESULT = 0x802A0006u32 as HRESULT;

/// A callback returned an invalid output parameter.
pub const UI_E_INVALID_OUTPUT: HRESULT = 0x802A0007u32 as HRESULT;

/// A callback returned a success code other than S_OK or S_FALSE.
pub const UI_E_BOOLEAN_EXPECTED: HRESULT = 0x802A0008u32 as HRESULT;

/// A parameter that should be owned by this object is owned by a different object.
pub const UI_E_DIFFERENT_OWNER: HRESULT = 0x802A0009u32 as HRESULT;

/// More than one item matched the search criteria.
pub const UI_E_AMBIGUOUS_MATCH: HRESULT = 0x802A000Au32 as HRESULT;

/// A floating-point overflow occurred.
pub const UI_E_FP_OVERFLOW: HRESULT = 0x802A000Bu32 as HRESULT;

/// This method can only be called from the thread that created the object.
pub const UI_E_WRONG_THREAD: HRESULT = 0x802A000Cu32 as HRESULT;

/// The storyboard is currently in the schedule.
pub const UI_E_STORYBOARD_ACTIVE: HRESULT = 0x802A0101u32 as HRESULT;

/// The storyboard is not playing.
pub const UI_E_STORYBOARD_NOT_PLAYING: HRESULT = 0x802A0102u32 as HRESULT;

/// The start keyframe might occur after the end keyframe.
pub const UI_E_START_KEYFRAME_AFTER_END: HRESULT = 0x802A0103u32 as HRESULT;

/// It might not be possible to determine the end keyframe time when the start keyframe is reached.
pub const UI_E_END_KEYFRAME_NOT_DETERMINED: HRESULT = 0x802A0104u32 as HRESULT;

/// Two repeated portions of a storyboard might overlap.
pub const UI_E_LOOPS_OVERLAP: HRESULT = 0x802A0105u32 as HRESULT;

/// The transition has already been added to a storyboard.
pub const UI_E_TRANSITION_ALREADY_USED: HRESULT = 0x802A0106u32 as HRESULT;

/// The transition has not been added to a storyboard.
pub const UI_E_TRANSITION_NOT_IN_STORYBOARD: HRESULT = 0x802A0107u32 as HRESULT;

/// The transition might eclipse the beginning of another transition in the storyboard.
pub const UI_E_TRANSITION_ECLIPSED: HRESULT = 0x802A0108u32 as HRESULT;

/// The given time is earlier than the time passed to the last update.
pub const UI_E_TIME_BEFORE_LAST_UPDATE: HRESULT = 0x802A0109u32 as HRESULT;

/// This client is already connected to a timer.
pub const UI_E_TIMER_CLIENT_ALREADY_CONNECTED: HRESULT = 0x802A010Au32 as HRESULT;

/// The passed dimension is invalid or does not match the object's dimension.
pub const UI_E_INVALID_DIMENSION: HRESULT = 0x802A010Bu32 as HRESULT;

/// The added primitive begins at or beyond the duration of the interpolator.
pub const UI_E_PRIMITIVE_OUT_OF_BOUNDS: HRESULT = 0x802A010Cu32 as HRESULT;

/// The operation cannot be completed because the window is being closed.
pub const UI_E_WINDOW_CLOSED: HRESULT = 0x802A0201u32 as HRESULT;

/// The attribute handle given was not valid on this server.
pub const E_BLUETOOTH_ATT_INVALID_HANDLE: HRESULT = 0x80650001u32 as HRESULT;

/// The attribute cannot be read.
pub const E_BLUETOOTH_ATT_READ_NOT_PERMITTED: HRESULT = 0x80650002u32 as HRESULT;

/// The attribute cannot be written.
pub const E_BLUETOOTH_ATT_WRITE_NOT_PERMITTED: HRESULT = 0x80650003u32 as HRESULT;

/// The attribute PDU was invalid.
pub const E_BLUETOOTH_ATT_INVALID_PDU: HRESULT = 0x80650004u32 as HRESULT;

/// The attribute requires authentication before it can be read or written.
pub const E_BLUETOOTH_ATT_INSUFFICIENT_AUTHENTICATION: HRESULT = 0x80650005u32 as HRESULT;

/// Attribute server does not support the request received from the client.
pub const E_BLUETOOTH_ATT_REQUEST_NOT_SUPPORTED: HRESULT = 0x80650006u32 as HRESULT;

/// Offset specified was past the end of the attribute.
pub const E_BLUETOOTH_ATT_INVALID_OFFSET: HRESULT = 0x80650007u32 as HRESULT;

/// The attribute requires authorization before it can be read or written.
pub const E_BLUETOOTH_ATT_INSUFFICIENT_AUTHORIZATION: HRESULT = 0x80650008u32 as HRESULT;

/// Too many prepare writes have been queued.
pub const E_BLUETOOTH_ATT_PREPARE_QUEUE_FULL: HRESULT = 0x80650009u32 as HRESULT;

/// No attribute found within the given attribute handle range.
pub const E_BLUETOOTH_ATT_ATTRIBUTE_NOT_FOUND: HRESULT = 0x8065000Au32 as HRESULT;

/// The attribute cannot be read or written using the Read Blob Request.
pub const E_BLUETOOTH_ATT_ATTRIBUTE_NOT_LONG: HRESULT = 0x8065000Bu32 as HRESULT;

/// The Encryption Key Size used for encrypting this link is insufficient.
pub const E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE: HRESULT = 0x8065000Cu32 as HRESULT;

/// The attribute value length is invalid for the operation.
pub const E_BLUETOOTH_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH: HRESULT = 0x8065000Du32 as HRESULT;

/// The attribute request that was requested has encountered an error that was unlikely, and therefore could not be completed as requested.
pub const E_BLUETOOTH_ATT_UNLIKELY: HRESULT = 0x8065000Eu32 as HRESULT;

/// The attribute requires encryption before it can be read or written.
pub const E_BLUETOOTH_ATT_INSUFFICIENT_ENCRYPTION: HRESULT = 0x8065000Fu32 as HRESULT;

/// The attribute type is not a supported grouping attribute as defined by a higher layer specification.
pub const E_BLUETOOTH_ATT_UNSUPPORTED_GROUP_TYPE: HRESULT = 0x80650010u32 as HRESULT;

/// Insufficient Resources to complete the request.
pub const E_BLUETOOTH_ATT_INSUFFICIENT_RESOURCES: HRESULT = 0x80650011u32 as HRESULT;

/// An error that lies in the reserved range has been received.
pub const E_BLUETOOTH_ATT_UNKNOWN_ERROR: HRESULT = 0x80651000u32 as HRESULT;

/// PortCls could not find an audio engine node exposed by a miniport driver claiming support for IMiniportAudioEngineNode.
pub const E_AUDIO_ENGINE_NODE_NOT_FOUND: HRESULT = 0x80660001u32 as HRESULT;

/// HD Audio widget encountered an unexpected empty connection list.
pub const E_HDAUDIO_EMPTY_CONNECTION_LIST: HRESULT = 0x80660002u32 as HRESULT;

/// HD Audio widget does not support the connection list parameter.
pub const E_HDAUDIO_CONNECTION_LIST_NOT_SUPPORTED: HRESULT = 0x80660003u32 as HRESULT;

/// No HD Audio subdevices were successfully created.
pub const E_HDAUDIO_NO_LOGICAL_DEVICES_CREATED: HRESULT = 0x80660004u32 as HRESULT;

/// An unexpected NULL pointer was encountered in a linked list.
pub const E_HDAUDIO_NULL_LINKED_LIST_ENTRY: HRESULT = 0x80660005u32 as HRESULT;

/// Optimistic locking failure. Data cannot be updated if it has changed since it was read.
pub const STATEREPOSITORY_E_CONCURRENCY_LOCKING_FAILURE: HRESULT = 0x80670001u32 as HRESULT;

/// A prepared statement has been stepped at least once but not run to completion or reset. This may result in busy waits.
pub const STATEREPOSITORY_E_STATEMENT_INPROGRESS: HRESULT = 0x80670002u32 as HRESULT;

/// The StateRepository configuration is not valid.
pub const STATEREPOSITORY_E_CONFIGURATION_INVALID: HRESULT = 0x80670003u32 as HRESULT;

/// The StateRepository schema version is not known.
pub const STATEREPOSITORY_E_UNKNOWN_SCHEMA_VERSION: HRESULT = 0x80670004u32 as HRESULT;

/// A StateRepository dictionary is not valid.
pub const STATEREPOSITORY_ERROR_DICTIONARY_CORRUPTED: HRESULT = 0x80670005u32 as HRESULT;

/// The request failed because the StateRepository is actively blocking requests.
pub const STATEREPOSITORY_E_BLOCKED: HRESULT = 0x80670006u32 as HRESULT;

/// The database file is locked. The request will be retried.
pub const STATEREPOSITORY_E_BUSY_RETRY: HRESULT = 0x80670007u32 as HRESULT;

/// The database file is locked because another process is busy recovering the database. The request will be retried.
pub const STATEREPOSITORY_E_BUSY_RECOVERY_RETRY: HRESULT = 0x80670008u32 as HRESULT;

/// A table in the database is locked. The request will be retried.
pub const STATEREPOSITORY_E_LOCKED_RETRY: HRESULT = 0x80670009u32 as HRESULT;

/// The shared cache for the database is locked by another connection. The request will be retried.
pub const STATEREPOSITORY_E_LOCKED_SHAREDCACHE_RETRY: HRESULT = 0x8067000Au32 as HRESULT;

/// A transaction is required to perform the request operation.
pub const STATEREPOSITORY_E_TRANSACTION_REQUIRED: HRESULT = 0x8067000Bu32 as HRESULT;

/// The database file is locked. The request has exceeded the allowed threshold.
pub const STATEREPOSITORY_E_BUSY_TIMEOUT_EXCEEDED: HRESULT = 0x8067000Cu32 as HRESULT;

/// The database file is locked because another process is busy recovering the database. The request has exceeded the allowed threshold.
pub const STATEREPOSITORY_E_BUSY_RECOVERY_TIMEOUT_EXCEEDED: HRESULT = 0x8067000Du32 as HRESULT;

/// A table in the database is locked. The request has exceeded the allowed threshold.
pub const STATEREPOSITORY_E_LOCKED_TIMEOUT_EXCEEDED: HRESULT = 0x8067000Eu32 as HRESULT;

/// The shared cache for the database is locked by another connection. The request has exceeded the allowed threshold.
pub const STATEREPOSITORY_E_LOCKED_SHAREDCACHE_TIMEOUT_EXCEEDED: HRESULT = 0x8067000Fu32 as HRESULT;

/// The StateRepository service Stop event is in progress.
pub const STATEREPOSITORY_E_SERVICE_STOP_IN_PROGRESS: HRESULT = 0x80670010u32 as HRESULT;

/// Nested transactions are not supported.
pub const STATEREPOSTORY_E_NESTED_TRANSACTION_NOT_SUPPORTED: HRESULT = 0x80670011u32 as HRESULT;

/// The StateRepository cache is not valid.
pub const STATEREPOSITORY_ERROR_CACHE_CORRUPTED: HRESULT = 0x80670012u32 as HRESULT;

/// The transaction caller id has changed.
pub const STATEREPOSITORY_TRANSACTION_CALLER_ID_CHANGED: HRESULT = 0x00670013;

/// A transaction is in progress for the database connection.
pub const STATEREPOSITORY_TRANSACTION_IN_PROGRESS: HRESULT = 0x80670014u32 as HRESULT;

/// The StateRepository cache is not initialized.
pub const STATEREPOSITORY_E_CACHE_NOT_INIITALIZED: HRESULT = 0x80670015u32 as HRESULT;

/// Package dependency criteria could not be resolved.
pub const STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED: HRESULT = 0x80670016u32 as HRESULT;

/// The storage pool was deleted by the driver. The object cache should be updated.
pub const ERROR_SPACES_POOL_WAS_DELETED: HRESULT = 0x00E70001;

/// The specified fault domain type or combination of minimum / maximum fault domain type is not valid.
pub const ERROR_SPACES_FAULT_DOMAIN_TYPE_INVALID: HRESULT = 0x80E70001u32 as HRESULT;

/// A Storage Spaces internal error occurred.
pub const ERROR_SPACES_INTERNAL_ERROR: HRESULT = 0x80E70002u32 as HRESULT;

/// The specified resiliency type is not valid.
pub const ERROR_SPACES_RESILIENCY_TYPE_INVALID: HRESULT = 0x80E70003u32 as HRESULT;

/// The physical disk's sector size is not supported by the storage pool.
pub const ERROR_SPACES_DRIVE_SECTOR_SIZE_INVALID: HRESULT = 0x80E70004u32 as HRESULT;

/// The requested redundancy is outside of the supported range of values.
pub const ERROR_SPACES_DRIVE_REDUNDANCY_INVALID: HRESULT = 0x80E70006u32 as HRESULT;

/// The number of data copies requested is outside of the supported range of values.
pub const ERROR_SPACES_NUMBER_OF_DATA_COPIES_INVALID: HRESULT = 0x80E70007u32 as HRESULT;

/// The value for ParityLayout is outside of the supported range of values.
pub const ERROR_SPACES_PARITY_LAYOUT_INVALID: HRESULT = 0x80E70008u32 as HRESULT;

/// The value for interleave length is outside of the supported range of values or is not a power of 2.
pub const ERROR_SPACES_INTERLEAVE_LENGTH_INVALID: HRESULT = 0x80E70009u32 as HRESULT;

/// The number of columns specified is outside of the supported range of values.
pub const ERROR_SPACES_NUMBER_OF_COLUMNS_INVALID: HRESULT = 0x80E7000Au32 as HRESULT;

/// There were not enough physical disks to complete the requested operation.
pub const ERROR_SPACES_NOT_ENOUGH_DRIVES: HRESULT = 0x80E7000Bu32 as HRESULT;

/// Extended error information is available.
pub const ERROR_SPACES_EXTENDED_ERROR: HRESULT = 0x80E7000Cu32 as HRESULT;

/// The specified provisioning type is not valid.
pub const ERROR_SPACES_PROVISIONING_TYPE_INVALID: HRESULT = 0x80E7000Du32 as HRESULT;

/// The allocation size is outside of the supported range of values.
pub const ERROR_SPACES_ALLOCATION_SIZE_INVALID: HRESULT = 0x80E7000Eu32 as HRESULT;

/// Enclosure awareness is not supported for this virtual disk.
pub const ERROR_SPACES_ENCLOSURE_AWARE_INVALID: HRESULT = 0x80E7000Fu32 as HRESULT;

/// The write cache size is outside of the supported range of values.
pub const ERROR_SPACES_WRITE_CACHE_SIZE_INVALID: HRESULT = 0x80E70010u32 as HRESULT;

/// The value for number of groups is outside of the supported range of values.
pub const ERROR_SPACES_NUMBER_OF_GROUPS_INVALID: HRESULT = 0x80E70011u32 as HRESULT;

/// The OperationalState of the physical disk is invalid for this operation.
pub const ERROR_SPACES_DRIVE_OPERATIONAL_STATE_INVALID: HRESULT = 0x80E70012u32 as HRESULT;

/// The specified log entry is not complete.
pub const ERROR_SPACES_ENTRY_INCOMPLETE: HRESULT = 0x80E70013u32 as HRESULT;

/// The specified log entry is not valid.
pub const ERROR_SPACES_ENTRY_INVALID: HRESULT = 0x80E70014u32 as HRESULT;

/// A column's state needs to be updated.
pub const ERROR_SPACES_UPDATE_COLUMN_STATE: HRESULT = 0x80E70015u32 as HRESULT;

/// An extent needs to be allocated.
pub const ERROR_SPACES_MAP_REQUIRED: HRESULT = 0x80E70016u32 as HRESULT;

/// The metadata version is unsupported.
pub const ERROR_SPACES_UNSUPPORTED_VERSION: HRESULT = 0x80E70017u32 as HRESULT;

/// The metadata read was corrupt.
pub const ERROR_SPACES_CORRUPT_METADATA: HRESULT = 0x80E70018u32 as HRESULT;

/// The DRT is full.
pub const ERROR_SPACES_DRT_FULL: HRESULT = 0x80E70019u32 as HRESULT;

/// An inconsistency was found.
pub const ERROR_SPACES_INCONSISTENCY: HRESULT = 0x80E7001Au32 as HRESULT;

/// The log is not ready.
pub const ERROR_SPACES_LOG_NOT_READY: HRESULT = 0x80E7001Bu32 as HRESULT;

/// No good copy of data was available.
pub const ERROR_SPACES_NO_REDUNDANCY: HRESULT = 0x80E7001Cu32 as HRESULT;

/// The drive is not ready.
pub const ERROR_SPACES_DRIVE_NOT_READY: HRESULT = 0x80E7001Du32 as HRESULT;

/// The data on this drive is stale.
pub const ERROR_SPACES_DRIVE_SPLIT: HRESULT = 0x80E7001Eu32 as HRESULT;

/// The data on this drive has been lost.
pub const ERROR_SPACES_DRIVE_LOST_DATA: HRESULT = 0x80E7001Fu32 as HRESULT;

/// A slab needs to be marked dirty.
pub const ERROR_SPACES_MARK_DIRTY: HRESULT = 0x80E70020u32 as HRESULT;

/// The cache metadata needs to be written and flushed.
pub const ERROR_SPACES_FLUSH_METADATA: HRESULT = 0x80E70025u32 as HRESULT;

/// The cache is full.
pub const ERROR_SPACES_CACHE_FULL: HRESULT = 0x80E70026u32 as HRESULT;

/// Repair is in progress.
pub const ERROR_SPACES_REPAIR_IN_PROGRESS: HRESULT = 0x80E70027u32 as HRESULT;

/// The bootfile is too small to support persistent snapshots.
pub const ERROR_VOLSNAP_BOOTFILE_NOT_VALID: HRESULT = 0x80820001u32 as HRESULT;

/// Activation of persistent snapshots on this volume took longer than was allowed.
pub const ERROR_VOLSNAP_ACTIVATION_TIMEOUT: HRESULT = 0x80820002u32 as HRESULT;

/// BypassIO cannot be enabled while a volume snapshot exists.
pub const ERROR_VOLSNAP_NO_BYPASSIO_WITH_SNAPSHOT: HRESULT = 0x80820003u32 as HRESULT;

/// The specified volume does not support storage tiers.
pub const ERROR_TIERING_NOT_SUPPORTED_ON_VOLUME: HRESULT = 0x80830001u32 as HRESULT;

/// The Storage Tiers Management service detected that the specified volume is in the process of being dismounted.
pub const ERROR_TIERING_VOLUME_DISMOUNT_IN_PROGRESS: HRESULT = 0x80830002u32 as HRESULT;

/// The specified storage tier could not be found on the volume. Confirm that the storage tier name is valid.
pub const ERROR_TIERING_STORAGE_TIER_NOT_FOUND: HRESULT = 0x80830003u32 as HRESULT;

/// The file identifier specified is not valid on the volume.
pub const ERROR_TIERING_INVALID_FILE_ID: HRESULT = 0x80830004u32 as HRESULT;

/// Storage tier operations must be called on the clustering node that owns the metadata volume.
pub const ERROR_TIERING_WRONG_CLUSTER_NODE: HRESULT = 0x80830005u32 as HRESULT;

/// The Storage Tiers Management service is already optimizing the storage tiers on the specified volume.
pub const ERROR_TIERING_ALREADY_PROCESSING: HRESULT = 0x80830006u32 as HRESULT;

/// The requested object type cannot be assigned to a storage tier.
pub const ERROR_TIERING_CANNOT_PIN_OBJECT: HRESULT = 0x80830007u32 as HRESULT;

/// The requested file is not pinned to a tier.
pub const ERROR_TIERING_FILE_IS_NOT_PINNED: HRESULT = 0x80830008u32 as HRESULT;

/// The volume is not a tiered volume.
pub const ERROR_NOT_A_TIERED_VOLUME: HRESULT = 0x80830009u32 as HRESULT;

/// The requested attribute is not present on the specified file or directory.
pub const ERROR_ATTRIBUTE_NOT_PRESENT: HRESULT = 0x8083000Au32 as HRESULT;

/// The command was not recognized by the security core
pub const ERROR_SECCORE_INVALID_COMMAND: HRESULT = 0xC0E80000u32 as HRESULT;

/// No applicable app licenses found.
pub const ERROR_NO_APPLICABLE_APP_LICENSES_FOUND: HRESULT = 0xC0EA0001u32 as HRESULT;

/// CLiP license not found.
pub const ERROR_CLIP_LICENSE_NOT_FOUND: HRESULT = 0xC0EA0002u32 as HRESULT;

/// CLiP device license not found.
pub const ERROR_CLIP_DEVICE_LICENSE_MISSING: HRESULT = 0xC0EA0003u32 as HRESULT;

/// CLiP license has an invalid signature.
pub const ERROR_CLIP_LICENSE_INVALID_SIGNATURE: HRESULT = 0xC0EA0004u32 as HRESULT;

/// CLiP keyholder license is invalid or missing.
pub const ERROR_CLIP_KEYHOLDER_LICENSE_MISSING_OR_INVALID: HRESULT = 0xC0EA0005u32 as HRESULT;

/// CLiP license has expired.
pub const ERROR_CLIP_LICENSE_EXPIRED: HRESULT = 0xC0EA0006u32 as HRESULT;

/// CLiP license is signed by an unknown source.
pub const ERROR_CLIP_LICENSE_SIGNED_BY_UNKNOWN_SOURCE: HRESULT = 0xC0EA0007u32 as HRESULT;

/// CLiP license is not signed.
pub const ERROR_CLIP_LICENSE_NOT_SIGNED: HRESULT = 0xC0EA0008u32 as HRESULT;

/// CLiP license hardware ID is out of tolerance.
pub const ERROR_CLIP_LICENSE_HARDWARE_ID_OUT_OF_TOLERANCE: HRESULT = 0xC0EA0009u32 as HRESULT;

/// CLiP license device ID does not match the device ID in the bound device license.
pub const ERROR_CLIP_LICENSE_DEVICE_ID_MISMATCH: HRESULT = 0xC0EA000Au32 as HRESULT;

/// The Present operation was invisible to the user.
pub const DXGI_STATUS_OCCLUDED: HRESULT = 0x087A0001;

/// The Present operation was partially invisible to the user.
pub const DXGI_STATUS_CLIPPED: HRESULT = 0x087A0002;

/// The driver is requesting that the DXGI runtime not use shared resources to communicate with the Desktop Window Manager.
pub const DXGI_STATUS_NO_REDIRECTION: HRESULT = 0x087A0004;

/// The Present operation was not visible because the Windows session has switched to another desktop (for example, ctrl-alt-del).
pub const DXGI_STATUS_NO_DESKTOP_ACCESS: HRESULT = 0x087A0005;

/// The Present operation was not visible because the target monitor was being used for some other purpose.
pub const DXGI_STATUS_GRAPHICS_VIDPN_SOURCE_IN_USE: HRESULT = 0x087A0006;

/// The Present operation was not visible because the display mode changed. DXGI will have re-attempted the presentation.
pub const DXGI_STATUS_MODE_CHANGED: HRESULT = 0x087A0007;

/// The Present operation was not visible because another Direct3D device was attempting to take fullscreen mode at the time.
pub const DXGI_STATUS_MODE_CHANGE_IN_PROGRESS: HRESULT = 0x087A0008;

///  The application made a call that is invalid. Either the parameters of the call or the state of some object was incorrect.
///
/// Enable the D3D debug layer in order to see details via debug messages.
pub const DXGI_ERROR_INVALID_CALL: HRESULT = 0x887A0001u32 as HRESULT;

/// The object was not found. If calling IDXGIFactory::EnumAdaptes, there is no adapter with the specified ordinal.
pub const DXGI_ERROR_NOT_FOUND: HRESULT = 0x887A0002u32 as HRESULT;

/// The caller did not supply a sufficiently large buffer.
pub const DXGI_ERROR_MORE_DATA: HRESULT = 0x887A0003u32 as HRESULT;

/// The specified device interface or feature level is not supported on this system.
pub const DXGI_ERROR_UNSUPPORTED: HRESULT = 0x887A0004u32 as HRESULT;

/// The GPU device instance has been suspended. Use GetDeviceRemovedReason to determine the appropriate action.
pub const DXGI_ERROR_DEVICE_REMOVED: HRESULT = 0x887A0005u32 as HRESULT;

/// The GPU will not respond to more commands, most likely because of an invalid command passed by the calling application.
pub const DXGI_ERROR_DEVICE_HUNG: HRESULT = 0x887A0006u32 as HRESULT;

///  The GPU will not respond to more commands, most likely because some other application submitted invalid commands.
///
/// The calling application should re-create the device and continue.
pub const DXGI_ERROR_DEVICE_RESET: HRESULT = 0x887A0007u32 as HRESULT;

/// The GPU was busy at the moment when the call was made, and the call was neither executed nor scheduled.
pub const DXGI_ERROR_WAS_STILL_DRAWING: HRESULT = 0x887A000Au32 as HRESULT;

///  An event (such as power cycle) interrupted the gathering of presentation statistics. Any previous statistics should be
///
/// considered invalid.
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: HRESULT = 0x887A000Bu32 as HRESULT;

/// Fullscreen mode could not be achieved because the specified output was already in use.
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: HRESULT = 0x887A000Cu32 as HRESULT;

///  An internal issue prevented the driver from carrying out the specified operation. The driver's state is probably suspect,
///
/// and the application should not continue.
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: HRESULT = 0x887A0020u32 as HRESULT;

/// A global counter resource was in use, and the specified counter cannot be used by this Direct3D device at this time.
pub const DXGI_ERROR_NONEXCLUSIVE: HRESULT = 0x887A0021u32 as HRESULT;

/// A resource is not available at the time of the call, but may become available later.
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: HRESULT = 0x887A0022u32 as HRESULT;

///  The application's remote device has been removed due to session disconnect or network disconnect.
///
/// The application should call IDXGIFactory1::IsCurrent to find out when the remote device becomes available again.
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: HRESULT = 0x887A0023u32 as HRESULT;

/// The device has been removed during a remote session because the remote computer ran out of memory.
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: HRESULT = 0x887A0024u32 as HRESULT;

/// The keyed mutex was abandoned.
pub const DXGI_ERROR_ACCESS_LOST: HRESULT = 0x887A0026u32 as HRESULT;

/// The timeout value has elapsed and the resource is not yet available.
pub const DXGI_ERROR_WAIT_TIMEOUT: HRESULT = 0x887A0027u32 as HRESULT;

///  The output duplication has been turned off because the Windows session ended or was disconnected.
///
/// This happens when a remote user disconnects, or when "switch user" is used locally.
pub const DXGI_ERROR_SESSION_DISCONNECTED: HRESULT = 0x887A0028u32 as HRESULT;

/// The DXGI output (monitor) to which the swapchain content was restricted, has been disconnected or changed.
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: HRESULT = 0x887A0029u32 as HRESULT;

///  DXGI is unable to provide content protection on the swapchain. This is typically caused by an older driver,
///
/// or by the application using a swapchain that is incompatible with content protection.
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: HRESULT = 0x887A002Au32 as HRESULT;

///  The application is trying to use a resource to which it does not have the required access privileges.
///
/// This is most commonly caused by writing to a shared resource with read-only access.
pub const DXGI_ERROR_ACCESS_DENIED: HRESULT = 0x887A002Bu32 as HRESULT;

/// The application is trying to create a shared handle using a name that is already associated with some other resource.
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: HRESULT = 0x887A002Cu32 as HRESULT;

/// The application requested an operation that depends on an SDK component that is missing or mismatched.
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: HRESULT = 0x887A002Du32 as HRESULT;

/// The DXGI objects that the application has created are no longer current & need to be recreated for this operation to be performed.
pub const DXGI_ERROR_NOT_CURRENT: HRESULT = 0x887A002Eu32 as HRESULT;

/// Insufficient HW protected memory exits for proper function.
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: HRESULT = 0x887A0030u32 as HRESULT;

/// Creating this device would violate the process's dynamic code policy.
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: HRESULT = 0x887A0031u32 as HRESULT;

/// The operation failed because the compositor is not in control of the output.
pub const DXGI_ERROR_NON_COMPOSITED_UI: HRESULT = 0x887A0032u32 as HRESULT;

/// The application failed to unregister from an event it registered for.
pub const DXCORE_ERROR_EVENT_NOT_UNREGISTERED: HRESULT = 0x88800001u32 as HRESULT;

/// The presentation manager has been lost and can no longer be used. The application should destroy this presentation manager and create a new presentation manager to use.
pub const PRESENTATION_ERROR_LOST: HRESULT = 0x88810001u32 as HRESULT;

/// The swapchain has become unoccluded.
pub const DXGI_STATUS_UNOCCLUDED: HRESULT = 0x087A0009;

/// The adapter did not have access to the required resources to complete the Desktop Duplication Present() call, the Present() call needs to be made again
pub const DXGI_STATUS_DDA_WAS_STILL_DRAWING: HRESULT = 0x087A000A;

/// An on-going mode change prevented completion of the call. The call may succeed if attempted later.
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: HRESULT = 0x887A0025u32 as HRESULT;

/// The present succeeded but the caller should present again on the next V-sync, even if there are no changes to the content.
pub const DXGI_STATUS_PRESENT_REQUIRED: HRESULT = 0x087A002F;

/// The cache is corrupt and either could not be opened or could not be reset.
pub const DXGI_ERROR_CACHE_CORRUPT: HRESULT = 0x887A0033u32 as HRESULT;

/// This entry would cause the cache to exceed its quota. On a load operation, this may indicate exceeding the maximum in-memory size.
pub const DXGI_ERROR_CACHE_FULL: HRESULT = 0x887A0034u32 as HRESULT;

/// A cache entry was found, but the key provided does not match the key stored in the entry.
pub const DXGI_ERROR_CACHE_HASH_COLLISION: HRESULT = 0x887A0035u32 as HRESULT;

/// The desired element already exists.
pub const DXGI_ERROR_ALREADY_EXISTS: HRESULT = 0x887A0036u32 as HRESULT;

/// The allocation of the MPO plane has been unpinned
pub const DXGI_ERROR_MPO_UNPINNED: HRESULT = 0x887A0064u32 as HRESULT;

/// The GPU was busy when the operation was requested.
pub const DXGI_DDI_ERR_WASSTILLDRAWING: HRESULT = 0x887B0001u32 as HRESULT;

/// The driver has rejected the creation of this resource.
pub const DXGI_DDI_ERR_UNSUPPORTED: HRESULT = 0x887B0002u32 as HRESULT;

/// The GPU counter was in use by another process or d3d device when application requested access to it.
pub const DXGI_DDI_ERR_NONEXCLUSIVE: HRESULT = 0x887B0003u32 as HRESULT;

///  The application has exceeded the maximum number of unique state objects per Direct3D device.
///
/// The limit is 4096 for feature levels up to 11.1.
pub const D3D10_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS: HRESULT = 0x88790001u32 as HRESULT;

/// The specified file was not found.
pub const D3D10_ERROR_FILE_NOT_FOUND: HRESULT = 0x88790002u32 as HRESULT;

///  The application has exceeded the maximum number of unique state objects per Direct3D device.
///
/// The limit is 4096 for feature levels up to 11.1.
pub const D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS: HRESULT = 0x887C0001u32 as HRESULT;

/// The specified file was not found.
pub const D3D11_ERROR_FILE_NOT_FOUND: HRESULT = 0x887C0002u32 as HRESULT;

///  The application has exceeded the maximum number of unique view objects per Direct3D device.
///
/// The limit is 2^20 for feature levels up to 11.1.
pub const D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS: HRESULT = 0x887C0003u32 as HRESULT;

/// The application's first call per command list to Map on a deferred context did not use D3D11_MAP_WRITE_DISCARD.
pub const D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD: HRESULT =
    0x887C0004u32 as HRESULT;

/// The blob provided does not match the adapter that the device was created on.
pub const D3D12_ERROR_ADAPTER_NOT_FOUND: HRESULT = 0x887E0001u32 as HRESULT;

/// The blob provided was created for a different version of the driver, and must be re-created.
pub const D3D12_ERROR_DRIVER_VERSION_MISMATCH: HRESULT = 0x887E0002u32 as HRESULT;

/// The D3D12 SDK version configuration of the host exe is invalid.
pub const D3D12_ERROR_INVALID_REDIST: HRESULT = 0x887E0003u32 as HRESULT;

/// The object was not in the correct state to process the method.
pub const D2DERR_WRONG_STATE: HRESULT = 0x88990001u32 as HRESULT;

/// The object has not yet been initialized.
pub const D2DERR_NOT_INITIALIZED: HRESULT = 0x88990002u32 as HRESULT;

/// The requested operation is not supported.
pub const D2DERR_UNSUPPORTED_OPERATION: HRESULT = 0x88990003u32 as HRESULT;

/// The geometry scanner failed to process the data.
pub const D2DERR_SCANNER_FAILED: HRESULT = 0x88990004u32 as HRESULT;

/// Direct2D could not access the screen.
pub const D2DERR_SCREEN_ACCESS_DENIED: HRESULT = 0x88990005u32 as HRESULT;

/// A valid display state could not be determined.
pub const D2DERR_DISPLAY_STATE_INVALID: HRESULT = 0x88990006u32 as HRESULT;

/// The supplied vector is zero.
pub const D2DERR_ZERO_VECTOR: HRESULT = 0x88990007u32 as HRESULT;

/// An internal error (Direct2D bug) occurred. On checked builds, we would assert. The application should close this instance of Direct2D and should consider restarting its process.
pub const D2DERR_INTERNAL_ERROR: HRESULT = 0x88990008u32 as HRESULT;

/// The display format Direct2D needs to render is not supported by the hardware device.
pub const D2DERR_DISPLAY_FORMAT_NOT_SUPPORTED: HRESULT = 0x88990009u32 as HRESULT;

/// A call to this method is invalid.
pub const D2DERR_INVALID_CALL: HRESULT = 0x8899000Au32 as HRESULT;

/// No hardware rendering device is available for this operation.
pub const D2DERR_NO_HARDWARE_DEVICE: HRESULT = 0x8899000Bu32 as HRESULT;

/// There has been a presentation error that may be recoverable. The caller needs to recreate, rerender the entire frame, and reattempt present.
pub const D2DERR_RECREATE_TARGET: HRESULT = 0x8899000Cu32 as HRESULT;

/// Shader construction failed because it was too complex.
pub const D2DERR_TOO_MANY_SHADER_ELEMENTS: HRESULT = 0x8899000Du32 as HRESULT;

/// Shader compilation failed.
pub const D2DERR_SHADER_COMPILE_FAILED: HRESULT = 0x8899000Eu32 as HRESULT;

/// Requested DirectX surface size exceeded maximum texture size.
pub const D2DERR_MAX_TEXTURE_SIZE_EXCEEDED: HRESULT = 0x8899000Fu32 as HRESULT;

/// The requested Direct2D version is not supported.
pub const D2DERR_UNSUPPORTED_VERSION: HRESULT = 0x88990010u32 as HRESULT;

/// Invalid number.
pub const D2DERR_BAD_NUMBER: HRESULT = 0x88990011u32 as HRESULT;

/// Objects used together must be created from the same factory instance.
pub const D2DERR_WRONG_FACTORY: HRESULT = 0x88990012u32 as HRESULT;

/// A layer resource can only be in use once at any point in time.
pub const D2DERR_LAYER_ALREADY_IN_USE: HRESULT = 0x88990013u32 as HRESULT;

/// The pop call did not match the corresponding push call.
pub const D2DERR_POP_CALL_DID_NOT_MATCH_PUSH: HRESULT = 0x88990014u32 as HRESULT;

/// The resource was realized on the wrong render target.
pub const D2DERR_WRONG_RESOURCE_DOMAIN: HRESULT = 0x88990015u32 as HRESULT;

/// The push and pop calls were unbalanced.
pub const D2DERR_PUSH_POP_UNBALANCED: HRESULT = 0x88990016u32 as HRESULT;

/// Attempt to copy from a render target while a layer or clip rect is applied.
pub const D2DERR_RENDER_TARGET_HAS_LAYER_OR_CLIPRECT: HRESULT = 0x88990017u32 as HRESULT;

/// The brush types are incompatible for the call.
pub const D2DERR_INCOMPATIBLE_BRUSH_TYPES: HRESULT = 0x88990018u32 as HRESULT;

/// An unknown win32 failure occurred.
pub const D2DERR_WIN32_ERROR: HRESULT = 0x88990019u32 as HRESULT;

/// The render target is not compatible with GDI.
pub const D2DERR_TARGET_NOT_GDI_COMPATIBLE: HRESULT = 0x8899001Au32 as HRESULT;

/// A text client drawing effect object is of the wrong type.
pub const D2DERR_TEXT_EFFECT_IS_WRONG_TYPE: HRESULT = 0x8899001Bu32 as HRESULT;

/// The application is holding a reference to the IDWriteTextRenderer interface after the corresponding DrawText or DrawTextLayout call has returned. The IDWriteTextRenderer instance will be invalid.
pub const D2DERR_TEXT_RENDERER_NOT_RELEASED: HRESULT = 0x8899001Cu32 as HRESULT;

/// The requested size is larger than the guaranteed supported texture size at the Direct3D device's current feature level.
pub const D2DERR_EXCEEDS_MAX_BITMAP_SIZE: HRESULT = 0x8899001Du32 as HRESULT;

/// There was a configuration error in the graph.
pub const D2DERR_INVALID_GRAPH_CONFIGURATION: HRESULT = 0x8899001Eu32 as HRESULT;

/// There was a internal configuration error in the graph.
pub const D2DERR_INVALID_INTERNAL_GRAPH_CONFIGURATION: HRESULT = 0x8899001Fu32 as HRESULT;

/// There was a cycle in the graph.
pub const D2DERR_CYCLIC_GRAPH: HRESULT = 0x88990020u32 as HRESULT;

/// Cannot draw with a bitmap that has the D2D1_BITMAP_OPTIONS_CANNOT_DRAW option.
pub const D2DERR_BITMAP_CANNOT_DRAW: HRESULT = 0x88990021u32 as HRESULT;

/// The operation cannot complete while there are outstanding references to the target bitmap.
pub const D2DERR_OUTSTANDING_BITMAP_REFERENCES: HRESULT = 0x88990022u32 as HRESULT;

/// The operation failed because the original target is not currently bound as a target.
pub const D2DERR_ORIGINAL_TARGET_NOT_BOUND: HRESULT = 0x88990023u32 as HRESULT;

/// Cannot set the image as a target because it is either an effect or is a bitmap that does not have the D2D1_BITMAP_OPTIONS_TARGET flag set.
pub const D2DERR_INVALID_TARGET: HRESULT = 0x88990024u32 as HRESULT;

/// Cannot draw with a bitmap that is currently bound as the target bitmap.
pub const D2DERR_BITMAP_BOUND_AS_TARGET: HRESULT = 0x88990025u32 as HRESULT;

/// D3D Device does not have sufficient capabilities to perform the requested action.
pub const D2DERR_INSUFFICIENT_DEVICE_CAPABILITIES: HRESULT = 0x88990026u32 as HRESULT;

/// The graph could not be rendered with the context's current tiling settings.
pub const D2DERR_INTERMEDIATE_TOO_LARGE: HRESULT = 0x88990027u32 as HRESULT;

/// The CLSID provided to Unregister did not correspond to a registered effect.
pub const D2DERR_EFFECT_IS_NOT_REGISTERED: HRESULT = 0x88990028u32 as HRESULT;

/// The specified property does not exist.
pub const D2DERR_INVALID_PROPERTY: HRESULT = 0x88990029u32 as HRESULT;

/// The specified sub-property does not exist.
pub const D2DERR_NO_SUBPROPERTIES: HRESULT = 0x8899002Au32 as HRESULT;

/// AddPage or Close called after print job is already closed.
pub const D2DERR_PRINT_JOB_CLOSED: HRESULT = 0x8899002Bu32 as HRESULT;

/// Error during print control creation. Indicates that none of the package target types (representing printer formats) are supported by Direct2D print control.
pub const D2DERR_PRINT_FORMAT_NOT_SUPPORTED: HRESULT = 0x8899002Cu32 as HRESULT;

/// An effect attempted to use a transform with too many inputs.
pub const D2DERR_TOO_MANY_TRANSFORM_INPUTS: HRESULT = 0x8899002Du32 as HRESULT;

/// An error was encountered while decoding or parsing the requested glyph image.
pub const D2DERR_INVALID_GLYPH_IMAGE: HRESULT = 0x8899002Eu32 as HRESULT;

/// Indicates an error in an input file such as a font file.
pub const DWRITE_E_FILEFORMAT: HRESULT = 0x88985000u32 as HRESULT;

/// Indicates an error originating in DirectWrite code, which is not expected to occur but is safe to recover from.
pub const DWRITE_E_UNEXPECTED: HRESULT = 0x88985001u32 as HRESULT;

/// Indicates the specified font does not exist.
pub const DWRITE_E_NOFONT: HRESULT = 0x88985002u32 as HRESULT;

/// A font file could not be opened because the file, directory, network location, drive, or other storage location does not exist or is unavailable.
pub const DWRITE_E_FILENOTFOUND: HRESULT = 0x88985003u32 as HRESULT;

/// A font file exists but could not be opened due to access denied, sharing violation, or similar error.
pub const DWRITE_E_FILEACCESS: HRESULT = 0x88985004u32 as HRESULT;

/// A font collection is obsolete due to changes in the system.
pub const DWRITE_E_FONTCOLLECTIONOBSOLETE: HRESULT = 0x88985005u32 as HRESULT;

/// The given interface is already registered.
pub const DWRITE_E_ALREADYREGISTERED: HRESULT = 0x88985006u32 as HRESULT;

/// The font cache contains invalid data.
pub const DWRITE_E_CACHEFORMAT: HRESULT = 0x88985007u32 as HRESULT;

/// A font cache file corresponds to a different version of DirectWrite.
pub const DWRITE_E_CACHEVERSION: HRESULT = 0x88985008u32 as HRESULT;

/// The operation is not supported for this type of font.
pub const DWRITE_E_UNSUPPORTEDOPERATION: HRESULT = 0x88985009u32 as HRESULT;

/// The version of the text renderer interface is not compatible.
pub const DWRITE_E_TEXTRENDERERINCOMPATIBLE: HRESULT = 0x8898500Au32 as HRESULT;

/// The flow direction conflicts with the reading direction. They must be perpendicular to each other.
pub const DWRITE_E_FLOWDIRECTIONCONFLICTS: HRESULT = 0x8898500Bu32 as HRESULT;

/// The font or glyph run does not contain any colored glyphs.
pub const DWRITE_E_NOCOLOR: HRESULT = 0x8898500Cu32 as HRESULT;

/// A font resource could not be accessed because it is remote.
pub const DWRITE_E_REMOTEFONT: HRESULT = 0x8898500Du32 as HRESULT;

/// A font download was canceled.
pub const DWRITE_E_DOWNLOADCANCELLED: HRESULT = 0x8898500Eu32 as HRESULT;

/// A font download failed.
pub const DWRITE_E_DOWNLOADFAILED: HRESULT = 0x8898500Fu32 as HRESULT;

/// A font download request was not added or a download failed because there are too many active downloads.
pub const DWRITE_E_TOOMANYDOWNLOADS: HRESULT = 0x88985010u32 as HRESULT;

/// The codec is in the wrong state.
pub const WINCODEC_ERR_WRONGSTATE: HRESULT = 0x88982F04u32 as HRESULT;

/// The value is out of range.
pub const WINCODEC_ERR_VALUEOUTOFRANGE: HRESULT = 0x88982F05u32 as HRESULT;

/// The image format is unknown.
pub const WINCODEC_ERR_UNKNOWNIMAGEFORMAT: HRESULT = 0x88982F07u32 as HRESULT;

/// The SDK version is unsupported.
pub const WINCODEC_ERR_UNSUPPORTEDVERSION: HRESULT = 0x88982F0Bu32 as HRESULT;

/// The component is not initialized.
pub const WINCODEC_ERR_NOTINITIALIZED: HRESULT = 0x88982F0Cu32 as HRESULT;

/// There is already an outstanding read or write lock.
pub const WINCODEC_ERR_ALREADYLOCKED: HRESULT = 0x88982F0Du32 as HRESULT;

/// The specified bitmap property cannot be found.
pub const WINCODEC_ERR_PROPERTYNOTFOUND: HRESULT = 0x88982F40u32 as HRESULT;

/// The bitmap codec does not support the bitmap property.
pub const WINCODEC_ERR_PROPERTYNOTSUPPORTED: HRESULT = 0x88982F41u32 as HRESULT;

/// The bitmap property size is invalid.
pub const WINCODEC_ERR_PROPERTYSIZE: HRESULT = 0x88982F42u32 as HRESULT;

/// An unknown error has occurred.
pub const WINCODEC_ERR_CODECPRESENT: HRESULT = 0x88982F43u32 as HRESULT;

/// The bitmap codec does not support a thumbnail.
pub const WINCODEC_ERR_CODECNOTHUMBNAIL: HRESULT = 0x88982F44u32 as HRESULT;

/// The bitmap palette is unavailable.
pub const WINCODEC_ERR_PALETTEUNAVAILABLE: HRESULT = 0x88982F45u32 as HRESULT;

/// Too many scanlines were requested.
pub const WINCODEC_ERR_CODECTOOMANYSCANLINES: HRESULT = 0x88982F46u32 as HRESULT;

/// An internal error occurred.
pub const WINCODEC_ERR_INTERNALERROR: HRESULT = 0x88982F48u32 as HRESULT;

/// The bitmap bounds do not match the bitmap dimensions.
pub const WINCODEC_ERR_SOURCERECTDOESNOTMATCHDIMENSIONS: HRESULT = 0x88982F49u32 as HRESULT;

/// The component cannot be found.
pub const WINCODEC_ERR_COMPONENTNOTFOUND: HRESULT = 0x88982F50u32 as HRESULT;

/// The bitmap size is outside the valid range.
pub const WINCODEC_ERR_IMAGESIZEOUTOFRANGE: HRESULT = 0x88982F51u32 as HRESULT;

/// There is too much metadata to be written to the bitmap.
pub const WINCODEC_ERR_TOOMUCHMETADATA: HRESULT = 0x88982F52u32 as HRESULT;

/// The image is unrecognized.
pub const WINCODEC_ERR_BADIMAGE: HRESULT = 0x88982F60u32 as HRESULT;

/// The image header is unrecognized.
pub const WINCODEC_ERR_BADHEADER: HRESULT = 0x88982F61u32 as HRESULT;

/// The bitmap frame is missing.
pub const WINCODEC_ERR_FRAMEMISSING: HRESULT = 0x88982F62u32 as HRESULT;

/// The image metadata header is unrecognized.
pub const WINCODEC_ERR_BADMETADATAHEADER: HRESULT = 0x88982F63u32 as HRESULT;

/// The stream data is unrecognized.
pub const WINCODEC_ERR_BADSTREAMDATA: HRESULT = 0x88982F70u32 as HRESULT;

/// Failed to write to the stream.
pub const WINCODEC_ERR_STREAMWRITE: HRESULT = 0x88982F71u32 as HRESULT;

/// Failed to read from the stream.
pub const WINCODEC_ERR_STREAMREAD: HRESULT = 0x88982F72u32 as HRESULT;

/// The stream is not available.
pub const WINCODEC_ERR_STREAMNOTAVAILABLE: HRESULT = 0x88982F73u32 as HRESULT;

/// The bitmap pixel format is unsupported.
pub const WINCODEC_ERR_UNSUPPORTEDPIXELFORMAT: HRESULT = 0x88982F80u32 as HRESULT;

/// The operation is unsupported.
pub const WINCODEC_ERR_UNSUPPORTEDOPERATION: HRESULT = 0x88982F81u32 as HRESULT;

/// The component registration is invalid.
pub const WINCODEC_ERR_INVALIDREGISTRATION: HRESULT = 0x88982F8Au32 as HRESULT;

/// The component initialization has failed.
pub const WINCODEC_ERR_COMPONENTINITIALIZEFAILURE: HRESULT = 0x88982F8Bu32 as HRESULT;

/// The buffer allocated is insufficient.
pub const WINCODEC_ERR_INSUFFICIENTBUFFER: HRESULT = 0x88982F8Cu32 as HRESULT;

/// Duplicate metadata is present.
pub const WINCODEC_ERR_DUPLICATEMETADATAPRESENT: HRESULT = 0x88982F8Du32 as HRESULT;

/// The bitmap property type is unexpected.
pub const WINCODEC_ERR_PROPERTYUNEXPECTEDTYPE: HRESULT = 0x88982F8Eu32 as HRESULT;

/// The size is unexpected.
pub const WINCODEC_ERR_UNEXPECTEDSIZE: HRESULT = 0x88982F8Fu32 as HRESULT;

/// The property query is invalid.
pub const WINCODEC_ERR_INVALIDQUERYREQUEST: HRESULT = 0x88982F90u32 as HRESULT;

/// The metadata type is unexpected.
pub const WINCODEC_ERR_UNEXPECTEDMETADATATYPE: HRESULT = 0x88982F91u32 as HRESULT;

/// The specified bitmap property is only valid at root level.
pub const WINCODEC_ERR_REQUESTONLYVALIDATMETADATAROOT: HRESULT = 0x88982F92u32 as HRESULT;

/// The query string contains an invalid character.
pub const WINCODEC_ERR_INVALIDQUERYCHARACTER: HRESULT = 0x88982F93u32 as HRESULT;

/// Windows Codecs received an error from the Win32 system.
pub const WINCODEC_ERR_WIN32ERROR: HRESULT = 0x88982F94u32 as HRESULT;

/// The requested level of detail is not present.
pub const WINCODEC_ERR_INVALIDPROGRESSIVELEVEL: HRESULT = 0x88982F95u32 as HRESULT;

/// The scan index is invalid.
pub const WINCODEC_ERR_INVALIDJPEGSCANINDEX: HRESULT = 0x88982F96u32 as HRESULT;

/// MILERR_OBJECTBUSY
pub const MILERR_OBJECTBUSY: HRESULT = 0x88980001u32 as HRESULT;

/// MILERR_INSUFFICIENTBUFFER
pub const MILERR_INSUFFICIENTBUFFER: HRESULT = 0x88980002u32 as HRESULT;

/// MILERR_WIN32ERROR
pub const MILERR_WIN32ERROR: HRESULT = 0x88980003u32 as HRESULT;

/// MILERR_SCANNER_FAILED
pub const MILERR_SCANNER_FAILED: HRESULT = 0x88980004u32 as HRESULT;

/// MILERR_SCREENACCESSDENIED
pub const MILERR_SCREENACCESSDENIED: HRESULT = 0x88980005u32 as HRESULT;

/// MILERR_DISPLAYSTATEINVALID
pub const MILERR_DISPLAYSTATEINVALID: HRESULT = 0x88980006u32 as HRESULT;

/// MILERR_NONINVERTIBLEMATRIX
pub const MILERR_NONINVERTIBLEMATRIX: HRESULT = 0x88980007u32 as HRESULT;

/// MILERR_ZEROVECTOR
pub const MILERR_ZEROVECTOR: HRESULT = 0x88980008u32 as HRESULT;

/// MILERR_TERMINATED
pub const MILERR_TERMINATED: HRESULT = 0x88980009u32 as HRESULT;

/// MILERR_BADNUMBER
pub const MILERR_BADNUMBER: HRESULT = 0x8898000Au32 as HRESULT;

/// An internal error (MIL bug) occurred. On checked builds, an assert would be raised.
pub const MILERR_INTERNALERROR: HRESULT = 0x88980080u32 as HRESULT;

/// The display format we need to render is not supported by the hardware device.
pub const MILERR_DISPLAYFORMATNOTSUPPORTED: HRESULT = 0x88980084u32 as HRESULT;

/// A call to this method is invalid.
pub const MILERR_INVALIDCALL: HRESULT = 0x88980085u32 as HRESULT;

/// Lock attempted on an already locked object.
pub const MILERR_ALREADYLOCKED: HRESULT = 0x88980086u32 as HRESULT;

/// Unlock attempted on an unlocked object.
pub const MILERR_NOTLOCKED: HRESULT = 0x88980087u32 as HRESULT;

/// No algorithm available to render text with this device
pub const MILERR_DEVICECANNOTRENDERTEXT: HRESULT = 0x88980088u32 as HRESULT;

/// Some glyph bitmaps, required for glyph run rendering, are not contained in glyph cache.
pub const MILERR_GLYPHBITMAPMISSED: HRESULT = 0x88980089u32 as HRESULT;

/// Some glyph bitmaps in glyph cache are unexpectedly big.
pub const MILERR_MALFORMEDGLYPHCACHE: HRESULT = 0x8898008Au32 as HRESULT;

/// Marker error for known Win32 errors that are currently being ignored by the compositor. This is to avoid returning S_OK when an error has occurred, but still unwind the stack in the correct location.
pub const MILERR_GENERIC_IGNORE: HRESULT = 0x8898008Bu32 as HRESULT;

/// Guideline coordinates are not sorted properly or contain NaNs.
pub const MILERR_MALFORMED_GUIDELINE_DATA: HRESULT = 0x8898008Cu32 as HRESULT;

/// No HW rendering device is available for this operation.
pub const MILERR_NO_HARDWARE_DEVICE: HRESULT = 0x8898008Du32 as HRESULT;

///  There has been a presentation error that may be recoverable. The caller needs to recreate, rerender the entire frame, and reattempt present.
///
/// There are two known case for this: 1) D3D Driver Internal error 2) D3D E_FAIL 2a) Unknown root cause b) When resizing too quickly for DWM and D3D stay in sync
pub const MILERR_NEED_RECREATE_AND_PRESENT: HRESULT = 0x8898008Eu32 as HRESULT;

/// The object has already been initialized.
pub const MILERR_ALREADY_INITIALIZED: HRESULT = 0x8898008Fu32 as HRESULT;

/// The size of the object does not match the expected size.
pub const MILERR_MISMATCHED_SIZE: HRESULT = 0x88980090u32 as HRESULT;

/// No Redirection surface available.
pub const MILERR_NO_REDIRECTION_SURFACE_AVAILABLE: HRESULT = 0x88980091u32 as HRESULT;

/// Remoting of this content is not supported.
pub const MILERR_REMOTING_NOT_SUPPORTED: HRESULT = 0x88980092u32 as HRESULT;

/// Queued Presents are not supported.
pub const MILERR_QUEUED_PRESENT_NOT_SUPPORTED: HRESULT = 0x88980093u32 as HRESULT;

/// Queued Presents are not being used.
pub const MILERR_NOT_QUEUING_PRESENTS: HRESULT = 0x88980094u32 as HRESULT;

/// No redirection surface was available. Caller should retry the call.
pub const MILERR_NO_REDIRECTION_SURFACE_RETRY_LATER: HRESULT = 0x88980095u32 as HRESULT;

/// Shader construction failed because it was too complex.
pub const MILERR_TOOMANYSHADERELEMNTS: HRESULT = 0x88980096u32 as HRESULT;

/// MROW attempt to get a read lock failed.
pub const MILERR_MROW_READLOCK_FAILED: HRESULT = 0x88980097u32 as HRESULT;

/// MROW attempt to update the data failed because another update was outstanding.
pub const MILERR_MROW_UPDATE_FAILED: HRESULT = 0x88980098u32 as HRESULT;

/// Shader compilation failed.
pub const MILERR_SHADER_COMPILE_FAILED: HRESULT = 0x88980099u32 as HRESULT;

/// Requested DX redirection surface size exceeded maximum texture size.
pub const MILERR_MAX_TEXTURE_SIZE_EXCEEDED: HRESULT = 0x8898009Au32 as HRESULT;

/// QueryPerformanceCounter returned a time in the past.
pub const MILERR_QPC_TIME_WENT_BACKWARD: HRESULT = 0x8898009Bu32 as HRESULT;

/// Primary Display device returned an invalid refresh rate.
pub const MILERR_DXGI_ENUMERATION_OUT_OF_SYNC: HRESULT = 0x8898009Du32 as HRESULT;

/// DWM can not find the adapter specified by the LUID.
pub const MILERR_ADAPTER_NOT_FOUND: HRESULT = 0x8898009Eu32 as HRESULT;

/// The requested bitmap color space is not supported.
pub const MILERR_COLORSPACE_NOT_SUPPORTED: HRESULT = 0x8898009Fu32 as HRESULT;

/// The requested bitmap pre-filtering state is not supported.
pub const MILERR_PREFILTER_NOT_SUPPORTED: HRESULT = 0x889800A0u32 as HRESULT;

/// Access is denied to the requested bitmap for the specified display id.
pub const MILERR_DISPLAYID_ACCESS_DENIED: HRESULT = 0x889800A1u32 as HRESULT;

/// UCEERR_INVALIDPACKETHEADER
pub const UCEERR_INVALIDPACKETHEADER: HRESULT = 0x88980400u32 as HRESULT;

/// UCEERR_UNKNOWNPACKET
pub const UCEERR_UNKNOWNPACKET: HRESULT = 0x88980401u32 as HRESULT;

/// UCEERR_ILLEGALPACKET
pub const UCEERR_ILLEGALPACKET: HRESULT = 0x88980402u32 as HRESULT;

/// UCEERR_MALFORMEDPACKET
pub const UCEERR_MALFORMEDPACKET: HRESULT = 0x88980403u32 as HRESULT;

/// UCEERR_ILLEGALHANDLE
pub const UCEERR_ILLEGALHANDLE: HRESULT = 0x88980404u32 as HRESULT;

/// UCEERR_HANDLELOOKUPFAILED
pub const UCEERR_HANDLELOOKUPFAILED: HRESULT = 0x88980405u32 as HRESULT;

/// UCEERR_RENDERTHREADFAILURE
pub const UCEERR_RENDERTHREADFAILURE: HRESULT = 0x88980406u32 as HRESULT;

/// UCEERR_CTXSTACKFRSTTARGETNULL
pub const UCEERR_CTXSTACKFRSTTARGETNULL: HRESULT = 0x88980407u32 as HRESULT;

/// UCEERR_CONNECTIONIDLOOKUPFAILED
pub const UCEERR_CONNECTIONIDLOOKUPFAILED: HRESULT = 0x88980408u32 as HRESULT;

/// UCEERR_BLOCKSFULL
pub const UCEERR_BLOCKSFULL: HRESULT = 0x88980409u32 as HRESULT;

/// UCEERR_MEMORYFAILURE
pub const UCEERR_MEMORYFAILURE: HRESULT = 0x8898040Au32 as HRESULT;

/// UCEERR_PACKETRECORDOUTOFRANGE
pub const UCEERR_PACKETRECORDOUTOFRANGE: HRESULT = 0x8898040Bu32 as HRESULT;

/// UCEERR_ILLEGALRECORDTYPE
pub const UCEERR_ILLEGALRECORDTYPE: HRESULT = 0x8898040Cu32 as HRESULT;

/// UCEERR_OUTOFHANDLES
pub const UCEERR_OUTOFHANDLES: HRESULT = 0x8898040Du32 as HRESULT;

/// UCEERR_UNCHANGABLE_UPDATE_ATTEMPTED
pub const UCEERR_UNCHANGABLE_UPDATE_ATTEMPTED: HRESULT = 0x8898040Eu32 as HRESULT;

/// UCEERR_NO_MULTIPLE_WORKER_THREADS
pub const UCEERR_NO_MULTIPLE_WORKER_THREADS: HRESULT = 0x8898040Fu32 as HRESULT;

/// UCEERR_REMOTINGNOTSUPPORTED
pub const UCEERR_REMOTINGNOTSUPPORTED: HRESULT = 0x88980410u32 as HRESULT;

/// UCEERR_MISSINGENDCOMMAND
pub const UCEERR_MISSINGENDCOMMAND: HRESULT = 0x88980411u32 as HRESULT;

/// UCEERR_MISSINGBEGINCOMMAND
pub const UCEERR_MISSINGBEGINCOMMAND: HRESULT = 0x88980412u32 as HRESULT;

/// UCEERR_CHANNELSYNCTIMEDOUT
pub const UCEERR_CHANNELSYNCTIMEDOUT: HRESULT = 0x88980413u32 as HRESULT;

/// UCEERR_CHANNELSYNCABANDONED
pub const UCEERR_CHANNELSYNCABANDONED: HRESULT = 0x88980414u32 as HRESULT;

/// UCEERR_UNSUPPORTEDTRANSPORTVERSION
pub const UCEERR_UNSUPPORTEDTRANSPORTVERSION: HRESULT = 0x88980415u32 as HRESULT;

/// UCEERR_TRANSPORTUNAVAILABLE
pub const UCEERR_TRANSPORTUNAVAILABLE: HRESULT = 0x88980416u32 as HRESULT;

/// UCEERR_FEEDBACK_UNSUPPORTED
pub const UCEERR_FEEDBACK_UNSUPPORTED: HRESULT = 0x88980417u32 as HRESULT;

/// UCEERR_COMMANDTRANSPORTDENIED
pub const UCEERR_COMMANDTRANSPORTDENIED: HRESULT = 0x88980418u32 as HRESULT;

/// UCEERR_GRAPHICSSTREAMUNAVAILABLE
pub const UCEERR_GRAPHICSSTREAMUNAVAILABLE: HRESULT = 0x88980419u32 as HRESULT;

/// UCEERR_GRAPHICSSTREAMALREADYOPEN
pub const UCEERR_GRAPHICSSTREAMALREADYOPEN: HRESULT = 0x88980420u32 as HRESULT;

/// UCEERR_TRANSPORTDISCONNECTED
pub const UCEERR_TRANSPORTDISCONNECTED: HRESULT = 0x88980421u32 as HRESULT;

/// UCEERR_TRANSPORTOVERLOADED
pub const UCEERR_TRANSPORTOVERLOADED: HRESULT = 0x88980422u32 as HRESULT;

/// UCEERR_PARTITION_ZOMBIED
pub const UCEERR_PARTITION_ZOMBIED: HRESULT = 0x88980423u32 as HRESULT;

/// MILAVERR_NOCLOCK
pub const MILAVERR_NOCLOCK: HRESULT = 0x88980500u32 as HRESULT;

/// MILAVERR_NOMEDIATYPE
pub const MILAVERR_NOMEDIATYPE: HRESULT = 0x88980501u32 as HRESULT;

/// MILAVERR_NOVIDEOMIXER
pub const MILAVERR_NOVIDEOMIXER: HRESULT = 0x88980502u32 as HRESULT;

/// MILAVERR_NOVIDEOPRESENTER
pub const MILAVERR_NOVIDEOPRESENTER: HRESULT = 0x88980503u32 as HRESULT;

/// MILAVERR_NOREADYFRAMES
pub const MILAVERR_NOREADYFRAMES: HRESULT = 0x88980504u32 as HRESULT;

/// MILAVERR_MODULENOTLOADED
pub const MILAVERR_MODULENOTLOADED: HRESULT = 0x88980505u32 as HRESULT;

/// MILAVERR_WMPFACTORYNOTREGISTERED
pub const MILAVERR_WMPFACTORYNOTREGISTERED: HRESULT = 0x88980506u32 as HRESULT;

/// MILAVERR_INVALIDWMPVERSION
pub const MILAVERR_INVALIDWMPVERSION: HRESULT = 0x88980507u32 as HRESULT;

/// MILAVERR_INSUFFICIENTVIDEORESOURCES
pub const MILAVERR_INSUFFICIENTVIDEORESOURCES: HRESULT = 0x88980508u32 as HRESULT;

/// MILAVERR_VIDEOACCELERATIONNOTAVAILABLE
pub const MILAVERR_VIDEOACCELERATIONNOTAVAILABLE: HRESULT = 0x88980509u32 as HRESULT;

/// MILAVERR_REQUESTEDTEXTURETOOBIG
pub const MILAVERR_REQUESTEDTEXTURETOOBIG: HRESULT = 0x8898050Au32 as HRESULT;

/// MILAVERR_SEEKFAILED
pub const MILAVERR_SEEKFAILED: HRESULT = 0x8898050Bu32 as HRESULT;

/// MILAVERR_UNEXPECTEDWMPFAILURE
pub const MILAVERR_UNEXPECTEDWMPFAILURE: HRESULT = 0x8898050Cu32 as HRESULT;

/// MILAVERR_MEDIAPLAYERCLOSED
pub const MILAVERR_MEDIAPLAYERCLOSED: HRESULT = 0x8898050Du32 as HRESULT;

/// MILAVERR_UNKNOWNHARDWAREERROR
pub const MILAVERR_UNKNOWNHARDWAREERROR: HRESULT = 0x8898050Eu32 as HRESULT;

/// MILEFFECTSERR_UNKNOWNPROPERTY
pub const MILEFFECTSERR_UNKNOWNPROPERTY: HRESULT = 0x8898060Eu32 as HRESULT;

/// MILEFFECTSERR_EFFECTNOTPARTOFGROUP
pub const MILEFFECTSERR_EFFECTNOTPARTOFGROUP: HRESULT = 0x8898060Fu32 as HRESULT;

/// MILEFFECTSERR_NOINPUTSOURCEATTACHED
pub const MILEFFECTSERR_NOINPUTSOURCEATTACHED: HRESULT = 0x88980610u32 as HRESULT;

/// MILEFFECTSERR_CONNECTORNOTCONNECTED
pub const MILEFFECTSERR_CONNECTORNOTCONNECTED: HRESULT = 0x88980611u32 as HRESULT;

/// MILEFFECTSERR_CONNECTORNOTASSOCIATEDWITHEFFECT
pub const MILEFFECTSERR_CONNECTORNOTASSOCIATEDWITHEFFECT: HRESULT = 0x88980612u32 as HRESULT;

/// MILEFFECTSERR_RESERVED
pub const MILEFFECTSERR_RESERVED: HRESULT = 0x88980613u32 as HRESULT;

/// MILEFFECTSERR_CYCLEDETECTED
pub const MILEFFECTSERR_CYCLEDETECTED: HRESULT = 0x88980614u32 as HRESULT;

/// MILEFFECTSERR_EFFECTINMORETHANONEGRAPH
pub const MILEFFECTSERR_EFFECTINMORETHANONEGRAPH: HRESULT = 0x88980615u32 as HRESULT;

/// MILEFFECTSERR_EFFECTALREADYINAGRAPH
pub const MILEFFECTSERR_EFFECTALREADYINAGRAPH: HRESULT = 0x88980616u32 as HRESULT;

/// MILEFFECTSERR_EFFECTHASNOCHILDREN
pub const MILEFFECTSERR_EFFECTHASNOCHILDREN: HRESULT = 0x88980617u32 as HRESULT;

/// MILEFFECTSERR_ALREADYATTACHEDTOLISTENER
pub const MILEFFECTSERR_ALREADYATTACHEDTOLISTENER: HRESULT = 0x88980618u32 as HRESULT;

/// MILEFFECTSERR_NOTAFFINETRANSFORM
pub const MILEFFECTSERR_NOTAFFINETRANSFORM: HRESULT = 0x88980619u32 as HRESULT;

/// MILEFFECTSERR_EMPTYBOUNDS
pub const MILEFFECTSERR_EMPTYBOUNDS: HRESULT = 0x8898061Au32 as HRESULT;

/// MILEFFECTSERR_OUTPUTSIZETOOLARGE
pub const MILEFFECTSERR_OUTPUTSIZETOOLARGE: HRESULT = 0x8898061Bu32 as HRESULT;

/// DWMERR_STATE_TRANSITION_FAILED
pub const DWMERR_STATE_TRANSITION_FAILED: HRESULT = 0x88980700u32 as HRESULT;

/// DWMERR_THEME_FAILED
pub const DWMERR_THEME_FAILED: HRESULT = 0x88980701u32 as HRESULT;

/// DWMERR_CATASTROPHIC_FAILURE
pub const DWMERR_CATASTROPHIC_FAILURE: HRESULT = 0x88980702u32 as HRESULT;

/// DCOMPOSITION_ERROR_WINDOW_ALREADY_COMPOSED
pub const DCOMPOSITION_ERROR_WINDOW_ALREADY_COMPOSED: HRESULT = 0x88980800u32 as HRESULT;

/// DCOMPOSITION_ERROR_SURFACE_BEING_RENDERED
pub const DCOMPOSITION_ERROR_SURFACE_BEING_RENDERED: HRESULT = 0x88980801u32 as HRESULT;

/// DCOMPOSITION_ERROR_SURFACE_NOT_BEING_RENDERED
pub const DCOMPOSITION_ERROR_SURFACE_NOT_BEING_RENDERED: HRESULT = 0x88980802u32 as HRESULT;

/// Authentication target is invalid or not configured correctly.
pub const ONL_E_INVALID_AUTHENTICATION_TARGET: HRESULT = 0x80860001u32 as HRESULT;

/// Your application cannot get the Online Id properties due to the Terms of Use accepted by the user.
pub const ONL_E_ACCESS_DENIED_BY_TOU: HRESULT = 0x80860002u32 as HRESULT;

/// The application requesting authentication tokens is either disabled or incorrectly configured.
pub const ONL_E_INVALID_APPLICATION: HRESULT = 0x80860003u32 as HRESULT;

/// Online Id password must be updated before signin.
pub const ONL_E_PASSWORD_UPDATE_REQUIRED: HRESULT = 0x80860004u32 as HRESULT;

/// Online Id account properties must be updated before signin.
pub const ONL_E_ACCOUNT_UPDATE_REQUIRED: HRESULT = 0x80860005u32 as HRESULT;

/// To help protect your Online Id account you must signin again.
pub const ONL_E_FORCESIGNIN: HRESULT = 0x80860006u32 as HRESULT;

/// Online Id account was locked because there have been too many attempts to sign in.
pub const ONL_E_ACCOUNT_LOCKED: HRESULT = 0x80860007u32 as HRESULT;

/// Online Id account requires parental consent before proceeding.
pub const ONL_E_PARENTAL_CONSENT_REQUIRED: HRESULT = 0x80860008u32 as HRESULT;

/// Online Id signin name is not yet verified. Email verification is required before signin.
pub const ONL_E_EMAIL_VERIFICATION_REQUIRED: HRESULT = 0x80860009u32 as HRESULT;

/// We have noticed some unusual activity in your Online Id account. Your action is needed to make sure no one else is using your account.
pub const ONL_E_ACCOUNT_SUSPENDED_COMPROIMISE: HRESULT = 0x8086000Au32 as HRESULT;

/// We detected some suspicious activity with your Online Id account. To help protect you, we've temporarily blocked your account.
pub const ONL_E_ACCOUNT_SUSPENDED_ABUSE: HRESULT = 0x8086000Bu32 as HRESULT;

/// User interaction is required for authentication.
pub const ONL_E_ACTION_REQUIRED: HRESULT = 0x8086000Cu32 as HRESULT;

/// User has reached the maximum device associations per user limit.
pub const ONL_CONNECTION_COUNT_LIMIT: HRESULT = 0x8086000Du32 as HRESULT;

/// Cannot sign out from the application since the user account is connected.
pub const ONL_E_CONNECTED_ACCOUNT_CAN_NOT_SIGNOUT: HRESULT = 0x8086000Eu32 as HRESULT;

/// User authentication is required for this operation.
pub const ONL_E_USER_AUTHENTICATION_REQUIRED: HRESULT = 0x8086000Fu32 as HRESULT;

/// We want to make sure this is you. User interaction is required for authentication.
pub const ONL_E_REQUEST_THROTTLED: HRESULT = 0x80860010u32 as HRESULT;

/// The maximum number of items for the access list has been reached. An item must be removed before another item is added.
pub const FA_E_MAX_PERSISTED_ITEMS_REACHED: HRESULT = 0x80270220u32 as HRESULT;

/// Cannot access Homegroup. Homegroup may not be set up or may have encountered an error.
pub const FA_E_HOMEGROUP_NOT_AVAILABLE: HRESULT = 0x80270222u32 as HRESULT;

/// This app can't start because the screen resolution is below 1024x768. Choose a higher screen resolution and then try again.
pub const E_MONITOR_RESOLUTION_TOO_LOW: HRESULT = 0x80270250u32 as HRESULT;

/// This app can't be activated from an elevated context.
pub const E_ELEVATED_ACTIVATION_NOT_SUPPORTED: HRESULT = 0x80270251u32 as HRESULT;

/// This app can't be activated when UAC is disabled.
pub const E_UAC_DISABLED: HRESULT = 0x80270252u32 as HRESULT;

/// This app can't be activated by the Built-in Administrator.
pub const E_FULL_ADMIN_NOT_SUPPORTED: HRESULT = 0x80270253u32 as HRESULT;

/// This app does not support the contract specified or is not installed.
pub const E_APPLICATION_NOT_REGISTERED: HRESULT = 0x80270254u32 as HRESULT;

/// This app has multiple extensions registered to support the specified contract. Activation by AppUserModelId is ambiguous.
pub const E_MULTIPLE_EXTENSIONS_FOR_APPLICATION: HRESULT = 0x80270255u32 as HRESULT;

/// This app's package family has more than one package installed. This is not supported.
pub const E_MULTIPLE_PACKAGES_FOR_FAMILY: HRESULT = 0x80270256u32 as HRESULT;

/// The app manager is required to activate applications, but is not running.
pub const E_APPLICATION_MANAGER_NOT_RUNNING: HRESULT = 0x80270257u32 as HRESULT;

/// The Store was launched instead of the specified app because the app's package was in an invalid state.
pub const S_STORE_LAUNCHED_FOR_REMEDIATION: HRESULT = 0x00270258;

/// This app failed to launch, but the error was handled with a dialog.
pub const S_APPLICATION_ACTIVATION_ERROR_HANDLED_BY_DIALOG: HRESULT = 0x00270259;

/// The app didn't start in the required time.
pub const E_APPLICATION_ACTIVATION_TIMED_OUT: HRESULT = 0x8027025Au32 as HRESULT;

/// The app didn't start.
pub const E_APPLICATION_ACTIVATION_EXEC_FAILURE: HRESULT = 0x8027025Bu32 as HRESULT;

/// This app failed to launch because of an issue with its license. Please try again in a moment.
pub const E_APPLICATION_TEMPORARY_LICENSE_ERROR: HRESULT = 0x8027025Cu32 as HRESULT;

/// This app failed to launch because its trial license has expired.
pub const E_APPLICATION_TRIAL_LICENSE_EXPIRED: HRESULT = 0x8027025Du32 as HRESULT;

/// Please choose a folder on a drive that's formatted with the NTFS file system.
pub const E_SKYDRIVE_ROOT_TARGET_FILE_SYSTEM_NOT_SUPPORTED: HRESULT = 0x80270260u32 as HRESULT;

/// This location is already being used. Please choose a different location.
pub const E_SKYDRIVE_ROOT_TARGET_OVERLAP: HRESULT = 0x80270261u32 as HRESULT;

/// This location cannot be indexed. Please choose a different location.
pub const E_SKYDRIVE_ROOT_TARGET_CANNOT_INDEX: HRESULT = 0x80270262u32 as HRESULT;

/// Sorry, the action couldn't be completed because the file hasn't finished uploading. Try again later.
pub const E_SKYDRIVE_FILE_NOT_UPLOADED: HRESULT = 0x80270263u32 as HRESULT;

/// Sorry, the action couldn't be completed.
pub const E_SKYDRIVE_UPDATE_AVAILABILITY_FAIL: HRESULT = 0x80270264u32 as HRESULT;

/// This content can only be moved to a folder. To move the content to this drive, please choose or create a folder.
pub const E_SKYDRIVE_ROOT_TARGET_VOLUME_ROOT_NOT_SUPPORTED: HRESULT = 0x80270265u32 as HRESULT;

/// The file size is larger than supported by the sync engine.
pub const E_SYNCENGINE_FILE_SIZE_OVER_LIMIT: HRESULT = 0x8802B001u32 as HRESULT;

/// The file cannot be uploaded because it doesn't fit in the user's available service provided storage space.
pub const E_SYNCENGINE_FILE_SIZE_EXCEEDS_REMAINING_QUOTA: HRESULT = 0x8802B002u32 as HRESULT;

/// The file name contains invalid characters.
pub const E_SYNCENGINE_UNSUPPORTED_FILE_NAME: HRESULT = 0x8802B003u32 as HRESULT;

/// The maximum file count has been reached for this folder in the sync engine.
pub const E_SYNCENGINE_FOLDER_ITEM_COUNT_LIMIT_EXCEEDED: HRESULT = 0x8802B004u32 as HRESULT;

/// The file sync has been delegated to another program and has run into an issue.
pub const E_SYNCENGINE_FILE_SYNC_PARTNER_ERROR: HRESULT = 0x8802B005u32 as HRESULT;

/// Sync has been delayed due to a throttling request from the service.
pub const E_SYNCENGINE_SYNC_PAUSED_BY_SERVICE: HRESULT = 0x8802B006u32 as HRESULT;

/// We can't seem to find that file. Please try again later.
pub const E_SYNCENGINE_FILE_IDENTIFIER_UNKNOWN: HRESULT = 0x8802C002u32 as HRESULT;

/// The account you're signed in with doesn't have permission to open this file.
pub const E_SYNCENGINE_SERVICE_AUTHENTICATION_FAILED: HRESULT = 0x8802C003u32 as HRESULT;

/// There was a problem connecting to the service. Please try again later.
pub const E_SYNCENGINE_UNKNOWN_SERVICE_ERROR: HRESULT = 0x8802C004u32 as HRESULT;

/// Sorry, there was a problem downloading the file.
pub const E_SYNCENGINE_SERVICE_RETURNED_UNEXPECTED_SIZE: HRESULT = 0x8802C005u32 as HRESULT;

/// We're having trouble downloading the file right now. Please try again later.
pub const E_SYNCENGINE_REQUEST_BLOCKED_BY_SERVICE: HRESULT = 0x8802C006u32 as HRESULT;

/// We're having trouble downloading the file right now. Please try again later.
pub const E_SYNCENGINE_REQUEST_BLOCKED_DUE_TO_CLIENT_ERROR: HRESULT = 0x8802C007u32 as HRESULT;

/// The sync engine does not have permissions to access a local folder under the sync root.
pub const E_SYNCENGINE_FOLDER_INACCESSIBLE: HRESULT = 0x8802D001u32 as HRESULT;

/// The folder name contains invalid characters.
pub const E_SYNCENGINE_UNSUPPORTED_FOLDER_NAME: HRESULT = 0x8802D002u32 as HRESULT;

/// The sync engine is not allowed to run in your current market.
pub const E_SYNCENGINE_UNSUPPORTED_MARKET: HRESULT = 0x8802D003u32 as HRESULT;

/// All files and folders can't be uploaded because a path of a file or folder is too long.
pub const E_SYNCENGINE_PATH_LENGTH_LIMIT_EXCEEDED: HRESULT = 0x8802D004u32 as HRESULT;

/// All file and folders cannot be synchronized because a path of a file or folder would exceed the local path limit.
pub const E_SYNCENGINE_REMOTE_PATH_LENGTH_LIMIT_EXCEEDED: HRESULT = 0x8802D005u32 as HRESULT;

/// Updates are needed in order to use the sync engine.
pub const E_SYNCENGINE_CLIENT_UPDATE_NEEDED: HRESULT = 0x8802D006u32 as HRESULT;

/// The sync engine needs to authenticate with a proxy server.
pub const E_SYNCENGINE_PROXY_AUTHENTICATION_REQUIRED: HRESULT = 0x8802D007u32 as HRESULT;

/// There was a problem setting up the storage services for the account.
pub const E_SYNCENGINE_STORAGE_SERVICE_PROVISIONING_FAILED: HRESULT = 0x8802D008u32 as HRESULT;

/// Files can't be uploaded because there's an unsupported reparse point.
pub const E_SYNCENGINE_UNSUPPORTED_REPARSE_POINT: HRESULT = 0x8802D009u32 as HRESULT;

/// The service has blocked your account from accessing the storage service.
pub const E_SYNCENGINE_STORAGE_SERVICE_BLOCKED: HRESULT = 0x8802D00Au32 as HRESULT;

/// The action can't be performed right now because this folder is being moved. Please try again later.
pub const E_SYNCENGINE_FOLDER_IN_REDIRECTION: HRESULT = 0x8802D00Bu32 as HRESULT;

/// Windows cannot evaluate this EAS policy since this is not managed by the operating system.
pub const EAS_E_POLICY_NOT_MANAGED_BY_OS: HRESULT = 0x80550001u32 as HRESULT;

/// The system can be made compliant to this EAS policy if certain actions are performed by the user.
pub const EAS_E_POLICY_COMPLIANT_WITH_ACTIONS: HRESULT = 0x80550002u32 as HRESULT;

/// The EAS policy being evaluated cannot be enforced by the system.
pub const EAS_E_REQUESTED_POLICY_NOT_ENFORCEABLE: HRESULT = 0x80550003u32 as HRESULT;

/// EAS password policies for the user cannot be evaluated as the user has a blank password.
pub const EAS_E_CURRENT_USER_HAS_BLANK_PASSWORD: HRESULT = 0x80550004u32 as HRESULT;

/// EAS password expiration policy cannot be satisfied as the password expiration interval is less than the minimum password interval of the system.
pub const EAS_E_REQUESTED_POLICY_PASSWORD_EXPIRATION_INCOMPATIBLE: HRESULT =
    0x80550005u32 as HRESULT;

/// The user is not allowed to change her password.
pub const EAS_E_USER_CANNOT_CHANGE_PASSWORD: HRESULT = 0x80550006u32 as HRESULT;

/// EAS password policies cannot be evaluated as one or more admins have blank passwords.
pub const EAS_E_ADMINS_HAVE_BLANK_PASSWORD: HRESULT = 0x80550007u32 as HRESULT;

/// One or more admins are not allowed to change their password.
pub const EAS_E_ADMINS_CANNOT_CHANGE_PASSWORD: HRESULT = 0x80550008u32 as HRESULT;

/// There are other standard users present who are not allowed to change their password.
pub const EAS_E_LOCAL_CONTROLLED_USERS_CANNOT_CHANGE_PASSWORD: HRESULT = 0x80550009u32 as HRESULT;

/// The EAS password policy cannot be enforced by the connected account provider of at least one administrator.
pub const EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CONNECTED_ADMINS: HRESULT =
    0x8055000Au32 as HRESULT;

/// There is at least one administrator whose connected account password needs to be changed for EAS password policy compliance.
pub const EAS_E_CONNECTED_ADMINS_NEED_TO_CHANGE_PASSWORD: HRESULT = 0x8055000Bu32 as HRESULT;

/// The EAS password policy cannot be enforced by the connected account provider of the current user.
pub const EAS_E_PASSWORD_POLICY_NOT_ENFORCEABLE_FOR_CURRENT_CONNECTED_USER: HRESULT =
    0x8055000Cu32 as HRESULT;

/// The connected account password of the current user needs to be changed for EAS password policy compliance.
pub const EAS_E_CURRENT_CONNECTED_USER_NEED_TO_CHANGE_PASSWORD: HRESULT = 0x8055000Du32 as HRESULT;

/// Unsupported format.
pub const WEB_E_UNSUPPORTED_FORMAT: HRESULT = 0x83750001u32 as HRESULT;

/// Invalid XML.
pub const WEB_E_INVALID_XML: HRESULT = 0x83750002u32 as HRESULT;

/// Missing required element.
pub const WEB_E_MISSING_REQUIRED_ELEMENT: HRESULT = 0x83750003u32 as HRESULT;

/// Missing required attribute.
pub const WEB_E_MISSING_REQUIRED_ATTRIBUTE: HRESULT = 0x83750004u32 as HRESULT;

/// Unexpected content.
pub const WEB_E_UNEXPECTED_CONTENT: HRESULT = 0x83750005u32 as HRESULT;

/// Resource too large.
pub const WEB_E_RESOURCE_TOO_LARGE: HRESULT = 0x83750006u32 as HRESULT;

/// Invalid JSON string.
pub const WEB_E_INVALID_JSON_STRING: HRESULT = 0x83750007u32 as HRESULT;

/// Invalid JSON number.
pub const WEB_E_INVALID_JSON_NUMBER: HRESULT = 0x83750008u32 as HRESULT;

/// JSON value not found.
pub const WEB_E_JSON_VALUE_NOT_FOUND: HRESULT = 0x83750009u32 as HRESULT;

/// Unexpected HTTP status code.
pub const HTTP_E_STATUS_UNEXPECTED: HRESULT = 0x80190001u32 as HRESULT;

/// Unexpected redirection status code (3xx).
pub const HTTP_E_STATUS_UNEXPECTED_REDIRECTION: HRESULT = 0x80190003u32 as HRESULT;

/// Unexpected client error status code (4xx).
pub const HTTP_E_STATUS_UNEXPECTED_CLIENT_ERROR: HRESULT = 0x80190004u32 as HRESULT;

/// Unexpected server error status code (5xx).
pub const HTTP_E_STATUS_UNEXPECTED_SERVER_ERROR: HRESULT = 0x80190005u32 as HRESULT;

/// Multiple choices (300).
pub const HTTP_E_STATUS_AMBIGUOUS: HRESULT = 0x8019012Cu32 as HRESULT;

/// Moved permanently (301).
pub const HTTP_E_STATUS_MOVED: HRESULT = 0x8019012Du32 as HRESULT;

/// Found (302).
pub const HTTP_E_STATUS_REDIRECT: HRESULT = 0x8019012Eu32 as HRESULT;

/// See Other (303).
pub const HTTP_E_STATUS_REDIRECT_METHOD: HRESULT = 0x8019012Fu32 as HRESULT;

/// Not modified (304).
pub const HTTP_E_STATUS_NOT_MODIFIED: HRESULT = 0x80190130u32 as HRESULT;

/// Use proxy (305).
pub const HTTP_E_STATUS_USE_PROXY: HRESULT = 0x80190131u32 as HRESULT;

/// Temporary redirect (307).
pub const HTTP_E_STATUS_REDIRECT_KEEP_VERB: HRESULT = 0x80190133u32 as HRESULT;

/// Bad request (400).
pub const HTTP_E_STATUS_BAD_REQUEST: HRESULT = 0x80190190u32 as HRESULT;

/// Unauthorized (401).
pub const HTTP_E_STATUS_DENIED: HRESULT = 0x80190191u32 as HRESULT;

/// Payment required (402).
pub const HTTP_E_STATUS_PAYMENT_REQ: HRESULT = 0x80190192u32 as HRESULT;

/// Forbidden (403).
pub const HTTP_E_STATUS_FORBIDDEN: HRESULT = 0x80190193u32 as HRESULT;

/// Not found (404).
pub const HTTP_E_STATUS_NOT_FOUND: HRESULT = 0x80190194u32 as HRESULT;

/// Method not allowed (405).
pub const HTTP_E_STATUS_BAD_METHOD: HRESULT = 0x80190195u32 as HRESULT;

/// Not acceptable (406).
pub const HTTP_E_STATUS_NONE_ACCEPTABLE: HRESULT = 0x80190196u32 as HRESULT;

/// Proxy authentication required (407).
pub const HTTP_E_STATUS_PROXY_AUTH_REQ: HRESULT = 0x80190197u32 as HRESULT;

/// Request timeout (408).
pub const HTTP_E_STATUS_REQUEST_TIMEOUT: HRESULT = 0x80190198u32 as HRESULT;

/// Conflict (409).
pub const HTTP_E_STATUS_CONFLICT: HRESULT = 0x80190199u32 as HRESULT;

/// Gone (410).
pub const HTTP_E_STATUS_GONE: HRESULT = 0x8019019Au32 as HRESULT;

/// Length required (411).
pub const HTTP_E_STATUS_LENGTH_REQUIRED: HRESULT = 0x8019019Bu32 as HRESULT;

/// Precondition failed (412).
pub const HTTP_E_STATUS_PRECOND_FAILED: HRESULT = 0x8019019Cu32 as HRESULT;

/// Request entity too large (413).
pub const HTTP_E_STATUS_REQUEST_TOO_LARGE: HRESULT = 0x8019019Du32 as HRESULT;

/// Request-URI too long (414).
pub const HTTP_E_STATUS_URI_TOO_LONG: HRESULT = 0x8019019Eu32 as HRESULT;

/// Unsupported media type (415).
pub const HTTP_E_STATUS_UNSUPPORTED_MEDIA: HRESULT = 0x8019019Fu32 as HRESULT;

/// Requested range not satisfiable (416).
pub const HTTP_E_STATUS_RANGE_NOT_SATISFIABLE: HRESULT = 0x801901A0u32 as HRESULT;

/// Expectation failed (417).
pub const HTTP_E_STATUS_EXPECTATION_FAILED: HRESULT = 0x801901A1u32 as HRESULT;

/// Internal server error (500).
pub const HTTP_E_STATUS_SERVER_ERROR: HRESULT = 0x801901F4u32 as HRESULT;

/// Not implemented (501).
pub const HTTP_E_STATUS_NOT_SUPPORTED: HRESULT = 0x801901F5u32 as HRESULT;

/// Bad gateway (502).
pub const HTTP_E_STATUS_BAD_GATEWAY: HRESULT = 0x801901F6u32 as HRESULT;

/// Service unavailable (503).
pub const HTTP_E_STATUS_SERVICE_UNAVAIL: HRESULT = 0x801901F7u32 as HRESULT;

/// Gateway timeout (504).
pub const HTTP_E_STATUS_GATEWAY_TIMEOUT: HRESULT = 0x801901F8u32 as HRESULT;

/// Version not supported (505).
pub const HTTP_E_STATUS_VERSION_NOT_SUP: HRESULT = 0x801901F9u32 as HRESULT;

/// Invalid operation performed by the protocol.
pub const E_INVALID_PROTOCOL_OPERATION: HRESULT = 0x83760001u32 as HRESULT;

/// Invalid data format for the specific protocol operation.
pub const E_INVALID_PROTOCOL_FORMAT: HRESULT = 0x83760002u32 as HRESULT;

/// Protocol extensions are not supported.
pub const E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED: HRESULT = 0x83760003u32 as HRESULT;

/// Subprotocol is not supported.
pub const E_SUBPROTOCOL_NOT_SUPPORTED: HRESULT = 0x83760004u32 as HRESULT;

/// Incorrect protocol version.
pub const E_PROTOCOL_VERSION_NOT_SUPPORTED: HRESULT = 0x83760005u32 as HRESULT;

/// Input data cannot be processed in the non-chronological order.
pub const INPUT_E_OUT_OF_ORDER: HRESULT = 0x80400000u32 as HRESULT;

/// Requested operation cannot be performed inside the callback or event handler.
pub const INPUT_E_REENTRANCY: HRESULT = 0x80400001u32 as HRESULT;

/// Input cannot be processed because there is ongoing interaction with another pointer type.
pub const INPUT_E_MULTIMODAL: HRESULT = 0x80400002u32 as HRESULT;

/// One or more fields in the input packet are invalid.
pub const INPUT_E_PACKET: HRESULT = 0x80400003u32 as HRESULT;

/// Packets in the frame are inconsistent. Either pointer ids are not unique or there is a discrepancy in timestamps, frame ids, pointer types or source devices.
pub const INPUT_E_FRAME: HRESULT = 0x80400004u32 as HRESULT;

/// The history of frames is inconsistent. Pointer ids, types, source devices don't match, or frame ids are not unique, or timestamps are out of order.
pub const INPUT_E_HISTORY: HRESULT = 0x80400005u32 as HRESULT;

/// Failed to retrieve information about the input device.
pub const INPUT_E_DEVICE_INFO: HRESULT = 0x80400006u32 as HRESULT;

/// Coordinate system transformation failed to transform the data.
pub const INPUT_E_TRANSFORM: HRESULT = 0x80400007u32 as HRESULT;

/// The property is not supported or not reported correctly by the input device.
pub const INPUT_E_DEVICE_PROPERTY: HRESULT = 0x80400008u32 as HRESULT;

/// The URL is invalid.
pub const INET_E_INVALID_URL: HRESULT = 0x800C0002u32 as HRESULT;

/// No Internet session has been established.
pub const INET_E_NO_SESSION: HRESULT = 0x800C0003u32 as HRESULT;

/// Unable to connect to the target server.
pub const INET_E_CANNOT_CONNECT: HRESULT = 0x800C0004u32 as HRESULT;

/// The system cannot locate the resource specified.
pub const INET_E_RESOURCE_NOT_FOUND: HRESULT = 0x800C0005u32 as HRESULT;

/// The system cannot locate the object specified.
pub const INET_E_OBJECT_NOT_FOUND: HRESULT = 0x800C0006u32 as HRESULT;

/// No data is available for the requested resource.
pub const INET_E_DATA_NOT_AVAILABLE: HRESULT = 0x800C0007u32 as HRESULT;

/// The download of the specified resource has failed.
pub const INET_E_DOWNLOAD_FAILURE: HRESULT = 0x800C0008u32 as HRESULT;

/// Authentication is required to access this resource.
pub const INET_E_AUTHENTICATION_REQUIRED: HRESULT = 0x800C0009u32 as HRESULT;

/// The server could not recognize the provided mime type.
pub const INET_E_NO_VALID_MEDIA: HRESULT = 0x800C000Au32 as HRESULT;

/// The operation was timed out.
pub const INET_E_CONNECTION_TIMEOUT: HRESULT = 0x800C000Bu32 as HRESULT;

/// The server did not understand the request, or the request was invalid.
pub const INET_E_INVALID_REQUEST: HRESULT = 0x800C000Cu32 as HRESULT;

/// The specified protocol is unknown.
pub const INET_E_UNKNOWN_PROTOCOL: HRESULT = 0x800C000Du32 as HRESULT;

/// A security problem occurred.
pub const INET_E_SECURITY_PROBLEM: HRESULT = 0x800C000Eu32 as HRESULT;

/// The system could not load the persisted data.
pub const INET_E_CANNOT_LOAD_DATA: HRESULT = 0x800C000Fu32 as HRESULT;

/// Unable to instantiate the object.
pub const INET_E_CANNOT_INSTANTIATE_OBJECT: HRESULT = 0x800C0010u32 as HRESULT;

/// Security certificate required to access this resource is invalid.
pub const INET_E_INVALID_CERTIFICATE: HRESULT = 0x800C0019u32 as HRESULT;

/// A redirection problem occurred.
pub const INET_E_REDIRECT_FAILED: HRESULT = 0x800C0014u32 as HRESULT;

/// The requested resource is a directory, not a file.
pub const INET_E_REDIRECT_TO_DIR: HRESULT = 0x800C0015u32 as HRESULT;

/// Could not create new process from ARM architecture device.
pub const ERROR_DBG_CREATE_PROCESS_FAILURE_LOCKDOWN: HRESULT = 0x80B00001u32 as HRESULT;

/// Could not attach to the application process from ARM architecture device.
pub const ERROR_DBG_ATTACH_PROCESS_FAILURE_LOCKDOWN: HRESULT = 0x80B00002u32 as HRESULT;

/// Could not connect to dbgsrv server from ARM architecture device.
pub const ERROR_DBG_CONNECT_SERVER_FAILURE_LOCKDOWN: HRESULT = 0x80B00003u32 as HRESULT;

/// Could not start dbgsrv server from ARM architecture device.
pub const ERROR_DBG_START_SERVER_FAILURE_LOCKDOWN: HRESULT = 0x80B00004u32 as HRESULT;

/// This is an error mask to convert HSP hardware errors to Win errors.
pub const HSP_E_ERROR_MASK: HRESULT = 0x81280000u32 as HRESULT;

/// Catastrophic internal failure in the HSP hardware.
pub const HSP_E_INTERNAL_ERROR: HRESULT = 0x81280FFFu32 as HRESULT;

/// This is an error mask to convert HSP base services errors to Win errors.
pub const HSP_BS_ERROR_MASK: HRESULT = 0x81281000u32 as HRESULT;

/// Catastrophic internal failure in the HSP base services.
pub const HSP_BS_INTERNAL_ERROR: HRESULT = 0x812810FFu32 as HRESULT;

/// This is an error mask to convert HSP driver errors to Win errors.
pub const HSP_DRV_ERROR_MASK: HRESULT = 0x81290000u32 as HRESULT;

/// Catastrophic internal failure in the HSP driver.
pub const HSP_DRV_INTERNAL_ERROR: HRESULT = 0x812900FFu32 as HRESULT;

/// This is an error mask to convert HSP base class errors to Win errors.
pub const HSP_BASE_ERROR_MASK: HRESULT = 0x81290100u32 as HRESULT;

/// Catastrophic internal failure in the HSP base class.
pub const HSP_BASE_INTERNAL_ERROR: HRESULT = 0x812901FFu32 as HRESULT;

/// This is an error mask to convert HSP KSP errors to Win errors.
pub const HSP_KSP_ERROR_MASK: HRESULT = 0x81290200u32 as HRESULT;

/// The Pluton processor is currently not ready for use.
pub const HSP_KSP_DEVICE_NOT_READY: HRESULT = 0x81290201u32 as HRESULT;

/// The handle to the HSP KSP is invalid.
pub const HSP_KSP_INVALID_PROVIDER_HANDLE: HRESULT = 0x81290202u32 as HRESULT;

/// The handle to a key stored by the HSP KSP is invalid.
pub const HSP_KSP_INVALID_KEY_HANDLE: HRESULT = 0x81290203u32 as HRESULT;

/// A parameter to the HSP KSP was invalid.
pub const HSP_KSP_INVALID_PARAMETER: HRESULT = 0x81290204u32 as HRESULT;

/// The supplied buffer is too small.
pub const HSP_KSP_BUFFER_TOO_SMALL: HRESULT = 0x81290205u32 as HRESULT;

/// The requested operation is not supported.
pub const HSP_KSP_NOT_SUPPORTED: HRESULT = 0x81290206u32 as HRESULT;

/// The provided data is invalid.
pub const HSP_KSP_INVALID_DATA: HRESULT = 0x81290207u32 as HRESULT;

/// The provided flags are invalid.
pub const HSP_KSP_INVALID_FLAGS: HRESULT = 0x81290208u32 as HRESULT;

/// The algorithm identifier is not supported.
pub const HSP_KSP_ALGORITHM_NOT_SUPPORTED: HRESULT = 0x81290209u32 as HRESULT;

/// The key has already been finalized.
pub const HSP_KSP_KEY_ALREADY_FINALIZED: HRESULT = 0x8129020Au32 as HRESULT;

/// The key has not been finalized.
pub const HSP_KSP_KEY_NOT_FINALIZED: HRESULT = 0x8129020Bu32 as HRESULT;

/// The key does not support the requested operation.
pub const HSP_KSP_INVALID_KEY_TYPE: HRESULT = 0x8129020Cu32 as HRESULT;

/// There is not enough memory for the operation.
pub const HSP_KSP_NO_MEMORY: HRESULT = 0x81290210u32 as HRESULT;

/// The parameter has not been set and has no default value.
pub const HSP_KSP_PARAMETER_NOT_SET: HRESULT = 0x81290211u32 as HRESULT;

/// Key object already exists.
pub const HSP_KSP_KEY_EXISTS: HRESULT = 0x81290215u32 as HRESULT;

/// The requsted key object does not exist.
pub const HSP_KSP_KEY_MISSING: HRESULT = 0x81290216u32 as HRESULT;

/// Failed to load the requested key.
pub const HSP_KSP_KEY_LOAD_FAIL: HRESULT = 0x81290217u32 as HRESULT;

/// No more data is available.
pub const HSP_KSP_NO_MORE_ITEMS: HRESULT = 0x81290218u32 as HRESULT;

/// Catastrophic internal failure in the HSP KSP.
pub const HSP_KSP_INTERNAL_ERROR: HRESULT = 0x812902FFu32 as HRESULT;

/// The operation was preempted by a higher priority operation. It must be resumed later.
pub const ERROR_IO_PREEMPTED: HRESULT = 0x89010001u32 as HRESULT;

/// Function could not execute because it was deleted or garbage collected.
pub const JSCRIPT_E_CANTEXECUTE: HRESULT = 0x89020001u32 as HRESULT;

/// One or more fixed volumes are not provisioned with the 3rd party encryption providers to support device encryption. Enable encryption with the 3rd party provider to comply with policy.
pub const WEP_E_NOT_PROVISIONED_ON_ALL_VOLUMES: HRESULT = 0x88010001u32 as HRESULT;

/// This computer is not fully encrypted. There are fixed volumes present which are not supported for encryption.
pub const WEP_E_FIXED_DATA_NOT_SUPPORTED: HRESULT = 0x88010002u32 as HRESULT;

/// This computer does not meet the hardware requirements to support device encryption with the installed 3rd party provider.
pub const WEP_E_HARDWARE_NOT_COMPLIANT: HRESULT = 0x88010003u32 as HRESULT;

/// This computer cannot support device encryption because the requisites for the device lock feature are not configured.
pub const WEP_E_LOCK_NOT_CONFIGURED: HRESULT = 0x88010004u32 as HRESULT;

/// Protection is enabled on this volume but is not in the active state.
pub const WEP_E_PROTECTION_SUSPENDED: HRESULT = 0x88010005u32 as HRESULT;

/// The 3rd party provider has been installed, but cannot activate encryption because a license has not been activated.
pub const WEP_E_NO_LICENSE: HRESULT = 0x88010006u32 as HRESULT;

/// The operating system drive is not protected by 3rd party drive encryption.
pub const WEP_E_OS_NOT_PROTECTED: HRESULT = 0x88010007u32 as HRESULT;

/// Unexpected failure was encountered while calling into the 3rd Party drive encryption plugin.
pub const WEP_E_UNEXPECTED_FAIL: HRESULT = 0x88010008u32 as HRESULT;

/// The input buffer size for the lockout metadata used by the 3rd party drive encryption is too large.
pub const WEP_E_BUFFER_TOO_LARGE: HRESULT = 0x88010009u32 as HRESULT;

/// The proper error code with sense data was stored on server side.
pub const ERROR_SVHDX_ERROR_STORED: HRESULT = 0xC05C0000u32 as HRESULT;

/// The requested error data is not available on the server.
pub const ERROR_SVHDX_ERROR_NOT_AVAILABLE: HRESULT = 0xC05CFF00u32 as HRESULT;

/// Unit Attention data is available for the initiator to query.
pub const ERROR_SVHDX_UNIT_ATTENTION_AVAILABLE: HRESULT = 0xC05CFF01u32 as HRESULT;

/// The data capacity of the device has changed, resulting in a Unit Attention condition.
pub const ERROR_SVHDX_UNIT_ATTENTION_CAPACITY_DATA_CHANGED: HRESULT = 0xC05CFF02u32 as HRESULT;

/// A previous operation resulted in this initiator's reservations being preempted, resulting in a Unit Attention condition.
pub const ERROR_SVHDX_UNIT_ATTENTION_RESERVATIONS_PREEMPTED: HRESULT = 0xC05CFF03u32 as HRESULT;

/// A previous operation resulted in this initiator's reservations being released, resulting in a Unit Attention condition.
pub const ERROR_SVHDX_UNIT_ATTENTION_RESERVATIONS_RELEASED: HRESULT = 0xC05CFF04u32 as HRESULT;

/// A previous operation resulted in this initiator's registrations being preempted, resulting in a Unit Attention condition.
pub const ERROR_SVHDX_UNIT_ATTENTION_REGISTRATIONS_PREEMPTED: HRESULT = 0xC05CFF05u32 as HRESULT;

/// The data storage format of the device has changed, resulting in a Unit Attention condition.
pub const ERROR_SVHDX_UNIT_ATTENTION_OPERATING_DEFINITION_CHANGED: HRESULT =
    0xC05CFF06u32 as HRESULT;

/// The current initiator is not allowed to perform the SCSI command because of a reservation conflict.
pub const ERROR_SVHDX_RESERVATION_CONFLICT: HRESULT = 0xC05CFF07u32 as HRESULT;

/// Multiple virtual machines sharing a virtual hard disk is supported only on Fixed or Dynamic VHDX format virtual hard disks.
pub const ERROR_SVHDX_WRONG_FILE_TYPE: HRESULT = 0xC05CFF08u32 as HRESULT;

/// The server version does not match the requested version.
pub const ERROR_SVHDX_VERSION_MISMATCH: HRESULT = 0xC05CFF09u32 as HRESULT;

/// The requested operation cannot be performed on the virtual disk as it is currently used in shared mode.
pub const ERROR_VHD_SHARED: HRESULT = 0xC05CFF0Au32 as HRESULT;

/// Invalid Shared VHDX open due to lack of initiator ID. Check for related Continuous Availability failures.
pub const ERROR_SVHDX_NO_INITIATOR: HRESULT = 0xC05CFF0Bu32 as HRESULT;

/// The requested operation failed due to a missing backing storage file.
pub const ERROR_VHDSET_BACKING_STORAGE_NOT_FOUND: HRESULT = 0xC05CFF0Cu32 as HRESULT;

/// Failed to negotiate a preauthentication integrity hash function.
pub const ERROR_SMB_NO_PREAUTH_INTEGRITY_HASH_OVERLAP: HRESULT = 0xC05D0000u32 as HRESULT;

/// The current cluster functional level does not support this SMB dialect.
pub const ERROR_SMB_BAD_CLUSTER_DIALECT: HRESULT = 0xC05D0001u32 as HRESULT;

/// Failed to negotiate a signing hash function.
pub const ERROR_SMB_NO_SIGNING_ALGORITHM_OVERLAP: HRESULT = 0xC05D0002u32 as HRESULT;

/// No more Internet handles can be allocated
pub const WININET_E_OUT_OF_HANDLES: HRESULT = 0x80072EE1u32 as HRESULT;

/// The operation timed out
pub const WININET_E_TIMEOUT: HRESULT = 0x80072EE2u32 as HRESULT;

/// The server returned extended information
pub const WININET_E_EXTENDED_ERROR: HRESULT = 0x80072EE3u32 as HRESULT;

/// An internal error occurred in the Microsoft Internet extensions
pub const WININET_E_INTERNAL_ERROR: HRESULT = 0x80072EE4u32 as HRESULT;

/// The URL is invalid
pub const WININET_E_INVALID_URL: HRESULT = 0x80072EE5u32 as HRESULT;

/// The URL does not use a recognized protocol
pub const WININET_E_UNRECOGNIZED_SCHEME: HRESULT = 0x80072EE6u32 as HRESULT;

/// The server name or address could not be resolved
pub const WININET_E_NAME_NOT_RESOLVED: HRESULT = 0x80072EE7u32 as HRESULT;

/// A protocol with the required capabilities was not found
pub const WININET_E_PROTOCOL_NOT_FOUND: HRESULT = 0x80072EE8u32 as HRESULT;

/// The option is invalid
pub const WININET_E_INVALID_OPTION: HRESULT = 0x80072EE9u32 as HRESULT;

/// The length is incorrect for the option type
pub const WININET_E_BAD_OPTION_LENGTH: HRESULT = 0x80072EEAu32 as HRESULT;

/// The option value cannot be set
pub const WININET_E_OPTION_NOT_SETTABLE: HRESULT = 0x80072EEBu32 as HRESULT;

/// Microsoft Internet Extension support has been shut down
pub const WININET_E_SHUTDOWN: HRESULT = 0x80072EECu32 as HRESULT;

/// The user name was not allowed
pub const WININET_E_INCORRECT_USER_NAME: HRESULT = 0x80072EEDu32 as HRESULT;

/// The password was not allowed
pub const WININET_E_INCORRECT_PASSWORD: HRESULT = 0x80072EEEu32 as HRESULT;

/// The login request was denied
pub const WININET_E_LOGIN_FAILURE: HRESULT = 0x80072EEFu32 as HRESULT;

/// The requested operation is invalid
pub const WININET_E_INVALID_OPERATION: HRESULT = 0x80072EF0u32 as HRESULT;

/// The operation has been canceled
pub const WININET_E_OPERATION_CANCELLED: HRESULT = 0x80072EF1u32 as HRESULT;

/// The supplied handle is the wrong type for the requested operation
pub const WININET_E_INCORRECT_HANDLE_TYPE: HRESULT = 0x80072EF2u32 as HRESULT;

/// The handle is in the wrong state for the requested operation
pub const WININET_E_INCORRECT_HANDLE_STATE: HRESULT = 0x80072EF3u32 as HRESULT;

/// The request cannot be made on a Proxy session
pub const WININET_E_NOT_PROXY_REQUEST: HRESULT = 0x80072EF4u32 as HRESULT;

/// The registry value could not be found
pub const WININET_E_REGISTRY_VALUE_NOT_FOUND: HRESULT = 0x80072EF5u32 as HRESULT;

/// The registry parameter is incorrect
pub const WININET_E_BAD_REGISTRY_PARAMETER: HRESULT = 0x80072EF6u32 as HRESULT;

/// Direct Internet access is not available
pub const WININET_E_NO_DIRECT_ACCESS: HRESULT = 0x80072EF7u32 as HRESULT;

/// No context value was supplied
pub const WININET_E_NO_CONTEXT: HRESULT = 0x80072EF8u32 as HRESULT;

/// No status callback was supplied
pub const WININET_E_NO_CALLBACK: HRESULT = 0x80072EF9u32 as HRESULT;

/// There are outstanding requests
pub const WININET_E_REQUEST_PENDING: HRESULT = 0x80072EFAu32 as HRESULT;

/// The information format is incorrect
pub const WININET_E_INCORRECT_FORMAT: HRESULT = 0x80072EFBu32 as HRESULT;

/// The requested item could not be found
pub const WININET_E_ITEM_NOT_FOUND: HRESULT = 0x80072EFCu32 as HRESULT;

/// A connection with the server could not be established
pub const WININET_E_CANNOT_CONNECT: HRESULT = 0x80072EFDu32 as HRESULT;

/// The connection with the server was terminated abnormally
pub const WININET_E_CONNECTION_ABORTED: HRESULT = 0x80072EFEu32 as HRESULT;

/// The connection with the server was reset
pub const WININET_E_CONNECTION_RESET: HRESULT = 0x80072EFFu32 as HRESULT;

/// The action must be retried
pub const WININET_E_FORCE_RETRY: HRESULT = 0x80072F00u32 as HRESULT;

/// The proxy request is invalid
pub const WININET_E_INVALID_PROXY_REQUEST: HRESULT = 0x80072F01u32 as HRESULT;

/// User interaction is required to complete the operation
pub const WININET_E_NEED_UI: HRESULT = 0x80072F02u32 as HRESULT;

/// The handle already exists
pub const WININET_E_HANDLE_EXISTS: HRESULT = 0x80072F04u32 as HRESULT;

/// The date in the certificate is invalid or has expired
pub const WININET_E_SEC_CERT_DATE_INVALID: HRESULT = 0x80072F05u32 as HRESULT;

/// The host name in the certificate is invalid or does not match
pub const WININET_E_SEC_CERT_CN_INVALID: HRESULT = 0x80072F06u32 as HRESULT;

/// A redirect request will change a non-secure to a secure connection
pub const WININET_E_HTTP_TO_HTTPS_ON_REDIR: HRESULT = 0x80072F07u32 as HRESULT;

/// A redirect request will change a secure to a non-secure connection
pub const WININET_E_HTTPS_TO_HTTP_ON_REDIR: HRESULT = 0x80072F08u32 as HRESULT;

/// Mixed secure and non-secure connections
pub const WININET_E_MIXED_SECURITY: HRESULT = 0x80072F09u32 as HRESULT;

/// Changing to non-secure post
pub const WININET_E_CHG_POST_IS_NON_SECURE: HRESULT = 0x80072F0Au32 as HRESULT;

/// Data is being posted on a non-secure connection
pub const WININET_E_POST_IS_NON_SECURE: HRESULT = 0x80072F0Bu32 as HRESULT;

/// A certificate is required to complete client authentication
pub const WININET_E_CLIENT_AUTH_CERT_NEEDED: HRESULT = 0x80072F0Cu32 as HRESULT;

/// The certificate authority is invalid or incorrect
pub const WININET_E_INVALID_CA: HRESULT = 0x80072F0Du32 as HRESULT;

/// Client authentication has not been correctly installed
pub const WININET_E_CLIENT_AUTH_NOT_SETUP: HRESULT = 0x80072F0Eu32 as HRESULT;

/// An error has occurred in a Wininet asynchronous thread. You may need to restart
pub const WININET_E_ASYNC_THREAD_FAILED: HRESULT = 0x80072F0Fu32 as HRESULT;

/// The protocol scheme has changed during a redirect operation
pub const WININET_E_REDIRECT_SCHEME_CHANGE: HRESULT = 0x80072F10u32 as HRESULT;

/// There are operations awaiting retry
pub const WININET_E_DIALOG_PENDING: HRESULT = 0x80072F11u32 as HRESULT;

/// The operation must be retried
pub const WININET_E_RETRY_DIALOG: HRESULT = 0x80072F12u32 as HRESULT;

/// There are no new cache containers
pub const WININET_E_NO_NEW_CONTAINERS: HRESULT = 0x80072F13u32 as HRESULT;

/// A security zone check indicates the operation must be retried
pub const WININET_E_HTTPS_HTTP_SUBMIT_REDIR: HRESULT = 0x80072F14u32 as HRESULT;

/// The SSL certificate contains errors.
pub const WININET_E_SEC_CERT_ERRORS: HRESULT = 0x80072F17u32 as HRESULT;

/// It was not possible to connect to the revocation server or a definitive response could not be obtained.
pub const WININET_E_SEC_CERT_REV_FAILED: HRESULT = 0x80072F19u32 as HRESULT;

/// The requested header was not found
pub const WININET_E_HEADER_NOT_FOUND: HRESULT = 0x80072F76u32 as HRESULT;

/// The server does not support the requested protocol level
pub const WININET_E_DOWNLEVEL_SERVER: HRESULT = 0x80072F77u32 as HRESULT;

/// The server returned an invalid or unrecognized response
pub const WININET_E_INVALID_SERVER_RESPONSE: HRESULT = 0x80072F78u32 as HRESULT;

/// The supplied HTTP header is invalid
pub const WININET_E_INVALID_HEADER: HRESULT = 0x80072F79u32 as HRESULT;

/// The request for a HTTP header is invalid
pub const WININET_E_INVALID_QUERY_REQUEST: HRESULT = 0x80072F7Au32 as HRESULT;

/// The HTTP header already exists
pub const WININET_E_HEADER_ALREADY_EXISTS: HRESULT = 0x80072F7Bu32 as HRESULT;

/// The HTTP redirect request failed
pub const WININET_E_REDIRECT_FAILED: HRESULT = 0x80072F7Cu32 as HRESULT;

/// An error occurred in the secure channel support
pub const WININET_E_SECURITY_CHANNEL_ERROR: HRESULT = 0x80072F7Du32 as HRESULT;

/// The file could not be written to the cache
pub const WININET_E_UNABLE_TO_CACHE_FILE: HRESULT = 0x80072F7Eu32 as HRESULT;

/// The TCP/IP protocol is not installed properly
pub const WININET_E_TCPIP_NOT_INSTALLED: HRESULT = 0x80072F7Fu32 as HRESULT;

/// The computer is disconnected from the network
pub const WININET_E_DISCONNECTED: HRESULT = 0x80072F83u32 as HRESULT;

/// The server is unreachable
pub const WININET_E_SERVER_UNREACHABLE: HRESULT = 0x80072F84u32 as HRESULT;

/// The proxy server is unreachable
pub const WININET_E_PROXY_SERVER_UNREACHABLE: HRESULT = 0x80072F85u32 as HRESULT;

/// The proxy auto-configuration script is in error
pub const WININET_E_BAD_AUTO_PROXY_SCRIPT: HRESULT = 0x80072F86u32 as HRESULT;

/// Could not download the proxy auto-configuration script file
pub const WININET_E_UNABLE_TO_DOWNLOAD_SCRIPT: HRESULT = 0x80072F87u32 as HRESULT;

/// The supplied certificate is invalid
pub const WININET_E_SEC_INVALID_CERT: HRESULT = 0x80072F89u32 as HRESULT;

/// The supplied certificate has been revoked
pub const WININET_E_SEC_CERT_REVOKED: HRESULT = 0x80072F8Au32 as HRESULT;

/// The Dialup failed because file sharing was turned on and a failure was requested if security check was needed
pub const WININET_E_FAILED_DUETOSECURITYCHECK: HRESULT = 0x80072F8Bu32 as HRESULT;

/// Initialization of the WinINet API has not occurred
pub const WININET_E_NOT_INITIALIZED: HRESULT = 0x80072F8Cu32 as HRESULT;

/// Login failed and the client should display the entity body to the user
pub const WININET_E_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: HRESULT = 0x80072F8Eu32 as HRESULT;

/// Content decoding has failed
pub const WININET_E_DECODING_FAILED: HRESULT = 0x80072F8Fu32 as HRESULT;

/// The HTTP request was not redirected
pub const WININET_E_NOT_REDIRECTED: HRESULT = 0x80072F80u32 as HRESULT;

/// A cookie from the server must be confirmed by the user
pub const WININET_E_COOKIE_NEEDS_CONFIRMATION: HRESULT = 0x80072F81u32 as HRESULT;

/// A cookie from the server has been declined acceptance
pub const WININET_E_COOKIE_DECLINED: HRESULT = 0x80072F82u32 as HRESULT;

/// The HTTP redirect request must be confirmed by the user
pub const WININET_E_REDIRECT_NEEDS_CONFIRMATION: HRESULT = 0x80072F88u32 as HRESULT;

/// SQL error or missing database
pub const SQLITE_E_ERROR: HRESULT = 0x87AF0001u32 as HRESULT;

/// Internal logic error in SQLite
pub const SQLITE_E_INTERNAL: HRESULT = 0x87AF0002u32 as HRESULT;

/// Access permission denied
pub const SQLITE_E_PERM: HRESULT = 0x87AF0003u32 as HRESULT;

/// Callback routine requested an abort
pub const SQLITE_E_ABORT: HRESULT = 0x87AF0004u32 as HRESULT;

/// The database file is locked
pub const SQLITE_E_BUSY: HRESULT = 0x87AF0005u32 as HRESULT;

/// A table in the database is locked
pub const SQLITE_E_LOCKED: HRESULT = 0x87AF0006u32 as HRESULT;

/// A malloc() failed
pub const SQLITE_E_NOMEM: HRESULT = 0x87AF0007u32 as HRESULT;

/// Attempt to write a readonly database
pub const SQLITE_E_READONLY: HRESULT = 0x87AF0008u32 as HRESULT;

/// Operation terminated by sqlite3_interrupt()
pub const SQLITE_E_INTERRUPT: HRESULT = 0x87AF0009u32 as HRESULT;

/// Some kind of disk I/O error occurred
pub const SQLITE_E_IOERR: HRESULT = 0x87AF000Au32 as HRESULT;

/// The database disk image is malformed
pub const SQLITE_E_CORRUPT: HRESULT = 0x87AF000Bu32 as HRESULT;

/// Unknown opcode in sqlite3_file_control()
pub const SQLITE_E_NOTFOUND: HRESULT = 0x87AF000Cu32 as HRESULT;

/// Insertion failed because database is full
pub const SQLITE_E_FULL: HRESULT = 0x87AF000Du32 as HRESULT;

/// Unable to open the database file
pub const SQLITE_E_CANTOPEN: HRESULT = 0x87AF000Eu32 as HRESULT;

/// Database lock protocol error
pub const SQLITE_E_PROTOCOL: HRESULT = 0x87AF000Fu32 as HRESULT;

/// Database is empty
pub const SQLITE_E_EMPTY: HRESULT = 0x87AF0010u32 as HRESULT;

/// The database schema changed
pub const SQLITE_E_SCHEMA: HRESULT = 0x87AF0011u32 as HRESULT;

/// String or BLOB exceeds size limit
pub const SQLITE_E_TOOBIG: HRESULT = 0x87AF0012u32 as HRESULT;

/// Abort due to constraint violation
pub const SQLITE_E_CONSTRAINT: HRESULT = 0x87AF0013u32 as HRESULT;

/// Data type mismatch
pub const SQLITE_E_MISMATCH: HRESULT = 0x87AF0014u32 as HRESULT;

/// Library used incorrectly
pub const SQLITE_E_MISUSE: HRESULT = 0x87AF0015u32 as HRESULT;

/// Uses OS features not supported on host
pub const SQLITE_E_NOLFS: HRESULT = 0x87AF0016u32 as HRESULT;

/// Authorization denied
pub const SQLITE_E_AUTH: HRESULT = 0x87AF0017u32 as HRESULT;

/// Auxiliary database format error
pub const SQLITE_E_FORMAT: HRESULT = 0x87AF0018u32 as HRESULT;

/// 2nd parameter to sqlite3_bind out of range
pub const SQLITE_E_RANGE: HRESULT = 0x87AF0019u32 as HRESULT;

/// File opened that is not a database file
pub const SQLITE_E_NOTADB: HRESULT = 0x87AF001Au32 as HRESULT;

/// Notifications from sqlite3_log()
pub const SQLITE_E_NOTICE: HRESULT = 0x87AF001Bu32 as HRESULT;

/// Warnings from sqlite3_log()
pub const SQLITE_E_WARNING: HRESULT = 0x87AF001Cu32 as HRESULT;

/// sqlite3_step() has another row ready
pub const SQLITE_E_ROW: HRESULT = 0x87AF0064u32 as HRESULT;

/// sqlite3_step() has finished executing
pub const SQLITE_E_DONE: HRESULT = 0x87AF0065u32 as HRESULT;

/// SQLITE_IOERR_READ
pub const SQLITE_E_IOERR_READ: HRESULT = 0x87AF010Au32 as HRESULT;

/// SQLITE_IOERR_SHORT_READ
pub const SQLITE_E_IOERR_SHORT_READ: HRESULT = 0x87AF020Au32 as HRESULT;

/// SQLITE_IOERR_WRITE
pub const SQLITE_E_IOERR_WRITE: HRESULT = 0x87AF030Au32 as HRESULT;

/// SQLITE_IOERR_FSYNC
pub const SQLITE_E_IOERR_FSYNC: HRESULT = 0x87AF040Au32 as HRESULT;

/// SQLITE_IOERR_DIR_FSYNC
pub const SQLITE_E_IOERR_DIR_FSYNC: HRESULT = 0x87AF050Au32 as HRESULT;

/// SQLITE_IOERR_TRUNCATE
pub const SQLITE_E_IOERR_TRUNCATE: HRESULT = 0x87AF060Au32 as HRESULT;

/// SQLITE_IOERR_FSTAT
pub const SQLITE_E_IOERR_FSTAT: HRESULT = 0x87AF070Au32 as HRESULT;

/// SQLITE_IOERR_UNLOCK
pub const SQLITE_E_IOERR_UNLOCK: HRESULT = 0x87AF080Au32 as HRESULT;

/// SQLITE_IOERR_RDLOCK
pub const SQLITE_E_IOERR_RDLOCK: HRESULT = 0x87AF090Au32 as HRESULT;

/// SQLITE_IOERR_DELETE
pub const SQLITE_E_IOERR_DELETE: HRESULT = 0x87AF0A0Au32 as HRESULT;

/// SQLITE_IOERR_BLOCKED
pub const SQLITE_E_IOERR_BLOCKED: HRESULT = 0x87AF0B0Au32 as HRESULT;

/// SQLITE_IOERR_NOMEM
pub const SQLITE_E_IOERR_NOMEM: HRESULT = 0x87AF0C0Au32 as HRESULT;

/// SQLITE_IOERR_ACCESS
pub const SQLITE_E_IOERR_ACCESS: HRESULT = 0x87AF0D0Au32 as HRESULT;

/// SQLITE_IOERR_CHECKRESERVEDLOCK
pub const SQLITE_E_IOERR_CHECKRESERVEDLOCK: HRESULT = 0x87AF0E0Au32 as HRESULT;

/// SQLITE_IOERR_LOCK
pub const SQLITE_E_IOERR_LOCK: HRESULT = 0x87AF0F0Au32 as HRESULT;

/// SQLITE_IOERR_CLOSE
pub const SQLITE_E_IOERR_CLOSE: HRESULT = 0x87AF100Au32 as HRESULT;

/// SQLITE_IOERR_DIR_CLOSE
pub const SQLITE_E_IOERR_DIR_CLOSE: HRESULT = 0x87AF110Au32 as HRESULT;

/// SQLITE_IOERR_SHMOPEN
pub const SQLITE_E_IOERR_SHMOPEN: HRESULT = 0x87AF120Au32 as HRESULT;

/// SQLITE_IOERR_SHMSIZE
pub const SQLITE_E_IOERR_SHMSIZE: HRESULT = 0x87AF130Au32 as HRESULT;

/// SQLITE_IOERR_SHMLOCK
pub const SQLITE_E_IOERR_SHMLOCK: HRESULT = 0x87AF140Au32 as HRESULT;

/// SQLITE_IOERR_SHMMAP
pub const SQLITE_E_IOERR_SHMMAP: HRESULT = 0x87AF150Au32 as HRESULT;

/// SQLITE_IOERR_SEEK
pub const SQLITE_E_IOERR_SEEK: HRESULT = 0x87AF160Au32 as HRESULT;

/// SQLITE_IOERR_DELETE_NOENT
pub const SQLITE_E_IOERR_DELETE_NOENT: HRESULT = 0x87AF170Au32 as HRESULT;

/// SQLITE_IOERR_MMAP
pub const SQLITE_E_IOERR_MMAP: HRESULT = 0x87AF180Au32 as HRESULT;

/// SQLITE_IOERR_GETTEMPPATH
pub const SQLITE_E_IOERR_GETTEMPPATH: HRESULT = 0x87AF190Au32 as HRESULT;

/// SQLITE_IOERR_CONVPATH
pub const SQLITE_E_IOERR_CONVPATH: HRESULT = 0x87AF1A0Au32 as HRESULT;

/// SQLITE_IOERR_VNODE
pub const SQLITE_E_IOERR_VNODE: HRESULT = 0x87AF1A02u32 as HRESULT;

/// SQLITE_IOERR_AUTH
pub const SQLITE_E_IOERR_AUTH: HRESULT = 0x87AF1A03u32 as HRESULT;

/// SQLITE_LOCKED_SHAREDCACHE
pub const SQLITE_E_LOCKED_SHAREDCACHE: HRESULT = 0x87AF0106u32 as HRESULT;

/// SQLITE_BUSY_RECOVERY
pub const SQLITE_E_BUSY_RECOVERY: HRESULT = 0x87AF0105u32 as HRESULT;

/// SQLITE_BUSY_SNAPSHOT
pub const SQLITE_E_BUSY_SNAPSHOT: HRESULT = 0x87AF0205u32 as HRESULT;

/// SQLITE_CANTOPEN_NOTEMPDIR
pub const SQLITE_E_CANTOPEN_NOTEMPDIR: HRESULT = 0x87AF010Eu32 as HRESULT;

/// SQLITE_CANTOPEN_ISDIR
pub const SQLITE_E_CANTOPEN_ISDIR: HRESULT = 0x87AF020Eu32 as HRESULT;

/// SQLITE_CANTOPEN_FULLPATH
pub const SQLITE_E_CANTOPEN_FULLPATH: HRESULT = 0x87AF030Eu32 as HRESULT;

/// SQLITE_CANTOPEN_CONVPATH
pub const SQLITE_E_CANTOPEN_CONVPATH: HRESULT = 0x87AF040Eu32 as HRESULT;

/// SQLITE_CORRUPT_VTAB
pub const SQLITE_E_CORRUPT_VTAB: HRESULT = 0x87AF010Bu32 as HRESULT;

/// SQLITE_READONLY_RECOVERY
pub const SQLITE_E_READONLY_RECOVERY: HRESULT = 0x87AF0108u32 as HRESULT;

/// SQLITE_READONLY_CANTLOCK
pub const SQLITE_E_READONLY_CANTLOCK: HRESULT = 0x87AF0208u32 as HRESULT;

/// SQLITE_READONLY_ROLLBACK
pub const SQLITE_E_READONLY_ROLLBACK: HRESULT = 0x87AF0308u32 as HRESULT;

/// SQLITE_READONLY_DBMOVED
pub const SQLITE_E_READONLY_DBMOVED: HRESULT = 0x87AF0408u32 as HRESULT;

/// SQLITE_ABORT_ROLLBACK
pub const SQLITE_E_ABORT_ROLLBACK: HRESULT = 0x87AF0204u32 as HRESULT;

/// SQLITE_CONSTRAINT_CHECK
pub const SQLITE_E_CONSTRAINT_CHECK: HRESULT = 0x87AF0113u32 as HRESULT;

/// SQLITE_CONSTRAINT_COMMITHOOK
pub const SQLITE_E_CONSTRAINT_COMMITHOOK: HRESULT = 0x87AF0213u32 as HRESULT;

/// SQLITE_CONSTRAINT_FOREIGNKEY
pub const SQLITE_E_CONSTRAINT_FOREIGNKEY: HRESULT = 0x87AF0313u32 as HRESULT;

/// SQLITE_CONSTRAINT_FUNCTION
pub const SQLITE_E_CONSTRAINT_FUNCTION: HRESULT = 0x87AF0413u32 as HRESULT;

/// SQLITE_CONSTRAINT_NOTNULL
pub const SQLITE_E_CONSTRAINT_NOTNULL: HRESULT = 0x87AF0513u32 as HRESULT;

/// SQLITE_CONSTRAINT_PRIMARYKEY
pub const SQLITE_E_CONSTRAINT_PRIMARYKEY: HRESULT = 0x87AF0613u32 as HRESULT;

/// SQLITE_CONSTRAINT_TRIGGER
pub const SQLITE_E_CONSTRAINT_TRIGGER: HRESULT = 0x87AF0713u32 as HRESULT;

/// SQLITE_CONSTRAINT_UNIQUE
pub const SQLITE_E_CONSTRAINT_UNIQUE: HRESULT = 0x87AF0813u32 as HRESULT;

/// SQLITE_CONSTRAINT_VTAB
pub const SQLITE_E_CONSTRAINT_VTAB: HRESULT = 0x87AF0913u32 as HRESULT;

/// SQLITE_CONSTRAINT_ROWID
pub const SQLITE_E_CONSTRAINT_ROWID: HRESULT = 0x87AF0A13u32 as HRESULT;

/// SQLITE_NOTICE_RECOVER_WAL
pub const SQLITE_E_NOTICE_RECOVER_WAL: HRESULT = 0x87AF011Bu32 as HRESULT;

/// SQLITE_NOTICE_RECOVER_ROLLBACK
pub const SQLITE_E_NOTICE_RECOVER_ROLLBACK: HRESULT = 0x87AF021Bu32 as HRESULT;

/// SQLITE_WARNING_AUTOINDEX
pub const SQLITE_E_WARNING_AUTOINDEX: HRESULT = 0x87AF011Cu32 as HRESULT;

/// Toggle (alternative) trace started
pub const UTC_E_TOGGLE_TRACE_STARTED: HRESULT = 0x87C51001u32 as HRESULT;

/// Cannot pre-empt running trace: The current trace has a higher priority
pub const UTC_E_ALTERNATIVE_TRACE_CANNOT_PREEMPT: HRESULT = 0x87C51002u32 as HRESULT;

/// The always-on-trace is not running
pub const UTC_E_AOT_NOT_RUNNING: HRESULT = 0x87C51003u32 as HRESULT;

/// RunScriptAction contains an invalid script type
pub const UTC_E_SCRIPT_TYPE_INVALID: HRESULT = 0x87C51004u32 as HRESULT;

/// Requested scenario definition cannot be found
pub const UTC_E_SCENARIODEF_NOT_FOUND: HRESULT = 0x87C51005u32 as HRESULT;

/// Requested trace profile cannot be found
pub const UTC_E_TRACEPROFILE_NOT_FOUND: HRESULT = 0x87C51006u32 as HRESULT;

/// Trigger forwarder is already enabled
pub const UTC_E_FORWARDER_ALREADY_ENABLED: HRESULT = 0x87C51007u32 as HRESULT;

/// Trigger forwarder is already disabled
pub const UTC_E_FORWARDER_ALREADY_DISABLED: HRESULT = 0x87C51008u32 as HRESULT;

/// Cannot parse EventLog XML: The entry is malformed
pub const UTC_E_EVENTLOG_ENTRY_MALFORMED: HRESULT = 0x87C51009u32 as HRESULT;

/// `diagrules` node contains a schemaversion which is not compatible with this client
pub const UTC_E_DIAGRULES_SCHEMAVERSION_MISMATCH: HRESULT = 0x87C5100Au32 as HRESULT;

/// `RunScriptAction` was forced to terminate a script
pub const UTC_E_SCRIPT_TERMINATED: HRESULT = 0x87C5100Bu32 as HRESULT;

/// `ToggleTraceWithCustomFilterAction` contains an invalid custom filter
pub const UTC_E_INVALID_CUSTOM_FILTER: HRESULT = 0x87C5100Cu32 as HRESULT;

/// The trace is not running
pub const UTC_E_TRACE_NOT_RUNNING: HRESULT = 0x87C5100Du32 as HRESULT;

/// A scenario failed to escalate: This scenario has escalated too recently
pub const UTC_E_REESCALATED_TOO_QUICKLY: HRESULT = 0x87C5100Eu32 as HRESULT;

/// A scenario failed to escalate: This scenario is already running an escalation
pub const UTC_E_ESCALATION_ALREADY_RUNNING: HRESULT = 0x87C5100Fu32 as HRESULT;

/// Cannot start tracing: PerfTrack component is already tracing
pub const UTC_E_PERFTRACK_ALREADY_TRACING: HRESULT = 0x87C51010u32 as HRESULT;

/// A scenario failed to escalate: This scenario has reached max escalations for this escalation type
pub const UTC_E_REACHED_MAX_ESCALATIONS: HRESULT = 0x87C51011u32 as HRESULT;

/// Cannot update forwarder: The forwarder passed to the function is of a different type
pub const UTC_E_FORWARDER_PRODUCER_MISMATCH: HRESULT = 0x87C51012u32 as HRESULT;

/// RunScriptAction failed intentionally to force this escalation to terminate
pub const UTC_E_INTENTIONAL_SCRIPT_FAILURE: HRESULT = 0x87C51013u32 as HRESULT;

/// Failed to initialize SQM logger
pub const UTC_E_SQM_INIT_FAILED: HRESULT = 0x87C51014u32 as HRESULT;

/// Failed to initialize WER logger: This system does not support WER for UTC
pub const UTC_E_NO_WER_LOGGER_SUPPORTED: HRESULT = 0x87C51015u32 as HRESULT;

/// The TraceManager has attempted to take a tracing action without initializing tracers
pub const UTC_E_TRACERS_DONT_EXIST: HRESULT = 0x87C51016u32 as HRESULT;

/// WinRT initialization failed
pub const UTC_E_WINRT_INIT_FAILED: HRESULT = 0x87C51017u32 as HRESULT;

/// `scenario` node contains a schemaversion that is not compatible with this client
pub const UTC_E_SCENARIODEF_SCHEMAVERSION_MISMATCH: HRESULT = 0x87C51018u32 as HRESULT;

/// Scenario contains an invalid filter that can never be satisfied
pub const UTC_E_INVALID_FILTER: HRESULT = 0x87C51019u32 as HRESULT;

/// RunExeWithArgsAction was forced to terminate a running executable
pub const UTC_E_EXE_TERMINATED: HRESULT = 0x87C5101Au32 as HRESULT;

/// Escalation for scenario failed due to insufficient permissions
pub const UTC_E_ESCALATION_NOT_AUTHORIZED: HRESULT = 0x87C5101Bu32 as HRESULT;

/// Setup for scenario failed due to insufficient permissions
pub const UTC_E_SETUP_NOT_AUTHORIZED: HRESULT = 0x87C5101Cu32 as HRESULT;

/// A process launched by UTC failed with a non-zero exit code.
pub const UTC_E_CHILD_PROCESS_FAILED: HRESULT = 0x87C5101Du32 as HRESULT;

/// A RunExeWithArgs action contains an unauthorized command line.
pub const UTC_E_COMMAND_LINE_NOT_AUTHORIZED: HRESULT = 0x87C5101Eu32 as HRESULT;

/// UTC cannot load Scenario Editor XML. Convert the scenario file to a DiagTrack XML using the editor.
pub const UTC_E_CANNOT_LOAD_SCENARIO_EDITOR_XML: HRESULT = 0x87C5101Fu32 as HRESULT;

/// Escalation for scenario has timed out
pub const UTC_E_ESCALATION_TIMED_OUT: HRESULT = 0x87C51020u32 as HRESULT;

/// Setup for scenario has timed out
pub const UTC_E_SETUP_TIMED_OUT: HRESULT = 0x87C51021u32 as HRESULT;

/// The given trigger does not match the expected trigger type
pub const UTC_E_TRIGGER_MISMATCH: HRESULT = 0x87C51022u32 as HRESULT;

/// Requested trigger cannot be found
pub const UTC_E_TRIGGER_NOT_FOUND: HRESULT = 0x87C51023u32 as HRESULT;

/// SIF is not supported on the machine
pub const UTC_E_SIF_NOT_SUPPORTED: HRESULT = 0x87C51024u32 as HRESULT;

/// The delay action was terminated
pub const UTC_E_DELAY_TERMINATED: HRESULT = 0x87C51025u32 as HRESULT;

/// The device ticket was not obtained
pub const UTC_E_DEVICE_TICKET_ERROR: HRESULT = 0x87C51026u32 as HRESULT;

/// The trace profile needs more memory than is available for tracing
pub const UTC_E_TRACE_BUFFER_LIMIT_EXCEEDED: HRESULT = 0x87C51027u32 as HRESULT;

/// The API was not completed successfully so the result is unavailable
pub const UTC_E_API_RESULT_UNAVAILABLE: HRESULT = 0x87C51028u32 as HRESULT;

/// The requested API encountered a timeout in the API manager
pub const UTC_E_RPC_TIMEOUT: HRESULT = 0x87C51029u32 as HRESULT;

/// The synchronous API encountered a wait failure
pub const UTC_E_RPC_WAIT_FAILED: HRESULT = 0x87C5102Au32 as HRESULT;

/// The UTC API is busy with another request
pub const UTC_E_API_BUSY: HRESULT = 0x87C5102Bu32 as HRESULT;

/// The running trace profile does not have a sufficient runtime to fulfill the escalation request
pub const UTC_E_TRACE_MIN_DURATION_REQUIREMENT_NOT_MET: HRESULT = 0x87C5102Cu32 as HRESULT;

/// The trace profile could not be started because it requires exclusivity and another higher priority trace is already running
pub const UTC_E_EXCLUSIVITY_NOT_AVAILABLE: HRESULT = 0x87C5102Du32 as HRESULT;

/// The file path is not approved for the GetFile escalation action
pub const UTC_E_GETFILE_FILE_PATH_NOT_APPROVED: HRESULT = 0x87C5102Eu32 as HRESULT;

/// The escalation working directory for the requested escalation could not be created because it already exists
pub const UTC_E_ESCALATION_DIRECTORY_ALREADY_EXISTS: HRESULT = 0x87C5102Fu32 as HRESULT;

/// Time triggers cannot be used on a transition originating from the "_start" state
pub const UTC_E_TIME_TRIGGER_ON_START_INVALID: HRESULT = 0x87C51030u32 as HRESULT;

/// Time triggers can only be attached to a single transition
pub const UTC_E_TIME_TRIGGER_ONLY_VALID_ON_SINGLE_TRANSITION: HRESULT = 0x87C51031u32 as HRESULT;

/// Time trigger duration must fall within an inclusive range of one second and 15 minutes
pub const UTC_E_TIME_TRIGGER_INVALID_TIME_RANGE: HRESULT = 0x87C51032u32 as HRESULT;

/// Only one Time Trigger is allowed per state
pub const UTC_E_MULTIPLE_TIME_TRIGGER_ON_SINGLE_STATE: HRESULT = 0x87C51033u32 as HRESULT;

/// A RunExeWithArgs action contains a binary which is not present on the targeted device.
pub const UTC_E_BINARY_MISSING: HRESULT = 0x87C51034u32 as HRESULT;

/// UTC failed to identify the container id to use for a scenario escalation action.
pub const UTC_E_FAILED_TO_RESOLVE_CONTAINER_ID: HRESULT = 0x87C51036u32 as HRESULT;

/// Failed to resolve session ID during API invocation.
pub const UTC_E_UNABLE_TO_RESOLVE_SESSION: HRESULT = 0x87C51037u32 as HRESULT;

/// UTC has throttled the event for firing too often.
pub const UTC_E_THROTTLED: HRESULT = 0x87C51038u32 as HRESULT;

/// The script is not approved to run as part of DiagTrack scenario.
pub const UTC_E_UNAPPROVED_SCRIPT: HRESULT = 0x87C51039u32 as HRESULT;

/// The script referenced in DiagTrack scenario is not present on the system.
pub const UTC_E_SCRIPT_MISSING: HRESULT = 0x87C5103Au32 as HRESULT;

/// A trigger in this scenario is throttled, blocking the scenario from being loaded.
pub const UTC_E_SCENARIO_THROTTLED: HRESULT = 0x87C5103Bu32 as HRESULT;

/// The requested UTC API call is not supported on this device.
pub const UTC_E_API_NOT_SUPPORTED: HRESULT = 0x87C5103Cu32 as HRESULT;

/// The file path is not approved for collection on external rings for the GetFile escalation action.
pub const UTC_E_GETFILE_EXTERNAL_PATH_NOT_APPROVED: HRESULT = 0x87C5103Du32 as HRESULT;

/// Querying a scenario definition exceeded the specified maximum timeout.
pub const UTC_E_TRY_GET_SCENARIO_TIMEOUT_EXCEEDED: HRESULT = 0x87C5103Eu32 as HRESULT;

/// Certification revocation checking has been enabled, but the revocation check failed to verify whether a certificate has been revoked. The server used to check for revocation might be unreachable.
pub const UTC_E_CERT_REV_FAILED: HRESULT = 0x87C5103Fu32 as HRESULT;

/// Failed to start NDISCAP service for network packet capture trace.
pub const UTC_E_FAILED_TO_START_NDISCAP: HRESULT = 0x87C51040u32 as HRESULT;

/// UTC can perform no more than one KernelDump action on a device every 24 hours.
pub const UTC_E_KERNELDUMP_LIMIT_REACHED: HRESULT = 0x87C51041u32 as HRESULT;

/// The event contained an aggregation or differential privacy structure, but did not specify MICROSOFT_EVENTTAG_AGGREGATE.
pub const UTC_E_MISSING_AGGREGATE_EVENT_TAG: HRESULT = 0x87C51042u32 as HRESULT;

/// The event contained an invalid aggregation or differential privacy structure.
pub const UTC_E_INVALID_AGGREGATION_STRUCT: HRESULT = 0x87C51043u32 as HRESULT;

/// The action cannot be completed in the specified destination.
pub const UTC_E_ACTION_NOT_SUPPORTED_IN_DESTINATION: HRESULT = 0x87C51044u32 as HRESULT;

/// Filter command is missing a required attribute.
pub const UTC_E_FILTER_MISSING_ATTRIBUTE: HRESULT = 0x87C51045u32 as HRESULT;

/// Filter command contains an unsupported type.
pub const UTC_E_FILTER_INVALID_TYPE: HRESULT = 0x87C51046u32 as HRESULT;

/// Filter variable does not exist at point of evaluation.
pub const UTC_E_FILTER_VARIABLE_NOT_FOUND: HRESULT = 0x87C51047u32 as HRESULT;

/// Filter command is not allowed in the current context.
pub const UTC_E_FILTER_FUNCTION_RESTRICTED: HRESULT = 0x87C51048u32 as HRESULT;

/// Requested filter version is incompatible with available version.
pub const UTC_E_FILTER_VERSION_MISMATCH: HRESULT = 0x87C51049u32 as HRESULT;

/// Filter does not support this function.
pub const UTC_E_FILTER_INVALID_FUNCTION: HRESULT = 0x87C51050u32 as HRESULT;

/// Filter function does not accept the provided parameter types and/or count.
pub const UTC_E_FILTER_INVALID_FUNCTION_PARAMS: HRESULT = 0x87C51051u32 as HRESULT;

/// Filter command does not exist or is incorrectly formatted.
pub const UTC_E_FILTER_INVALID_COMMAND: HRESULT = 0x87C51052u32 as HRESULT;

/// Filter types can not be compared to each other.
pub const UTC_E_FILTER_ILLEGAL_EVAL: HRESULT = 0x87C51053u32 as HRESULT;

/// TTTracer executable returned a code other than ERROR_SUCCESS.
pub const UTC_E_TTTRACER_RETURNED_ERROR: HRESULT = 0x87C51054u32 as HRESULT;

/// The total size of the compressed escalation data payload exceeded the allowable limit.
pub const UTC_E_AGENT_DIAGNOSTICS_TOO_LARGE: HRESULT = 0x87C51055u32 as HRESULT;

/// Escalation data was not completely transferred from agent to host.
pub const UTC_E_FAILED_TO_RECEIVE_AGENT_DIAGNOSTICS: HRESULT = 0x87C51056u32 as HRESULT;

/// An escalation was requested for a scenario which has no actions for the passed type.
pub const UTC_E_SCENARIO_HAS_NO_ACTIONS: HRESULT = 0x87C51057u32 as HRESULT;

/// UTC allocated space for TTTracer escalations is full.
pub const UTC_E_TTTRACER_STORAGE_FULL: HRESULT = 0x87C51058u32 as HRESULT;

/// Disk needs minimum of 15GB to start TTD recording session.
pub const UTC_E_INSUFFICIENT_SPACE_TO_START_TRACE: HRESULT = 0x87C51059u32 as HRESULT;

/// Escalation was cancelled due to component shutdown.
pub const UTC_E_ESCALATION_CANCELLED_AT_SHUTDOWN: HRESULT = 0x87C5105Au32 as HRESULT;

/// The file for the GetFileInfo action must be under the \Windows, \Program Files, or \Program Files (x86) directories.
pub const UTC_E_GETFILEINFOACTION_FILE_NOT_APPROVED: HRESULT = 0x87C5105Bu32 as HRESULT;

/// The registry value type for SetRegKey action must be REG_SZ, REG_MULTI_SZ, REG_EXPAND_SZ, REG_BINARY, REG_DWORD, or REG_QWORD.
pub const UTC_E_SETREGKEYACTION_TYPE_NOT_APPROVED: HRESULT = 0x87C5105Cu32 as HRESULT;

/// An operation which requires a running un-throttled trace failed due to the trace being throttled.
pub const UTC_E_TRACE_THROTTLED: HRESULT = 0x87C5105Du32 as HRESULT;

/// The device is invalid or does not support machine learning.
pub const WINML_ERR_INVALID_DEVICE: HRESULT = 0x88900001u32 as HRESULT;

/// The binding is incomplete or does not match the input/output description.
pub const WINML_ERR_INVALID_BINDING: HRESULT = 0x88900002u32 as HRESULT;

/// An attempt was made to bind an unknown input or output.
pub const WINML_ERR_VALUE_NOTFOUND: HRESULT = 0x88900003u32 as HRESULT;

/// The size of the buffer provided for a bound variable is invalid.
pub const WINML_ERR_SIZE_MISMATCH: HRESULT = 0x88900004u32 as HRESULT;

/// The QUIC connection handshake failed.
pub const ERROR_QUIC_HANDSHAKE_FAILURE: HRESULT = 0x80410000u32 as HRESULT;

/// The QUIC connection failed to negotiate a compatible protocol version.
pub const ERROR_QUIC_VER_NEG_FAILURE: HRESULT = 0x80410001u32 as HRESULT;

/// The QUIC connection was canceled by the user.
pub const ERROR_QUIC_USER_CANCELED: HRESULT = 0x80410002u32 as HRESULT;

/// The QUIC connection encountered an internal error.
pub const ERROR_QUIC_INTERNAL_ERROR: HRESULT = 0x80410003u32 as HRESULT;

/// The QUIC connection encountered a protocol violation.
pub const ERROR_QUIC_PROTOCOL_VIOLATION: HRESULT = 0x80410004u32 as HRESULT;

/// The QUIC connection was idle.
pub const ERROR_QUIC_CONNECTION_IDLE: HRESULT = 0x80410005u32 as HRESULT;

/// The QUIC connection timed out while trying to contact the peer.
pub const ERROR_QUIC_CONNECTION_TIMEOUT: HRESULT = 0x80410006u32 as HRESULT;

/// The QUIC connection failed to negotiate a compatible ALPN.
pub const ERROR_QUIC_ALPN_NEG_FAILURE: HRESULT = 0x80410007u32 as HRESULT;

/// One or more of the required flags provided is unknown by the implementation.
pub const IORING_E_REQUIRED_FLAG_NOT_SUPPORTED: HRESULT = 0x80460001u32 as HRESULT;

/// The submission queue is full.
pub const IORING_E_SUBMISSION_QUEUE_FULL: HRESULT = 0x80460002u32 as HRESULT;

/// The version specified is not known or supported.
pub const IORING_E_VERSION_NOT_SUPPORTED: HRESULT = 0x80460003u32 as HRESULT;

/// The submission queue size specified for the IoRing is too big.
pub const IORING_E_SUBMISSION_QUEUE_TOO_BIG: HRESULT = 0x80460004u32 as HRESULT;

/// The completion queue size specified for the IoRing is too big.
pub const IORING_E_COMPLETION_QUEUE_TOO_BIG: HRESULT = 0x80460005u32 as HRESULT;

/// A submit operation is already in progress for this IoRing on another thread.
pub const IORING_E_SUBMIT_IN_PROGRESS: HRESULT = 0x80460006u32 as HRESULT;

/// The shared ring buffers of the IoRing are corrupt.
pub const IORING_E_CORRUPT: HRESULT = 0x80460007u32 as HRESULT;

/// The completion queue does not have enough free space, to post completions, for all entries being submitted.
pub const IORING_E_COMPLETION_QUEUE_TOO_FULL: HRESULT = 0x80460008u32 as HRESULT;
