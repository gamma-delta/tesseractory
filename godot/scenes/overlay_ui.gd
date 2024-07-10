extends Control

func mark_paused(paused: bool):
  get_tree().get_nodes_in_group("ShowPaused").map(func (n): n.visible = paused)
  get_tree().get_nodes_in_group("ShowUnpaused").map(func (n): n.visible = !paused)

func set_debug_info(info: String):
  %DebugInfo.text = info
