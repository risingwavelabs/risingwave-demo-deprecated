#!/usr/bin/env python3

from pyiceberg.catalog import load_catalog
from pyiceberg.schema import Schema
from pyiceberg.types import StringType, NestedField

schema = Schema(
    NestedField(field_id=1, name="user_id",
                field_type=StringType(), required=False),
    NestedField(field_id=2, name="target_id",
                field_type=StringType(), required=False),
    NestedField(field_id=3, name="event_timestamp",
                field_type=StringType(), required=False),
)

properties = {
    "uri": "http://rest:8181"
}
catalog = load_catalog("demo", **properties)

has_ns = False
for ns in catalog.list_namespaces():
    if ns[0] == "demo_db":
        has_ns = True
if not has_ns:
    catalog.create_namespace("demo_db")

catalog.create_table(
    identifier="demo_db.demo_table",
    location="s3a://hummock001/iceberg-data",
    schema=schema,
)
