package gen

import (
	"context"
	"datagen/sink"
	"datagen/sink/kafka"
	"datagen/sink/kinesis"
	"datagen/sink/postgres"
	"datagen/sink/pulsar"
)

type GeneratorConfig struct {
	Postgres postgres.PostgresConfig
	Kafka    kafka.KafkaConfig
	Pulsar   pulsar.PulsarConfig
	Kinesis  kinesis.KinesisConfig

	// Whether to print the content of every event.
	PrintInsert bool
	// The datagen mode, e.g. "ad-ctr".
	Mode string
	// The sink type.
	Sink string
	// The throttled requests-per-second.
	Qps int
}

type LoadGenerator interface {
	KafkaTopics() []string

	Load(ctx context.Context, outCh chan<- sink.SinkRecord)
}

const RwTimestampLayout = "2006-01-02 15:04:05.07"
