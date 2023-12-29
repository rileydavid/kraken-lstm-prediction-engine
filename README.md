# trading

# How to launch: 
Navigate to the root of the project (where the docker-compose.yaml is located)

* Build and launch 
    docker-compose --env-file config.env up --build 

* Launch (if the container has already been built)
    docker-compose --env-file config.env up 

# Initial setup of Minio: 
Navigate to: http://172.1.0.12:2001/login 
Login using the specified minio_user and minio_password (see config.env file)

Create two buckets:
* mlflow-models (only needed when training new models)
* prod-model (used for storing production ready models)

Upload model.pkl into the prod-model bucket which is located in ./data/model-ethusd

Create access keys: 
Replace the current access keys in the config.env file
MINIO_ACCESS_KEY=eITEO5kyE7hccuy7UTHv
MINIO_SECRET_ACCESS_KEY=5KBCscit30Z70bSVGIMvMBBoqV8ydn232o2MW9RA

restart the containers using 
1. docker-compose --env-file config.env down
2. docker-compose --env-file config.env up 

# Import Data
Download historical data: https://drive.google.com/file/d/1w8tAQn9vR_MC0fY54WJtv5fZr_gCsJR0/view

Extract: ETHUSD.csv

Copy/Move file to ./data/import

-- Call endpoint using get: localhost:8000/import/ETHUSD.csv/ETHUSD

this will convert the data and insert it into trade:

-- still missing continous aggregate refresh... could be easily fixed by calling it after inserting

# Launch Frontend: 
localhost:4200

