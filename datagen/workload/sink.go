package workload

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/Shopify/sarama"
)

type GeneratorConfig struct {
	DbHost      string
	Database    string
	DbPort      int
	DbUser      string
	PrintInsert bool
	Mode        string
	Sink        string
	Qps         int
	Brokers     string
}

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

type PostgresSink struct {
	db *sql.DB
}

func OpenPostgresSink(cfg GeneratorConfig) (*PostgresSink, error) {
	db, err := sql.Open("postgres", fmt.Sprintf("postgresql://%s:@%s:%d/%s?sslmode=disable",
		cfg.DbUser, cfg.DbHost, cfg.DbPort, cfg.Database))
	if err != nil {
		return nil, err
	}
	return &PostgresSink{db}, nil
}

func (p *PostgresSink) Close() error {
	return p.db.Close()
}

func (p *PostgresSink) WriteRecord(ctx context.Context, record SinkRecord) error {
	query := record.ToPostgresSql()
	_, err := p.db.ExecContext(ctx, query)
	return err
}

type KafkaSink struct {
	client sarama.AsyncProducer
}

func newKafkaConfig() *sarama.Config {
	version, err := sarama.ParseKafkaVersion("1.1.1")
	if err != nil {
		panic(fmt.Sprintf("failed to parse Kafka version: %v", err))
	}
	config := sarama.NewConfig()
	config.Version = version
	config.Net.DialTimeout = 3 * time.Second
	return config
}

func OpenKafkaSink(ctx context.Context, cfg GeneratorConfig) (*KafkaSink, error) {
	client, err := sarama.NewAsyncProducer(strings.Split(cfg.Brokers, ","), newKafkaConfig())
	if err != nil {
		return nil, fmt.Errorf("NewAsyncProducer failed: %v", err)
	}
	p := &KafkaSink{client: client}
	go func() {
		p.consumeSuccesses(ctx)
	}()
	return p, nil
}

func (p *KafkaSink) consumeSuccesses(ctx context.Context) {
	for {
		select {
		case <-ctx.Done():
			return
		case <-p.client.Successes():
		}
	}
}

func createRequiredTopics(brokers string, keys []string) error {
	admin, err := sarama.NewClusterAdmin(strings.Split(brokers, ","), newKafkaConfig())
	if err != nil {
		return err
	}
	topics, err := admin.ListTopics()
	if err != nil {
		return err
	}
	if len(topics) != 0 {
		var topicNames []string
		for k := range topics {
			topicNames = append(topicNames, k)
		}
		log.Printf("Existing topics: %s", topicNames)
	}
	for _, t := range keys {
		if err := createTopic(admin, t, topics); err != nil {
			return err
		}
	}
	return nil
}

func createTopic(admin sarama.ClusterAdmin, key string, topics map[string]sarama.TopicDetail) error {
	if _, exists := topics[key]; exists {
		if err := admin.DeleteTopic(key); err != nil {
			log.Printf("Deleted an existing topic: %s", key)
			return err
		}
	}
	log.Printf("Creating topic: %s", key)
	return admin.CreateTopic(key, &sarama.TopicDetail{
		NumPartitions:     16,
		ReplicationFactor: 1,
	}, false)
}

func (p *KafkaSink) Close() error {
	p.client.AsyncClose()
	return nil
}

func (p *KafkaSink) WriteRecord(ctx context.Context, record SinkRecord) error {
	topic, data := record.ToKafka()
	msg := &sarama.ProducerMessage{}
	msg.Topic = topic
	msg.Key = sarama.StringEncoder(topic)
	msg.Value = sarama.ByteEncoder(data)
	select {
	case <-ctx.Done():
	case p.client.Input() <- msg:
	case err := <-p.client.Errors():
		log.Printf("failed to produce message: %s", err)
	}
	return nil
}

func LoadGen(ctx context.Context, cfg GeneratorConfig) error {
	sinkImpl := Sink(nil)
	err := error(nil)
	if cfg.Sink == "postgres" {
		sinkImpl, err = OpenPostgresSink(cfg)
	} else if cfg.Sink == "kafka" {
		sinkImpl, err = OpenKafkaSink(ctx, cfg)
	} else {
		return fmt.Errorf("invalid sink type: %s", cfg.Sink)
	}
	if err != nil {
		return err
	}
	defer func() {
		if err = sinkImpl.Close(); err != nil {
			log.Print(err)
		}
	}()
	if cfg.Mode == "ad-click" {
		return LoadAdClick(ctx, cfg, sinkImpl)
	} else if cfg.Mode == "ad-ctr" {
		return nil // TODO
	} else if cfg.Mode == "twitter" {
		return LoadTwitterEvents(ctx, cfg, sinkImpl)
	} else {
		return fmt.Errorf("invalid mode: %s", cfg.Mode)
	}
}
