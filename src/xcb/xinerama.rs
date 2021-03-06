pub use crate::ffi::xinerama::{
  xcb_xinerama_get_screen_count as get_screen_count,
  xcb_xinerama_get_screen_count_cookie_t as get_screen_count_cookie_t,
  xcb_xinerama_get_screen_count_reply as get_screen_count_reply,
  xcb_xinerama_get_screen_count_reply_t as get_screen_count_reply_t,
  xcb_xinerama_get_screen_count_request_t as get_screen_count_request_t,
  xcb_xinerama_get_screen_count_unchecked as get_screen_count_unchecked,
  xcb_xinerama_get_screen_size as get_screen_size,
  xcb_xinerama_get_screen_size_cookie_t as get_screen_size_cookie_t,
  xcb_xinerama_get_screen_size_reply as get_screen_size_reply,
  xcb_xinerama_get_screen_size_reply_t as get_screen_size_reply_t,
  xcb_xinerama_get_screen_size_request_t as get_screen_size_request_t,
  xcb_xinerama_get_screen_size_unchecked as get_screen_size_unchecked,
  xcb_xinerama_get_state as get_state,
  xcb_xinerama_get_state_cookie_t as get_state_cookie_t,
  xcb_xinerama_get_state_reply as get_state_reply,
  xcb_xinerama_get_state_reply_t as get_state_reply_t,
  xcb_xinerama_get_state_request_t as get_state_request_t,
  xcb_xinerama_get_state_unchecked as get_state_unchecked,
  xcb_xinerama_id as id,
  xcb_xinerama_is_active as is_active,
  xcb_xinerama_is_active_cookie_t as is_active_cookie_t,
  xcb_xinerama_is_active_reply as is_active_reply,
  xcb_xinerama_is_active_reply_t as is_active_reply_t,
  xcb_xinerama_is_active_request_t as is_active_request_t,
  xcb_xinerama_is_active_unchecked as is_active_unchecked,
  xcb_xinerama_query_screens as query_screens,
  xcb_xinerama_query_screens_cookie_t as query_screens_cookie_t,
  xcb_xinerama_query_screens_reply as query_screens_reply,
  xcb_xinerama_query_screens_reply_t as query_screens_reply_t,
  xcb_xinerama_query_screens_request_t as query_screens_request_t,
  xcb_xinerama_query_screens_screen_info as query_screens_screen_info,
  xcb_xinerama_query_screens_screen_info_iterator as query_screens_screen_info_iterator,
  xcb_xinerama_query_screens_screen_info_length as query_screens_screen_info_length,
  xcb_xinerama_query_screens_sizeof as query_screens_sizeof,
  xcb_xinerama_query_screens_unchecked as query_screens_unchecked,
  xcb_xinerama_query_version as query_version,
  xcb_xinerama_query_version_cookie_t as query_version_cookie_t,
  xcb_xinerama_query_version_reply as query_version_reply,
  xcb_xinerama_query_version_reply_t as query_version_reply_t,
  xcb_xinerama_query_version_request_t as query_version_request_t,
  xcb_xinerama_query_version_unchecked as query_version_unchecked,
  xcb_xinerama_screen_info_end as screen_info_end,
  xcb_xinerama_screen_info_iterator_t as screen_info_iterator_t,
  xcb_xinerama_screen_info_next as screen_info_next,
  xcb_xinerama_screen_info_t as screen_info_t,
  XCB_XINERAMA_GET_SCREEN_COUNT as GET_SCREEN_COUNT,
  XCB_XINERAMA_GET_SCREEN_SIZE as GET_SCREEN_SIZE,
  XCB_XINERAMA_GET_STATE as GET_STATE,
  XCB_XINERAMA_IS_ACTIVE as IS_ACTIVE,
  XCB_XINERAMA_MAJOR_VERSION as MAJOR_VERSION,
  XCB_XINERAMA_MINOR_VERSION as MINOR_VERSION,
  XCB_XINERAMA_QUERY_SCREENS as QUERY_SCREENS,
  XCB_XINERAMA_QUERY_VERSION as QUERY_VERSION,
};
