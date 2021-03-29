// «init»
var width = 24
var height = 24
var depth = 24
var image = []

for (y in 0...height) {
  var row = []
  for (x in 0...width) {
    row.add({"r": 0, "g": 0, "b": 0})
  }
  image.add(row)
}
// «end»

// «checkers»
for (x in 0...width) {
  for (y in 0...height) {
    image[y][x]["r"] = ((x / width) * depth).floor
    image[y][x]["g"] = ((y / height) * depth).floor
    image[y][x]["b"] = ((x + y) % 2) * depth
  }
}
// «end»

// «print»
System.print("P3")
System.print("%(width) %(height)")
System.print(depth)
for (y in 0...height) {
  for (x in 0...width) {
    var p = image[y][x]
    System.write("%(p["r"]) %(p["g"]) %(p["b"]) ")
  }
  System.print()
}
// «end»
