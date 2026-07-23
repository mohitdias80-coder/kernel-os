#[derive(Debug)]
pub enum AbiError {
    InvalidCommand,
    InvalidPayload,
    UnsupportedVersion,
}
