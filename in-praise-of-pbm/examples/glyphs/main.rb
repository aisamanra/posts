class Image
  attr_reader :width, :height

  def initialize(width:, height:)
    @width = width
    @height = height
    @data = Array.new(width * height, 1)
  end

  def [](x, y)
    @data[x + y * @height]
  end

  def []=(x, y, r)
    @data[x + y * @height] = r
  end

  def scale(factor)
    img = Image.new(width: @width * factor, height: @height * factor)
    (0...@width).each do |x|
      (0...@height).each do |y|
        (0...factor).each do |dx|
          (0...factor).each do |dy|
            img[x*factor+dx,y*factor+dy] = self[x,y]
          end
        end
      end
    end
    img
  end

  def blit(img, at_x, at_y)
    (0...img.width).each do |x|
      (0...img.height).each do |y|
        self[at_x + x, at_y + y] = img[x, y]
      end
    end
  end

  def to_pbm
    buf = String.new
    buf << "P1\n"
    buf << "#{@width} #{@height}\n"
    @data.each {|x| buf << "#{x} "}
    buf
  end

  def self.glyph(size)
    img = Image.new(width: size, height: size)
    (0...size).each do |x|
      (0...size).each do |y|
        if x % 2 == 0 && y % 2 == 0 then
          img[x, y] = 0
        elsif (x % 2 == 0 || y % 2 == 0) && rand > 0.5 then
          img[x, y] = 0
        end
      end
    end
    img
  end
end

module Main
  def self.main
    glyphs_w = 4
    glyphs_h = 4
    size = 5
    img = Image.new(width: glyphs_w * (size + 2) + 2, height: glyphs_h * (size + 2) + 2)
    (0...glyphs_w).each do |x|
      (0...glyphs_h).each do |y|
        img.blit(Image.glyph(size), 2 + x * (size + 2), 2 + y * (size + 2))
      end
    end
    puts img.scale(20).to_pbm
  end
end

if $PROGRAM_NAME == __FILE__ then
  Main.main
end
