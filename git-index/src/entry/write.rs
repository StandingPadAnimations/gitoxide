use crate::{Entry, State, Version};
use std::convert::TryInto;

impl Entry {
    /// Serialize ourselves to `out` with path access via `state`.
    pub fn write_to(&self, mut out: impl std::io::Write, state: &State) -> std::io::Result<()> {
        let stat = self.stat;
        out.write_all(&stat.ctime.secs.to_be_bytes())?;
        out.write_all(&stat.ctime.nsecs.to_be_bytes())?;
        out.write_all(&stat.mtime.secs.to_be_bytes())?;
        out.write_all(&stat.mtime.nsecs.to_be_bytes())?;
        out.write_all(&stat.dev.to_be_bytes())?;
        out.write_all(&stat.ino.to_be_bytes())?;
        out.write_all(&self.mode.bits().to_be_bytes())?;
        out.write_all(&stat.uid.to_be_bytes())?;
        out.write_all(&stat.gid.to_be_bytes())?;
        out.write_all(&stat.size.to_be_bytes())?;
        out.write_all(self.id.as_bytes())?;
        let path = self.path(state);
        let path_len: u16 = path
            .len()
            .try_into()
            .expect("Cannot handle paths longer than 16bits ever");
        assert!(
            path_len <= 0xFFF,
            "Paths can't be longer than 12 bits as they share space with bit flags in a u16"
        );
        let version = Version::V2; // TODO: don't hardcode once `to_storage()` can do its work without assertion
        out.write_all(&(self.flags.to_storage(version).bits() | path_len).to_be_bytes())?;
        out.write_all(path)?;
        out.write_all(b"\0")
    }
}
