from flask_restful import reqparse, fields, marshal_with, Resource
from . import utils
from . import models
from .. import main

input_args = reqparse.RequestParser()
input_args.add_argument("url", type=str, help="Url of the wikipedia page", required=True)
input_args.add_argument("question", type=str, help="Question you have", required=True)

response_args = reqparse.RequestParser()
response_args.add_argument("result", type=str, help="Result of the query", required=True)

inputFields = {
    'url': fields.String,
    'question': fields.String,
    'id': fields.Integer
}

responseFields = {
    'result': fields.String,
    'id': fields.Integer
}

class QueryWiki(Resource):
    @marshal_with(responseFields)
    def post(self):
        args = input_args.parse_args()
        url = args['url']
        question = args['question']
        
        result_str = utils.process_data(url, question)
        
        response = main.ResponseModel(result_str)
        print(response.result)
        return response, 201