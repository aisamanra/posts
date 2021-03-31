// «init»
var width = 3
var height = 3
var image = []

for (y in 0...height) {
  var row = []
  for (x in 0...width) {
    row.add(0)
  }
  image.add(row)
}
// «end»

// «checkers»
for (x in 0...width) {
  for (y in 0...height) {
    image[y][x] = (x + y) % 2
  }
}
// «end»

// «print»
System.print("P1")
System.print("%(width) %(height)")
for (y in 0...height) {
  for (x in 0...width) {
    System.write("%(image[y][x]) ")
  }
  System.print()
}
// «end»
