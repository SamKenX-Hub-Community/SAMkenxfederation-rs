//! This file includes types used to convert JSON from the function in do_compose.js
//! into strongly typed Rust data structures.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display};

use apollo_federation_types::build::BuildError;

/// An error which occurred during JavaScript composition.
///
/// The shape of this error is meant to mimic that of the error created within
/// JavaScript, which is a [`GraphQLError`] from the [`graphql-js`] library.
///
/// [`graphql-js']: https://npm.im/graphql
/// [`GraphQLError`]: https://github.com/graphql/graphql-js/blob/3869211/src/error/GraphQLError.js#L18-L75
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct CompositionError {
    /// A human-readable description of the error that prevented composition.
    message: Option<String>,

    #[serde(flatten)]
    /// [`JsCompositionErrorExtensions`]
    extensions: Option<JsCompositionErrorExtensions>,
}

impl CompositionError {
    pub(crate) fn generic(message: String) -> Self {
        Self {
            message: Some(message),
            extensions: None,
        }
    }
}

impl Display for CompositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = if let Some(extensions) = &self.extensions {
            &extensions.code
        } else {
            "UNKNOWN"
        };
        if let Some(message) = &self.message {
            write!(f, "{}: {}", code, &message)
        } else {
            write!(f, "{code}")
        }
    }
}

impl From<CompositionError> for BuildError {
    fn from(input: CompositionError) -> Self {
        let code = input.extensions.map(|x| x.code);
        let message = input.message;
        Self::composition_error(code, message)
    }
}

impl Error for CompositionError {}

/// Mimicking the JavaScript-world from which this error comes, this represents
/// the `extensions` property of a JavaScript [`GraphQLError`] from the
/// [`graphql-js`] library. Such errors are created when errors have prevented
/// successful composition, which is accomplished using [`errorWithCode`]. An
/// [example] of this can be seen within the `federation-js` JavaScript library.
///
/// [`graphql-js']: https://npm.im/graphql
/// [`GraphQLError`]: https://github.com/graphql/graphql-js/blob/3869211/src/error/GraphQLError.js#L18-L75
/// [`errorWithCode`]: https://github.com/apollographql/federation/blob/d7ca0bc2/federation-js/src/composition/utils.ts#L200-L216
/// [example]: https://github.com/apollographql/federation/blob/d7ca0bc2/federation-js/src/composition/validate/postComposition/executableDirectivesInAllServices.ts#L47-L53
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct JsCompositionErrorExtensions {
    /// An Apollo Federation composition error code.
    ///
    /// A non-exhaustive list of error codes that this includes, is:
    ///
    ///   - EXTERNAL_TYPE_MISMATCH
    ///   - EXTERNAL_UNUSED
    ///   - KEY_FIELDS_MISSING_ON_BASE
    ///   - KEY_MISSING_ON_BASE
    ///   - KEY_NOT_SPECIFIED
    ///   - PROVIDES_FIELDS_MISSING_EXTERNAL
    ///
    /// ...and many more!  See the `federation-js` composition library for
    /// more details (and search for `errorWithCode`).
    code: String,

    #[serde(flatten)]
    other: BTreeMap<String, Value>,
}
