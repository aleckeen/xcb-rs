pub use crate::ffi::xfixes::{
  xcb_xfixes_bad_region_error_t as bad_region_error_t,
  xcb_xfixes_barrier_directions_t as barrier_directions_t,
  xcb_xfixes_barrier_end as barrier_end,
  xcb_xfixes_barrier_iterator_t as barrier_iterator_t,
  xcb_xfixes_barrier_next as barrier_next,
  xcb_xfixes_barrier_t as barrier_t,
  xcb_xfixes_change_cursor as change_cursor,
  xcb_xfixes_change_cursor_by_name as change_cursor_by_name,
  xcb_xfixes_change_cursor_by_name_checked as change_cursor_by_name_checked,
  xcb_xfixes_change_cursor_by_name_name as change_cursor_by_name_name,
  xcb_xfixes_change_cursor_by_name_name_end as change_cursor_by_name_name_end,
  xcb_xfixes_change_cursor_by_name_name_length as change_cursor_by_name_name_length,
  xcb_xfixes_change_cursor_by_name_request_t as change_cursor_by_name_request_t,
  xcb_xfixes_change_cursor_by_name_sizeof as change_cursor_by_name_sizeof,
  xcb_xfixes_change_cursor_checked as change_cursor_checked,
  xcb_xfixes_change_cursor_request_t as change_cursor_request_t,
  xcb_xfixes_change_save_set as change_save_set,
  xcb_xfixes_change_save_set_checked as change_save_set_checked,
  xcb_xfixes_change_save_set_request_t as change_save_set_request_t,
  xcb_xfixes_copy_region as copy_region,
  xcb_xfixes_copy_region_checked as copy_region_checked,
  xcb_xfixes_copy_region_request_t as copy_region_request_t,
  xcb_xfixes_create_pointer_barrier as create_pointer_barrier,
  xcb_xfixes_create_pointer_barrier_checked as create_pointer_barrier_checked,
  xcb_xfixes_create_pointer_barrier_devices as create_pointer_barrier_devices,
  xcb_xfixes_create_pointer_barrier_devices_end as create_pointer_barrier_devices_end,
  xcb_xfixes_create_pointer_barrier_devices_length as create_pointer_barrier_devices_length,
  xcb_xfixes_create_pointer_barrier_request_t as create_pointer_barrier_request_t,
  xcb_xfixes_create_pointer_barrier_sizeof as create_pointer_barrier_sizeof,
  xcb_xfixes_create_region as create_region,
  xcb_xfixes_create_region_checked as create_region_checked,
  xcb_xfixes_create_region_from_bitmap as create_region_from_bitmap,
  xcb_xfixes_create_region_from_bitmap_checked as create_region_from_bitmap_checked,
  xcb_xfixes_create_region_from_bitmap_request_t as create_region_from_bitmap_request_t,
  xcb_xfixes_create_region_from_gc as create_region_from_gc,
  xcb_xfixes_create_region_from_gc_checked as create_region_from_gc_checked,
  xcb_xfixes_create_region_from_gc_request_t as create_region_from_gc_request_t,
  xcb_xfixes_create_region_from_picture as create_region_from_picture,
  xcb_xfixes_create_region_from_picture_checked as create_region_from_picture_checked,
  xcb_xfixes_create_region_from_picture_request_t as create_region_from_picture_request_t,
  xcb_xfixes_create_region_from_window as create_region_from_window,
  xcb_xfixes_create_region_from_window_checked as create_region_from_window_checked,
  xcb_xfixes_create_region_from_window_request_t as create_region_from_window_request_t,
  xcb_xfixes_create_region_rectangles as create_region_rectangles,
  xcb_xfixes_create_region_rectangles_iterator as create_region_rectangles_iterator,
  xcb_xfixes_create_region_rectangles_length as create_region_rectangles_length,
  xcb_xfixes_create_region_request_t as create_region_request_t,
  xcb_xfixes_create_region_sizeof as create_region_sizeof,
  xcb_xfixes_cursor_notify_event_t as cursor_notify_event_t,
  xcb_xfixes_cursor_notify_mask_t as cursor_notify_mask_t,
  xcb_xfixes_cursor_notify_t as cursor_notify_t,
  xcb_xfixes_delete_pointer_barrier as delete_pointer_barrier,
  xcb_xfixes_delete_pointer_barrier_checked as delete_pointer_barrier_checked,
  xcb_xfixes_delete_pointer_barrier_request_t as delete_pointer_barrier_request_t,
  xcb_xfixes_destroy_region as destroy_region,
  xcb_xfixes_destroy_region_checked as destroy_region_checked,
  xcb_xfixes_destroy_region_request_t as destroy_region_request_t,
  xcb_xfixes_expand_region as expand_region,
  xcb_xfixes_expand_region_checked as expand_region_checked,
  xcb_xfixes_expand_region_request_t as expand_region_request_t,
  xcb_xfixes_fetch_region as fetch_region,
  xcb_xfixes_fetch_region_cookie_t as fetch_region_cookie_t,
  xcb_xfixes_fetch_region_rectangles as fetch_region_rectangles,
  xcb_xfixes_fetch_region_rectangles_iterator as fetch_region_rectangles_iterator,
  xcb_xfixes_fetch_region_rectangles_length as fetch_region_rectangles_length,
  xcb_xfixes_fetch_region_reply as fetch_region_reply,
  xcb_xfixes_fetch_region_reply_t as fetch_region_reply_t,
  xcb_xfixes_fetch_region_request_t as fetch_region_request_t,
  xcb_xfixes_fetch_region_sizeof as fetch_region_sizeof,
  xcb_xfixes_fetch_region_unchecked as fetch_region_unchecked,
  xcb_xfixes_get_cursor_image as get_cursor_image,
  xcb_xfixes_get_cursor_image_and_name as get_cursor_image_and_name,
  xcb_xfixes_get_cursor_image_and_name_cookie_t as get_cursor_image_and_name_cookie_t,
  xcb_xfixes_get_cursor_image_and_name_cursor_image as get_cursor_image_and_name_cursor_image,
  xcb_xfixes_get_cursor_image_and_name_cursor_image_end as get_cursor_image_and_name_cursor_image_end,
  xcb_xfixes_get_cursor_image_and_name_cursor_image_length as get_cursor_image_and_name_cursor_image_length,
  xcb_xfixes_get_cursor_image_and_name_name as get_cursor_image_and_name_name,
  xcb_xfixes_get_cursor_image_and_name_name_end as get_cursor_image_and_name_name_end,
  xcb_xfixes_get_cursor_image_and_name_name_length as get_cursor_image_and_name_name_length,
  xcb_xfixes_get_cursor_image_and_name_reply as get_cursor_image_and_name_reply,
  xcb_xfixes_get_cursor_image_and_name_reply_t as get_cursor_image_and_name_reply_t,
  xcb_xfixes_get_cursor_image_and_name_request_t as get_cursor_image_and_name_request_t,
  xcb_xfixes_get_cursor_image_and_name_sizeof as get_cursor_image_and_name_sizeof,
  xcb_xfixes_get_cursor_image_and_name_unchecked as get_cursor_image_and_name_unchecked,
  xcb_xfixes_get_cursor_image_cookie_t as get_cursor_image_cookie_t,
  xcb_xfixes_get_cursor_image_cursor_image as get_cursor_image_cursor_image,
  xcb_xfixes_get_cursor_image_cursor_image_end as get_cursor_image_cursor_image_end,
  xcb_xfixes_get_cursor_image_cursor_image_length as get_cursor_image_cursor_image_length,
  xcb_xfixes_get_cursor_image_reply as get_cursor_image_reply,
  xcb_xfixes_get_cursor_image_reply_t as get_cursor_image_reply_t,
  xcb_xfixes_get_cursor_image_request_t as get_cursor_image_request_t,
  xcb_xfixes_get_cursor_image_sizeof as get_cursor_image_sizeof,
  xcb_xfixes_get_cursor_image_unchecked as get_cursor_image_unchecked,
  xcb_xfixes_get_cursor_name as get_cursor_name,
  xcb_xfixes_get_cursor_name_cookie_t as get_cursor_name_cookie_t,
  xcb_xfixes_get_cursor_name_name as get_cursor_name_name,
  xcb_xfixes_get_cursor_name_name_end as get_cursor_name_name_end,
  xcb_xfixes_get_cursor_name_name_length as get_cursor_name_name_length,
  xcb_xfixes_get_cursor_name_reply as get_cursor_name_reply,
  xcb_xfixes_get_cursor_name_reply_t as get_cursor_name_reply_t,
  xcb_xfixes_get_cursor_name_request_t as get_cursor_name_request_t,
  xcb_xfixes_get_cursor_name_sizeof as get_cursor_name_sizeof,
  xcb_xfixes_get_cursor_name_unchecked as get_cursor_name_unchecked,
  xcb_xfixes_hide_cursor as hide_cursor,
  xcb_xfixes_hide_cursor_checked as hide_cursor_checked,
  xcb_xfixes_hide_cursor_request_t as hide_cursor_request_t,
  xcb_xfixes_id as id,
  xcb_xfixes_intersect_region as intersect_region,
  xcb_xfixes_intersect_region_checked as intersect_region_checked,
  xcb_xfixes_intersect_region_request_t as intersect_region_request_t,
  xcb_xfixes_invert_region as invert_region,
  xcb_xfixes_invert_region_checked as invert_region_checked,
  xcb_xfixes_invert_region_request_t as invert_region_request_t,
  xcb_xfixes_query_version as query_version,
  xcb_xfixes_query_version_cookie_t as query_version_cookie_t,
  xcb_xfixes_query_version_reply as query_version_reply,
  xcb_xfixes_query_version_reply_t as query_version_reply_t,
  xcb_xfixes_query_version_request_t as query_version_request_t,
  xcb_xfixes_query_version_unchecked as query_version_unchecked,
  xcb_xfixes_region_end as region_end,
  xcb_xfixes_region_enum_t as region_enum_t,
  xcb_xfixes_region_extents as region_extents,
  xcb_xfixes_region_extents_checked as region_extents_checked,
  xcb_xfixes_region_extents_request_t as region_extents_request_t,
  xcb_xfixes_region_iterator_t as region_iterator_t,
  xcb_xfixes_region_next as region_next,
  xcb_xfixes_region_t as region_t,
  xcb_xfixes_save_set_mapping_t as save_set_mapping_t,
  xcb_xfixes_save_set_mode_t as save_set_mode_t,
  xcb_xfixes_save_set_target_t as save_set_target_t,
  xcb_xfixes_select_cursor_input as select_cursor_input,
  xcb_xfixes_select_cursor_input_checked as select_cursor_input_checked,
  xcb_xfixes_select_cursor_input_request_t as select_cursor_input_request_t,
  xcb_xfixes_select_selection_input as select_selection_input,
  xcb_xfixes_select_selection_input_checked as select_selection_input_checked,
  xcb_xfixes_select_selection_input_request_t as select_selection_input_request_t,
  xcb_xfixes_selection_event_mask_t as selection_event_mask_t,
  xcb_xfixes_selection_event_t as selection_event_t,
  xcb_xfixes_selection_notify_event_t as selection_notify_event_t,
  xcb_xfixes_set_cursor_name as set_cursor_name,
  xcb_xfixes_set_cursor_name_checked as set_cursor_name_checked,
  xcb_xfixes_set_cursor_name_name as set_cursor_name_name,
  xcb_xfixes_set_cursor_name_name_end as set_cursor_name_name_end,
  xcb_xfixes_set_cursor_name_name_length as set_cursor_name_name_length,
  xcb_xfixes_set_cursor_name_request_t as set_cursor_name_request_t,
  xcb_xfixes_set_cursor_name_sizeof as set_cursor_name_sizeof,
  xcb_xfixes_set_gc_clip_region as set_gc_clip_region,
  xcb_xfixes_set_gc_clip_region_checked as set_gc_clip_region_checked,
  xcb_xfixes_set_gc_clip_region_request_t as set_gc_clip_region_request_t,
  xcb_xfixes_set_picture_clip_region as set_picture_clip_region,
  xcb_xfixes_set_picture_clip_region_checked as set_picture_clip_region_checked,
  xcb_xfixes_set_picture_clip_region_request_t as set_picture_clip_region_request_t,
  xcb_xfixes_set_region as set_region,
  xcb_xfixes_set_region_checked as set_region_checked,
  xcb_xfixes_set_region_rectangles as set_region_rectangles,
  xcb_xfixes_set_region_rectangles_iterator as set_region_rectangles_iterator,
  xcb_xfixes_set_region_rectangles_length as set_region_rectangles_length,
  xcb_xfixes_set_region_request_t as set_region_request_t,
  xcb_xfixes_set_region_sizeof as set_region_sizeof,
  xcb_xfixes_set_window_shape_region as set_window_shape_region,
  xcb_xfixes_set_window_shape_region_checked as set_window_shape_region_checked,
  xcb_xfixes_set_window_shape_region_request_t as set_window_shape_region_request_t,
  xcb_xfixes_show_cursor as show_cursor,
  xcb_xfixes_show_cursor_checked as show_cursor_checked,
  xcb_xfixes_show_cursor_request_t as show_cursor_request_t,
  xcb_xfixes_subtract_region as subtract_region,
  xcb_xfixes_subtract_region_checked as subtract_region_checked,
  xcb_xfixes_subtract_region_request_t as subtract_region_request_t,
  xcb_xfixes_translate_region as translate_region,
  xcb_xfixes_translate_region_checked as translate_region_checked,
  xcb_xfixes_translate_region_request_t as translate_region_request_t,
  xcb_xfixes_union_region as union_region,
  xcb_xfixes_union_region_checked as union_region_checked,
  xcb_xfixes_union_region_request_t as union_region_request_t,
  XCB_XFIXES_BAD_REGION as BAD_REGION,
  XCB_XFIXES_CHANGE_CURSOR as CHANGE_CURSOR,
  XCB_XFIXES_CHANGE_CURSOR_BY_NAME as CHANGE_CURSOR_BY_NAME,
  XCB_XFIXES_CHANGE_SAVE_SET as CHANGE_SAVE_SET,
  XCB_XFIXES_COPY_REGION as COPY_REGION,
  XCB_XFIXES_CREATE_POINTER_BARRIER as CREATE_POINTER_BARRIER,
  XCB_XFIXES_CREATE_REGION as CREATE_REGION,
  XCB_XFIXES_CREATE_REGION_FROM_BITMAP as CREATE_REGION_FROM_BITMAP,
  XCB_XFIXES_CREATE_REGION_FROM_GC as CREATE_REGION_FROM_GC,
  XCB_XFIXES_CREATE_REGION_FROM_PICTURE as CREATE_REGION_FROM_PICTURE,
  XCB_XFIXES_CREATE_REGION_FROM_WINDOW as CREATE_REGION_FROM_WINDOW,
  XCB_XFIXES_CURSOR_NOTIFY as CURSOR_NOTIFY,
  XCB_XFIXES_DELETE_POINTER_BARRIER as DELETE_POINTER_BARRIER,
  XCB_XFIXES_DESTROY_REGION as DESTROY_REGION,
  XCB_XFIXES_EXPAND_REGION as EXPAND_REGION,
  XCB_XFIXES_FETCH_REGION as FETCH_REGION,
  XCB_XFIXES_GET_CURSOR_IMAGE as GET_CURSOR_IMAGE,
  XCB_XFIXES_GET_CURSOR_IMAGE_AND_NAME as GET_CURSOR_IMAGE_AND_NAME,
  XCB_XFIXES_GET_CURSOR_NAME as GET_CURSOR_NAME,
  XCB_XFIXES_HIDE_CURSOR as HIDE_CURSOR,
  XCB_XFIXES_INTERSECT_REGION as INTERSECT_REGION,
  XCB_XFIXES_INVERT_REGION as INVERT_REGION,
  XCB_XFIXES_MAJOR_VERSION as MAJOR_VERSION,
  XCB_XFIXES_MINOR_VERSION as MINOR_VERSION,
  XCB_XFIXES_QUERY_VERSION as QUERY_VERSION,
  XCB_XFIXES_REGION_EXTENTS as REGION_EXTENTS,
  XCB_XFIXES_SELECTION_NOTIFY as SELECTION_NOTIFY,
  XCB_XFIXES_SELECT_CURSOR_INPUT as SELECT_CURSOR_INPUT,
  XCB_XFIXES_SELECT_SELECTION_INPUT as SELECT_SELECTION_INPUT,
  XCB_XFIXES_SET_CURSOR_NAME as SET_CURSOR_NAME,
  XCB_XFIXES_SET_GC_CLIP_REGION as SET_GC_CLIP_REGION,
  XCB_XFIXES_SET_PICTURE_CLIP_REGION as SET_PICTURE_CLIP_REGION,
  XCB_XFIXES_SET_REGION as SET_REGION,
  XCB_XFIXES_SET_WINDOW_SHAPE_REGION as SET_WINDOW_SHAPE_REGION,
  XCB_XFIXES_SHOW_CURSOR as SHOW_CURSOR,
  XCB_XFIXES_SUBTRACT_REGION as SUBTRACT_REGION,
  XCB_XFIXES_TRANSLATE_REGION as TRANSLATE_REGION,
  XCB_XFIXES_UNION_REGION as UNION_REGION,
};
