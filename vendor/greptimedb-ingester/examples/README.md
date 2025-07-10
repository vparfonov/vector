## Examples Descriptions

- **config_utils.rs**  
  Provides utilities for managing database connection configurations. It includes functions to read connection parameters from environment variables or from a configuration file (`db-connection.toml`). Used for sharing configuration logic among the examples.

- **db-connection.toml**  
  A sample database connection configuration file. Defines the GreptimeDB endpoints and database name for the examples to use as default connection parameters.

- **high_level_ingest.rs**  
  Demonstrates how to use the high-level API (via the `Database` object) to batch insert records into GreptimeDB. This example shows basic row insertion using a weather data schema.

- **high_level_stream_ingest.rs**  
  Demonstrates high-level streaming ingestion using the client’s `streaming_inserter` API. It shows how to write batches of data in a streaming fashion, suitable for scenarios requiring continuous or batched ingestion.

- **low_level_ingest.rs**  
  Shows usage of the low-level API for inserting different types of data into GreptimeDB. This includes sensor data and batch insertion with various data types, suitable for advanced scenarios requiring custom schemas.

- **low_level_stream_ingest.rs**  
  Demonstrates low-level streaming ingestion, including:
    - Streaming sensor data continuously.
    - Concurrent streaming of multiple data types (e.g., metrics and events).
    - Error handling and recovery during streaming.
    - Batch processing for streaming scenarios.
      This is the most advanced example and covers robust ingestion patterns.