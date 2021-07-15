extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	#pause everything immedietly
	pause_shit()
	pass # Replace with function body.

#gets all nodes and pauses them
func pause_shit():
	get_tree().paused = true
	
#gets all nodes and unpauses them
func unpause_shit():
	get_tree().paused = false

#func _draw():
#	var color = Color.aqua
#	for particle in $playfield._return_positons():
#		draw_circle(particle, 1.0, color)

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	update()
