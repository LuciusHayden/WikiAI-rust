from flask import Flask
from flask_cors import CORS
from flask_restful import Api
from .app import resources
from flask_sqlalchemy import SQLAlchemy

app = Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///database.db'
db = SQLAlchemy(app)
api = Api(app)

class InputModel(db.Model):
    url = db.Column(db.String(80), unique=True, nullable=False)
    question = db.Column(db.String(120), unique=True, nullable=False)
    id = db.Column(db.Integer, primary_key=True)

    def __repr__(self):
        return f"{self.url} - {self.question}"

class ResponseModel(db.Model):
    result = db.Column(db.String(120), unique=True, nullable=False)
    id = db.Column(db.Integer, primary_key=True)
    def __init__(self, result):
        self.result = result

    def __repr__(self):
        return f"{self.result}"
    
# API routes     
api.add_resource(resources.QueryWiki, '/api/query')

CORS(app, resources={r"/api/*": {"origins": "http://localhost:4200", "methods": ["GET", "POST"]}})

# moved to run.py
# if __name__ == '__main__':
#     app.run(debug=True)