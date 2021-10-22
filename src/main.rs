fn main() {
  use notify_rust::{
        set_application, Notification,
    };
  use std::time;

    set_application("com.apple.calculator").map_err(|f| format!("{}", f)).unwrap();

    // set_application(&safari_id).map_err(|f| format!("{}", f)).unwrap();

    let mut notification = Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Notes")
        .icon("Notes")
        .timeout(3)
        .show()
        .map_err(|f| format!("{}", f)).unwrap();
    for i in 0..11 {
      std::thread::sleep(time::Duration::from_millis(1_000));
      notification.body(&format!("I'm GROOT {}", i));
      notification
        .timeout(1)
        .show()
        .map_err(|f| format!("{}", f))
        .unwrap();
    }
}
