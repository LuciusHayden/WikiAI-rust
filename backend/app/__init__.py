from flask import Flask
from flask_cors import CORS

# Initialize the Flask app
def create_app():
    app = Flask(__name__)

    # Enable CORS if you're using a frontend like Angular
    CORS(app)

    # Import routes
    from .routes import main
    app.register_blueprint(main)

    return app