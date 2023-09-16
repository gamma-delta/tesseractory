extends Node3D

@onready var tesser : TesseractoryWorldHandler = $TesseractoryWorldHandler
@onready var screen : TextureRect = $TextureRect
@onready var label : Label = $TextureRect/Label

# Called when the node enters the scene tree for the first time.
func _ready():
  self.screen.texture = self.tesser.get_canvas_tex()

func _process(_delta: float):
  self.label.text = self.tesser.debug_string()
