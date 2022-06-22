package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/Shopify/sarama"
)

type sinkRecord interface {
	toPostgresSql() string
	toKafka() (topic string, data []byte)
}

type iSink interface {
	writeRecord(ctx context.Context, record sinkRecord) error
	close() error
}

type postgresSink struct {
	db *sql.DB
}

func openPostgresSink() (*postgresSink, error) {
	db, err := sql.Open("postgres", fmt.Sprintf("postgresql://%s:@%s:%d/%s?sslmode=disable", dbUser, dbHost, dbPort, database))
	if err != nil {
		return nil, err
	}
	return &postgresSink{db}, nil
}

func (p *postgresSink) close() error {
	return p.db.Close()
}

func (p *postgresSink) writeRecord(ctx context.Context, record sinkRecord) error {
	query := record.toPostgresSql()
	_, err := p.db.ExecContext(ctx, query)
	return err
}

type kafkaSink struct {
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

func openKafkaSink() (*kafkaSink, error) {
	client, err := sarama.NewAsyncProducer(strings.Split(brokers, ","), newKafkaConfig())
	if err != nil {
		return nil, fmt.Errorf("NewAsyncProducer failed: %v", err)
	}
	return &kafkaSink{client: client}, nil
}

func createRequiredTopics(keys []string) error {
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
		for k, _ := range topics {
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

func (p *kafkaSink) close() error {
	return p.client.Close()
}

func (p *kafkaSink) writeRecord(ctx context.Context, record sinkRecord) error {
	topic, data := record.toKafka()
	msg := &sarama.ProducerMessage{}
	msg.Topic = topic
	msg.Key = sarama.StringEncoder(topic)
	msg.Value = sarama.ByteEncoder(data)
	select {
	case <-ctx.Done():
	case p.client.Input() <- msg:
	}
	return nil
}

func loadGen(ctx context.Context) error {
	sinkImpl := iSink(nil)
	err := error(nil)
	if sink == "postgres" {
		sinkImpl, err = openPostgresSink()
	} else if sink == "kafka" {
		sinkImpl, err = openKafkaSink()
	} else {
		return fmt.Errorf("invalid sink type: %s", sink)
	}
	if err != nil {
		return err
	}
	defer func() {
		if err = sinkImpl.close(); err != nil {
			log.Print(err)
		}
	}()
	if mode == "ad-click" {
		return loadAdClick(ctx, sinkImpl)
	} else if mode == "ad-ctr" {
		return nil // TODO
	} else {
		return fmt.Errorf("invalid mode: %s", mode)
	}
}
