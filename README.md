# Workload Generator

This is a tool that populates fake streaming data to a sink, for example, a Kafka.
The data is generated according to the schema configuration:

```yml
# The throughput per second.
qps: 10
# The total lines of records.
total: 1000000
# The data format. We currently only support JSON.
format: json

# The destination where the data will be sent to.
# We support Kafka and Stdout.
connector:
  kafka:
    broker: 127.0.0.1:29092
    topic: test_topic
    timeout_ms: 1000

# The schema configuration.
schema:
  user_id:
    type: long
    cardinality: 10
  start_time:
    type: timestamp
  end_time:
    type: timestamp
    delay: 1s
  platform:
    type: enum
    enum:
    - ios
    - android
```

## Data Types

We have supported multiple data types so far, including:

- `long`: i64.
- `int`: i32.
- `float`: f64.
- `string`: string.
- `stringzh`: A Chinese string.
- `timestamp`: a timestamp formatted with `2022-05-09 13:26:07.396503`. By default, it will be the time when it's generated.
    - If `random_delay` is set, it will added with a random value within `[0, random_delay]`.
- `enum`: The value will be randomized through a predefined set of variants.
    - The variants must be provided in the `enum` section.

Except for enum, every type can be configured with a `cardinality` which is the number of distinct values.
In some use cases, we may want to limit the value space of a field, for example, we may want to count the sales of each item in the market. If the item is completely random, the transactions on every item could be always 1.
