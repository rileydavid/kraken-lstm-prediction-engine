import boto3
from botocore.exceptions import NoCredentialsError, ClientError

class ModelProvider:
    def __init__(self, bucket_name, model_name, access_key, secret_key, endpoint_url):
        self.bucket_name = bucket_name
        self.model_name = model_name
        self.client = boto3.resource(
            's3',
            aws_access_key_id=access_key,
            aws_secret_access_key=secret_key,
            endpoint_url=endpoint_url  
        )

    def fetch_model(self, local_path):
        try:
            print("Starting Download")
            self.client.meta.client.download_file(self.bucket_name, self.model_name, local_path)
            print(f"Model downloaded successfully: {local_path}")
        except NoCredentialsError:
            print("Credentials not available for S3.")
        except ClientError as e:
            print(f"Failed to download the file: {e}")


