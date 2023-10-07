/// Validates BPIR. Looks for common mistakes, and produces related output.

use crate::bpir;

pub struct ValidationResult {
	errors: usize,
	warnings: usize,
}

pub fn validate_protocol(protocol: &bpir::representation::Protocol) -> ValidationResult {
	// TODO
	return ValidationResult{
		errors: 0usize,
		warnings: 0usize,
	}
}
