class_name AxisCamera extends PlayerCamera

# Pretty sure that "no rotation" means that forward is +Y

var rot_yz: float = 0.0
var rot_xy: float = 0.0
var imaginary_axis_idx: int = 0

const AXES = [
  Vector4(0, 0, 0, 1), 
  Vector4(0, 1, 0, 0),
  Vector4(0, 0, 1, 0),
]

func imaginary_axis() -> Vector4:
  return AXES[self.imaginary_axis_idx]

func _process(delta: float) -> void:
  if Input.is_action_just_released("imaginary_turn_minus"):
    self.imaginary_axis_idx -= 1
  elif Input.is_action_just_released("imaginary_turn_plus"):
    self.imaginary_axis_idx += 1
  self.imaginary_axis_idx = posmod(self.imaginary_axis_idx, AXES.size())

func _physics_process(delta: float) -> void:
  var mouse := Input.get_last_mouse_velocity()
  var d_yz := -mouse.x * player.LOOK_SPEED / 100 * delta
  var d_xy := mouse.y * player.LOOK_SPEED / 100 * delta
  
  self.rot_yz = fposmod(self.rot_yz + d_yz, TAU)
  self.rot_xy = clampf(self.rot_xy + d_xy, -TAU / 4.0, TAU / 4.0)

func rotor() -> GdRotor4:
  var w2imag := GdRotor4.from_rotation_between(Vector4(0, 0, 0, 1), self.imaginary_axis())
  var local_yz := GdRotor4.from_angle_plane(self.rot_yz, GdBivec4.unit_yz())
  var local_xy := GdRotor4.from_angle_plane(self.rot_xy, GdBivec4.unit_xy())
  
  return w2imag.composed(local_yz).composed(local_xy)

func movement(delta: float) -> Vector4:
  var dv := Vector4(
    Input.get_axis("down", "up") * player.FLY_SPEED,
    Input.get_axis("back", "forward") * player.WALK_SPEED,
    Input.get_axis("right", "left") * player.WALK_SPEED,
    Input.get_axis("imaginary_minus", "imaginary_plus") * player.WALK_SPEED,
  ) * delta
  #dv.w = (Input.is_action_just_pressed("imaginary_plus") as int)\
  #  - (Input.is_action_just_pressed("imaginary_minus") as int)
  
  var non_x_movement := Vector4(0, 1, 1, 1) * dv
  var raw_move := GdRotor4.from_rotation_between(Vector4(0, 0, 0, 1), self.imaginary_axis())\
    .composed(GdRotor4.from_angle_plane(self.rot_yz, GdBivec4.unit_yz()))\
    .transform_vec(non_x_movement)
  raw_move.x = dv.x
  return raw_move
  #return non_x_movement
