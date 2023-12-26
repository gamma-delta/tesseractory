class_name PlayerCamera extends Node

func rotor(player: Player) -> GdRotor4:
  return GdRotor4.identity()

func transformed_movement(raw: Vector4) -> Vector4:
  return raw
