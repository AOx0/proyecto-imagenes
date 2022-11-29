import cv2

imgp = '{img}'
imagen = cv2.imread(imgp)
imagen = cv2.cvtColor(imagen, cv2.COLOR_BGR2GRAY)
imagen_eq = cv2.equalizeHist(imagen)
cv2.imwrite('{}/img.png', imagen_eq)