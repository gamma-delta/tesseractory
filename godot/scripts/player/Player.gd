class_name Player extends Node

@export var WALK_SPEED := 0.0
@export var FLY_SPEED := 0.0
@export var LOOK_SPEED := 0.0

var camera: PlayerCamera = PlayerCamera.new()
var position: Vector4 = Vector4.ZERO

func _process(delta: float) -> void:
  %TesseractoryGodotBridge.render_from(self.position, self.camera.rotor(self))

func _physics_process(delta: float) -> void:
  var dv := Vector4(
    Input.get_axis("down", "up") * FLY_SPEED * delta,
    Input.get_axis("back", "forward") * WALK_SPEED * delta,
    Input.get_axis("right", "left") * WALK_SPEED * delta,
    Input.get_axis("imaginary_minus", "imaginary_plus")
  )
  self.position += self.camera.transformed_movement(dv)
