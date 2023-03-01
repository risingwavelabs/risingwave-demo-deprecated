FROM python:3.9

RUN pip3 install "pyiceberg"
RUN pip3 install "pyarrow"

COPY iceberg.py /iceberg.py

CMD ["python3", "/iceberg.py"]
