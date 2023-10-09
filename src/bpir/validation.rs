/// Validates BPIR. Looks for common mistakes, and produces related output.

use crate::bpir;

pub struct Result {
	errors: usize,
	warnings: usize,
}

pub fn validate_protocol(protocol: &bpir::representation::Protocol) -> Result {
	// TODO
	return Result{
		errors: 0usize,
		warnings: 0usize,
	}
}
