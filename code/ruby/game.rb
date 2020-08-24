require 'matrix'

class Unit
  attr_accessor :val, :nextVal, :x, :y
  
  def initialize(x, y, val, game)
    @x = x
    @y = y
    @val = val
    @game = game
    @nextVal = val
  end
  
end


class Game
  def initialize( rows:, cols: )
    @grid = Matrix.build( rows, cols )
    @grid = @grid.each_with_index do |unit|
      x, y = *unit
      Unit.new( x,y,[0,1].sample, self )
    end
  end

  def showGeneration
    @grid.to_a.each do |r|
       puts r.map { |p| (p.val==1)? 'o': ' '}.join(" ")
    end
  end

  def apply_rules
    @grid.to_a.each do |row|
      row.each do |single|
        neighs = []
        for i in -1..1
          for j in -1..1
            neighX = single.x + i
            neighY = single.y + j
            if neighX == single.x and neighY == single.y then 
              next
            end
            if neighX < 0 or neighY < 0 then
              next
            end
            unless ( @grid[neighX, neighY].nil? or @grid[neighX, neighY].val == 0 ) then
              neighs.push( @grid[neighX, neighY] )
            end
          end
        end
        n_count = neighs.count
        # Rule 1 - Lonely
        if single.val == 1 && n_count < 2
          single.nextVal = 0 
        end
        # Rule 2 - Overcrowded
        if single.val == 1 && n_count > 3
          single.nextVal = 0 
        end
        # Rule 3 - Lives
        if single.val == 1 && ( n_count == 2 or n_count == 3 )
          single.nextVal = 1 
        end
        # Rules 4 - It takes three to give birth!
        if single.val == 0 && n_count == 3
          single.nextVal = 1 
        end
        # Rule 5 - Rest all, if it was dead, it stays dead.   
      end
    end

    @grid.to_a.each do |row|
      row.each do |single|
        single.val = single.nextVal
      end
    end

  end

end 


class GOL
  def initialize(game)
    @game = game
    @iterations = 250 # default
    @game.showGeneration
  end

  def play
    for i in 1..@iterations do
      puts "\e[H\e[2J"
      @game.apply_rules
      @game.showGeneration
      print("Generation ",i,"\n")
      print("... < enter x to exit > ...")
      # sleep(0.1)
      nextInput = gets.chomp
      if nextInput=='x' then break end
    end
  end
end

#Plays from here.
game = GOL.new(Game.new( rows:20, cols:20)).play