import sys
import os
import tensorflow as tf
from PIL import Image
import numpy as np

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '3'
tf.get_logger().setLevel('ERROR')

def process_image(image_path):
    img = Image.open(image_path)
    
    img = img.resize((640, 640))
    
    img = img.convert("L")
    img = img.transpose(Image.FLIP_TOP_BOTTOM)

    #img.save("Flipped Grayscale.jpg")
    
    img_array = np.array(img, dtype=np.float32)
    
    img_array /= 255.0
    
    img_array = np.expand_dims(img_array, axis=0)
    
    return img_array

def load_and_predict(model_path, image_array):
    model = tf.keras.models.load_model(model_path)
    
    prediction = model.predict(image_array)
    
    return prediction

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python script.py <path_to_image>")
        sys.exit(1)
    
    image_path = sys.argv[1]
    
    model_path = "models/distracted_driver_model_92.keras"
    
    processed_image = process_image(image_path)
    
    prediction = load_and_predict(model_path, processed_image)
    
    print(prediction[0][0])

