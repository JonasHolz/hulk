function spawn_robot(number)
  table.insert(state.robots, create_robot(number))
end

spawn_robot(7)

function on_cycle()
  if state.cycle_count == 100 then
    state.game_controller_state.game_state = "Ready"
  end

  if state.cycle_count == 1600 then
    state.game_controller_state.game_state = "Set"
    state.ball = {
      position = { 2.0, 0.0 },
      velocity = { 0.0, 0.0 },
    }
  end

  if state.cycle_count == 1700 then
    state.game_controller_state.game_state = "Playing"
  end

  if state.cycle_count == 1800 then
    state.ball.velocity = { -3.0, 0.7 }
  end

  if state.cycle_count == 1900 then
    state.finished = true
  end
end
