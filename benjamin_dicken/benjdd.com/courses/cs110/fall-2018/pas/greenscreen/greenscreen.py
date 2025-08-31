###
### Author: ?
### Description: ?
###

def get_image_dimensions_string(file_name):
    '''
    Given the file name for a valid PPM file, this function will return the
    image dimensions as a string. For example, if the image stored in the
    file is 150 pixels wide and 100 pixels tall, this function should return
    the string '150 100'.
    file_name: A string. A PPM file name.
    '''
    image_file = open(file_name, 'r')
    image_file.readline()
    return image_file.readline().strip('\n')

def load_image_pixels(file_name):
    ''' Load the pixels from the image saved in the file named file_name.
    The pixels will be stored in a 3d list, and the 3d list will be returned.
    Each list in the outer-most list are the rows of pixels.
    Each list within each row represents and individual pixel.
    Each pixels is representd by a list of three ints, which are the RGB values of that pixel.
    '''
    pixels = []
    image_file = open(file_name, 'r')

    image_file.readline()
    image_file.readline()
    image_file.readline()

    width_height = get_image_dimensions_string(file_name)
    width_height = width_height.split(' ')
    width = int(width_height[0])
    height = int(width_height[1])

    for line in image_file:
        rgb_row = line.split(' ')
        row = []
        for i in range(0, len(rgb_row), 3):
            pixel = [int(rgb_row[i]), int(rgb_row[i+1]), int(rgb_row[i+2])]
            row.append(pixel)
        pixels.append(row)

    return pixels

def main():

    # Get the 5 input values from the user, as described in the PA specification
    # These input values will be validated later in main
    channel = input('Enter color channel:\n')
    channel_difference = input('Enter color channel difference:\n')
    gs_file = input('Enter greenscreen image file name:\n')
    fi_file = input('Enter fill image file name:\n')
    out_file = input('Enter output file name:\n')
    
    # Next, Do some valiation of the input values
    # The PA specification tell's you what you need to validate
    
    # If the the input is valid, implement the greenscreen.
    # You should:
    #    * Load in the image data from the two input image files.
    #      Use the provided load_image_pixels functions for this!
    #    * Generate a NEW image based on the two input values,
    #      using the greenscreen algorithm described in the specification
    #    * Save the newly-generated image to a file
    # You probably will want to create other function(s) that you call from here.

main()

