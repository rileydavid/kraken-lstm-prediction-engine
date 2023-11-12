#FROM continuumio/miniconda3
#RUN apt-get update && apt-get install -y git
#
#RUN conda install -y -c conda-forge shap  # not sure if this is needed 
#RUN pip install mlflow psycopg2-binary pymysql boto3
#
#RUN cd /home && git clone https://github.com/mlflow/mlflow-example.git
#
#EXPOSE 5000

FROM python:3.9-slim

WORKDIR /mlflow/

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt && \
  rm requirements.txt

EXPOSE 5000

ENV BACKEND_URI postgresql://admin:password@172.1.0.10:5432/mlflow
ENV ARTIFACT_ROOT /storage/mlruns

CMD mlflow server --backend-store-uri ${BACKEND_URI} --default-artifact-root ${ARTIFACT_ROOT} --host 0.0.0.0 --port 5000