import sys
from PIL import Image
import numpy as np

def process_image(image_path):
    img = Image.open(image_path)
    
    img = img.resize((640, 640))
    
    img = img.convert("L")
    img.save("rescaled.jpg")
    
    img_array = np.array(img, dtype=np.float32)
    
    img_array /= 255.0
    
    return img_array

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python script.py <path_to_image>")
        sys.exit(1)
    
    image_path = sys.argv[1]
    
    processed_image = process_image(image_path)
    
    print(0)
