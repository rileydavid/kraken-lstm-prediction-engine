from flask import Flask, request, jsonify
from model_execution import * 

app = Flask(__name__)

@app.route('/execute', methods=['POST'])
def execute_model():
    try:
        config = request.json
        model_execution = ModelExecution(config)
        timestamp, price = model_execution.execute()
        return jsonify({"bucket": timestamp, "price": price.item()}), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    app.run(debug=False, host='0.0.0.0', port=7000, load_dotenv=True)
