var width = 12
var height = 12
var image = []

for (y in 0...height) {
  var row = []
  for (x in 0...width) {
    row.add(0)
  }
  image.add(row)
}

// «checkers»
for (x in 0...width) {
  for (y in 0...height) {
    image[y][x] = (x + y) % 4
  }
}
// «end»

// «print»
System.print("P2") // the PGM header
System.print("%(width) %(height)")
System.print(3) // the maximum value which will appear
for (y in 0...height) {
  for (x in 0...width) {
    System.write("%(image[y][x]) ")
  }
  System.print()
}
// «end»
