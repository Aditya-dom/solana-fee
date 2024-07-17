#[macro_use]
extern crate snafu;

type ConnectionError = SnafuError<ConnectionSnafu>;
type DeserializationError = SnafuError<DeserializationSnafu>;

#[derive(Debug, Snafu)]
struct ConnectionSnafu {
    #[source]
    source: String,
}

#[derive(Debug, Snafu)]
struct DeserializationSnafu {
    #[source]
    source: serde_json::Error,
}
