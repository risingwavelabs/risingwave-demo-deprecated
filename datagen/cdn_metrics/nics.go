package cdn_metrics

import (
	"context"
	"crypto/md5"
	"datagen/gen"
	"datagen/sink"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"strconv"
	"time"

	"gonum.org/v1/gonum/stat/distuv"
)

type nicsMetric struct {
	DeviceId    string  `json:"device_id"`
	MetricName  string  `json:"metric_name"`
	Aggregation string  `json:"aggregation"`
	NicName     string  `json:"nic_name"`
	ReportTime  string  `json:"report_time"`
	Bandwidth   float64 `json:"bandwidth"`
	Value       float64 `json:"metric_value"`
}

func (r *nicsMetric) ToPostgresSql() string {
	return fmt.Sprintf(
		`INSERT INTO %s
(device_id, metric_name, aggregation, nic_name, report_time, link_bandwidth, metric_value)
values ('%s', '%s', '%s' '%s', '%s', '%f', '%f')`,
		"nics_metrics", r.DeviceId, r.MetricName, r.Aggregation, r.NicName, r.ReportTime, r.Bandwidth, r.Value)
}

func (r *nicsMetric) ToKafka() (topic string, data []byte) {
	data, _ = json.Marshal(r)
	return "nics_metrics", data
}

// Each device has a nics monitor.
type deviceNicsMonitor struct {
	deviceId string
	// Bandwidth in bytes.
	Bandwidth int64
}

func newDeviceNicsMonitor(id int) deviceNicsMonitor {
	hash := md5.Sum([]byte(strconv.Itoa(id)))
	return deviceNicsMonitor{
		deviceId:  hex.EncodeToString(hash[:]),
		Bandwidth: 10 * 1024 * 1024 * 1024 / 8, // 10Gb
	}
}

func (m *deviceNicsMonitor) emulate(ctx context.Context, outCh chan<- sink.SinkRecord) {
	for {
		metrics := m.generate()
		for _, metric := range metrics {
			select {
			case outCh <- metric:
			case <-ctx.Done():
				return
			}
		}
		select {
		case <-ctx.Done():
		case <-time.NewTicker(60 * time.Second).C:
		}
	}
}

func (impl *deviceNicsMonitor) generate() []*nicsMetric {
	curTime := time.Now()
	var metrics []*nicsMetric
	for nicId := 0; nicId < 4; nicId++ {
		txBytesAvg := distuv.Poisson{
			Lambda: float64(impl.Bandwidth) / 100,
		}.Rand()
		txBytesPeak := distuv.Poisson{
			Lambda: 1.3,
		}.Rand() * txBytesAvg
		metrics = append(metrics,
			impl.newMetrics(nicId, "tx_bytes", "avg", curTime, int64(txBytesAvg)),
			impl.newMetrics(nicId, "tx_bytes", "peak", curTime, int64(txBytesPeak)),
		)
	}
	return metrics
}

func (impl *deviceNicsMonitor) newMetrics(
	NicId int,
	metricName string,
	aggregation string,
	reportTime time.Time,
	value int64) *nicsMetric {

	return &nicsMetric{
		DeviceId:    impl.deviceId,
		MetricName:  metricName,
		Aggregation: aggregation,
		NicName:     "eth" + strconv.Itoa(NicId),
		ReportTime:  reportTime.Format(gen.RwTimestampLayout),
		Bandwidth:   float64(impl.Bandwidth),
		Value:       float64(value),
	}
}
