pub struct Permissions {
    read: bool,
    write: bool,
    audit: bool,
}
impl Permissions {
    fn has_read_permission(&self) -> bool {
        return self.read;
    }
    fn has_write_permission(&self) -> bool {
        return self.audit;
    }
    fn has_audit_permission(&self) -> bool {
        return self.audit;
    }
}
