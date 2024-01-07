/// Validates BPIR. Looks for common mistakes, and produces related output.
use crate::bpir::representation;
use std::boxed;
use std::vec;
use std::string;

enum MessageFieldLintResult {
    Ok,
    Warning(string::String),
}

/// A linter implementing `MessageFieldLint` checks the correctness of a
/// message's fields.
///
/// - The linter MAY be stateful;
/// - The validation framework calls a set of linters on each one field;
/// - The linter is GUARANTEED to be called in the same order as fields in a
/// message are defined;
/// - On each field, a linter is GUARANTEED to be called once and only once;
/// - Linters called on the same field MUST NOT make assumptions on the order of
/// mutual execution;
/// - Linters MUST BE functionally independent from each other;
/// - The scope of the linter is limited by one message. The linter MAY perform
/// composite (cross-field) checking, i.e. MAY make it so linting result depends
/// on juxtaposition of 2 or more fields;
/// - The scope of a field linter is limited by one message. If 2 or more
/// messages are supported by the protocol, the linter MUST NOT implement
/// cross-message checking.
trait MessageFieldLint {
    fn lint_field(
        &mut self,
        message: &representation::Message,
        field: &representation::Field,
    ) -> MessageFieldLintResult;
}

#[derive(Default)]
struct MockLinter {}

impl MessageFieldLint for MockLinter {
    fn lint_field(
        &mut self,
        message: &representation::Message,
        field: &representation::Field,
    ) -> MessageFieldLintResult {
        MessageFieldLintResult::Ok
    }
}

/// Makes sure that a regex field has "max length" attribute
struct RegexFieldMaxLengthLinter {}

impl MessageFieldLint for RegexFieldMaxLengthLinter {
    fn lint_field(
        &mut self,
        message: &representation::Message,
        field: &representation::Field,
    ) -> MessageFieldLintResult {
        match field.field_type {
            representation::FieldType::Regex(_) => {
                for attribute in &field.attributes {
                    if let representation::FieldAttribute::MaxLength(_) = attribute {
                        return MessageFieldLintResult::Ok;
                    }
                }
            },
            _ => {},
        }

        MessageFieldLintResult::Warning(format!("in message {0} field {1} does not have MaxLength attribute", message.name, field.name))
    }
}

struct CompositeMessageLinter {
    pending_linters: vec::Vec<boxed::Box<dyn MessageFieldLint>>,
}

impl CompositeMessageLinter {
    pub fn new() -> Self {
        let mut instance = CompositeMessageLinter {
            pending_linters: vec::Vec::default(),
        };
        instance
            .pending_linters
            .push(boxed::Box::new(MockLinter::default()));

        instance
    }

    pub fn lint_message(&mut self, message: &representation::Message) {
        for field in &message.fields {
            self.lint_field(message, field);
        }
    }

    fn lint_field(&mut self, message: &representation::Message, field: &representation::Field) {
        for linter in &mut self.pending_linters {
            linter.lint_field(message, field); // TODO: save result
        }
    }
}

pub fn validate_protocol(protocol: &representation::Protocol) {
    for message in &protocol.messages {
        let mut linter = CompositeMessageLinter::new();
        linter.lint_message(message);
    }
}
