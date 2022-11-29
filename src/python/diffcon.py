import cv2 as cv
import numpy as np

def calculare_diff(img1: str, img2: str, ext: str, out_dir: str):
    img1 = cv.imread(rf"{img1}")
    img2 = cv.imread(rf"{img2}")

    # Resizing the images to max 500 pixels
    img1 = cv.resize(img1, (500, 500))
    img2 = cv.resize(img2, (500, 500))

    # Convert to grayscale and apply Gaussian blur
    img1_gray = cv.cvtColor(img1, cv.COLOR_BGR2GRAY)
    img1_blur = cv.GaussianBlur(img1_gray, (5, 5), 0)

    img2_gray = cv.cvtColor(img2, cv.COLOR_BGR2GRAY)
    img2_blur = cv.GaussianBlur(img2_gray, (5, 5), 0)

    # Binary thresholding of the images
    ret, img1_thresh = cv.threshold(img1_blur, 127, 255, cv.THRESH_BINARY)
    ret, img2_thresh = cv.threshold(img2_blur, 127, 255, cv.THRESH_BINARY)

    # Get difference between images
    img_diff = cv.absdiff(img1_thresh, img2_thresh)

    # Reduce noise
    kernel = np.ones((8, 2), np.uint8)
    img_diff = cv.morphologyEx(img_diff, cv.MORPH_OPEN, kernel)

    # Dilate the image
    img_diff = cv.dilate(img_diff, kernel, iterations=5)

    # Show connected components in the image
    num_labels, labels, stats, centroids = cv.connectedComponentsWithStats(img_diff)

    # Draw the bounding boxes of 
    for i in range(1, num_labels):
        x = stats[i, cv.CC_STAT_LEFT]
        y = stats[i, cv.CC_STAT_TOP]
        w = stats[i, cv.CC_STAT_WIDTH]
        h = stats[i, cv.CC_STAT_HEIGHT]
        cv.rectangle(img1, (x, y), (x + w, y + h), (0, 255, 0), 2)

    path = rf"{out_dir}/img.{ext}"
    if cv.imwrite(path, img1):
        return (num_labels - 1, rf"{path}")
    else:
        return (num_labels - 1, "ERROR")