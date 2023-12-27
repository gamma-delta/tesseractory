class_name PlayerCamera extends Node

var player: Player

func rotor() -> GdRotor4:
  return GdRotor4.identity()

func movement(delta: float) -> Vector4:
  return Vector4.ZERO
