package sink

import (
	"context"
	"fmt"

	"github.com/apache/pulsar-client-go/pulsar"
)

type PulsarSink struct {
	client    pulsar.Client
	producers map[string]pulsar.Producer
}

func OpenPulsarSink(ctx context.Context, brokers string) (*PulsarSink, error) {
	client, err := pulsar.NewClient(pulsar.ClientOptions{
		URL: fmt.Sprintf("pulsar://%s", brokers),
	})
	if err != nil {
		return nil, err
	}
	return &PulsarSink{
		client:    client,
		producers: make(map[string]pulsar.Producer),
	}, nil
}

func (p *PulsarSink) Close() error {
	p.client.Close()
	return nil
}

func (p *PulsarSink) WriteRecord(ctx context.Context, record SinkRecord) error {
	var err error
	topic, data := record.ToKafka()
	producer, ok := p.producers[topic]
	if !ok {
		producer, err = p.client.CreateProducer(pulsar.ProducerOptions{
			Topic: topic,
		})
		if err != nil {
			return err
		}
		p.producers[topic] = producer
	}
	_, err = producer.Send(ctx, &pulsar.ProducerMessage{
		Payload: data,
	})
	return err
}
