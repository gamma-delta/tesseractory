extends Node


# Called when the node enters the scene tree for the first time.
func _ready():
  Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
  get_window().connect("focus_entered", self.on_focus)
  get_window().connect("focus_exited", self.on_unfocus)

func _input(event: InputEvent):
  if event.is_action_pressed("exit"):
    get_tree().quit()

func on_focus():
  Input.mouse_mode = Input.MOUSE_MODE_CAPTURED

func on_unfocus():
  Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
