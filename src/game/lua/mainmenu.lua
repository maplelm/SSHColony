
return {
	init = function(self, engine)
		-- Build Menu
		-- This seems to have left out the actions that would be tied to these menu options
		self.menu = Menu({
			{ label = "Play" },
			{ label = "Settings" },
			{ label = "Quit" }
		})

		-- Draw the Menu
		self.menu:output(engine.render_tx)
		engine:send_render("Redraw")
	end,

	is_init = function(self)
		return true
	end,

	update = function(self, engine, dt, events)
		for _, event in ipairs(events) do
			if event.type == "keyboard" then
				if event.key == "q" then
					return engine:quit()
				elseif event.key == "up" or event.key == "w" then
					if self.menu:cursor_up(1) then
						self.menu:output(engine.render_tx)
					end
				elseif event.key == "right" or event.key == "d" then
					local selected = self.menu:get_selected_label()
					if selected == "Play" then
						engine:new_scene("LoadGame")
					elseif selected == "Settings" then
						engine:new_scene("Settings")
					elseif selected == "Quit" then
						engine:quit()
					end
				end
			end
		end
	end,
	
	resume = function(self, engine)
		engine:clear_screen()
		self.menu:output(engine.render_tx)
	end,

	suspend = function(self, engine)
		engine:clear_screen()
	end,

	is_paused = function(self)
		return false
	end,

	reset = function(self)
	end
}
