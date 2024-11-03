import redis

def test_redis_operations():
    # Connect to Redis server
    client = redis.StrictRedis(host='localhost', port=8080, db=0)

    # Test data
    test_data = {
        'string_key': 'Hello, Redis!',
        'int_key': 42,
        'float_key': 3.14,
        'list_key': ['a', 'b', 'c'],
        'hash_key': {'field1': 'value1', 'field2': 'value2'},
    }

    # Test SET and GET for strings, integers, and floats
    for key, value in test_data.items():
        if isinstance(value, (str, int, float)):
            client.set(key, value)
            retrieved_value = client.get(key)

            # Decode if the value is a string, convert if it's an int or float
            if isinstance(value, str):
                retrieved_value = retrieved_value.decode('utf-8')
            elif isinstance(value, int):
                retrieved_value = int(retrieved_value)
            elif isinstance(value, float):
                retrieved_value = float(retrieved_value)

            # Assert retrieved value
            assert retrieved_value == value, f"Expected '{value}', but got '{retrieved_value}'"

 
  
    print("All tests passed successfully!")

if __name__ == "__main__":
    test_redis_operations()
