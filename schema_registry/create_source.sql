CREATE SOURCE student WITH (
    connector = 'kafka',
    kafka.topic = 'sr-test',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
)
ROW FORMAT avro message 'student'
row schema location confluent schema registry 'http://message_queue:8081';