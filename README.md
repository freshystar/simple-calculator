## Creating a simple calculator in rust

This calculator does simple operations like, addition, subtraction, multiplication and division.

### Pulling the image
 ```sh
 $ docker pull ghcr.io/freshystar/calc-image:latest
 ```
 Paste this image on your terminal and run it to have it locally.

 ### Running the container
 After pasting the image, you can do:
 ```sh
 $ docker images
 ```
 This will be to see the image you just created. Copy the *image ID* under **IMAGE ID**.
 
 ```sh
 $ docker run -it <image/Id> 
 ```
You will be logged in directly into the container and you will just have to follow the prompts meticulously. The container will be exited immidiately after displaying the result or any in case of any invalid input.