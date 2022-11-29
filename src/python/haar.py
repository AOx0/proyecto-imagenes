import cv2 as cv
import numpy as np

def haar_cascade(ruta: str, ext: str, out_dir: str, xml: str):
    img = cv.imread(ruta)
    img_arr = np.array(img)
    img = img[:,:,::-1]
    imgray = cv.cvtColor(img, cv.COLOR_BGR2GRAY)
    blur = cv.GaussianBlur(imgray,(5,5),0)
    dilated = cv.dilate(blur,np.ones((3,3)))
    kernel = cv.getStructuringElement(cv.MORPH_ELLIPSE, (2, 2))
    closing = cv.morphologyEx(dilated, cv.MORPH_CLOSE, kernel)
    car_cascade = cv.CascadeClassifier(xml)
    cars = car_cascade.detectMultiScale(closing, 1.1, 1)
    cnt = 0
    for (x,y,w,h) in cars:
        cv.rectangle(img_arr, (x, y), (x + w, y + h), (255, 0, 0), 2)
        cnt += 1
    
    path = rf"{out_dir}/img.{ext}"
    if cv.imwrite(path, img_arr):
        return (cnt, rf"{path}")
    else:
        return (cnt, "ERROR")