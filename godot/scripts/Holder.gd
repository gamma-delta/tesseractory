extends Node

@onready var tesser : TesseractoryGodotBridge = %TesseractoryGodotBridge
#@onready var viewport : SubViewport = %ScreenViewport
@onready var screen : Control = %ScreenRender

@onready var player : Player = %Player

# Called when the node enters the scene tree for the first time.
func _ready():
  Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
  get_window().connect("focus_entered", self.on_focus)
  get_window().connect("focus_exited", self.on_unfocus)

  #self.viewport.size = self.tesser.viewport_size()                                       
  (self.screen.material as ShaderMaterial).set_shader_parameter("TREE_TEXTURE", self.tesser.tree_tex())

func _process(_delta: float):
  %OverlayUi.set_debug_info(tesser.debug_string())
  self.apply_shader_params()
  %TesseractoryGodotBridge.upload_foxels(GdPlayerCamera.make(
    self.player.position, self.player.rotation(), self.player.FOV, self.player.FOCAL_DIST))

func apply_shader_params() -> void:
  var shader := self.screen.material as ShaderMaterial
  shader.set_shader_parameter("playerPos", self.player.position)
  var look := self.player.rotation()
  shader.set_shader_parameter("playerLookRaw", look.splat_to_array())
  
  shader.set_shader_parameter("focalDist", self.player.FOCAL_DIST)
  shader.set_shader_parameter("fov", self.player.FOV)
  
  shader.set_shader_parameter("aspectRatio", 1000.0 / 600.0)

func _input(event: InputEvent):
  if event.is_action_pressed("exit"):
    get_tree().quit()
  
  if event.is_action_pressed("pause"):
    var paused := get_tree().paused
    if paused:
      # unpause
      Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
    else:
      # Pause
      Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
    get_tree().paused = !paused
    %OverlayUi.mark_paused(!paused)

func on_focus():
  if !get_tree().paused:
    Input.mouse_mode = Input.MOUSE_MODE_CAPTURED

func on_unfocus():
  Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
