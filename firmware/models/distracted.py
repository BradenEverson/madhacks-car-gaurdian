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
    img = img.transpose(Image.FLIP_TOP_BOTTOM)

    img_array = np.array(img, dtype=np.float32)
    
    grayscale_image = 0.2989 * img_array[:, :, 0] + 0.5870 * img_array[:, :, 1] + 0.1140 * img_array[:, :, 2]
    
    grayscale_image = np.flipud(grayscale_image)
    
    grayscale_image /= 255.0
    
    grayscale_image = np.expand_dims(grayscale_image, axis=0)
    
    return grayscale_image

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
