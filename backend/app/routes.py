from flask import Blueprint, request, jsonify
from .utils import process_data

# Create a Blueprint for the main route
main = Blueprint('main', __name__)

@main.route('/api/scrape', methods=['POST'])
def scrape_and_process():
    data = request.json
    url = data.get('url', 'https://en.wikipedia.org/wiki/Cookie')  # Default URL if not provided
    question = data.get('question', '')

    # Call the function with the Wikipedia URL and user's question
    result = process_data(url, question)
    return jsonify({'result': result})
