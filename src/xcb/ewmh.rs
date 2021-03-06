pub use crate::ffi::ewmh::{
  xcb_ewmh_append_wm_icon as append_wm_icon,
  xcb_ewmh_append_wm_icon_checked as append_wm_icon_checked,
  xcb_ewmh_client_source_type_t as client_source_type_t,
  xcb_ewmh_connection_t as connection_t,
  xcb_ewmh_connection_wipe as connection_wipe,
  xcb_ewmh_coordinates_t as coordinates_t,
  xcb_ewmh_desktop_layout_orientation_t as desktop_layout_orientation_t,
  xcb_ewmh_desktop_layout_starting_corner_t as desktop_layout_starting_corner_t,
  xcb_ewmh_geometry_t as geometry_t,
  xcb_ewmh_get_active_window as get_active_window,
  xcb_ewmh_get_active_window_from_reply as get_active_window_from_reply,
  xcb_ewmh_get_active_window_reply as get_active_window_reply,
  xcb_ewmh_get_active_window_unchecked as get_active_window_unchecked,
  xcb_ewmh_get_atoms_from_reply as get_atoms_from_reply,
  xcb_ewmh_get_atoms_reply as get_atoms_reply,
  xcb_ewmh_get_atoms_reply_t as get_atoms_reply_t,
  xcb_ewmh_get_atoms_reply_wipe as get_atoms_reply_wipe,
  xcb_ewmh_get_cardinal_from_reply as get_cardinal_from_reply,
  xcb_ewmh_get_cardinal_reply as get_cardinal_reply,
  xcb_ewmh_get_client_list as get_client_list,
  xcb_ewmh_get_client_list_from_reply as get_client_list_from_reply,
  xcb_ewmh_get_client_list_reply as get_client_list_reply,
  xcb_ewmh_get_client_list_stacking as get_client_list_stacking,
  xcb_ewmh_get_client_list_stacking_from_reply as get_client_list_stacking_from_reply,
  xcb_ewmh_get_client_list_stacking_reply as get_client_list_stacking_reply,
  xcb_ewmh_get_client_list_stacking_unchecked as get_client_list_stacking_unchecked,
  xcb_ewmh_get_client_list_unchecked as get_client_list_unchecked,
  xcb_ewmh_get_current_desktop as get_current_desktop,
  xcb_ewmh_get_current_desktop_from_reply as get_current_desktop_from_reply,
  xcb_ewmh_get_current_desktop_reply as get_current_desktop_reply,
  xcb_ewmh_get_current_desktop_unchecked as get_current_desktop_unchecked,
  xcb_ewmh_get_desktop_geometry as get_desktop_geometry,
  xcb_ewmh_get_desktop_geometry_from_reply as get_desktop_geometry_from_reply,
  xcb_ewmh_get_desktop_geometry_reply as get_desktop_geometry_reply,
  xcb_ewmh_get_desktop_geometry_unchecked as get_desktop_geometry_unchecked,
  xcb_ewmh_get_desktop_layout as get_desktop_layout,
  xcb_ewmh_get_desktop_layout_from_reply as get_desktop_layout_from_reply,
  xcb_ewmh_get_desktop_layout_reply as get_desktop_layout_reply,
  xcb_ewmh_get_desktop_layout_reply_t as get_desktop_layout_reply_t,
  xcb_ewmh_get_desktop_layout_unchecked as get_desktop_layout_unchecked,
  xcb_ewmh_get_desktop_names as get_desktop_names,
  xcb_ewmh_get_desktop_names_from_reply as get_desktop_names_from_reply,
  xcb_ewmh_get_desktop_names_reply as get_desktop_names_reply,
  xcb_ewmh_get_desktop_names_unchecked as get_desktop_names_unchecked,
  xcb_ewmh_get_desktop_viewport as get_desktop_viewport,
  xcb_ewmh_get_desktop_viewport_from_reply as get_desktop_viewport_from_reply,
  xcb_ewmh_get_desktop_viewport_reply as get_desktop_viewport_reply,
  xcb_ewmh_get_desktop_viewport_reply_t as get_desktop_viewport_reply_t,
  xcb_ewmh_get_desktop_viewport_reply_wipe as get_desktop_viewport_reply_wipe,
  xcb_ewmh_get_desktop_viewport_unchecked as get_desktop_viewport_unchecked,
  xcb_ewmh_get_extents_reply_t as get_extents_reply_t,
  xcb_ewmh_get_frame_extents as get_frame_extents,
  xcb_ewmh_get_frame_extents_from_reply as get_frame_extents_from_reply,
  xcb_ewmh_get_frame_extents_reply as get_frame_extents_reply,
  xcb_ewmh_get_frame_extents_unchecked as get_frame_extents_unchecked,
  xcb_ewmh_get_number_of_desktops as get_number_of_desktops,
  xcb_ewmh_get_number_of_desktops_from_reply as get_number_of_desktops_from_reply,
  xcb_ewmh_get_number_of_desktops_reply as get_number_of_desktops_reply,
  xcb_ewmh_get_number_of_desktops_unchecked as get_number_of_desktops_unchecked,
  xcb_ewmh_get_showing_desktop as get_showing_desktop,
  xcb_ewmh_get_showing_desktop_from_reply as get_showing_desktop_from_reply,
  xcb_ewmh_get_showing_desktop_reply as get_showing_desktop_reply,
  xcb_ewmh_get_showing_desktop_unchecked as get_showing_desktop_unchecked,
  xcb_ewmh_get_supported as get_supported,
  xcb_ewmh_get_supported_from_reply as get_supported_from_reply,
  xcb_ewmh_get_supported_reply as get_supported_reply,
  xcb_ewmh_get_supported_unchecked as get_supported_unchecked,
  xcb_ewmh_get_supporting_wm_check as get_supporting_wm_check,
  xcb_ewmh_get_supporting_wm_check_from_reply as get_supporting_wm_check_from_reply,
  xcb_ewmh_get_supporting_wm_check_reply as get_supporting_wm_check_reply,
  xcb_ewmh_get_supporting_wm_check_unchecked as get_supporting_wm_check_unchecked,
  xcb_ewmh_get_utf8_strings_from_reply as get_utf8_strings_from_reply,
  xcb_ewmh_get_utf8_strings_reply as get_utf8_strings_reply,
  xcb_ewmh_get_utf8_strings_reply_t as get_utf8_strings_reply_t,
  xcb_ewmh_get_utf8_strings_reply_wipe as get_utf8_strings_reply_wipe,
  xcb_ewmh_get_virtual_roots as get_virtual_roots,
  xcb_ewmh_get_virtual_roots_from_reply as get_virtual_roots_from_reply,
  xcb_ewmh_get_virtual_roots_reply as get_virtual_roots_reply,
  xcb_ewmh_get_virtual_roots_unchecked as get_virtual_roots_unchecked,
  xcb_ewmh_get_window_from_reply as get_window_from_reply,
  xcb_ewmh_get_window_reply as get_window_reply,
  xcb_ewmh_get_windows_from_reply as get_windows_from_reply,
  xcb_ewmh_get_windows_reply as get_windows_reply,
  xcb_ewmh_get_windows_reply_t as get_windows_reply_t,
  xcb_ewmh_get_windows_reply_wipe as get_windows_reply_wipe,
  xcb_ewmh_get_wm_allowed_actions as get_wm_allowed_actions,
  xcb_ewmh_get_wm_allowed_actions_from_reply as get_wm_allowed_actions_from_reply,
  xcb_ewmh_get_wm_allowed_actions_reply as get_wm_allowed_actions_reply,
  xcb_ewmh_get_wm_allowed_actions_unchecked as get_wm_allowed_actions_unchecked,
  xcb_ewmh_get_wm_cm_owner as get_wm_cm_owner,
  xcb_ewmh_get_wm_cm_owner_from_reply as get_wm_cm_owner_from_reply,
  xcb_ewmh_get_wm_cm_owner_reply as get_wm_cm_owner_reply,
  xcb_ewmh_get_wm_cm_owner_unchecked as get_wm_cm_owner_unchecked,
  xcb_ewmh_get_wm_desktop as get_wm_desktop,
  xcb_ewmh_get_wm_desktop_from_reply as get_wm_desktop_from_reply,
  xcb_ewmh_get_wm_desktop_reply as get_wm_desktop_reply,
  xcb_ewmh_get_wm_desktop_unchecked as get_wm_desktop_unchecked,
  xcb_ewmh_get_wm_fullscreen_monitors as get_wm_fullscreen_monitors,
  xcb_ewmh_get_wm_fullscreen_monitors_from_reply as get_wm_fullscreen_monitors_from_reply,
  xcb_ewmh_get_wm_fullscreen_monitors_reply as get_wm_fullscreen_monitors_reply,
  xcb_ewmh_get_wm_fullscreen_monitors_reply_t as get_wm_fullscreen_monitors_reply_t,
  xcb_ewmh_get_wm_fullscreen_monitors_unchecked as get_wm_fullscreen_monitors_unchecked,
  xcb_ewmh_get_wm_handled_icons as get_wm_handled_icons,
  xcb_ewmh_get_wm_handled_icons_from_reply as get_wm_handled_icons_from_reply,
  xcb_ewmh_get_wm_handled_icons_reply as get_wm_handled_icons_reply,
  xcb_ewmh_get_wm_handled_icons_unchecked as get_wm_handled_icons_unchecked,
  xcb_ewmh_get_wm_icon as get_wm_icon,
  xcb_ewmh_get_wm_icon_from_reply as get_wm_icon_from_reply,
  xcb_ewmh_get_wm_icon_geometry as get_wm_icon_geometry,
  xcb_ewmh_get_wm_icon_geometry_from_reply as get_wm_icon_geometry_from_reply,
  xcb_ewmh_get_wm_icon_geometry_reply as get_wm_icon_geometry_reply,
  xcb_ewmh_get_wm_icon_geometry_unchecked as get_wm_icon_geometry_unchecked,
  xcb_ewmh_get_wm_icon_iterator as get_wm_icon_iterator,
  xcb_ewmh_get_wm_icon_length as get_wm_icon_length,
  xcb_ewmh_get_wm_icon_name as get_wm_icon_name,
  xcb_ewmh_get_wm_icon_name_from_reply as get_wm_icon_name_from_reply,
  xcb_ewmh_get_wm_icon_name_reply as get_wm_icon_name_reply,
  xcb_ewmh_get_wm_icon_name_unchecked as get_wm_icon_name_unchecked,
  xcb_ewmh_get_wm_icon_next as get_wm_icon_next,
  xcb_ewmh_get_wm_icon_reply as get_wm_icon_reply,
  xcb_ewmh_get_wm_icon_reply_t as get_wm_icon_reply_t,
  xcb_ewmh_get_wm_icon_reply_wipe as get_wm_icon_reply_wipe,
  xcb_ewmh_get_wm_icon_unchecked as get_wm_icon_unchecked,
  xcb_ewmh_get_wm_name as get_wm_name,
  xcb_ewmh_get_wm_name_from_reply as get_wm_name_from_reply,
  xcb_ewmh_get_wm_name_reply as get_wm_name_reply,
  xcb_ewmh_get_wm_name_unchecked as get_wm_name_unchecked,
  xcb_ewmh_get_wm_pid as get_wm_pid,
  xcb_ewmh_get_wm_pid_from_reply as get_wm_pid_from_reply,
  xcb_ewmh_get_wm_pid_reply as get_wm_pid_reply,
  xcb_ewmh_get_wm_pid_unchecked as get_wm_pid_unchecked,
  xcb_ewmh_get_wm_state as get_wm_state,
  xcb_ewmh_get_wm_state_from_reply as get_wm_state_from_reply,
  xcb_ewmh_get_wm_state_reply as get_wm_state_reply,
  xcb_ewmh_get_wm_state_unchecked as get_wm_state_unchecked,
  xcb_ewmh_get_wm_strut as get_wm_strut,
  xcb_ewmh_get_wm_strut_from_reply as get_wm_strut_from_reply,
  xcb_ewmh_get_wm_strut_partial as get_wm_strut_partial,
  xcb_ewmh_get_wm_strut_partial_from_reply as get_wm_strut_partial_from_reply,
  xcb_ewmh_get_wm_strut_partial_reply as get_wm_strut_partial_reply,
  xcb_ewmh_get_wm_strut_partial_unchecked as get_wm_strut_partial_unchecked,
  xcb_ewmh_get_wm_strut_reply as get_wm_strut_reply,
  xcb_ewmh_get_wm_strut_unchecked as get_wm_strut_unchecked,
  xcb_ewmh_get_wm_sync_request_counter as get_wm_sync_request_counter,
  xcb_ewmh_get_wm_sync_request_counter_from_reply as get_wm_sync_request_counter_from_reply,
  xcb_ewmh_get_wm_sync_request_counter_reply as get_wm_sync_request_counter_reply,
  xcb_ewmh_get_wm_sync_request_counter_unchecked as get_wm_sync_request_counter_unchecked,
  xcb_ewmh_get_wm_user_time as get_wm_user_time,
  xcb_ewmh_get_wm_user_time_from_reply as get_wm_user_time_from_reply,
  xcb_ewmh_get_wm_user_time_reply as get_wm_user_time_reply,
  xcb_ewmh_get_wm_user_time_unchecked as get_wm_user_time_unchecked,
  xcb_ewmh_get_wm_user_time_window as get_wm_user_time_window,
  xcb_ewmh_get_wm_user_time_window_from_reply as get_wm_user_time_window_from_reply,
  xcb_ewmh_get_wm_user_time_window_reply as get_wm_user_time_window_reply,
  xcb_ewmh_get_wm_user_time_window_unchecked as get_wm_user_time_window_unchecked,
  xcb_ewmh_get_wm_visible_icon_name as get_wm_visible_icon_name,
  xcb_ewmh_get_wm_visible_icon_name_from_reply as get_wm_visible_icon_name_from_reply,
  xcb_ewmh_get_wm_visible_icon_name_reply as get_wm_visible_icon_name_reply,
  xcb_ewmh_get_wm_visible_icon_name_unchecked as get_wm_visible_icon_name_unchecked,
  xcb_ewmh_get_wm_visible_name as get_wm_visible_name,
  xcb_ewmh_get_wm_visible_name_from_reply as get_wm_visible_name_from_reply,
  xcb_ewmh_get_wm_visible_name_reply as get_wm_visible_name_reply,
  xcb_ewmh_get_wm_visible_name_unchecked as get_wm_visible_name_unchecked,
  xcb_ewmh_get_wm_window_type as get_wm_window_type,
  xcb_ewmh_get_wm_window_type_from_reply as get_wm_window_type_from_reply,
  xcb_ewmh_get_wm_window_type_reply as get_wm_window_type_reply,
  xcb_ewmh_get_wm_window_type_unchecked as get_wm_window_type_unchecked,
  xcb_ewmh_get_workarea as get_workarea,
  xcb_ewmh_get_workarea_from_reply as get_workarea_from_reply,
  xcb_ewmh_get_workarea_reply as get_workarea_reply,
  xcb_ewmh_get_workarea_reply_t as get_workarea_reply_t,
  xcb_ewmh_get_workarea_reply_wipe as get_workarea_reply_wipe,
  xcb_ewmh_get_workarea_unchecked as get_workarea_unchecked,
  xcb_ewmh_init_atoms as init_atoms,
  xcb_ewmh_init_atoms_replies as init_atoms_replies,
  xcb_ewmh_moveresize_direction_t as moveresize_direction_t,
  xcb_ewmh_moveresize_window_opt_flags_t as moveresize_window_opt_flags_t,
  xcb_ewmh_request_change_active_window as request_change_active_window,
  xcb_ewmh_request_change_current_desktop as request_change_current_desktop,
  xcb_ewmh_request_change_desktop_geometry as request_change_desktop_geometry,
  xcb_ewmh_request_change_desktop_viewport as request_change_desktop_viewport,
  xcb_ewmh_request_change_number_of_desktops as request_change_number_of_desktops,
  xcb_ewmh_request_change_showing_desktop as request_change_showing_desktop,
  xcb_ewmh_request_change_wm_desktop as request_change_wm_desktop,
  xcb_ewmh_request_change_wm_fullscreen_monitors as request_change_wm_fullscreen_monitors,
  xcb_ewmh_request_change_wm_state as request_change_wm_state,
  xcb_ewmh_request_close_window as request_close_window,
  xcb_ewmh_request_frame_extents as request_frame_extents,
  xcb_ewmh_request_moveresize_window as request_moveresize_window,
  xcb_ewmh_request_restack_window as request_restack_window,
  xcb_ewmh_request_wm_moveresize as request_wm_moveresize,
  xcb_ewmh_send_client_message as send_client_message,
  xcb_ewmh_send_wm_ping as send_wm_ping,
  xcb_ewmh_send_wm_sync_request as send_wm_sync_request,
  xcb_ewmh_set_active_window as set_active_window,
  xcb_ewmh_set_active_window_checked as set_active_window_checked,
  xcb_ewmh_set_client_list as set_client_list,
  xcb_ewmh_set_client_list_checked as set_client_list_checked,
  xcb_ewmh_set_client_list_stacking as set_client_list_stacking,
  xcb_ewmh_set_client_list_stacking_checked as set_client_list_stacking_checked,
  xcb_ewmh_set_current_desktop as set_current_desktop,
  xcb_ewmh_set_current_desktop_checked as set_current_desktop_checked,
  xcb_ewmh_set_desktop_geometry as set_desktop_geometry,
  xcb_ewmh_set_desktop_geometry_checked as set_desktop_geometry_checked,
  xcb_ewmh_set_desktop_layout as set_desktop_layout,
  xcb_ewmh_set_desktop_layout_checked as set_desktop_layout_checked,
  xcb_ewmh_set_desktop_names as set_desktop_names,
  xcb_ewmh_set_desktop_names_checked as set_desktop_names_checked,
  xcb_ewmh_set_desktop_viewport as set_desktop_viewport,
  xcb_ewmh_set_desktop_viewport_checked as set_desktop_viewport_checked,
  xcb_ewmh_set_frame_extents as set_frame_extents,
  xcb_ewmh_set_frame_extents_checked as set_frame_extents_checked,
  xcb_ewmh_set_number_of_desktops as set_number_of_desktops,
  xcb_ewmh_set_number_of_desktops_checked as set_number_of_desktops_checked,
  xcb_ewmh_set_showing_desktop as set_showing_desktop,
  xcb_ewmh_set_showing_desktop_checked as set_showing_desktop_checked,
  xcb_ewmh_set_supported as set_supported,
  xcb_ewmh_set_supported_checked as set_supported_checked,
  xcb_ewmh_set_supporting_wm_check as set_supporting_wm_check,
  xcb_ewmh_set_supporting_wm_check_checked as set_supporting_wm_check_checked,
  xcb_ewmh_set_virtual_roots as set_virtual_roots,
  xcb_ewmh_set_virtual_roots_checked as set_virtual_roots_checked,
  xcb_ewmh_set_wm_allowed_actions as set_wm_allowed_actions,
  xcb_ewmh_set_wm_allowed_actions_checked as set_wm_allowed_actions_checked,
  xcb_ewmh_set_wm_cm_owner as set_wm_cm_owner,
  xcb_ewmh_set_wm_cm_owner_checked as set_wm_cm_owner_checked,
  xcb_ewmh_set_wm_desktop as set_wm_desktop,
  xcb_ewmh_set_wm_desktop_checked as set_wm_desktop_checked,
  xcb_ewmh_set_wm_fullscreen_monitors as set_wm_fullscreen_monitors,
  xcb_ewmh_set_wm_fullscreen_monitors_checked as set_wm_fullscreen_monitors_checked,
  xcb_ewmh_set_wm_handled_icons as set_wm_handled_icons,
  xcb_ewmh_set_wm_handled_icons_checked as set_wm_handled_icons_checked,
  xcb_ewmh_set_wm_icon as set_wm_icon,
  xcb_ewmh_set_wm_icon_checked as set_wm_icon_checked,
  xcb_ewmh_set_wm_icon_geometry as set_wm_icon_geometry,
  xcb_ewmh_set_wm_icon_geometry_checked as set_wm_icon_geometry_checked,
  xcb_ewmh_set_wm_icon_name as set_wm_icon_name,
  xcb_ewmh_set_wm_icon_name_checked as set_wm_icon_name_checked,
  xcb_ewmh_set_wm_name as set_wm_name,
  xcb_ewmh_set_wm_name_checked as set_wm_name_checked,
  xcb_ewmh_set_wm_pid as set_wm_pid,
  xcb_ewmh_set_wm_pid_checked as set_wm_pid_checked,
  xcb_ewmh_set_wm_state as set_wm_state,
  xcb_ewmh_set_wm_state_checked as set_wm_state_checked,
  xcb_ewmh_set_wm_strut as set_wm_strut,
  xcb_ewmh_set_wm_strut_checked as set_wm_strut_checked,
  xcb_ewmh_set_wm_strut_partial as set_wm_strut_partial,
  xcb_ewmh_set_wm_strut_partial_checked as set_wm_strut_partial_checked,
  xcb_ewmh_set_wm_sync_request_counter as set_wm_sync_request_counter,
  xcb_ewmh_set_wm_sync_request_counter_checked as set_wm_sync_request_counter_checked,
  xcb_ewmh_set_wm_user_time as set_wm_user_time,
  xcb_ewmh_set_wm_user_time_checked as set_wm_user_time_checked,
  xcb_ewmh_set_wm_user_time_window as set_wm_user_time_window,
  xcb_ewmh_set_wm_user_time_window_checked as set_wm_user_time_window_checked,
  xcb_ewmh_set_wm_visible_icon_name as set_wm_visible_icon_name,
  xcb_ewmh_set_wm_visible_icon_name_checked as set_wm_visible_icon_name_checked,
  xcb_ewmh_set_wm_visible_name as set_wm_visible_name,
  xcb_ewmh_set_wm_visible_name_checked as set_wm_visible_name_checked,
  xcb_ewmh_set_wm_window_type as set_wm_window_type,
  xcb_ewmh_set_wm_window_type_checked as set_wm_window_type_checked,
  xcb_ewmh_set_workarea as set_workarea,
  xcb_ewmh_set_workarea_checked as set_workarea_checked,
  xcb_ewmh_wm_icon_iterator_t as wm_icon_iterator_t,
  xcb_ewmh_wm_state_action_t as wm_state_action_t,
  xcb_ewmh_wm_strut_partial_t as wm_strut_partial_t,
};
