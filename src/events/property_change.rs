use crate::mpv::{MpvEvent, MpvFormat, MpvHandle};
use crate::sponsorblock::segment::Segments;
use crate::WATCHER_TIME;

fn event_time_change(mpv_handle: &MpvHandle, mpv_event: MpvEvent, segments: &Option<Segments>) {
    if let Some(segments) = segments {
        let event_property = mpv_event.get_event_property();
        let old_time_pos: f64 = match event_property.get_data() {
            Some(v) => v,
            None => return,
        };
        let mut new_time_pos: f64 = old_time_pos;

        for segment in segments.iter().filter(|s| s.action.as_str() == "skip") {
            if new_time_pos >= segment.segment[0] && new_time_pos < segment.segment[1] {
                log::info!(
                    "Skipping segment [{}] from {} to {}.",
                    segment.category,
                    segment.segment[0],
                    segment.segment[1]
                );

                new_time_pos = segment.segment[1];
            }
        }

        if old_time_pos != new_time_pos {
            if let Err(e) = mpv_handle.set_property("time-pos", MpvFormat::Double, new_time_pos) {
                log::error!("{}", e);
                return;
            }
        }
    }
}

pub fn event(mpv_handle: &MpvHandle, mpv_event: MpvEvent, segments: &Option<Segments>) {
    match mpv_event.get_reply_userdata() {
        WATCHER_TIME => event_time_change(mpv_handle, mpv_event, segments),
        _ => {}
    }
}
