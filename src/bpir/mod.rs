//! BPIR stands for "Binary Protocol Intermediate Representation". It is the
//! core of "robusto" library, the intermediate representation for binary
//! protocols which, in a long shot, covers each and every serial embedded
//! binary protocol: CRC checksums, preambles and parser synchronization,
//! conditional interpretation, etc.

pub mod representation;
pub mod validation;
