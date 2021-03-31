import "random" for Random

class GrayscaleImage {
  construct new(width, height, depth) {
    _width = width
    _height = height
    _depth = depth
    _image = []
    for (y in 0...height) {
      _image.add(List.filled(width, 0))
    }
  }

  pixel(x, y, shade) {
    _image[y][x] = shade
  }

// «rectangle»
  rectangle(x, y, width, height, shade) {
    // the two horizontal lines
    for (dx in 0..width) {
      pixel(x + dx, y,          shade)
      pixel(x + dx, y + height, shade)
    }

    // the two vertical lines
    for (dy in 0..height) {
      pixel(x,         y + dy, shade)
      pixel(x + width, y + dy, shade)
    }
  }
// «end»

  showPGM() {
    System.print("P2") // the PGM header
    System.print("%(_width) %(_height)")
    System.print(_depth) // the maximum value which will appear
    for (y in 0..._height) {
      for (x in 0..._width) {
        System.write("%(_image[y][x]) ")
      }
      System.print()
    }
  }
}

var rand = Random.new()

var width = 24
var height = 24
var depth = 8

// «main»
var image = GrayscaleImage.new(width, height, depth)
// create up to 6 rectangles
for (i in 0..rand.int(3, 6)) {
  // choose the color from the depth
  var color = rand.int(2, 8)
  // choose top-left point randomly
  var x = rand.int(0, width-3)
  var y = rand.int(0, height-3)
  // choose width and height from remaining
  var w = rand.int(x+2, width) - x
  var h = rand.int(y+2, height) - y
  // draw the rectangle
  image.rectangle(x, y, w, h, color)
}
image.showPGM()
// «end»
