package sink

import (
	"context"
)

type SinkRecord interface {
	// Convert the event to an INSERT INTO command.
	ToPostgresSql() string

	// Convert the event to a Kakfa message in JSON format.
	// This interface will also be used for Pulsar.
	ToKafka() (topic string, data []byte)
}

type Sink interface {
	WriteRecord(ctx context.Context, record SinkRecord) error

	Close() error
}
