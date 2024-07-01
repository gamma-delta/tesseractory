class_name Player extends Node

@export var WALK_SPEED := 0.0
@export var FLY_SPEED := 0.0
@export var LOOK_SPEED := 0.0
@export var FOCAL_DIST := 0.0
@export var FOV := 0.0

@export var camera: PlayerCamera

var position: Vector4 = Vector4.ZERO

func _ready():
  self.camera.player = self

func _process(delta: float) -> void:
  pass

func _physics_process(delta: float) -> void:
  self.position += self.camera.movement(delta)

func rotation() -> GdRotor4:
  return self.camera.rotor()
