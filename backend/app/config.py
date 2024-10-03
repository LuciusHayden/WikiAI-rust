import os
from dotenv import load_dotenv

# Load environment variables from the .env file
load_dotenv()

OPENAI_KEY = os.getenv("OPENAI_KEY")
ORGANIZATION_KEY = os.getenv("ORGANIZATION_KEY")
