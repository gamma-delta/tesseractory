class_name Player extends Node

@export var WALK_SPEED := 0.0
@export var FLY_SPEED := 0.0
@export var LOOK_SPEED := 0.0

@export var camera: PlayerCamera

var position: Vector4 = Vector4.ZERO

func _ready():
  self.camera.player = self

func _process(delta: float) -> void:
  %TesseractoryGodotBridge.render_from(self.position, self.camera.rotor())

func _physics_process(delta: float) -> void:
  self.position += self.camera.movement(delta)
