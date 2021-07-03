# syntax=docker/dockerfile:1

# Buster python3-* packages will assume v3.7.
FROM tiangolo/uvicorn-gunicorn-fastapi:python3.7
ENV DISTRIBUTION=buster
ENV PYTHONPATH=/usr/lib/python3/dist-packages

# Install graph-tool and dependencies:
RUN apt-get update
RUN apt-get install -y gnupg software-properties-common
RUN add-apt-repository "deb [ arch=amd64 ] https://downloads.skewed.de/apt $DISTRIBUTION main"
RUN apt-key adv --keyserver keys.openpgp.org --recv-key 612DEFB798507F25
RUN apt-get update
RUN apt-get install -y python3-cairo python3-matplotlib python3-graph-tool

# Install py-school-match:
RUN pip install py-school-match

COPY ./app /app