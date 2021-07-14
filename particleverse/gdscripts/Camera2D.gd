extends Camera2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var camerascale = zoom*zoom+Vector2(1,1)
var paused = true


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


func _input(event):
	if event.is_action_pressed("pause_time"):
		if self.paused== false:
			get_parent().pause_shit()
			self.paused = true
		else:
			get_parent().unpause_shit()
			self.paused = false

	if event.is_action_pressed("ui_up"):
		self.position += Vector2(0,-10)*camerascale
	if event.is_action_pressed("ui_down"):
		self.position += Vector2(0,10)*camerascale
	if event.is_action_pressed("ui_left"):
		self.position += Vector2(-10,0)*camerascale
	if event.is_action_pressed("ui_right"):
		self.translate(Vector2(10,0)*camerascale)

	if event.is_action_pressed("scroll_up"):
		print("zoom in")
		_zoom_camera(-1)
	# Wheel Down Event
	elif event.is_action_pressed("scroll_down"):
		print("zoom out")
		_zoom_camera(1)

		
# Zoom Camera
func _zoom_camera(dir):
	if self.zoom >= Vector2(.001,.001):
		self.zoom += Vector2(.01, .01) * dir

	else:
		self.zoom += Vector2(0.001,0.001)



func _on_Timer_timeout():
	pass # Replace with function body.
