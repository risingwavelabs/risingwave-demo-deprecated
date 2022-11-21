package sink

import (
	"context"
)

type SinkRecord interface {
	// Convert the event to an INSERT INTO command.
	ToPostgresSql() string

	// Convert the event to a Kakfa message in JSON format.
	// This interface will also be used for Pulsar and Kinesis.
	ToJson() (topic string, key string, data []byte)

	// Convert the event to a Kakfa message in Protobuf format.
	// This interface will also be used for Pulsar and Kinesis.
	ToProtobuf() (topic string, key string, data []byte)
}

// Convert the event to a Kakfa message in the given format.
// This interface will also be used for Pulsar and Kinesis.
func RecordToKafka(r SinkRecord, format string) (topic string, key string, data []byte) {
	if format == "json" {
		return r.ToJson()
	} else if format == "protobuf" {
		return r.ToProtobuf()
	} else {
		panic("unsupported format")
	}
}

type Sink interface {
	Prepare(topics []string) error

	WriteRecord(ctx context.Context, format string, record SinkRecord) error

	Close() error
}
