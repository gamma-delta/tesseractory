extends Node3D

@onready var tesser : TesseractoryWorldHandler = %TesseractoryWorldHandler
@onready var viewport : SubViewport = %ScreenViewport
@onready var screen : Control = %ScreenRender

@onready var world_ui : WorldUI = %WorldUI
@onready var pause_ui : Control = %PauseUI

# Called when the node enters the scene tree for the first time.
func _ready():
  Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
  get_window().connect("focus_entered", self.on_focus)
  get_window().connect("focus_exited", self.on_unfocus)

  self.viewport.size = self.tesser.viewport_size()
  (self.screen.material as ShaderMaterial).set_shader_parameter("tree", self.tesser.tree_tex())


func _process(_delta: float):
  world_ui.set_debug_info(tesser.debug_string())
  
  self.tesser.apply_per_tick_uniforms(self.screen.material)

func _input(event: InputEvent):
  if event.is_action_pressed("exit"):
    get_tree().quit()
  
  if event.is_action_pressed("pause"):
    var paused := get_tree().paused
    if paused:
      # Unpause
      pause_ui.hide()
      world_ui.show()
      Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
    else:
      # Pause
      pause_ui.show()
      world_ui.hide()
      Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
    get_tree().paused = !paused

func on_focus():
  if !get_tree().paused:
    Input.mouse_mode = Input.MOUSE_MODE_CAPTURED

func on_unfocus():
  Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
