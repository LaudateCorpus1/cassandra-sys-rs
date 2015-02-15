extern crate libc;

/* automatically generated by rust-bindgen */

pub type ptrdiff_t = ::libc::c_long;
pub type size_t = ::libc::c_ulong;
pub type wchar_t = ::libc::c_int;
pub type Enum_Unnamed1 = ::libc::c_uint;
pub const cass_false: ::libc::c_uint = 0;
pub const cass_true: ::libc::c_uint = 1;
pub type cass_bool_t = Enum_Unnamed1;
pub type cass_float_t = ::libc::c_float;
pub type cass_double_t = ::libc::c_double;
pub type cass_int8_t = ::libc::c_char;
pub type cass_uint8_t = ::libc::c_uchar;
pub type cass_int16_t = ::libc::c_short;
pub type cass_uint16_t = ::libc::c_ushort;
pub type cass_int32_t = ::libc::c_int;
pub type cass_uint32_t = ::libc::c_uint;
pub type cass_int64_t = ::libc::c_long;
pub type cass_uint64_t = ::libc::c_ulong;
pub type cass_size_t = size_t;
pub type cass_byte_t = cass_uint8_t;
pub type cass_duration_t = cass_uint64_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CassBytes_ {
    pub data: *const cass_byte_t,
    pub size: cass_size_t,
}
impl ::std::default::Default for Struct_CassBytes_ {
    fn default() -> Struct_CassBytes_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassBytes = Struct_CassBytes_;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CassString_ {
    pub data: *const ::libc::c_char,
    pub length: cass_size_t,
}
impl ::std::default::Default for Struct_CassString_ {
    fn default() -> Struct_CassString_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassString = Struct_CassString_;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CassInet_ {
    pub address: [cass_uint8_t; 16us],
    pub address_length: cass_uint8_t,
}
impl ::std::default::Default for Struct_CassInet_ {
    fn default() -> Struct_CassInet_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassInet = Struct_CassInet_;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CassDecimal_ {
    pub scale: cass_int32_t,
    pub varint: CassBytes,
}
impl ::std::default::Default for Struct_CassDecimal_ {
    fn default() -> Struct_CassDecimal_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassDecimal = Struct_CassDecimal_;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CassUuid_ {
    pub time_and_version: cass_uint64_t,
    pub clock_seq_and_node: cass_uint64_t,
}
impl ::std::default::Default for Struct_CassUuid_ {
    fn default() -> Struct_CassUuid_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassUuid = Struct_CassUuid_;
pub enum Struct_CassCluster_ { }
pub type CassCluster = Struct_CassCluster_;
pub enum Struct_CassSession_ { }
pub type CassSession = Struct_CassSession_;
pub enum Struct_CassStatement_ { }
pub type CassStatement = Struct_CassStatement_;
pub enum Struct_CassBatch_ { }
pub type CassBatch = Struct_CassBatch_;
pub enum Struct_CassFuture_ { }
pub type CassFuture = Struct_CassFuture_;
pub enum Struct_CassPrepared_ { }
pub type CassPrepared = Struct_CassPrepared_;
pub enum Struct_CassResult_ { }
pub type CassResult = Struct_CassResult_;
pub enum Struct_CassIterator_ { }
pub type CassIterator = Struct_CassIterator_;
pub enum Struct_CassRow_ { }
pub type CassRow = Struct_CassRow_;
pub enum Struct_CassValue_ { }
pub type CassValue = Struct_CassValue_;
pub enum Struct_CassCollection_ { }
pub type CassCollection = Struct_CassCollection_;
pub enum Struct_CassSsl_ { }
pub type CassSsl = Struct_CassSsl_;
pub enum Struct_CassSchema_ { }
pub type CassSchema = Struct_CassSchema_;
pub enum Struct_CassSchemaMeta_ { }
pub type CassSchemaMeta = Struct_CassSchemaMeta_;
pub enum Struct_CassSchemaMetaField_ { }
pub type CassSchemaMetaField = Struct_CassSchemaMetaField_;
pub enum Struct_CassUuidGen_ { }
pub type CassUuidGen = Struct_CassUuidGen_;
pub type Enum_CassConsistency_ = ::libc::c_uint;
pub const CASS_CONSISTENCY_ANY: ::libc::c_uint = 0;
pub const CASS_CONSISTENCY_ONE: ::libc::c_uint = 1;
pub const CASS_CONSISTENCY_TWO: ::libc::c_uint = 2;
pub const CASS_CONSISTENCY_THREE: ::libc::c_uint = 3;
pub const CASS_CONSISTENCY_QUORUM: ::libc::c_uint = 4;
pub const CASS_CONSISTENCY_ALL: ::libc::c_uint = 5;
pub const CASS_CONSISTENCY_LOCAL_QUORUM: ::libc::c_uint = 6;
pub const CASS_CONSISTENCY_EACH_QUORUM: ::libc::c_uint = 7;
pub const CASS_CONSISTENCY_SERIAL: ::libc::c_uint = 8;
pub const CASS_CONSISTENCY_LOCAL_SERIAL: ::libc::c_uint = 9;
pub const CASS_CONSISTENCY_LOCAL_ONE: ::libc::c_uint = 10;
pub type CassConsistency = Enum_CassConsistency_;
pub type Enum_CassValueType_ = ::libc::c_uint;
pub const CASS_VALUE_TYPE_UNKNOWN: ::libc::c_uint = 65535;
pub const CASS_VALUE_TYPE_CUSTOM: ::libc::c_uint = 0;
pub const CASS_VALUE_TYPE_ASCII: ::libc::c_uint = 1;
pub const CASS_VALUE_TYPE_BIGINT: ::libc::c_uint = 2;
pub const CASS_VALUE_TYPE_BLOB: ::libc::c_uint = 3;
pub const CASS_VALUE_TYPE_BOOLEAN: ::libc::c_uint = 4;
pub const CASS_VALUE_TYPE_COUNTER: ::libc::c_uint = 5;
pub const CASS_VALUE_TYPE_DECIMAL: ::libc::c_uint = 6;
pub const CASS_VALUE_TYPE_DOUBLE: ::libc::c_uint = 7;
pub const CASS_VALUE_TYPE_FLOAT: ::libc::c_uint = 8;
pub const CASS_VALUE_TYPE_INT: ::libc::c_uint = 9;
pub const CASS_VALUE_TYPE_TEXT: ::libc::c_uint = 10;
pub const CASS_VALUE_TYPE_TIMESTAMP: ::libc::c_uint = 11;
pub const CASS_VALUE_TYPE_UUID: ::libc::c_uint = 12;
pub const CASS_VALUE_TYPE_VARCHAR: ::libc::c_uint = 13;
pub const CASS_VALUE_TYPE_VARINT: ::libc::c_uint = 14;
pub const CASS_VALUE_TYPE_TIMEUUID: ::libc::c_uint = 15;
pub const CASS_VALUE_TYPE_INET: ::libc::c_uint = 16;
pub const CASS_VALUE_TYPE_LIST: ::libc::c_uint = 32;
pub const CASS_VALUE_TYPE_MAP: ::libc::c_uint = 33;
pub const CASS_VALUE_TYPE_SET: ::libc::c_uint = 34;
pub type CassValueType = Enum_CassValueType_;
pub type Enum_CassCollectionType_ = ::libc::c_uint;
pub const CASS_COLLECTION_TYPE_LIST: ::libc::c_uint = 32;
pub const CASS_COLLECTION_TYPE_MAP: ::libc::c_uint = 33;
pub const CASS_COLLECTION_TYPE_SET: ::libc::c_uint = 34;
pub type CassCollectionType = Enum_CassCollectionType_;
pub type Enum_CassBatchType_ = ::libc::c_uint;
pub const CASS_BATCH_TYPE_LOGGED: ::libc::c_uint = 0;
pub const CASS_BATCH_TYPE_UNLOGGED: ::libc::c_uint = 1;
pub const CASS_BATCH_TYPE_COUNTER: ::libc::c_uint = 2;
pub type CassBatchType = Enum_CassBatchType_;
pub type Enum_CassIteratorType_ = ::libc::c_uint;
pub const CASS_ITERATOR_TYPE_RESULT: ::libc::c_uint = 0;
pub const CASS_ITERATOR_TYPE_ROW: ::libc::c_uint = 1;
pub const CASS_ITERATOR_TYPE_COLLECTION: ::libc::c_uint = 2;
pub const CASS_ITERATOR_TYPE_MAP: ::libc::c_uint = 3;
pub const CASS_ITERATOR_TYPE_SCHEMA_META: ::libc::c_uint = 4;
pub const CASS_ITERATOR_TYPE_SCHEMA_META_FIELD: ::libc::c_uint = 5;
pub type CassIteratorType = Enum_CassIteratorType_;
pub type Enum_CassSchemaMetaType_ = ::libc::c_uint;
pub const CASS_SCHEMA_META_TYPE_KEYSPACE: ::libc::c_uint = 0;
pub const CASS_SCHEMA_META_TYPE_TABLE: ::libc::c_uint = 1;
pub const CASS_SCHEMA_META_TYPE_COLUMN: ::libc::c_uint = 2;
pub type CassSchemaMetaType = Enum_CassSchemaMetaType_;
pub type Enum_CassLogLevel_ = ::libc::c_uint;
pub const CASS_LOG_DISABLED: ::libc::c_uint = 0;
pub const CASS_LOG_CRITICAL: ::libc::c_uint = 1;
pub const CASS_LOG_ERROR: ::libc::c_uint = 2;
pub const CASS_LOG_WARN: ::libc::c_uint = 3;
pub const CASS_LOG_INFO: ::libc::c_uint = 4;
pub const CASS_LOG_DEBUG: ::libc::c_uint = 5;
pub const CASS_LOG_TRACE: ::libc::c_uint = 6;
pub const CASS_LOG_LAST_ENTRY: ::libc::c_uint = 7;
pub type CassLogLevel = Enum_CassLogLevel_;
pub type Enum_CassSslVerifyFlags = ::libc::c_uint;
pub const CASS_SSL_VERIFY_NONE: ::libc::c_uint = 0;
pub const CASS_SSL_VERIFY_PEER_CERT: ::libc::c_uint = 1;
pub const CASS_SSL_VERIFY_PEER_IDENTITY: ::libc::c_uint = 2;
pub type CassSslVerifyFlags = Enum_CassSslVerifyFlags;
pub type Enum_CassErrorSource_ = ::libc::c_uint;
pub const CASS_ERROR_SOURCE_NONE: ::libc::c_uint = 0;
pub const CASS_ERROR_SOURCE_LIB: ::libc::c_uint = 1;
pub const CASS_ERROR_SOURCE_SERVER: ::libc::c_uint = 2;
pub const CASS_ERROR_SOURCE_SSL: ::libc::c_uint = 3;
pub const CASS_ERROR_SOURCE_COMPRESSION: ::libc::c_uint = 4;
pub type CassErrorSource = Enum_CassErrorSource_;
pub type Enum_CassError_ = ::libc::c_uint;
pub const CASS_OK: ::libc::c_uint = 0;
pub const CASS_ERROR_LIB_BAD_PARAMS: ::libc::c_uint = 16777217;
pub const CASS_ERROR_LIB_NO_STREAMS: ::libc::c_uint = 16777218;
pub const CASS_ERROR_LIB_UNABLE_TO_INIT: ::libc::c_uint = 16777219;
pub const CASS_ERROR_LIB_MESSAGE_ENCODE: ::libc::c_uint = 16777220;
pub const CASS_ERROR_LIB_HOST_RESOLUTION: ::libc::c_uint = 16777221;
pub const CASS_ERROR_LIB_UNEXPECTED_RESPONSE: ::libc::c_uint = 16777222;
pub const CASS_ERROR_LIB_REQUEST_QUEUE_FULL: ::libc::c_uint = 16777223;
pub const CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD: ::libc::c_uint = 16777224;
pub const CASS_ERROR_LIB_WRITE_ERROR: ::libc::c_uint = 16777225;
pub const CASS_ERROR_LIB_NO_HOSTS_AVAILABLE: ::libc::c_uint = 16777226;
pub const CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS: ::libc::c_uint = 16777227;
pub const CASS_ERROR_LIB_INVALID_ITEM_COUNT: ::libc::c_uint = 16777228;
pub const CASS_ERROR_LIB_INVALID_VALUE_TYPE: ::libc::c_uint = 16777229;
pub const CASS_ERROR_LIB_REQUEST_TIMED_OUT: ::libc::c_uint = 16777230;
pub const CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE: ::libc::c_uint = 16777231;
pub const CASS_ERROR_LIB_CALLBACK_ALREADY_SET: ::libc::c_uint = 16777232;
pub const CASS_ERROR_LIB_INVALID_STATEMENT_TYPE: ::libc::c_uint = 16777233;
pub const CASS_ERROR_LIB_NAME_DOES_NOT_EXIST: ::libc::c_uint = 16777234;
pub const CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL: ::libc::c_uint =
    16777235;
pub const CASS_ERROR_LIB_NULL_VALUE: ::libc::c_uint = 16777236;
pub const CASS_ERROR_LIB_NOT_IMPLEMENTED: ::libc::c_uint = 16777237;
pub const CASS_ERROR_LIB_UNABLE_TO_CONNECT: ::libc::c_uint = 16777238;
pub const CASS_ERROR_LIB_UNABLE_TO_CLOSE: ::libc::c_uint = 16777239;
pub const CASS_ERROR_SERVER_SERVER_ERROR: ::libc::c_uint = 33554432;
pub const CASS_ERROR_SERVER_PROTOCOL_ERROR: ::libc::c_uint = 33554442;
pub const CASS_ERROR_SERVER_BAD_CREDENTIALS: ::libc::c_uint = 33554688;
pub const CASS_ERROR_SERVER_UNAVAILABLE: ::libc::c_uint = 33558528;
pub const CASS_ERROR_SERVER_OVERLOADED: ::libc::c_uint = 33558529;
pub const CASS_ERROR_SERVER_IS_BOOTSTRAPPING: ::libc::c_uint = 33558530;
pub const CASS_ERROR_SERVER_TRUNCATE_ERROR: ::libc::c_uint = 33558531;
pub const CASS_ERROR_SERVER_WRITE_TIMEOUT: ::libc::c_uint = 33558784;
pub const CASS_ERROR_SERVER_READ_TIMEOUT: ::libc::c_uint = 33559040;
pub const CASS_ERROR_SERVER_SYNTAX_ERROR: ::libc::c_uint = 33562624;
pub const CASS_ERROR_SERVER_UNAUTHORIZED: ::libc::c_uint = 33562880;
pub const CASS_ERROR_SERVER_INVALID_QUERY: ::libc::c_uint = 33563136;
pub const CASS_ERROR_SERVER_CONFIG_ERROR: ::libc::c_uint = 33563392;
pub const CASS_ERROR_SERVER_ALREADY_EXISTS: ::libc::c_uint = 33563648;
pub const CASS_ERROR_SERVER_UNPREPARED: ::libc::c_uint = 33563904;
pub const CASS_ERROR_SSL_INVALID_CERT: ::libc::c_uint = 50331649;
pub const CASS_ERROR_SSL_INVALID_PRIVATE_KEY: ::libc::c_uint = 50331650;
pub const CASS_ERROR_SSL_NO_PEER_CERT: ::libc::c_uint = 50331651;
pub const CASS_ERROR_SSL_INVALID_PEER_CERT: ::libc::c_uint = 50331652;
pub const CASS_ERROR_SSL_IDENTITY_MISMATCH: ::libc::c_uint = 50331653;
pub const CASS_ERROR_LAST_ENTRY: ::libc::c_uint = 50331654;
pub type CassError = Enum_CassError_;
pub type CassFutureCallback =
    ::std::option::Option<extern "C" fn
                              (future: *mut CassFuture,
                               data: *mut ::libc::c_void) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_CassLogMessage_ {
    pub time_ms: cass_uint64_t,
    pub severity: CassLogLevel,
    pub file: *const ::libc::c_char,
    pub line: ::libc::c_int,
    pub function: *const ::libc::c_char,
    pub message: [::libc::c_char; 256us],
}
impl ::std::default::Default for Struct_CassLogMessage_ {
    fn default() -> Struct_CassLogMessage_ { unsafe { ::std::mem::zeroed() } }
}
pub type CassLogMessage = Struct_CassLogMessage_;
pub type CassLogCallback =
    ::std::option::Option<extern "C" fn
                              (message: *const CassLogMessage,
                               data: *mut ::libc::c_void) -> ()>;
#[link(name = "cassandra")]
extern "C" {
    pub fn cass_cluster_new() -> *mut CassCluster;
    pub fn cass_cluster_free(cluster: *mut CassCluster) -> ();
    pub fn cass_cluster_set_contact_points(cluster: *mut CassCluster,
                                           contact_points:
                                               *const ::libc::c_char)
     -> CassError;
    pub fn cass_cluster_set_port(cluster: *mut CassCluster,
                                 port: ::libc::c_int) -> CassError;
    pub fn cass_cluster_set_ssl(cluster: *mut CassCluster, ssl: *mut CassSsl)
     -> ();
    pub fn cass_cluster_set_protocol_version(cluster: *mut CassCluster,
                                             protocol_version: ::libc::c_int)
     -> CassError;
    pub fn cass_cluster_set_num_threads_io(cluster: *mut CassCluster,
                                           num_threads: ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_queue_size_io(cluster: *mut CassCluster,
                                          queue_size: ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_queue_size_event(cluster: *mut CassCluster,
                                             queue_size: ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_queue_size_log(cluster: *mut CassCluster,
                                           queue_size: ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_core_connections_per_host(cluster:
                                                          *mut CassCluster,
                                                      num_connections:
                                                          ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_max_connections_per_host(cluster:
                                                         *mut CassCluster,
                                                     num_connections:
                                                         ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_reconnect_wait_time(cluster: *mut CassCluster,
                                                wait_time: ::libc::c_uint)
     -> ();
    pub fn cass_cluster_set_max_concurrent_creation(cluster: *mut CassCluster,
                                                    num_connections:
                                                        ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_max_concurrent_requests_threshold(cluster:
                                                                  *mut CassCluster,
                                                              num_requests:
                                                                  ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_max_requests_per_flush(cluster: *mut CassCluster,
                                                   num_requests:
                                                       ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_write_bytes_high_water_mark(cluster:
                                                            *mut CassCluster,
                                                        num_bytes:
                                                            ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_write_bytes_low_water_mark(cluster:
                                                           *mut CassCluster,
                                                       num_bytes:
                                                           ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_pending_requests_high_water_mark(cluster:
                                                                 *mut CassCluster,
                                                             num_requests:
                                                                 ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_pending_requests_low_water_mark(cluster:
                                                                *mut CassCluster,
                                                            num_requests:
                                                                ::libc::c_uint)
     -> CassError;
    pub fn cass_cluster_set_connect_timeout(cluster: *mut CassCluster,
                                            timeout_ms: ::libc::c_uint) -> ();
    pub fn cass_cluster_set_request_timeout(cluster: *mut CassCluster,
                                            timeout_ms: ::libc::c_uint) -> ();
    pub fn cass_cluster_set_credentials(cluster: *mut CassCluster,
                                        username: *const ::libc::c_char,
                                        password: *const ::libc::c_char)
     -> ();
    pub fn cass_cluster_set_load_balance_round_robin(cluster:
                                                         *mut CassCluster)
     -> ();
    pub fn cass_cluster_set_load_balance_dc_aware(cluster: *mut CassCluster,
                                                  local_dc:
                                                      *const ::libc::c_char,
                                                  used_hosts_per_remote_dc:
                                                      ::libc::c_uint,
                                                  allow_remote_dcs_for_local_cl:
                                                      cass_bool_t)
     -> CassError;
    pub fn cass_cluster_set_token_aware_routing(cluster: *mut CassCluster,
                                                enabled: cass_bool_t) -> ();
    pub fn cass_cluster_set_tcp_nodelay(cluster: *mut CassCluster,
                                        enabled: cass_bool_t) -> ();
    pub fn cass_cluster_set_tcp_keepalive(cluster: *mut CassCluster,
                                          enabled: cass_bool_t,
                                          delay_secs: ::libc::c_uint) -> ();
    pub fn cass_session_new() -> *mut CassSession;
    pub fn cass_session_free(session: *mut CassSession) -> ();
    pub fn cass_session_connect(session: *mut CassSession,
                                cluster: *const CassCluster)
     -> *mut CassFuture;
    pub fn cass_session_connect_keyspace(session: *mut CassSession,
                                         cluster: *const CassCluster,
                                         keyspace: *const ::libc::c_char)
     -> *mut CassFuture;
    pub fn cass_session_close(session: *mut CassSession) -> *mut CassFuture;
    pub fn cass_session_prepare(session: *mut CassSession, query: CassString)
     -> *mut CassFuture;
    pub fn cass_session_execute(session: *mut CassSession,
                                statement: *const CassStatement)
     -> *mut CassFuture;
    pub fn cass_session_execute_batch(session: *mut CassSession,
                                      batch: *const CassBatch)
     -> *mut CassFuture;
    pub fn cass_session_get_schema(session: *mut CassSession)
     -> *const CassSchema;
    pub fn cass_schema_free(schema: *const CassSchema) -> ();
    pub fn cass_schema_get_keyspace(schema: *const CassSchema,
                                    keyspace_name: *const ::libc::c_char)
     -> *const CassSchemaMeta;
    pub fn cass_schema_meta_type(meta: *const CassSchemaMeta)
     -> CassSchemaMetaType;
    pub fn cass_schema_meta_get_entry(meta: *const CassSchemaMeta,
                                      name: *const ::libc::c_char)
     -> *const CassSchemaMeta;
    pub fn cass_schema_meta_get_field(meta: *const CassSchemaMeta,
                                      name: *const ::libc::c_char)
     -> *const CassSchemaMetaField;
    pub fn cass_schema_meta_field_name(field: *const CassSchemaMetaField)
     -> CassString;
    pub fn cass_schema_meta_field_value(field: *const CassSchemaMetaField)
     -> *const CassValue;
    pub fn cass_ssl_new() -> *mut CassSsl;
    pub fn cass_ssl_free(ssl: *mut CassSsl) -> ();
    pub fn cass_ssl_add_trusted_cert(ssl: *mut CassSsl, cert: CassString)
     -> CassError;
    pub fn cass_ssl_set_verify_flags(ssl: *mut CassSsl, flags: ::libc::c_int)
     -> ();
    pub fn cass_ssl_set_cert(ssl: *mut CassSsl, cert: CassString)
     -> CassError;
    pub fn cass_ssl_set_private_key(ssl: *mut CassSsl, key: CassString,
                                    password: *const ::libc::c_char)
     -> CassError;
    pub fn cass_future_free(future: *mut CassFuture) -> ();
    pub fn cass_future_set_callback(future: *mut CassFuture,
                                    callback: CassFutureCallback,
                                    data: *mut ::libc::c_void) -> CassError;
    pub fn cass_future_ready(future: *mut CassFuture) -> cass_bool_t;
    pub fn cass_future_wait(future: *mut CassFuture) -> ();
    pub fn cass_future_wait_timed(future: *mut CassFuture,
                                  timeout_us: cass_duration_t) -> cass_bool_t;
    pub fn cass_future_get_result(future: *mut CassFuture)
     -> *const CassResult;
    pub fn cass_future_get_prepared(future: *mut CassFuture)
     -> *const CassPrepared;
    pub fn cass_future_error_code(future: *mut CassFuture) -> CassError;
    pub fn cass_future_error_message(future: *mut CassFuture) -> CassString;
    pub fn cass_statement_new(query: CassString, parameter_count: cass_size_t)
     -> *mut CassStatement;
    pub fn cass_statement_free(statement: *mut CassStatement) -> ();
    pub fn cass_statement_add_key_index(statement: *mut CassStatement,
                                        index: cass_size_t) -> CassError;
    pub fn cass_statement_set_keyspace(statement: *mut CassStatement,
                                       keyspace: *const ::libc::c_char)
     -> CassError;
    pub fn cass_statement_set_consistency(statement: *mut CassStatement,
                                          consistency: CassConsistency)
     -> CassError;
    pub fn cass_statement_set_serial_consistency(statement:
                                                     *mut CassStatement,
                                                 serial_consistency:
                                                     CassConsistency)
     -> CassError;
    pub fn cass_statement_set_paging_size(statement: *mut CassStatement,
                                          page_size: ::libc::c_int)
     -> CassError;
    pub fn cass_statement_set_paging_state(statement: *mut CassStatement,
                                           result: *const CassResult)
     -> CassError;
    pub fn cass_statement_bind_null(statement: *mut CassStatement,
                                    index: cass_size_t) -> CassError;
    pub fn cass_statement_bind_int32(statement: *mut CassStatement,
                                     index: cass_size_t, value: cass_int32_t)
     -> CassError;
    pub fn cass_statement_bind_int64(statement: *mut CassStatement,
                                     index: cass_size_t, value: cass_int64_t)
     -> CassError;
    pub fn cass_statement_bind_float(statement: *mut CassStatement,
                                     index: cass_size_t, value: cass_float_t)
     -> CassError;
    pub fn cass_statement_bind_double(statement: *mut CassStatement,
                                      index: cass_size_t,
                                      value: cass_double_t) -> CassError;
    pub fn cass_statement_bind_bool(statement: *mut CassStatement,
                                    index: cass_size_t, value: cass_bool_t)
     -> CassError;
    pub fn cass_statement_bind_string(statement: *mut CassStatement,
                                      index: cass_size_t, value: CassString)
     -> CassError;
    pub fn cass_statement_bind_bytes(statement: *mut CassStatement,
                                     index: cass_size_t, value: CassBytes)
     -> CassError;
    pub fn cass_statement_bind_uuid(statement: *mut CassStatement,
                                    index: cass_size_t, value: CassUuid)
     -> CassError;
    pub fn cass_statement_bind_inet(statement: *mut CassStatement,
                                    index: cass_size_t, value: CassInet)
     -> CassError;
    pub fn cass_statement_bind_decimal(statement: *mut CassStatement,
                                       index: cass_size_t, value: CassDecimal)
     -> CassError;
    pub fn cass_statement_bind_custom(statement: *mut CassStatement,
                                      index: cass_size_t, size: cass_size_t,
                                      output: *mut *mut cass_byte_t)
     -> CassError;
    pub fn cass_statement_bind_collection(statement: *mut CassStatement,
                                          index: cass_size_t,
                                          collection: *const CassCollection)
     -> CassError;
    pub fn cass_statement_bind_int32_by_name(statement: *mut CassStatement,
                                             name: *const ::libc::c_char,
                                             value: cass_int32_t)
     -> CassError;
    pub fn cass_statement_bind_int64_by_name(statement: *mut CassStatement,
                                             name: *const ::libc::c_char,
                                             value: cass_int64_t)
     -> CassError;
    pub fn cass_statement_bind_float_by_name(statement: *mut CassStatement,
                                             name: *const ::libc::c_char,
                                             value: cass_float_t)
     -> CassError;
    pub fn cass_statement_bind_double_by_name(statement: *mut CassStatement,
                                              name: *const ::libc::c_char,
                                              value: cass_double_t)
     -> CassError;
    pub fn cass_statement_bind_bool_by_name(statement: *mut CassStatement,
                                            name: *const ::libc::c_char,
                                            value: cass_bool_t) -> CassError;
    pub fn cass_statement_bind_string_by_name(statement: *mut CassStatement,
                                              name: *const ::libc::c_char,
                                              value: CassString) -> CassError;
    pub fn cass_statement_bind_bytes_by_name(statement: *mut CassStatement,
                                             name: *const ::libc::c_char,
                                             value: CassBytes) -> CassError;
    pub fn cass_statement_bind_uuid_by_name(statement: *mut CassStatement,
                                            name: *const ::libc::c_char,
                                            value: CassUuid) -> CassError;
    pub fn cass_statement_bind_inet_by_name(statement: *mut CassStatement,
                                            name: *const ::libc::c_char,
                                            value: CassInet) -> CassError;
    pub fn cass_statement_bind_decimal_by_name(statement: *mut CassStatement,
                                               name: *const ::libc::c_char,
                                               value: CassDecimal)
     -> CassError;
    pub fn cass_statement_bind_custom_by_name(statement: *mut CassStatement,
                                              name: *const ::libc::c_char,
                                              size: cass_size_t,
                                              output: *mut *mut cass_byte_t)
     -> CassError;
    pub fn cass_statement_bind_collection_by_name(statement:
                                                      *mut CassStatement,
                                                  name: *const ::libc::c_char,
                                                  collection:
                                                      *const CassCollection)
     -> CassError;
    pub fn cass_prepared_free(prepared: *const CassPrepared) -> ();
    pub fn cass_prepared_bind(prepared: *const CassPrepared)
     -> *mut CassStatement;
    pub fn cass_batch_new(_type: CassBatchType) -> *mut CassBatch;
    pub fn cass_batch_free(batch: *mut CassBatch) -> ();
    pub fn cass_batch_set_consistency(batch: *mut CassBatch,
                                      consistency: CassConsistency)
     -> CassError;
    pub fn cass_batch_add_statement(batch: *mut CassBatch,
                                    statement: *mut CassStatement)
     -> CassError;
    pub fn cass_collection_new(_type: CassCollectionType,
                               item_count: cass_size_t)
     -> *mut CassCollection;
    pub fn cass_collection_free(collection: *mut CassCollection) -> ();
    pub fn cass_collection_append_int32(collection: *mut CassCollection,
                                        value: cass_int32_t) -> CassError;
    pub fn cass_collection_append_int64(collection: *mut CassCollection,
                                        value: cass_int64_t) -> CassError;
    pub fn cass_collection_append_float(collection: *mut CassCollection,
                                        value: cass_float_t) -> CassError;
    pub fn cass_collection_append_double(collection: *mut CassCollection,
                                         value: cass_double_t) -> CassError;
    pub fn cass_collection_append_bool(collection: *mut CassCollection,
                                       value: cass_bool_t) -> CassError;
    pub fn cass_collection_append_string(collection: *mut CassCollection,
                                         value: CassString) -> CassError;
    pub fn cass_collection_append_bytes(collection: *mut CassCollection,
                                        value: CassBytes) -> CassError;
    pub fn cass_collection_append_uuid(collection: *mut CassCollection,
                                       value: CassUuid) -> CassError;
    pub fn cass_collection_append_inet(collection: *mut CassCollection,
                                       value: CassInet) -> CassError;
    pub fn cass_collection_append_decimal(collection: *mut CassCollection,
                                          value: CassDecimal) -> CassError;
    pub fn cass_result_free(result: *const CassResult) -> ();
    pub fn cass_result_row_count(result: *const CassResult) -> cass_size_t;
    pub fn cass_result_column_count(result: *const CassResult) -> cass_size_t;
    pub fn cass_result_column_name(result: *const CassResult,
                                   index: cass_size_t) -> CassString;
    pub fn cass_result_column_type(result: *const CassResult,
                                   index: cass_size_t) -> CassValueType;
    pub fn cass_result_first_row(result: *const CassResult) -> *const CassRow;
    pub fn cass_result_has_more_pages(result: *const CassResult)
     -> cass_bool_t;
    pub fn cass_iterator_free(iterator: *mut CassIterator) -> ();
    pub fn cass_iterator_type(iterator: *mut CassIterator)
     -> CassIteratorType;
    pub fn cass_iterator_from_result(result: *const CassResult)
     -> *mut CassIterator;
    pub fn cass_iterator_from_row(row: *const CassRow) -> *mut CassIterator;
    pub fn cass_iterator_from_collection(value: *const CassValue)
     -> *mut CassIterator;
    pub fn cass_iterator_from_map(value: *const CassValue)
     -> *mut CassIterator;
    pub fn cass_iterator_from_schema(schema: *const CassSchema)
     -> *mut CassIterator;
    pub fn cass_iterator_from_schema_meta(meta: *const CassSchemaMeta)
     -> *mut CassIterator;
    pub fn cass_iterator_fields_from_schema_meta(meta: *const CassSchemaMeta)
     -> *mut CassIterator;
    pub fn cass_iterator_next(iterator: *mut CassIterator) -> cass_bool_t;
    pub fn cass_iterator_get_row(iterator: *mut CassIterator)
     -> *const CassRow;
    pub fn cass_iterator_get_column(iterator: *mut CassIterator)
     -> *const CassValue;
    pub fn cass_iterator_get_value(iterator: *mut CassIterator)
     -> *const CassValue;
    pub fn cass_iterator_get_map_key(iterator: *mut CassIterator)
     -> *const CassValue;
    pub fn cass_iterator_get_map_value(iterator: *mut CassIterator)
     -> *const CassValue;
    pub fn cass_iterator_get_schema_meta(iterator: *mut CassIterator)
     -> *const CassSchemaMeta;
    pub fn cass_iterator_get_schema_meta_field(iterator: *mut CassIterator)
     -> *const CassSchemaMetaField;
    pub fn cass_row_get_column(row: *const CassRow, index: cass_size_t)
     -> *const CassValue;
    pub fn cass_row_get_column_by_name(row: *const CassRow,
                                       name: *const ::libc::c_char)
     -> *const CassValue;
    pub fn cass_value_get_int32(value: *const CassValue,
                                output: *mut cass_int32_t) -> CassError;
    pub fn cass_value_get_int64(value: *const CassValue,
                                output: *mut cass_int64_t) -> CassError;
    pub fn cass_value_get_float(value: *const CassValue,
                                output: *mut cass_float_t) -> CassError;
    pub fn cass_value_get_double(value: *const CassValue,
                                 output: *mut cass_double_t) -> CassError;
    pub fn cass_value_get_bool(value: *const CassValue,
                               output: *mut cass_bool_t) -> CassError;
    pub fn cass_value_get_uuid(value: *const CassValue, output: *mut CassUuid)
     -> CassError;
    pub fn cass_value_get_inet(value: *const CassValue, output: *mut CassInet)
     -> CassError;
    pub fn cass_value_get_string(value: *const CassValue,
                                 output: *mut CassString) -> CassError;
    pub fn cass_value_get_bytes(value: *const CassValue,
                                output: *mut CassBytes) -> CassError;
    pub fn cass_value_get_decimal(value: *const CassValue,
                                  output: *mut CassDecimal) -> CassError;
    pub fn cass_value_type(value: *const CassValue) -> CassValueType;
    pub fn cass_value_is_null(value: *const CassValue) -> cass_bool_t;
    pub fn cass_value_is_collection(value: *const CassValue) -> cass_bool_t;
    pub fn cass_value_item_count(collection: *const CassValue) -> cass_size_t;
    pub fn cass_value_primary_sub_type(collection: *const CassValue)
     -> CassValueType;
    pub fn cass_value_secondary_sub_type(collection: *const CassValue)
     -> CassValueType;
    pub fn cass_uuid_gen_new() -> *mut CassUuidGen;
    pub fn cass_uuid_gen_new_with_node(node: cass_uint64_t)
     -> *mut CassUuidGen;
    pub fn cass_uuid_gen_free(uuid_gen: *mut CassUuidGen) -> ();
    pub fn cass_uuid_gen_time(uuid_gen: *mut CassUuidGen,
                              output: *mut CassUuid) -> ();
    pub fn cass_uuid_gen_random(uuid_gen: *mut CassUuidGen,
                                output: *mut CassUuid) -> ();
    pub fn cass_uuid_gen_from_time(uuid_gen: *mut CassUuidGen,
                                   timestamp: cass_uint64_t,
                                   output: *mut CassUuid) -> ();
    pub fn cass_uuid_min_from_time(time: cass_uint64_t, output: *mut CassUuid)
     -> ();
    pub fn cass_uuid_max_from_time(time: cass_uint64_t, output: *mut CassUuid)
     -> ();
    pub fn cass_uuid_timestamp(uuid: CassUuid) -> cass_uint64_t;
    pub fn cass_uuid_version(uuid: CassUuid) -> cass_uint8_t;
    pub fn cass_uuid_string(uuid: CassUuid, output: *mut ::libc::c_char)
     -> ();
    pub fn cass_uuid_from_string(str: *const ::libc::c_char,
                                 output: *mut CassUuid) -> CassError;
    pub fn cass_error_desc(error: CassError) -> *const ::libc::c_char;
    pub fn cass_log_cleanup() -> ();
    pub fn cass_log_set_level(log_level: CassLogLevel) -> ();
    pub fn cass_log_set_callback(callback: CassLogCallback,
                                 data: *mut ::libc::c_void) -> ();
    pub fn cass_log_set_queue_size(queue_size: cass_size_t) -> ();
    pub fn cass_log_level_string(log_level: CassLogLevel)
     -> *const ::libc::c_char;
    pub fn cass_inet_init_v4(address: *const cass_uint8_t) -> CassInet;
    pub fn cass_inet_init_v6(address: *const cass_uint8_t) -> CassInet;
    pub fn cass_decimal_init(scale: cass_int32_t, varint: CassBytes)
     -> CassDecimal;
    pub fn cass_bytes_init(data: *const cass_byte_t, size: cass_size_t)
     -> CassBytes;
    pub fn cass_string_init(null_terminated: *const ::libc::c_char)
     -> CassString;
    pub fn cass_string_init2(data: *const ::libc::c_char, length: cass_size_t)
     -> CassString;
}
