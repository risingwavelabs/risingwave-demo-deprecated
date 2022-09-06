package kafka

import (
	"context"
	"datagen/sink"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/Shopify/sarama"
)

type KafkaConfig struct {
	Brokers string
}

type KafkaSink struct {
	admin   sarama.ClusterAdmin
	brokers string
	client  sarama.AsyncProducer
}

func newKafkaConfig() *sarama.Config {
	version, err := sarama.ParseKafkaVersion("1.1.1")
	if err != nil {
		panic(fmt.Sprintf("failed to parse Kafka version: %v", err))
	}
	config := sarama.NewConfig()
	config.Version = version
	config.Net.DialTimeout = 3 * time.Second
	config.Admin.Timeout = 5 * time.Second
	config.Producer.Timeout = 5 * time.Second
	return config
}

func OpenKafkaSink(ctx context.Context, cfg KafkaConfig) (*KafkaSink, error) {
	admin, err := sarama.NewClusterAdmin(strings.Split(cfg.Brokers, ","), newKafkaConfig())
	if err != nil {
		return nil, err
	}
	topics, err := admin.ListTopics()
	if err != nil {
		return nil, err
	}
	var topicNames []string
	for k := range topics {
		topicNames = append(topicNames, k)
	}
	log.Printf("Existing topics: %s", topicNames)
	client, err := sarama.NewAsyncProducer(strings.Split(cfg.Brokers, ","), newKafkaConfig())
	if err != nil {
		return nil, fmt.Errorf("NewAsyncProducer failed: %v", err)
	}
	p := &KafkaSink{
		admin:   admin,
		brokers: cfg.Brokers,
		client:  client,
	}
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

func CreateRequiredTopics(admin sarama.ClusterAdmin, keys []string) error {
	topics, err := admin.ListTopics()
	if err != nil {
		return err
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

func (p *KafkaSink) Prepare(topics []string) error {
	return CreateRequiredTopics(p.admin, topics)
}

func (p *KafkaSink) Close() error {
	p.client.AsyncClose()
	return nil
}

func (p *KafkaSink) WriteRecord(ctx context.Context, record sink.SinkRecord) error {
	topic, data := record.ToKafka()
	msg := &sarama.ProducerMessage{}
	msg.Topic = topic
	msg.Key = nil
	msg.Value = sarama.ByteEncoder(data)
	select {
	case <-ctx.Done():
	case p.client.Input() <- msg:
	case err := <-p.client.Errors():
		log.Printf("failed to produce message: %s", err)
	}
	return nil
}
