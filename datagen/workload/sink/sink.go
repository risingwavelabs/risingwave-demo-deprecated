package sink

import (
	"context"
)

type SinkRecord interface {
	// Convert the event to an INSERT INTO command to a Postgres table.
	ToPostgresSql() string

	// Convert the event to a Kakfa message to a specific topic.
	ToKafka() (topic string, data []byte)
}

type Sink interface {
	WriteRecord(ctx context.Context, record SinkRecord) error

	Close() error
}
