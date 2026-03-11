/// Asserts that an [`Output`](crate::Output) contains a diagnostic at the given level
/// whose `Display` output contains the expected substring.
///
/// Walks all nested diagnostics (via [`Diagnostic::walk()`](crate::mark::Diagnostic::walk))
/// to find a match.
///
/// # Examples
///
/// ```ignore
/// let output = zyn::Render::render(&my_element, &input);
/// zyn::assert_diagnostic!(output, zyn::mark::Level::Error, "field not found");
/// ```
#[macro_export]
macro_rules! assert_diagnostic {
    ($output:expr, $level:expr, $msg:expr) => {{
        let __output = &$output;
        let __diag = __output.diagnostic();
        let __level = $level;
        let __msg = $msg;
        let __check =
            |d: &$crate::Diagnostic| d.level() == __level && d.to_string().contains(__msg);
        let __found = __check(__diag) || __diag.walk().any(__check);
        assert!(
            __found,
            "no {:?} diagnostic containing {:?}\n\ndiagnostics:\n{}",
            __level, __msg, __diag,
        );
    }};
}

/// Asserts that an [`Output`](crate::Output) has an error-level diagnostic
/// containing the given message.
///
/// Equivalent to `assert_diagnostic!(output, Level::Error, "msg")`.
///
/// # Examples
///
/// ```ignore
/// zyn::assert_compile_error!(output, "missing field");
/// ```
#[macro_export]
macro_rules! assert_compile_error {
    ($output:expr, $msg:expr) => {
        $crate::assert_diagnostic!($output, $crate::mark::Level::Error, $msg)
    };
}

/// Asserts that an [`Output`](crate::Output) contains an error diagnostic
/// with the given message.
#[macro_export]
macro_rules! assert_diagnostic_error {
    ($output:expr, $msg:expr) => {
        $crate::assert_diagnostic!($output, $crate::mark::Level::Error, $msg)
    };
}

/// Asserts that an [`Output`](crate::Output) contains a warning diagnostic
/// with the given message.
#[macro_export]
macro_rules! assert_diagnostic_warning {
    ($output:expr, $msg:expr) => {
        $crate::assert_diagnostic!($output, $crate::mark::Level::Warning, $msg)
    };
}

/// Asserts that an [`Output`](crate::Output) contains a note diagnostic
/// with the given message.
#[macro_export]
macro_rules! assert_diagnostic_note {
    ($output:expr, $msg:expr) => {
        $crate::assert_diagnostic!($output, $crate::mark::Level::Note, $msg)
    };
}

/// Asserts that an [`Output`](crate::Output) contains a help diagnostic
/// with the given message.
#[macro_export]
macro_rules! assert_diagnostic_help {
    ($output:expr, $msg:expr) => {
        $crate::assert_diagnostic!($output, $crate::mark::Level::Help, $msg)
    };
}

#[cfg(test)]
mod tests {
    use crate::Output;
    use crate::mark;

    #[test]
    fn error_diagnostic() {
        let output = Output::new()
            .diagnostic(mark::error("field not found"))
            .build();
        assert_diagnostic_error!(output, "field not found");
        assert_compile_error!(output, "field not found");
    }

    #[test]
    fn warning_diagnostic() {
        let output = Output::new()
            .diagnostic(mark::new().add(mark::warning("unused field")))
            .build();
        assert_diagnostic_warning!(output, "unused field");
    }

    #[test]
    fn note_diagnostic() {
        let output = Output::new()
            .diagnostic(mark::new().add(mark::note("derived from Foo")))
            .build();
        assert_diagnostic_note!(output, "derived from Foo");
    }

    #[test]
    fn help_diagnostic() {
        let output = Output::new()
            .diagnostic(mark::new().add(mark::help("add #[zyn(skip)]")))
            .build();
        assert_diagnostic_help!(output, "add #[zyn(skip)]");
    }

    #[test]
    #[should_panic(expected = "no")]
    fn missing_diagnostic() {
        let output = Output::default();
        assert_diagnostic_error!(output, "does not exist");
    }

    #[test]
    fn nested_diagnostic() {
        let output = Output::new()
            .diagnostic(mark::new().add(mark::error("outer").add(mark::help("inner hint"))))
            .build();
        assert_diagnostic_help!(output, "inner hint");
    }
}
