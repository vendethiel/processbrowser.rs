use sysinfo::{System, SystemExt, PidExt, ProcessExt, UserExt};
use super::data::ProcessData;

pub fn current_processes() -> Vec<ProcessData> {
    let mut s = System::new_all();
    s.refresh_all();
    s.processes()
        .into_iter()
        .map(|(id, process)| {
            let pid = id.as_u32();
            let name = process.name().to_string();
            let uid = process.user_id().map(|uid| **uid);
            let username = process.user_id().and_then(|uid| s.get_user_by_id(uid)).map(|user| user.name().to_string());
            ProcessData::new(pid, name, uid, username)
        })
        .collect()
}
