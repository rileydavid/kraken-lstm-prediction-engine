FROM python:3.10.13-slim 

WORKDIR /usr/src/app

# copy directory contents into the container
COPY . /usr/src/app

RUN pip install --upgrade pip
# install requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

RUN mkdir model

# launch model service 
CMD ["gunicorn", "-b", "0.0.0.0:7000", "wsgi:app"]

