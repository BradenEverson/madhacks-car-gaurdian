# Madhacks Hackathon Car Guardian Project
Welcome to the Car Guardian repository! This is a repository dedicated to storing code towards the Car Guardian project! The Car Guardian is a project that is designed to alert drivers whenever they're distracted
or about to pass out from a health-related issue. For more information, be sure to roam around the repository!

## Inspiration
Distracted driving is still an issue today like how it was years ago, with over 3,000 individuals dying each year in the U.S. and tens to hundreds of thousands more injured, [Forbes](https://www.forbes.com/advisor/legal/auto-accident/distracted-driving-statistics/) we wanted to create a solution that leverages advanced AI to help keep drivers focused, ultimately aiming to reduce accidents and improve road safety.
 
## What it Does
Car Guardian uses real-time analysis to monitor driver attention. Powered by a convolutional neural network (CNN), and ran by a high performance, Rust based embedded execution engine run through a Jetson Orin Nano. It detects distractions instantly, providing alerts via peripheral interfaces to refocus drivers and prevent accidents.
 
## How We Built It
We trained our CNN model on the [ROSIE Supercomputer](https://www.msoe.edu/about-msoe/news/details/meet-rosie/) enabling high-speed data processing and accuracy. We trained the model using a dataset of distracted drivers on [Roboflow](https://universe.roboflow.com/yolov8-z7kip/distracted-driver-detection-bvtnl) through Jupyter Notebooks. The model itself is serialized and hosted on the Jetson, where the main firmware interfaces directly with Python code to submit image buffers for classification. From the callback that Python writes to Rust, actions such as an alertive buzzer or automatic hazard lights are performed.
 
## Retrieving the Dataset, Performing Data Pre-Processing & Building / Running the CNN Model
### Getting the Datasets
We had retrieved the distracted driving datasets from Roboflow using the Roboflow API. Using the secret key, we had downloaded the dataset onto the ROSIE Supercomputer, and had begun the pre-processing stage.
 
### Performing Data Preprocessing
We perform data pre-processing by first extracting the the columns from the `.csv` file, which contains binary values on whether a driver is distracted or not. We then moved on to open the images and then gray scaling and normalizing them via the numpy arrays of the images.
 
### Building the CNN Model
We then perform data augmentation which generates new & varied versions of the training images in each epoch. We then build a sequential model with the input, convolutional, max pooling, flatten, dense and output layers. Once the model had been trained, we compile and run the model and have it exported.
 
## Challenges We Faced
Integrating real-time data processing and achieving accuracy in distraction detection posed challenges, as did optimizing model performance on embedded systems. Another challenge faced was being able to learn how to build a CNN model, and the figure out how perform the data pre-processing so that the model can be trained from the pre-processing of the data. Next, we had to figure out how to improve the accuracy of the model, and had to add and change the parameters of the layers in the Convolutional Neural Network
 
## Accomplishments We’re Proud Of
We're proud of our successful use of the ROSIE Supercomputer to train a high-performing model and our ability to integrate AI with robust real-time processing for improved driver safety. Furthermore, the design choices made allow for the combination of two extremely powerful languages. Through Rust, we are able to enable a high performance embedded firmware that can stream webcam footage through both a web-based frontend and to the Python interface. With Python, we are able to leverage the powers of TensorFlow and NumPy to evaluate models efficiently. Another great accomplishment is managing to improve the accuracy of the model from 64% to a whopping 96% version. The combination of these two in a effective matter allows for a performant real time system that can potentially save lives.
 
## What We Learned
This project deepened our understanding of deploying machine learning models on embedded systems and reinforced the importance of optimizing for speed and accuracy in real-world applications. Furthermore, we developed a brand new method for interfacing between Rust and Python. Using this method, we are able to leverage the capabilities of these languages in a really cool way. This project touched almost every aspect of the Software Engineering world, from low level firmware development to ML training and Data Science analysis, we were able to learn and harness several technologies new to us in a really *mad* way. We also learnt how a CNN model works, what layers are involved, and how to prepare image files and open them for data pre-processing. Lastly, we worked a little bit on web development to document our progress and make our GitHub and project more professional.
 
## What’s Next for Car Guardian
Future plans include expanding detection capabilities to recognize a broader range of distractions and enhancing the user interface to improve accessibility and ease of use. We plan to use this system on our drive home from MadHacks, keeping us all safer!
Distracted Driving Statistics & Facts In 2024
Distracted driving is one of the most dangerous behaviors on the road today. But what exactly is distracted driving and how does it impact your risks behind the wheel? These distracted driving stat...
 
